use iced::{Size, Task};

use super::{block_inspector, State};

#[derive(Debug)]
pub enum Message {
    WindowResized(Size),
    BlockInspector(block_inspector::Message),
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        use super::Message::*;
        match message {
            WindowResized(size) => {
                self.window_size = size;

                Task::none()
            }
            BlockInspector(m) => self.block_inspector.update(m).map(Message::BlockInspector),
        }
    }
}
