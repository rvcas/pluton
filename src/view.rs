use iced::Element;

use super::{Message, State};

impl State {
    pub fn view(&self) -> Element<Message> {
        self.workspace.view().map(Message::Workspace).into()
    }
}
