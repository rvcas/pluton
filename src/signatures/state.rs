use iced::widget::text_editor;

use crate::encoding::Encoding;

#[derive(Debug)]
pub struct State {
    pub message: text_editor::Content,
    pub private_key: text_editor::Content,
    pub encoding: Option<Encoding>,
    pub warning: Option<String>,
    pub ed25519_pub: String,
    pub ed25519_sig: String,
    pub ecdsa_secp256k1_pub: String,
    pub ecdsa_secp256k1_sig: String,
    pub schnorr_secp256k1: String,
}

impl State {
    // Temporary, until text_editor can return unmodified contents
    pub fn text_message(&self) -> String {
        self.message
            .lines()
            .enumerate()
            .fold(String::new(), |mut contents, (i, line)| {
                if i > 0 {
                    contents.push('\n');
                }
                contents.push_str(&line);
                contents
            })
    }

    pub fn text_key(&self) -> String {
        self.private_key
            .lines()
            .enumerate()
            .fold(String::new(), |mut contents, (i, line)| {
                if i > 0 {
                    contents.push('\n');
                }
                contents.push_str(&line);
                contents
            })
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            message: text_editor::Content::default(),
            private_key: text_editor::Content::default(),
            encoding: None,
            warning: None,
            ed25519_pub: "".to_string(),
            ed25519_sig: "".to_string(),
            ecdsa_secp256k1_pub: "".to_string(),
            ecdsa_secp256k1_sig: "".to_string(),
            schnorr_secp256k1: "".to_string(),
        }
    }
}
