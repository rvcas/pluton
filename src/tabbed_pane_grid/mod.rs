use iced::{
    widget::{button, column, container, pane_grid, row, text, PaneGrid, Space},
    Center, Color, Element,
    Length::Fill,
    Task, Theme,
};

use crate::{block_inspector, hashes};
use iced_font_awesome::fa_icon_solid;

pub struct State {
    focus: Option<pane_grid::Pane>,
    panes: pane_grid::State<Pane>,
}

impl Default for State {
    fn default() -> Self {
        let (panes, focus) = pane_grid::State::new(Pane::new(Tool::Select));
        Self {
            focus: Some(focus),
            panes,
        }
    }
}

struct Pane {
    pinned: bool,
    content: Tool,
}

impl Pane {
    fn new(content: Tool) -> Self {
        Pane {
            pinned: false,
            content,
        }
    }
}

#[derive(Debug)]
pub enum Tool {
    Select,
    BlockInspector(block_inspector::State),
    Hashes(hashes::State),
}

impl Tool {
    fn title(&self) -> &str {
        match self {
            Tool::Select => "Select",
            Tool::BlockInspector(_) => "Block Inspector",
            Tool::Hashes(_) => "Hashes",
        }
    }
}

#[derive(Debug, Clone)]
pub enum ToolMessage {
    SelectTool(fn() -> Tool),
    BlockInspector(block_inspector::Message),
    Hashes(hashes::Message),
}

#[expect(dead_code)] // The remaining things can be bound to hotkeys eventually
#[derive(Debug, Clone)]
pub enum Message {
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    TogglePin(pane_grid::Pane),
    Maximize(pane_grid::Pane),
    Restore,
    Close(pane_grid::Pane),
    CloseFocused,
    Dispatch(pane_grid::Pane, ToolMessage),
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        use Message::*;
        match message {
            Message::Split(axis, pane) => {
                let result = self.panes.split(axis, pane, Pane::new(Tool::Select));

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }
            }
            Message::SplitFocused(axis) => {
                if let Some(pane) = self.focus {
                    let result = self.panes.split(axis, pane, Pane::new(Tool::Select));

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }
                }
            }
            Message::FocusAdjacent(direction) => {
                if let Some(pane) = self.focus {
                    if let Some(adjacent) = self.panes.adjacent(pane, direction) {
                        self.focus = Some(adjacent);
                    }
                }
            }
            Message::Clicked(pane) => {
                self.focus = Some(pane);
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
            }
            Message::Dragged(_) => {}
            Message::TogglePin(pane) => {
                if let Some(Pane { pinned, .. }) = self.panes.get_mut(pane) {
                    *pinned = !*pinned;
                }
            }
            Message::Maximize(pane) => self.panes.maximize(pane),
            Message::Restore => {
                self.panes.restore();
            }
            Message::Close(pane) => {
                if self.panes.len() == 1 {
                    self.panes.get_mut(pane).unwrap().content = Tool::Select;
                } else if let Some((_, sibling)) = self.panes.close(pane) {
                    self.focus = Some(sibling);
                }
            }
            Message::CloseFocused => {
                if let Some(pane) = self.focus {
                    if let Some(Pane { pinned, .. }) = self.panes.get(pane) {
                        if !pinned {
                            if let Some((_, sibling)) = self.panes.close(pane) {
                                self.focus = Some(sibling);
                            }
                        }
                    }
                }
            }
            Dispatch(pane, message) => {
                let pane_state = self.panes.panes.get_mut(&pane);
                if pane_state.is_none() {
                    return Task::none();
                }
                let pane_state = pane_state.unwrap();
                use ToolMessage::*;
                match (&mut pane_state.content, message) {
                    (Tool::Select, SelectTool(content_fn)) => {
                        pane_state.content = content_fn();
                    }
                    (Tool::BlockInspector(state), BlockInspector(message)) => {
                        return state.update(message).map(dispatch(pane, BlockInspector));
                    }
                    (Tool::Hashes(state), Hashes(m)) => {
                        return state.update(m).map(dispatch(pane, Hashes));
                    }
                    _ => {}
                }
            }
        };
        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        let total_panes = self.panes.len();

        let pane_grid = PaneGrid::new(&self.panes, |id, pane, is_maximized| {
            let pin_button = button(
                if pane.pinned {
                    fa_icon_solid("thumbtack-slash")
                } else {
                    fa_icon_solid("thumbtack")
                }
                .size(14.),
            )
            .on_press(Message::TogglePin(id))
            .padding(3);
            let title_text = text(pane.content.title());
            let title = row![]
                .push_maybe(if total_panes > 1 {
                    Some(pin_button)
                } else {
                    None
                })
                .push_maybe(Some(title_text))
                .spacing(5);

            let title_bar = pane_grid::TitleBar::new(title)
                .controls(pane_grid::Controls::dynamic(
                    view_controls(id, total_panes, pane.pinned, is_maximized),
                    button(text("X").size(14))
                        .style(button::danger)
                        .padding(3)
                        .on_press_maybe(if !pane.pinned {
                            Some(Message::Close(id))
                        } else {
                            None
                        }),
                ))
                .padding(10);

            pane_grid::Content::new(view_content(id, &pane.content)).title_bar(title_bar)
        })
        .width(Fill)
        .height(Fill)
        .spacing(10)
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);

        column![
            Space::new(Fill, 20),
            container(pane_grid).width(Fill).height(Fill).padding(10)
        ]
        .into()
    }
}

fn dispatch<M>(pane: pane_grid::Pane, ctor: fn(m: M) -> ToolMessage) -> impl Fn(M) -> Message {
    move |m: M| -> Message { Message::Dispatch(pane, ctor(m)) }
}

fn view_content(id: pane_grid::Pane, tool: &Tool) -> Element<'_, Message> {
    let tool_button = |icon: &'static str, name: &'static str, tool: fn() -> Tool| -> Element<_> {
        container(
            button(
                row![column![
                    fa_icon_solid(icon).color(Color::WHITE).size(50.),
                    Space::new(Fill, 15),
                    text(name).align_x(Center).size(14)
                ]
                .align_x(Center)
                .width(Fill)]
                .align_y(Center)
                .height(Fill),
            )
            .style(|theme: &Theme, status| {
                let mut defaults = button::secondary(theme, status);

                defaults.border.radius = 4.0.into();

                defaults
            })
            .padding(5)
            .width(110)
            .height(110)
            .on_press(Message::Dispatch(id, ToolMessage::SelectTool(tool))),
        )
        .padding(5)
        .into()
    };
    use ToolMessage::*;
    match tool {
        Tool::Select => container(row![
            tool_button("cube", "Block Inspector", || Tool::BlockInspector(
                block_inspector::State::default()
            )),
            tool_button("hashtag", "Hashes", || Tool::Hashes(
                hashes::State::default()
            )),
        ])
        .center(Fill)
        .width(Fill)
        .height(Fill)
        .into(),
        Tool::BlockInspector(state) => state.view().map(dispatch(id, BlockInspector)),
        Tool::Hashes(state) => state.view().map(dispatch(id, Hashes)),
    }
}

fn view_controls<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    is_maximized: bool,
) -> Element<'a, Message> {
    let row = row![].spacing(5).push_maybe(if total_panes > 1 {
        let (content, message) = if is_maximized {
            (fa_icon_solid("minimize"), Message::Restore)
        } else {
            (fa_icon_solid("maximize"), Message::Maximize(pane))
        };

        Some(
            button(content.size(14.))
                .style(button::secondary)
                .padding(3)
                .on_press(message),
        )
    } else {
        None
    });

    let split_vertical = button(fa_icon_solid("grip-lines-vertical").size(14.))
        .padding(3)
        .on_press(Message::Split(pane_grid::Axis::Vertical, pane));
    let split_horizontal = button(fa_icon_solid("grip-lines").size(14.))
        .padding(3)
        .on_press(Message::Split(pane_grid::Axis::Horizontal, pane));

    let close = button(fa_icon_solid("xmark").size(14.))
        .style(button::danger)
        .padding(3)
        .on_press_maybe(if !is_pinned {
            Some(Message::Close(pane))
        } else {
            None
        });

    row.push(split_vertical)
        .push(split_horizontal)
        .push(close)
        .into()
}
