use iced::{widget::text_editor, Size};

use crate::block_inspector;

#[derive(Debug)]
pub enum Message {
    WindowResized(Size),
    BlockInspector(block_inspector::Message),
}
