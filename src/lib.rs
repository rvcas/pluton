mod message;
pub mod settings;
mod state;

pub use message::*;
pub use state::*;

pub fn theme(_state: &State) -> iced::Theme {
    iced::Theme::CatppuccinMocha
}
