//! See <https://en.wikipedia.org/wiki/Sixteen-segment_display#/media/File:16-segmente.png>

use std::f32::consts::{FRAC_1_SQRT_2, SQRT_2};

use glam::{Mat2, Vec2};
use iced::{widget::canvas::path, Point, Size};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SegmentPoint {
    pub pos: Vec2,
    pub thickness_offset: Vec2,
    pub gap_offset: Vec2,
}

impl SegmentPoint {
    pub const fn new(pos: Vec2) -> Self {
        Self {
            pos,
            thickness_offset: Vec2::ZERO,
            gap_offset: Vec2::ZERO,
        }
    }

    pub const fn with_thickness_offset(self, thickness_offset: Vec2) -> Self {
        Self {
            thickness_offset,
            ..self
        }
    }

    pub const fn with_gap_offset(self, gap_offset: Vec2) -> Self {
        Self { gap_offset, ..self }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DrawingOptions {
    pub size: Size,
    pub gap: f32,
    pub thickness: f32,
    pub pos_transform: Mat2,
    pub transform: Mat2,
}

impl Default for DrawingOptions {
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

impl DrawingOptions {
    pub fn transform(self, mat: Mat2) -> Self {
        Self {
            transform: self.transform * mat,
            ..self
        }
    }
}

pub fn draw_path(
    d: &mut path::Builder,
    points: &[SegmentPoint],
    &DrawingOptions {
        gap,
        thickness: thick,
        size,
        pos_transform,
        transform,
    }: &DrawingOptions,
) {
    let Some((first, rest)) = points.split_first() else {
        return;
    };

    let pos_ref = Vec2::new(size.width, size.height) * 0.5;

    d.move_to(point(
        transform
            * (pos_transform
                * (pos_ref * first.pos + thick * first.thickness_offset)
                + gap * first.gap_offset),
    ));

    for sp in rest {
        d.line_to(point(
            transform
                * (pos_transform
                    * (pos_ref * sp.pos + thick * sp.thickness_offset)
                    + gap * sp.gap_offset),
        ));
    }

    d.close();

    const fn point(vec: Vec2) -> Point {
        Point::new(vec.x, vec.y)
    }
}

/// Indices to render a 4-point segment using triangle strip encoding.
pub const TRI_STRIP_4: [usize; 4] = [0, 1, 3, 2];
/// Indices to render a 5-point segment using triangle strip encoding.
pub const TRI_STRIP_5: [usize; 5] = [0, 1, 4, 2, 3];
/// Indices to render a 6-point segment using triangle strip encoding.
pub const TRI_STRIP_6: [usize; 6] = [0, 1, 5, 2, 4, 3];

const DGAP: f32 = FRAC_1_SQRT_2 * 0.5;
const DGAP_INNER: f32 = SQRT_2 * 0.5;
const TOP_LEFT: Vec2 = Vec2::NEG_ONE;
const TOP: Vec2 = Vec2::NEG_Y;
const LEFT: Vec2 = Vec2::NEG_X;
const MID: Vec2 = Vec2::ZERO;

pub const A1: [SegmentPoint; 5] = [
    SegmentPoint::new(TOP_LEFT)
        .with_thickness_offset(Vec2::new(0.5, 0.5))
        .with_gap_offset(Vec2::new(DGAP, -DGAP)),
    SegmentPoint::new(TOP_LEFT).with_thickness_offset(Vec2::X),
    SegmentPoint::new(TOP).with_gap_offset(Vec2::new(-0.5, 0.)),
    SegmentPoint::new(TOP)
        .with_thickness_offset(Vec2::Y)
        .with_gap_offset(Vec2::new(-0.5, 0.)),
    SegmentPoint::new(TOP_LEFT)
        .with_thickness_offset(Vec2::ONE)
        .with_gap_offset(Vec2::new(DGAP_INNER, 0.)),
];

pub const F: [SegmentPoint; 6] = [
    SegmentPoint::new(TOP_LEFT)
        .with_thickness_offset(Vec2::new(0.5, 0.5))
        .with_gap_offset(Vec2::new(-DGAP, DGAP)),
    SegmentPoint::new(TOP_LEFT)
        .with_thickness_offset(Vec2::ONE)
        .with_gap_offset(Vec2::new(0., DGAP_INNER)),
    SegmentPoint::new(LEFT)
        .with_thickness_offset(Vec2::new(1., -0.5))
        .with_gap_offset(Vec2::new(0., -DGAP_INNER)),
    SegmentPoint::new(LEFT)
        .with_thickness_offset(Vec2::new(0.5, 0.))
        .with_gap_offset(Vec2::new(-0.5, -0.5)),
    SegmentPoint::new(LEFT).with_thickness_offset(Vec2::new(0., -0.5)),
    SegmentPoint::new(TOP_LEFT).with_thickness_offset(Vec2::Y),
];

pub const G1: [SegmentPoint; 5] = [
    SegmentPoint::new(LEFT)
        .with_thickness_offset(Vec2::new(0.5, 0.))
        .with_gap_offset(Vec2::new(DGAP_INNER, 0.)),
    SegmentPoint::new(LEFT)
        .with_thickness_offset(Vec2::new(1., -0.5))
        .with_gap_offset(Vec2::new(DGAP_INNER, 0.)),
    SegmentPoint::new(MID)
        .with_thickness_offset(Vec2::new(0., -0.5))
        .with_gap_offset(Vec2::new(-0.5, 0.)),
    SegmentPoint::new(MID)
        .with_thickness_offset(Vec2::new(0., 0.5))
        .with_gap_offset(Vec2::new(-0.5, 0.)),
    SegmentPoint::new(LEFT)
        .with_thickness_offset(Vec2::new(1., 0.5))
        .with_gap_offset(Vec2::new(DGAP_INNER, 0.)),
];

pub const H: [SegmentPoint; 6] = [
    SegmentPoint::new(TOP_LEFT)
        .with_thickness_offset(Vec2::ONE)
        .with_gap_offset(Vec2::ONE),
    SegmentPoint::new(TOP_LEFT)
        .with_thickness_offset(Vec2::new(1.5, 1.))
        .with_gap_offset(Vec2::ONE),
    SegmentPoint::new(Vec2::new(0., -0.5))
        .with_thickness_offset(Vec2::new(-0.5, 0.))
        .with_gap_offset(Vec2::NEG_X),
    SegmentPoint::new(MID)
        .with_thickness_offset(Vec2::new(-0.5, -0.5))
        .with_gap_offset(Vec2::NEG_ONE),
    SegmentPoint::new(MID)
        .with_thickness_offset(Vec2::new(-1., -0.5))
        .with_gap_offset(Vec2::NEG_ONE),
    SegmentPoint::new(Vec2::new(-1., -0.5))
        .with_thickness_offset(Vec2::X)
        .with_gap_offset(Vec2::X),
];

pub const I: [SegmentPoint; 4] = [
    SegmentPoint::new(TOP)
        .with_thickness_offset(Vec2::new(-0.5, 1.))
        .with_gap_offset(Vec2::Y),
    SegmentPoint::new(TOP)
        .with_thickness_offset(Vec2::new(0.5, 1.))
        .with_gap_offset(Vec2::Y),
    SegmentPoint::new(MID)
        .with_thickness_offset(Vec2::new(0.5, -0.5))
        .with_gap_offset(Vec2::NEG_Y),
    SegmentPoint::new(MID)
        .with_thickness_offset(Vec2::new(-0.5, -0.5))
        .with_gap_offset(Vec2::NEG_Y),
];

pub struct SegmentInstruction<'a> {
    pub points: &'a [SegmentPoint],
    pub transform: Mat2,
}

pub const SEGMENT_INSTRUCTIONS: [SegmentInstruction; 16] = {
    const IDENT: Mat2 = Mat2::IDENTITY;
    const MIRROR_X: Mat2 = Mat2::from_diagonal(Vec2::new(-1., 1.));
    const MIRROR_Y: Mat2 = Mat2::from_diagonal(Vec2::new(1., -1.));
    const MIRROR_XY: Mat2 = Mat2::from_diagonal(Vec2::new(-1., -1.));

    [
        /*A1*/
        SegmentInstruction {
            points: &A1,
            transform: IDENT,
        },
        /*A2*/
        SegmentInstruction {
            points: &A1,
            transform: MIRROR_X,
        },
        /*B*/
        SegmentInstruction {
            points: &F,
            transform: MIRROR_X,
        },
        /*C*/
        SegmentInstruction {
            points: &F,
            transform: MIRROR_XY,
        },
        /*D1*/
        SegmentInstruction {
            points: &A1,
            transform: MIRROR_Y,
        },
        /*D2*/
        SegmentInstruction {
            points: &A1,
            transform: MIRROR_XY,
        },
        /*E*/
        SegmentInstruction {
            points: &F,
            transform: MIRROR_Y,
        },
        /*F*/
        SegmentInstruction {
            points: &F,
            transform: IDENT,
        },
        /*G1*/
        SegmentInstruction {
            points: &G1,
            transform: IDENT,
        },
        /*G2*/
        SegmentInstruction {
            points: &G1,
            transform: MIRROR_X,
        },
        /*H*/
        SegmentInstruction {
            points: &H,
            transform: IDENT,
        },
        /*I*/
        SegmentInstruction {
            points: &I,
            transform: IDENT,
        },
        /*J*/
        SegmentInstruction {
            points: &H,
            transform: MIRROR_X,
        },
        /*K*/
        SegmentInstruction {
            points: &H,
            transform: MIRROR_Y,
        },
        /*L*/
        SegmentInstruction {
            points: &I,
            transform: MIRROR_Y,
        },
        /*M*/
        SegmentInstruction {
            points: &H,
            transform: MIRROR_XY,
        },
    ]
};
