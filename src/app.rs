use std::iter::repeat;

use iced::{Application, Color, Length};

use crate::segments::{self, DigitOptions};

struct LoadingStatus {
    current: u32,
    total: u32,
}

impl LoadingStatus {
    pub const fn with_total(total: u32) -> Self {
        Self { current: 0, total }
    }

    pub const fn done(&self) -> bool {
        self.current == self.total
    }

    pub fn increment(&mut self) {
        debug_assert!(!self.done());
        self.current += 1;
    }

    pub fn progress_bar<Theme: iced::widget::progress_bar::StyleSheet>(
        &self,
    ) -> iced::widget::ProgressBar<Theme> {
        iced::widget::progress_bar(0. ..=self.total as f32, self.current as f32)
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded {
        name: &'static str,
        result: Result<(), iced::font::Error>,
    },
    SetDigitThickness(f32),
    SetDigitGap(f32),
    TextAreaAction(iced::widget::text_editor::Action),
    Scrolled(iced::widget::scrollable::Viewport),
}

pub struct CatoDisplayApp {
    loading: LoadingStatus,
    digit_display: segments::DigitDisplay,
    text: iced::widget::text_editor::Content,
}

impl Application for CatoDisplayApp {
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                loading: LoadingStatus::with_total(
                    crate::fonts::NUM_FONTS as u32,
                ),
                digit_display: segments::DigitDisplay::new(DigitOptions {
                    ..Default::default()
                }),
                text: Default::default(),
            },
            crate::fonts::load_fonts(),
        )
    }

    fn title(&self) -> String {
        "Cato 17-Segment Display".into()
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::TokyoNight
    }

    fn update(
        &mut self,
        message: Self::Message,
    ) -> iced::Command<Self::Message> {
        match message {
            Message::FontLoaded { name, result } => {
                if result.is_err() {
                    eprintln!("Failed to load font {name}");
                }
                self.loading.increment();
            }
            Message::SetDigitThickness(v) => {
                self.digit_display.modify_options(|o| o.thickness = v)
            }
            Message::SetDigitGap(v) => {
                self.digit_display.modify_options(|o| o.gap = v)
            }
            Message::TextAreaAction(action) => self.text.perform(action),
            Message::Scrolled(_viewport) => (),
        }
        iced::Command::none()
    }

    fn view(
        &self,
    ) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        use iced::widget as w;

        if !self.loading.done() {
            let prog_bar = self
                .loading
                .progress_bar()
                .width(Length::Fill)
                .height(Length::Fixed(8.));
            return w::container(prog_bar)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_y()
                .padding(32.)
                .into();
        }

        let font = &*segments::segmented_font::DEFAULT;
        let display = {
            const H_SPACING: f32 = 8.;

            let mut display =
                w::column(self.text.lines().take(4).map(|line| {
                    w::row(line.chars().chain(repeat(' ')).take(24).map(|ch| {
                        self.digit_display.instantiate(
                            font.get(&ch).cloned().unwrap_or_default(),
                        )
                    }))
                    .spacing(H_SPACING)
                    .clip(true)
                    .into()
                }))
                .spacing(16.);

            for _ in 0..4usize.saturating_sub(self.text.line_count()) {
                display = display.push(
                    w::row((0..24).map(|_| {
                        self.digit_display.instantiate(Default::default())
                    }))
                    .spacing(H_SPACING),
                );
            }
            let display = w::container(display)
                .width(Length::Shrink)
                .padding(8.)
                .style(|theme: &iced::Theme| {
                    w::container::Appearance::default()
                        .with_background(Color::BLACK)
                        .with_border(
                            theme.extended_palette().secondary.weak.color,
                            4.,
                        )
                });
            let display = w::container(display).width(Length::Fill).center_x();
            w::scrollable(display)
                .on_scroll(Message::Scrolled)
                .height(Length::Fill)
        };

        let thickness = {
            let thickness = self.digit_display.options().thickness;
            let display = w::text(format!("{thickness:.2}")).width(80.);
            let slider =
                w::slider(1. ..=100., thickness, Message::SetDigitThickness)
                    .step(0.1);
            let space = w::Space::with_width(4.);
            w::row!(display, space, slider)
        };

        let gap = {
            let gap = self.digit_display.options().gap;
            let display = w::text(format!("{gap:.2}")).width(80.);
            let slider =
                w::slider(1. ..=100., gap, Message::SetDigitGap).step(0.1);
            w::row!(display, slider).spacing(4.)
        };

        let input =
            w::text_editor(&self.text).on_action(Message::TextAreaAction);

        // w::text(format!("{:#?}", self.digit))
        w::container(w::column!(thickness, gap, input, display).spacing(16.))
            .padding(16.)
            .into()
    }
}
