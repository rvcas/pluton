use iced::{
    widget::{column, row, text, text_editor, toggler, Space},
    Element,
    Length::{Fill, FillPortion, Shrink},
    Theme,
};

use super::{looks_hex, Message, State};
use crate::copyable_text::copyable_text;

impl State {
    pub fn view(&self) -> Element<Message> {
        row![
            column![
                row![
                    "Hex?",
                    Space::new(10, Fill),
                    toggler(self.is_hex.unwrap_or(looks_hex(self.contents.text())))
                        .on_toggle(Message::HexSet),
                    if looks_hex(self.contents.text()) {
                        "Yes"
                    } else {
                        "No"
                    }
                ]
                .height(Shrink),
                text_editor(&self.contents)
                    .placeholder("paste some text...")
                    .on_action(Message::ContentsChanged)
                    .height(Fill)
                    .wrapping(text::Wrapping::Glyph)
                    .style(|theme: &Theme, status| {
                        let mut defaults = text_editor::default(theme, status);

                        defaults.border.radius = 4.0.into();

                        defaults
                    }),
            ]
            .width(FillPortion(1))
            .padding(5),
            column![
                row![
                    "Blake2b 224: ",
                    copyable_text(&self.blake2b_224, Message::CopyHash)
                ]
                .wrap(),
                row![
                    "Blake2b 256: ",
                    copyable_text(&self.blake2b_256, Message::CopyHash)
                ]
                .wrap(),
                row!["SHA-512: ", copyable_text(&self.sha512, Message::CopyHash)].wrap(),
                row!["SHA-256: ", copyable_text(&self.sha256, Message::CopyHash)].wrap(),
            ]
            .push_maybe(
                self.warning
                    .as_ref()
                    .map(|warning| { text(warning).color([1.0, 0.0, 0.0]) })
            )
            .padding(5)
            .width(FillPortion(3))
        ]
        .into()
    }
}
