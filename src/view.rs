use iced::{widget::column, Element};

use super::{Message, State};

impl State {
    pub fn view(&self) -> Element<Message> {
        column![self.block_inspector.view().map(Message::BlockInspector)].into()
    }
}
