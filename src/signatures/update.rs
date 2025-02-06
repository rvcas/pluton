use base64::{prelude::BASE64_STANDARD, Engine};
use cryptoxide::ed25519;
use iced::{clipboard, widget::text_editor, Task};
use secp256k1::rand::rngs::OsRng;

use crate::encoding::{detect_encoding, Encoding};

use super::state::State;

#[derive(Debug, Clone)]
pub enum Message {
    ContentsChanged(text_editor::Action),
    KeyChanged(text_editor::Action),
    GenerateKey,
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
            GenerateKey => {
                let mut rng = OsRng;
                let sk = secp256k1::Secp256k1::new();
                let x = sk.generate_keypair(&mut rng).0.secret_bytes();

                self.warning = None;
                self.private_key = text_editor::Content::with_text(&hex::encode(x));
                self.update_signatures();
                Task::none()
            }
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

        match sign_with_edd25519(private_key.clone(), message.clone()) {
            Ok((pub_key, sig)) => {
                self.ed25519_pub = hex::encode(pub_key);
                self.ed25519_sig = hex::encode(sig);
            }
            Err(warn) => {
                self.warning = Some(warn);
                self.ed25519_pub = "".to_string();
                self.ed25519_sig = "".to_string();
                return;
            }
        }

        match sign_with_ecdsa_secp256k1(private_key, message) {
            Ok((pub_key, sig)) => {
                self.ecdsa_secp256k1_pub = hex::encode(pub_key);
                self.ecdsa_secp256k1_sig = hex::encode(sig);
            }
            Err(warn) => {
                self.warning = Some(warn);
                self.ecdsa_secp256k1_pub = "".to_string();
                self.ecdsa_secp256k1_sig = "".to_string();
                return;
            }
        }
    }

    fn clear_signatures(&mut self) {
        self.ed25519_pub = "".to_string();
        self.ed25519_sig = "".to_string();
        self.ecdsa_secp256k1_pub = "".to_string();
        self.ecdsa_secp256k1_sig = "".to_string();
        self.schnorr_secp256k1 = "".to_string();
    }
}

fn sign_with_edd25519(
    private_key: Vec<u8>,
    message: Vec<u8>,
) -> Result<
    (
        [u8; ed25519::PUBLIC_KEY_LENGTH],
        [u8; ed25519::SIGNATURE_LENGTH],
    ),
    String,
> {
    let private_key = &private_key
        .try_into()
        .map_err(|_| "Invalid private key length..".to_string())?;

    let (private_key, public_key) = ed25519::keypair(private_key);

    let sig = ed25519::signature(&message, &private_key);

    let valid = ed25519::verify(&message, &public_key, &sig);

    if valid {
        Ok((public_key, sig))
    } else {
        Err("Something went horribly wrong. Sig did not verify with same message.".to_string())
    }
}

fn sign_with_ecdsa_secp256k1(
    private_key: Vec<u8>,
    message: Vec<u8>,
) -> Result<
    (
        [u8; secp256k1::constants::PUBLIC_KEY_SIZE],
        [u8; secp256k1::constants::COMPACT_SIGNATURE_SIZE],
    ),
    String,
> {
    let private_key = &private_key
        .try_into()
        .map_err(|_| "Invalid private key length..".to_string())?;

    let private_key = secp256k1::SecretKey::from_byte_array(private_key)
        .map_err(|_| "Invalid secp256k1 private key..".to_string())?;

    let ecdsa_signer = secp256k1::Secp256k1::new();

    let pub_key = private_key.public_key(&ecdsa_signer);

    let message: [u8; 32] = message
        .try_into()
        .map_err(|_| "Invalid secp256k1 message length..".to_string())?;

    let message = secp256k1::Message::from_digest(message);

    let sig = ecdsa_signer.sign_ecdsa(&message, &private_key);

    ecdsa_signer
        .verify_ecdsa(&message, &sig, &pub_key)
        .map(|_| (pub_key.serialize(), sig.serialize_compact()))
        .map_err(|_| {
            "Something went horribly wrong. Sig did not verify with same message.".to_string()
        })
}
