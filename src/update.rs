use iced::{Size, Task};

use super::{tabbed_pane_grid, State};

#[derive(Debug)]
pub enum Message {
    WindowResized(Size),
    Workspace(tabbed_pane_grid::Message),
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        use super::Message::*;
        match message {
            WindowResized(size) => {
                self.window_size = size;

                Task::none()
            }
            Workspace(m) => self.workspace.update(m).map(Message::Workspace),
        }
    }
}
