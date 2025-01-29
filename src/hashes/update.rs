use base64::{prelude::BASE64_STANDARD, Engine};
use iced::{clipboard, widget::text_editor, Task};
use pallas::crypto::hash::Hasher;
use regex::Regex;
use sha2::Digest;

use super::{Encoding, State};

#[derive(Debug, Clone)]
pub enum Message {
    ContentsChanged(text_editor::Action),
    EncodingSet(Encoding),
    CopyHash(String),
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        use Message::*;
        match message {
            ContentsChanged(action) => {
                self.warning = None;
                self.contents.perform(action);
                self.update_hashes();
                Task::none()
            }
            EncodingSet(enc) => {
                self.warning = None;
                self.encoding = Some(enc);
                self.update_hashes();
                Task::none()
            }
            CopyHash(s) => clipboard::write(s.clone()),
        }
    }

    fn update_hashes(&mut self) {
        let contents = self.text();

        if contents.is_empty() {
            self.clear_hashes();
            self.warning = Some("Empty String".to_string());
            return;
        }

        let contents = match self.encoding.clone().unwrap_or(detect_encoding(&contents)) {
            Encoding::Hex => match hex::decode(contents.replace('\n', "")) {
                Ok(raw) => raw,
                Err(e) => {
                    self.warning = Some(format!("Invalid hex: {}", e));
                    self.clear_hashes();
                    return;
                }
            },
            Encoding::Base64 => match BASE64_STANDARD.decode(contents.replace('\n', "")) {
                Ok(raw) => raw,
                Err(e) => {
                    self.warning = Some(format!("Invalid base64: {}", e));
                    self.clear_hashes();
                    return;
                }
            },
            Encoding::UTF8 => contents.into_bytes(),
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

pub fn detect_encoding(s: impl AsRef<str>) -> Encoding {
    if looks_hex(&s) {
        Encoding::Hex
    } else if looks_base64(&s) {
        Encoding::Base64
    } else {
        Encoding::UTF8
    }
}

pub fn looks_base64(s: impl AsRef<str>) -> bool {
    let s = s.as_ref().replace('\n', "");
    Regex::new(r"^([A-Za-z0-9+/]{4})*([A-Za-z0-9+/]{3}=|[A-Za-z0-9+/]{2}==)?$")
        .unwrap()
        .is_match(&s)
}

pub fn looks_hex(s: impl AsRef<str>) -> bool {
    let s = s.as_ref().replace('\n', "");
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
