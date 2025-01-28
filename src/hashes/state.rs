use iced::widget::text_editor;

#[derive(Debug)]
pub struct State {
    pub contents: text_editor::Content,
    pub is_hex: Option<bool>,
    pub warning: Option<String>,
    pub blake2b_256: String,
    pub blake2b_224: String,
    pub sha512: String,
    pub sha256: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            contents: text_editor::Content::default(),
            is_hex: None,
            warning: None,
            blake2b_256: "".to_string(),
            blake2b_224: "".to_string(),
            sha512: "".to_string(),
            sha256: "".to_string(),
        }
    }
}
