use iced::{widget::text_editor, Task};
use pallas::crypto::hash::Hasher;

use super::State;

#[derive(Debug, Clone)]
pub enum Message {
    HexChanged(text_editor::Action),
    LengthChanged(usize),
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::HexChanged(action) => {
                self.hex_contents.perform(action);
                self.update_hash();
                Task::none()
            }
            Message::LengthChanged(l) => {
                self.hash_length = l;
                self.update_hash();
                Task::none()
            }
        }
    }

    fn update_hash(&mut self) {
        if self.hex_contents.text().trim().len() == 0 {
            return;
        }
        let contents = match hex::decode(self.hex_contents.text().trim()) {
            Ok(h) => h,
            Err(_) => {
                self.hash = "".to_string();
                return;
            }
        };
        self.hash = match self.hash_length {
            224 => hex::encode(Hasher::<224>::hash(&contents)),
            256 => hex::encode(Hasher::<256>::hash(&contents)),
            _ => return,
        };
    }
}
