use iced::{clipboard, widget::text_editor, Task};
use pallas::crypto::hash::Hasher;
use sha2::Digest;

use super::State;

#[derive(Debug, Clone)]
pub enum Message {
    ContentsChanged(text_editor::Action),
    HexSet(bool),
    CopyHash(String),
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        use Message::*;
        match message {
            ContentsChanged(action) => {
                self.contents.perform(action);
                self.update_hashes();
                self.warning = None;
                Task::none()
            }
            HexSet(is_hex) => {
                self.is_hex = Some(is_hex);
                self.update_hashes();
                self.warning = None;
                Task::none()
            }
            CopyHash(s) => clipboard::write(s.clone()),
        }
    }

    fn update_hashes(&mut self) {
        let mut hex_contents = self.contents.text().clone();
        hex_contents.truncate(hex_contents.len() - 1);
        if hex_contents.len() == 0 {
            self.clear_hashes();
            self.warning = Some("Empty String".to_string());
            return;
        }
        let contents = if self.is_hex.unwrap_or(looks_hex(&hex_contents)) {
            match hex::decode(&hex_contents) {
                Ok(h) => h,
                Err(err) => {
                    self.warning = Some(format!("Invalid hex: {}", err));
                    self.clear_hashes();
                    return;
                }
            }
        } else {
            hex_contents.into_bytes()
        };
        self.blake2b_224 = hex::encode(Hasher::<224>::hash(&contents));
        self.blake2b_256 = hex::encode(Hasher::<256>::hash(&contents));
        let mut sha512 = sha2::Sha512::new();
        sha512.update(&contents);
        self.sha512 = hex::encode(sha512.finalize());
        let mut sha256 = sha2::Sha256::new();
        sha256.update(&contents);
        self.sha256 = hex::encode(sha256.finalize());
    }

    fn clear_hashes(&mut self) {
        self.blake2b_224 = "".to_string();
        self.blake2b_256 = "".to_string();
        self.sha512 = "".to_string();
        self.sha256 = "".to_string();
    }
}

pub fn looks_hex(s: impl AsRef<str>) -> bool {
    let s = s.as_ref();
    if s.len() % 2 != 0 {
        return false;
    }
    for c in s.chars() {
        if !c.is_ascii_hexdigit() {
            return false;
        }
    }
    true
}
