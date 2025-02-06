use iced::{
    widget::{button, column, pick_list, row, text, text_editor, Space},
    Element,
    Length::{Fill, FillPortion, Shrink},
    Theme,
};

use crate::{
    copyable_text::copyable_text,
    encoding::{detect_encoding, Encoding},
};

use super::{Message, State};

impl State {
    pub fn view(&self) -> Element<Message> {
        let message = self.text_message();
        let key = self.text_key();
        row![
            column![
                row![
                    "Encoding?",
                    Space::new(10, Fill),
                    pick_list(
                        &Encoding::ALL[..],
                        Some(self.encoding.clone().unwrap_or(detect_encoding(message))),
                        Message::EncodingSet,
                    )
                ]
                .height(Shrink),
                text_editor(&self.message)
                    .placeholder("paste your message...")
                    .on_action(Message::ContentsChanged)
                    .height(Fill)
                    .wrapping(text::Wrapping::Glyph)
                    .style(|theme: &Theme, status| {
                        let mut defaults = text_editor::default(theme, status);

                        defaults.border.radius = 4.0.into();

                        defaults
                    }),
                row![
                    "Encoding?",
                    Space::new(10, Fill),
                    pick_list(
                        &Encoding::ALL[..],
                        Some(self.encoding.clone().unwrap_or(detect_encoding(key))),
                        Message::EncodingSet,
                    )
                ]
                .height(Shrink),
                text_editor(&self.private_key)
                    .placeholder("paste your key...")
                    .on_action(Message::KeyChanged)
                    .height(Fill)
                    .wrapping(text::Wrapping::Glyph)
                    .style(|theme: &Theme, status| {
                        let mut defaults = text_editor::default(theme, status);

                        defaults.border.radius = 4.0.into();

                        defaults
                    }),
                button("Generate Key").on_press(Message::GenerateKey),
            ]
            .width(FillPortion(1))
            .padding(5),
            column![
                row![
                    "Ed25519 Pub: ",
                    copyable_text(&self.ed25519_pub, Message::CopyHash)
                ]
                .wrap(),
                row![
                    "Ed25519 Sig: ",
                    copyable_text(&self.ed25519_sig, Message::CopyHash)
                ]
                .wrap(),
                row![
                    "EcdsaSecp256k1 Pub: ",
                    copyable_text(&self.ecdsa_secp256k1_pub, Message::CopyHash)
                ]
                .wrap(),
                row![
                    "EcdsaSecp256k1 Sig: ",
                    copyable_text(&self.ecdsa_secp256k1_sig, Message::CopyHash)
                ]
                .wrap(),
                row![
                    "SchnorrSecp256k1: ",
                    copyable_text(&self.schnorr_secp256k1, Message::CopyHash)
                ]
                .wrap(),
            ]
            .push_maybe(
                self.warning
                    .as_ref()
                    .map(|warning| text(warning).color([1.0, 0.0, 0.0]))
            )
            .padding(5)
            .width(FillPortion(3))
        ]
        .into()
    }
}
