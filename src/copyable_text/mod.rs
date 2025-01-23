use iced::{
    color,
    widget::{button, container, row, text},
    Element,
    Length::Fill,
};
use iced_font_awesome::fa_icon_solid;

pub fn copyable_text<Message>(
    content: &str,
    on_copy: impl Fn(String) -> Message + 'static,
) -> Element<Message>
where
    Message: Clone + 'static,
{
    row![
        container(text(content).size(16).width(Fill).center())
            .style(container::bordered_box)
            .center_y(24),
        button(
            fa_icon_solid("clone")
                .size(14.)
                .color(color!(255, 255, 255))
        )
        .on_press(on_copy(content.to_string()))
        .style(button::text)
        .padding(5)
    ]
    .spacing(10)
    .into()
}
