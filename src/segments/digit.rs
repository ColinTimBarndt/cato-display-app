use std::ops::{BitAnd, BitOr};

use iced::{
    widget::canvas::{fill::Rule, Cache, Fill, Geometry, Path, Program},
    Color, Length, Size, Vector,
};
use path::SEGMENT_INSTRUCTIONS;

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
        if value <= Segment::DP as u8 {
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

pub type SegmentsCache = [Cache; SEGMENT_INSTRUCTIONS.len()];

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
    ) -> [Geometry; SEGMENT_INSTRUCTIONS.len()] {
        let size = self.digit.options.size;
        let options = &path::Options {
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
                let instructions = &path::SEGMENT_INSTRUCTIONS[segment];
                let path = Path::new(|d| {
                    (instructions.draw)(
                        d,
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

/// See <https://en.wikipedia.org/wiki/Sixteen-segment_display#/media/File:16-segmente.png>
mod path {
    use glam::{Mat2, Vec2};
    use iced::{widget::canvas::path::Builder, Point, Size};

    pub struct SegmentInstruction {
        pub draw: for<'a> fn(&'a mut Builder, &'a Options),
        pub transform: Mat2,
    }

    pub const SEGMENT_INSTRUCTIONS: [SegmentInstruction; 17] = {
        const IDENT: Mat2 = Mat2::IDENTITY;
        const MIRROR_X: Mat2 = Mat2::from_diagonal(Vec2::new(-1., 1.));
        const MIRROR_Y: Mat2 = Mat2::from_diagonal(Vec2::new(1., -1.));
        const MIRROR_XY: Mat2 = Mat2::from_diagonal(Vec2::new(-1., -1.));

        [
            /*A1*/
            SegmentInstruction {
                draw: segment_a1,
                transform: IDENT,
            },
            /*A2*/
            SegmentInstruction {
                draw: segment_a1,
                transform: MIRROR_X,
            },
            /*B*/
            SegmentInstruction {
                draw: segment_f,
                transform: MIRROR_X,
            },
            /*C*/
            SegmentInstruction {
                draw: segment_f,
                transform: MIRROR_XY,
            },
            /*D1*/
            SegmentInstruction {
                draw: segment_a1,
                transform: MIRROR_Y,
            },
            /*D2*/
            SegmentInstruction {
                draw: segment_a1,
                transform: MIRROR_XY,
            },
            /*E*/
            SegmentInstruction {
                draw: segment_f,
                transform: MIRROR_Y,
            },
            /*F*/
            SegmentInstruction {
                draw: segment_f,
                transform: IDENT,
            },
            /*G1*/
            SegmentInstruction {
                draw: segment_g1,
                transform: IDENT,
            },
            /*G2*/
            SegmentInstruction {
                draw: segment_g1,
                transform: MIRROR_X,
            },
            /*H*/
            SegmentInstruction {
                draw: segment_h,
                transform: IDENT,
            },
            /*I*/
            SegmentInstruction {
                draw: segment_i,
                transform: IDENT,
            },
            /*J*/
            SegmentInstruction {
                draw: segment_h,
                transform: MIRROR_X,
            },
            /*K*/
            SegmentInstruction {
                draw: segment_h,
                transform: MIRROR_Y,
            },
            /*L*/
            SegmentInstruction {
                draw: segment_i,
                transform: MIRROR_Y,
            },
            /*M*/
            SegmentInstruction {
                draw: segment_h,
                transform: MIRROR_XY,
            },
            /*DP*/
            SegmentInstruction {
                draw: segment_dot,
                transform: IDENT,
            },
        ]
    };

    #[derive(Debug, Clone, Copy)]
    pub struct Options {
        pub size: Size,
        pub gap: f32,
        pub thickness: f32,
        pub pos_transform: Mat2,
        pub transform: Mat2,
    }

    impl Default for Options {
        fn default() -> Self {
            Self {
                gap: 2.,
                thickness: 12.,
                size: Size::new(100., 200.),
                pos_transform: Mat2::IDENTITY,
                transform: Mat2::IDENTITY,
            }
        }
    }

    impl Options {
        pub fn transform(mut self, transform: Mat2) -> Self {
            self.transform = transform * self.transform;
            self
        }
    }

    const fn point(vec: Vec2) -> Point {
        Point::new(vec.x, vec.y)
    }

    pub fn segment_a1(
        d: &mut Builder,
        &Options {
            gap,
            thickness: thick,
            size,
            pos_transform,
            transform,
        }: &Options,
    ) {
        let topleft = Vec2::new(size.width, size.height) * -0.5;
        let hgap = gap * 0.5;
        let dgap = gap * std::f32::consts::FRAC_1_SQRT_2 * 0.5;
        let dgap_inner = gap * std::f32::consts::SQRT_2 * 0.5;
        let hthick = thick * 0.5;

        d.move_to(point(
            transform
                * (pos_transform * (topleft + Vec2::new(hthick, hthick))
                    + Vec2::new(dgap, -dgap)),
        ));
        d.line_to(point(
            transform * pos_transform * (topleft + Vec2::new(thick, 0.)),
        ));
        d.line_to(point(
            transform
                * (pos_transform * Vec2::new(0., topleft.y)
                    + Vec2::new(-hgap, 0.)),
        ));
        d.line_to(point(
            transform
                * (pos_transform * Vec2::new(0., topleft.y + thick)
                    + Vec2::new(-hgap, 0.)),
        ));
        d.line_to(point(
            transform
                * (pos_transform * (topleft + Vec2::new(thick, thick))
                    + Vec2::new(dgap_inner, 0.)),
        ));
        d.close();
    }

    pub fn segment_f(
        d: &mut Builder,
        &Options {
            gap,
            thickness: thick,
            size,
            pos_transform,
            transform,
        }: &Options,
    ) {
        let topleft = Vec2::new(size.width, size.height) * -0.5;
        let dgap = gap * std::f32::consts::FRAC_1_SQRT_2 * 0.5;
        let dgap_inner = gap * std::f32::consts::SQRT_2 * 0.5;
        let hthick = thick * 0.5;

        d.move_to(point(
            transform
                * (pos_transform * (topleft + Vec2::new(hthick, hthick))
                    + Vec2::new(-dgap, dgap)),
        ));
        d.line_to(point(
            transform
                * (pos_transform * (topleft + Vec2::new(thick, thick))
                    + Vec2::new(0., dgap_inner)),
        ));
        d.line_to(point(
            transform
                * (pos_transform * (Vec2::new(topleft.x + thick, -hthick))
                    + Vec2::new(0., -dgap_inner)),
        ));
        d.line_to(point(
            transform
                * (pos_transform * (Vec2::new(topleft.x + hthick, 0.))
                    + Vec2::new(-dgap, -dgap)),
        ));
        d.line_to(point(
            transform * (pos_transform * Vec2::new(topleft.x, -hthick)),
        ));
        d.line_to(point(
            transform * pos_transform * (topleft + Vec2::new(0., thick)),
        ));
        d.close();
    }

    pub fn segment_g1(
        d: &mut Builder,
        &Options {
            gap,
            thickness: thick,
            size,
            pos_transform,
            transform,
        }: &Options,
    ) {
        let left = size.width * -0.5;
        let hgap = gap * 0.5;
        let dgap_inner = gap * std::f32::consts::SQRT_2 * 0.5;
        let hthick = thick * 0.5;

        d.move_to(point(
            transform
                * (pos_transform * Vec2::new(left + hthick, 0.)
                    + Vec2::new(dgap_inner, 0.)),
        ));
        d.line_to(point(
            transform
                * (pos_transform * Vec2::new(left + thick, -hthick)
                    + Vec2::new(dgap_inner, 0.)),
        ));
        d.line_to(point(
            transform
                * (pos_transform * Vec2::new(0., -hthick)
                    + Vec2::new(-hgap, 0.)),
        ));
        d.line_to(point(
            transform
                * (pos_transform * Vec2::new(0., hthick)
                    + Vec2::new(-hgap, 0.)),
        ));
        d.line_to(point(
            transform
                * (pos_transform * Vec2::new(left + thick, hthick)
                    + Vec2::new(dgap_inner, 0.)),
        ));
        d.close();
    }

    pub fn segment_h(
        d: &mut Builder,
        &Options {
            gap,
            thickness: thick,
            size,
            pos_transform,
            transform,
        }: &Options,
    ) {
        let topleft = Vec2::new(size.width, size.height) * -0.5;
        let hthick = thick * 0.5;

        const SHORT_SIDE: f32 = 0.08;
        const LONG_SIDE: f32 = 0.15;

        d.move_to(point(
            transform
                * (pos_transform * (topleft + Vec2::new(thick, thick))
                    + Vec2::new(gap, gap)),
        ));
        d.line_to(point(
            transform
                * (pos_transform
                    * (topleft
                        + Vec2::new(thick + size.width * SHORT_SIDE, thick))
                    + Vec2::new(gap, gap)),
        ));
        d.line_to(point(
            transform
                * (pos_transform
                    * (Vec2::new(-hthick, -hthick - size.height * LONG_SIDE))
                    + Vec2::new(-gap, -gap)),
        ));
        d.line_to(point(
            transform
                * (pos_transform * (Vec2::new(-hthick, -hthick))
                    + Vec2::new(-gap, -gap)),
        ));
        d.line_to(point(
            transform
                * (pos_transform
                    * (Vec2::new(-hthick - size.width * SHORT_SIDE, -hthick))
                    + Vec2::new(-gap, -gap)),
        ));
        d.line_to(point(
            transform
                * (pos_transform
                    * (topleft
                        + Vec2::new(thick, thick + size.height * LONG_SIDE))
                    + Vec2::new(gap, gap)),
        ));
        d.close();
    }

    pub fn segment_i(
        d: &mut Builder,
        &Options {
            gap,
            thickness: thick,
            size,
            pos_transform,
            transform,
        }: &Options,
    ) {
        let top = size.height * -0.5;
        let hthick = thick * 0.5;

        d.move_to(point(
            transform
                * (pos_transform * (Vec2::new(-hthick, top + thick))
                    + Vec2::new(0., gap)),
        ));
        d.line_to(point(
            transform
                * (pos_transform * (Vec2::new(hthick, top + thick))
                    + Vec2::new(0., gap)),
        ));
        d.line_to(point(
            transform
                * (pos_transform * (Vec2::new(hthick, -hthick))
                    + Vec2::new(0., -gap)),
        ));
        d.line_to(point(
            transform
                * (pos_transform * (Vec2::new(-hthick, -hthick))
                    + Vec2::new(0., -gap)),
        ));
        d.close();
    }

    pub fn segment_dot(_d: &mut Builder, _options: &Options) {
        // TODO
    }
}
