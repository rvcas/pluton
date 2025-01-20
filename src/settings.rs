use iced::{window, Size};

#[cfg(target_os = "linux")]
pub fn platform_specific() -> window::settings::PlatformSpecific {
    window::settings::PlatformSpecific::default()
}

#[cfg(target_os = "windows")]
pub fn platform_specific() -> window::settings::PlatformSpecific {
    window::settings::PlatformSpecific::default()
}

#[cfg(target_os = "macos")]
pub fn platform_specfic() -> window::settings::PlatformSpecific {
    window::settings::PlatformSpecific {
        titlebar_transparent: true,
        fullsize_content_view: true,
        title_hidden: false,
    };
}

pub fn window() -> window::Settings {
    window::Settings {
        platform_specific: platform_specific(),
        size: Size::new(2048.0, 1536.0),
        ..window::Settings::default()
    }
}
