use iced::Command;

macro_rules! fonts {
    [$($name:literal),*] => (&[$(($name, include_bytes!(concat!("../fonts/", $name, ".ttf")))),*]);
}

const FONTS: &[(&str, &[u8])] = fonts![
    "Nunito-Regular",
    "Nunito-Bold",
    "Nunito-Italic",
    "Nunito-BoldItalic"
];

pub const NUM_FONTS: usize = FONTS.len();

pub fn load_fonts() -> Command<crate::app::Message> {
    Command::batch(FONTS.iter().map(|(name, bytes)| {
        iced::font::load(*bytes).map(|result| crate::app::Message::FontLoaded { name, result })
    }))
}
