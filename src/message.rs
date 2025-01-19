use iced::{widget::text_editor, Size};

use crate::MultiEraTx;

#[derive(Debug, Clone)]
pub enum Message {
    TxDecoded(Box<Option<MultiEraTx>>),
    WindowResized(Size),
    TxCborChanged(text_editor::Action),
}
