use iced::{Application, Size};

pub mod app;
pub mod fonts;
pub mod segments;

fn main() -> iced::Result {
    app::CatoDisplayApp::run(iced::Settings {
        default_font: iced::Font::with_name("Nunito"),
        window: iced::window::Settings {
            size: Size::new(800., 600.),
            ..Default::default()
        },
        ..Default::default()
    })
}
