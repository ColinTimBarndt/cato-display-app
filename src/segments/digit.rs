use std::ops::{BitAnd, BitOr};

use iced::{
    widget::canvas::{fill::Rule, Cache, Fill, Geometry, Path, Program},
    Color, Length, Size, Vector,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DigitOptions {
    pub size: Size<f32>,
    pub gap: f32,
    pub thickness: f32,
    pub slant: f32,
    pub fill: iced::widget::canvas::Style,
}

pub struct DigitDisplay {
    options: DigitOptions,
    cache: SegmentsCache,
}

pub const SEGMENT_COUNT: usize = 17;

#[repr(u8)]
pub enum Segment {
    A1 = 0,
    A2,
    B,
    C,
    D1,
    D2,
    E,
    F,
    G1,
    G2,
    H,
    I,
    J,
    K,
    L,
    M,
    DP,
}

impl TryFrom<u8> for Segment {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < SEGMENT_COUNT as u8 {
            Ok(unsafe { std::mem::transmute::<u8, Segment>(value) })
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SegmentBits(u32);

impl SegmentBits {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl Default for SegmentBits {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u32> for SegmentBits {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<SegmentBits> for u32 {
    fn from(value: SegmentBits) -> Self {
        value.0
    }
}

impl BitOr for Segment {
    type Output = SegmentBits;

    fn bitor(self, rhs: Self) -> Self::Output {
        SegmentBits::new() | self | rhs
    }
}

impl BitOr<Segment> for SegmentBits {
    type Output = SegmentBits;

    fn bitor(self, rhs: Segment) -> Self::Output {
        Self(self.0 | (1 << rhs as u8))
    }
}

impl BitOr for SegmentBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for SegmentBits {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAnd<Segment> for SegmentBits {
    type Output = bool;

    fn bitand(self, rhs: Segment) -> Self::Output {
        self.0 & (1 << rhs as u8) != 0
    }
}

pub type SegmentsCache = [Cache; SEGMENT_COUNT];

impl Default for DigitOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl DigitOptions {
    pub const fn new() -> Self {
        Self {
            size: Size::new(40., 80.),
            thickness: 5.7,
            gap: 1.3,
            slant: 0.,
            fill: iced::widget::canvas::Style::Solid(Color::from_rgb(
                1., 0., 0.,
            )),
        }
    }
}

impl DigitDisplay {
    pub fn new(options: DigitOptions) -> Self {
        Self {
            options,
            cache: SegmentsCache::default(),
        }
    }

    pub fn options(&self) -> &DigitOptions {
        &self.options
    }

    pub fn set_options(&mut self, options: DigitOptions) {
        self.clear_cache();
        self.options = options;
    }

    pub fn modify_options(&mut self, modifier: impl FnOnce(&mut DigitOptions)) {
        self.clear_cache();
        modifier(&mut self.options);
    }

    fn clear_cache(&self) {
        self.cache.iter().for_each(Cache::clear);
    }

    pub fn instantiate(
        &self,
        segments: SegmentBits,
    ) -> iced::Element<'_, crate::app::Message, iced::Theme, iced::Renderer>
    {
        use iced::widget;

        widget::canvas(DigitProgram {
            digit: self,
            segments,
        })
        .width(Length::Fixed(self.options.size.width))
        .height(Length::Fixed(self.options.size.height))
        .into()
    }
}

struct DigitProgram<'a> {
    digit: &'a DigitDisplay,
    segments: SegmentBits,
}

impl DigitProgram<'_> {
    fn draw_segments(
        &self,
        renderer: &iced::Renderer,
    ) -> [Geometry; SEGMENT_COUNT] {
        let size = self.digit.options.size;
        let options = &geometry::DrawingOptions {
            size,
            gap: self.digit.options.gap,
            thickness: self.digit.options.thickness,
            ..Default::default()
        };

        let segments_cache = &self.digit.cache;
        let fill = &self.digit.options.fill;

        std::array::from_fn(|segment| {
            let cache = &segments_cache[segment];
            cache.draw(renderer, size, |frame| {
                frame.translate(Vector::new(size.width, size.height) * 0.5);
                frame.scale(1.);
                match geometry::SEGMENT_INSTRUCTIONS.get(segment) {
                    Some(instructions) => {
                let path = Path::new(|d| {
                            geometry::draw_path(
                        d,
                                instructions.points,
                        &options.transform(instructions.transform),
                    )
                });
                frame.fill(
                    &path,
                    Fill {
                        style: fill.clone(),
                        rule: Rule::NonZero,
                    },
                );
                    }
                    None => {
                        // TODO: dot
                    }
                }
            })
        })
    }
}

impl Program<crate::app::Message> for DigitProgram<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        if self.segments.is_empty() || bounds.size() != self.digit.options.size
        {
            return Vec::new();
        }

        let segments = self.draw_segments(renderer);
        let mut shown = Vec::with_capacity(segments.len());

        for (segment, geometry) in segments.into_iter().enumerate() {
            let segment = Segment::try_from(segment as u8).unwrap();
            if self.segments & segment {
                shown.push(geometry);
            }
        }

        shown
    }
}
