use iced::{widget::column, window, Element, Subscription, Task};
use pluton::Message;
mod block_inspector;

fn main() -> iced::Result {
    iced::application("", update, view)
        .theme(pluton::theme)
        .subscription(subscription)
        .window(pluton::settings::window())
        .run()
}

fn subscription(_state: &pluton::State) -> Subscription<Message> {
    window::resize_events().map(|(_id, size)| Message::WindowResized(size))
}

fn update(state: &mut pluton::State, message: Message) -> Task<Message> {
    use pluton::Message::*;

    match message {
        WindowResized(size) => {
            state.window_size = size;

            Task::none()
        }
        BlockInspector(m) => state.block_inspector.update(m).map(Message::BlockInspector),
    }
}

fn view(state: &pluton::State) -> Element<pluton::Message> {
    column![state.block_inspector.view().map(Message::BlockInspector)].into()
}
