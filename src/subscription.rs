use iced::{window, Subscription};

use super::{Message, State};

impl State {
    pub fn subscription(&self) -> Subscription<Message> {
        window::resize_events().map(|(_id, size)| Message::WindowResized(size))
    }
}
