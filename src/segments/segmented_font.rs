use std::{collections::HashMap, sync::LazyLock};

use super::SegmentBits;

pub struct SegmentedFont {
    characters: HashMap<char, SegmentBits>,
}

impl SegmentedFont {
    pub const fn new(characters: HashMap<char, SegmentBits>) -> Self {
        Self { characters }
    }

    pub fn get(&self, ch: &char) -> Option<&SegmentBits> {
        self.characters.get(ch)
    }
}

#[macro_export]
macro_rules! segmented_font {
    [$($char:literal => $($bits:tt),+);* $(;)?] => {
        SegmentedFont::new([
            $(($char, $crate::segments::segmented_font!(@bits $($bits),*))),*
        ].into_iter().collect())
    };
    (@bits 0) => {$crate::segments::SegmentBits::new()};
    (@bits $($name:ident),+) => {$crate::segments::SegmentBits::new() | $($crate::segments::Segment::$name)|*};
}

pub use segmented_font;

/// Stolen from <https://github.com/CatoLynx/Cheetah_Firmware/blob/main/components/driver_display_char_16seg_led_spi/char_16seg_font.h>
pub static DEFAULT: LazyLock<SegmentedFont> = LazyLock::new(|| {
    segmented_font![
        ' ' => 0;
        '!' => A1, A2, H, I, J, D1, D2;
        '"' => F, B;
        '#' => I, L, B, C, G1, G2, D1, D2;
        '$' => A1, A2, F, G1, G2, C, D1, D2, I, L;
        '%' => A1, J, K, D2;
        '&' => A1, F, I, G1, G2, E, M, D1, D2;
        '\'' => I;
        '(' => A1, F, E, D1;
        ')' => A2, B, C, D2;
        '*' => G1, G2, H, I, J, K, L, M;
        '+' => I, L, G1, G2;
        ',' => K;
        '-' => G1, G2;
        '.' => DP;
        '/' => J, K;
        '0' => A1, A2, B, C, D1, D2, E, F, J, K;
        '1' => J, B, C;
        '2' => A1, A2, B, G1, G2, E, D1, D2;
        '3' => A1, A2, B, G1, G2, C, D1, D2;
        '4' => F, B, G1, G2, C;
        '5' => A1, A2, F, G1, G2, C, D1, D2;
        '6' => A1, A2, F, G1, G2, E, C, D1, D2;
        '7' => A1, A2, B, C;
        '8' => A1, A2, B, C, D1, D2, E, F, G1, G2;
        '9' => A1, A2, B, C, D1, D2, F, G1, G2;
        ':' => G1, D1;
        ';' => A1, K;
        '<' => J, M;
        '=' => G1, G2, D1, D2;
        '>' => H, K;
        '?' => A1, A2, B, G2, L;
        '@' => A1, A2, B, C, D2, E, F, G2, L;
        // UPPERCASE
        'A' => A1, A2, B, C, E, F, G1, G2;
        'B' => A1, A2, B, C, D1, D2, G2, I, L;
        'C' => A1, A2, D1, D2, E, F;
        'D' => A1, A2, B, C, D1, D2, I, L;
        'E' => A1, A2, D1, D2, E, F, G1;
        'F' => A1, A2, E, F, G1;
        'G' => A1, A2, C, D1, D2, E, F, G2;
        'H' => B, C, E, F, G1, G2;
        'I' => A1, A2, D1, D2, I, L;
        'J' => B, C, D1, D2;
        'K' => E, F, G1, J, M;
        'L' => D1, D2, E, F;
        'M' => B, C, E, F, H, J;
        'N' => B, C, E, F, H, M;
        'O' => A1, A2, B, C, D1, D2, E, F;
        'P' => A1, A2, B, E, F, G1, G2;
        'Q' => A1, A2, B, C, D1, D2, E, F, M;
        'R' => A1, A2, B, E, F, G1, G2, M;
        'S' => A1, A2, C, D1, D2, F, G1, G2;
        'T' => A1, A2, I, L;
        'U' => B, C, D1, D2, E, F;
        'V' => E, F, J, K;
        'W' => B, C, E, F, K, M;
        'X' => H, J, K, M;
        'Y' => H, J, L;
        'Z' => A1, A2, D1, D2, J, K;
        // TODO
        // LOWERCASE
        'a' => G1, E, L, D1, D2;
        'b' => E, F, G1, G2, D1, D2, C;
        'c' => G1, G2, E, D1, D2;
        'd' => B, C, D1, D2, E, G1, G2;
        'e' => A1, A2, B, D1, D2, E, F, G1, G2;
        'f' => A2, I, L, G1, G2;
        'g' => A1, I, L, D1, F, G1;
        'h' => E, F, G1, G2, C;
        'i' => A1, A2, L;
        'j' => A1, A2, L, D1;
        'k' => E, F, G1, J, M;
        'l' => E, F;
        'm' => G1, G2, E, L, C;
        'n' => G1, G2, E, C;
        'o' => G1, G2, E, C, D1, D2;
        'p' => A1, I, G1, E, F;
        'q' => A2, B, C, I, G2;
        'r' => G1, E;
        's' => G2, M, D2;
        't' => E, F, G1, D1;
        'u' => E, D1, D2, C;
        'v' => E, K;
        'w' => E, K, M, C;
        'x' => H, J, K, M;
        'y' => H, J, L, D1;
        'z' => G1, K, D1;
    ]
});
