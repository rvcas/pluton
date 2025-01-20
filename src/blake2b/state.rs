use iced::widget::text_editor;

#[derive(Debug)]
pub struct State {
    pub hex_contents: text_editor::Content,
    pub hash_length: usize,
    pub hash: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            hex_contents: text_editor::Content::default(),
            hash_length: 256,
            hash: "".to_string(),
        }
    }
}
