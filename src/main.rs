use iced::{window, Element, Subscription, Task};
mod blake2b;
mod block_inspector;
mod copyable_text;
mod tabbed_pane_grid;

mod settings;
mod state;
mod update;
mod view;

use settings::*;
use state::*;
use update::*;

fn main() -> iced::Result {
    iced::application("", update, view)
        .theme(theme)
        .subscription(subscription)
        .window(window())
        .run()
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    state.update(message)
}

fn view(state: &State) -> Element<Message> {
    state.view()
}
fn theme(_state: &State) -> iced::Theme {
    iced::Theme::CatppuccinMocha
}
fn subscription(_state: &State) -> Subscription<Message> {
    window::resize_events().map(|(_id, size)| Message::WindowResized(size))
}
