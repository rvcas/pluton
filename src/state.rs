use crate::tabbed_pane_grid;

#[derive(Default)]
pub struct State {
    pub window_size: iced::Size,
    pub workspace: tabbed_pane_grid::State,
}
