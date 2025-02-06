use base64::{prelude::BASE64_STANDARD, Engine};
use cryptoxide::ed25519::SIGNATURE_LENGTH;
use iced::{clipboard, widget::text_editor, Task};

use crate::encoding::{detect_encoding, Encoding};

use super::state::State;

#[derive(Debug, Clone)]
pub enum Message {
    ContentsChanged(text_editor::Action),
    KeyChanged(text_editor::Action),
    EncodingSet(Encoding),
    CopyHash(String),
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        use Message::*;
        match message {
            ContentsChanged(action) => {
                self.warning = None;
                self.message.perform(action);

                self.update_signatures();
                Task::none()
            }
            KeyChanged(action) => {
                self.warning = None;
                self.private_key.perform(action);
                self.update_signatures();
                Task::none()
            }
            EncodingSet(enc) => {
                self.warning = None;
                self.encoding = Some(enc);
                self.update_signatures();
                Task::none()
            }
            CopyHash(s) => clipboard::write(s.clone()),
        }
    }

    fn update_signatures(&mut self) {
        let message = self.text_message();
        let private_key = self.text_key();

        if message.is_empty() || private_key.is_empty() {
            self.clear_signatures();
            self.warning = Some("Empty String".to_string());
            return;
        }

        let message = match self.encoding.clone().unwrap_or(detect_encoding(&message)) {
            Encoding::Hex => match hex::decode(message.replace('\n', "")) {
                Ok(raw) => raw,
                Err(e) => {
                    self.warning = Some(format!("Invalid hex: {}", e));
                    self.clear_signatures();
                    return;
                }
            },
            Encoding::Base64 => match BASE64_STANDARD.decode(message.replace('\n', "")) {
                Ok(raw) => raw,
                Err(e) => {
                    self.warning = Some(format!("Invalid base64: {}", e));
                    self.clear_signatures();
                    return;
                }
            },
            Encoding::UTF8 => message.into_bytes(),
        };

        let private_key = match self
            .encoding
            .clone()
            .unwrap_or(detect_encoding(&private_key))
        {
            Encoding::Hex => match hex::decode(private_key.replace('\n', "")) {
                Ok(raw) => raw,
                Err(e) => {
                    self.warning = Some(format!("Invalid hex: {}", e));
                    self.clear_signatures();
                    return;
                }
            },
            Encoding::Base64 => match BASE64_STANDARD.decode(private_key.replace('\n', "")) {
                Ok(raw) => raw,
                Err(e) => {
                    self.warning = Some(format!("Invalid base64: {}", e));
                    self.clear_signatures();
                    return;
                }
            },
            Encoding::UTF8 => private_key.into_bytes(),
        };

        match sign_with_edd25519(private_key, message) {
            Ok(sig) => {
                self.edd25519 = hex::encode(sig);
            }
            Err(warn) => self.warning = Some(warn),
        }

        // self.blake2b_256 = hex::encode(Hasher::<256>::hash(&contents));

        // let mut sha512 = sha2::Sha512::new();

        // sha512.update(&contents);

        // self.sha512 = hex::encode(sha512.finalize());

        // let mut sha256 = sha2::Sha256::new();

        // sha256.update(&contents);

        // self.sha256 = hex::encode(sha256.finalize());
    }

    fn clear_signatures(&mut self) {
        self.edd25519 = "".to_string();
        self.ecdsa_secp256k1 = "".to_string();
        self.schnorr_secp256k1 = "".to_string();
    }
}

fn sign_with_edd25519(
    private_key: Vec<u8>,
    message: Vec<u8>,
) -> Result<[u8; SIGNATURE_LENGTH], String> {
    use cryptoxide::ed25519;

    let private_key = &private_key
        .try_into()
        .map_err(|_| "Invalid private key length..".to_string())?;

    let keypair = ed25519::keypair(private_key);

    let sig = ed25519::signature(&message, &keypair.0);

    let public_key: [u8; 32] = keypair.1;

    let valid = ed25519::verify(&message, &public_key, &sig);

    if valid {
        Ok(sig)
    } else {
        Err("Something went horribly wrong. Sig did not verify with same message.".to_string())
    }
}
