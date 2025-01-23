use iced::{
    widget::{column, row, text, text_editor, toggler, Space},
    Element,
    Length::{Fill, Shrink},
    Theme,
};

use super::{Message, State};
use crate::copyable_text::copyable_text;

impl State {
    pub fn view(&self) -> Element<Message> {
        column![
            row![
                "Hash: ",
                copyable_text(&self.hash, Message::CopyHash),
                Space::new(Fill, Shrink),
                "256",
                toggler(self.hash_length == 256).on_toggle(|t| Message::LengthChanged(if t {
                    256
                } else {
                    224
                })),
                "224"
            ],
            Space::new(Fill, 10),
            text_editor(&self.hex_contents)
                .placeholder("hex contents...")
                .on_action(Message::HexChanged)
                .height(Fill)
                .wrapping(text::Wrapping::Glyph)
                .style(|theme: &Theme, status| {
                    let mut defaults = text_editor::default(theme, status);

                    defaults.border.radius = 4.0.into();

                    defaults
                }),
        ]
        .into()
    }
}
