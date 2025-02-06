use std::fmt::Display;

use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub enum Encoding {
    UTF8,
    Hex,
    Base64,
}

impl Encoding {
    pub const ALL: [Encoding; 3] = [Encoding::Hex, Encoding::Base64, Encoding::UTF8];
}

impl Display for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Encoding::UTF8 => "UTF8",
                Encoding::Hex => "Hex",
                Encoding::Base64 => "Base64",
            }
        )
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
