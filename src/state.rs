use crate::block_inspector;

#[derive(Default)]
pub struct State {
    pub window_size: iced::Size,
    pub block_inspector: block_inspector::State,
}
