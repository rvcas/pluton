use iced::widget::text_editor;

use crate::encoding::Encoding;

#[derive(Debug)]
pub struct State {
    pub contents: text_editor::Content,
    pub encoding: Option<Encoding>,
    pub warning: Option<String>,
    pub blake2b_256: String,
    pub blake2b_224: String,
    pub sha512: String,
    pub sha256: String,
}

impl State {
    // Temporary, until text_editor can return unmodified contents
    pub fn text(&self) -> String {
        self.contents
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
            contents: text_editor::Content::default(),
            encoding: None,
            warning: None,
            blake2b_256: "".to_string(),
            blake2b_224: "".to_string(),
            sha512: "".to_string(),
            sha256: "".to_string(),
        }
    }
}
