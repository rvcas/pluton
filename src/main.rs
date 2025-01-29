mod block_inspector;
mod copyable_text;
mod hashes;
mod tabbed_pane_grid;

mod settings;
mod state;
mod subscription;
mod update;
mod view;

use settings::*;
use state::*;
use update::*;

fn main() -> iced::Result {
    iced::application("", State::update, State::view)
        .theme(theme)
        .subscription(State::subscription)
        .window(window())
        .run()
}

fn theme(_state: &State) -> iced::Theme {
    iced::Theme::CatppuccinMocha
}
