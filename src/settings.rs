use iced::{window, Size};

pub fn window() -> window::Settings {
    let platform_specific = window::settings::PlatformSpecific {
        titlebar_transparent: true,
        fullsize_content_view: true,
        title_hidden: false,
    };

    window::Settings {
        platform_specific,
        size: Size::new(2048.0, 1536.0),
        ..window::Settings::default()
    }
}
