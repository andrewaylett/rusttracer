use std::fmt;
use std::ops::{Add, Sub, Mul};
use std::iter::{Iterator,IntoIterator};
use rayon::prelude::*;
use rayon::par_iter::internal::*;
use rayon::par_iter::slice::SliceProducer;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

pub enum NextColor {
    Red, Green, Blue, Done
}

pub struct ColorIterator {
    color: [u8;3],
    next: NextColor
}

impl Iterator for ColorIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            NextColor::Red => {
                self.next = NextColor::Green;
                Some(self.color[0])
            }
            NextColor::Green => {
                self.next = NextColor::Blue;
                Some(self.color[1])
            }
            NextColor::Blue => {
                self.next = NextColor::Done;
                Some(self.color[2])
            }
            NextColor::Done => None
        }
    }
}

//impl ParallelIterator for ColorIterator {
//    type Item = u8;
//    fn drive_unindexed<C>(self, consumer: C) -> C::Result where C: UnindexedConsumer<Self::Item> {
//        bridge(self, consumer)
//    }
//}
//
//impl BoundedParallelIterator for ColorIterator {
//    fn upper_bound(&mut self) -> usize {
//        ExactParallelIterator::len(self)
//    }
//
//    fn drive<C>(self, consumer: C) -> C::Result
//        where C: Consumer<Self::Item>
//    {
//        bridge(self, consumer)
//    }
//}
//
//impl ExactParallelIterator for ColorIterator {
//    fn len(&mut self) -> usize {
//        3
//    }
//}
//
//impl IndexedParallelIterator for ColorIterator {
//    fn with_producer<CB>(self, callback: CB) -> CB::Output
//        where CB: ProducerCallback<Self::Item>
//    {
//        callback.callback(SliceProducer { slice: &self.color })
//    }
//}

impl IntoIterator for Color {
    type Item = u8;
    type IntoIter = ColorIterator;
    fn into_iter(self) -> Self::IntoIter {
        ColorIterator { color:[self.r, self.g, self.b], next:NextColor::Red }
    }
}

//impl IntoParallelIterator for Color {
//    type Item = u8;
//    type Iter = ColorIterator;
//    fn into_par_iter(self) -> Self::Iter {
//        ColorIterator { color:[self.r, self.g, self.b], next:NextColor::Red }
//    }
//}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r: r, g: g, b: b }
    }

    pub fn new_f64(r: f64, g: f64, b: f64) -> Color {
        Color::new(Color::clamp((r * 255.0) as i16),
                   Color::clamp((g * 255.0) as i16),
                   Color::clamp((b * 255.0) as i16))
    }

    #[inline(always)]
    pub fn r(&self) -> u8 {
        self.r
    }

    #[inline(always)]
    pub fn r_f64(&self) -> f64 {
        self.r() as f64 / 255.0
    }

    #[inline(always)]
    pub fn g(&self) -> u8 {
        self.g
    }

    #[inline(always)]
    pub fn g_f64(&self) -> f64 {
        self.g() as f64 / 255.0
    }

    #[inline(always)]
    pub fn b(&self) -> u8 {
        self.b
    }

    #[inline(always)]
    pub fn b_f64(&self) -> f64 {
        self.b() as f64 / 255.0
    }

    #[inline(always)]
    pub fn as_u32(&self) -> u32 {
        0xFF000000 
            & (self.r as u32) 
            & (self.g as u32) << 8 
            & (self.b as u32) << 16
    }

    fn clamp(value: i16) -> u8 {
        if value < 0 {
            return 0;
        }

        if value > (u8::max_value() as i16) {
            return u8::max_value();
        }

        value as u8
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(r: {}, g: {}, b: {})", self.r(), self.g(), self.b())
    }
}

// Math

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let r = Color::clamp((self.r() as i16) + (other.r() as i16));
        let g = Color::clamp((self.g() as i16) + (other.g() as i16));
        let b = Color::clamp((self.b() as i16) + (other.b() as i16));

        Color::new(r, g, b)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        let r = Color::clamp((self.r() as i16) - (other.r() as i16));
        let g = Color::clamp((self.g() as i16) - (other.g() as i16));
        let b = Color::clamp((self.b() as i16) - (other.b() as i16));

        Color::new(r, g, b)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        let r = self.r_f64() * other.r_f64();
        let g = self.g_f64() * other.g_f64();
        let b = self.b_f64() * other.b_f64();

        Color::new_f64(r, g, b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        let r = self.r_f64() * other;
        let g = self.g_f64() * other;
        let b = self.b_f64() * other;

        Color::new_f64(r, g, b)
    }
}


// Factor methods for common colors
macro_rules! define_color {
    ($name: ident, $r: expr, $g: expr, $b: expr) => (
        #[inline(always)]
        pub fn $name() -> Color {
            Color::new($r, $g, $b)
        }
    )
}

impl Color {
    define_color!(black, 0, 0, 0);
    define_color!(white, 0xFF, 0xFF, 0xFF);
    define_color!(red, 0xFF, 0, 0);
    define_color!(green, 0, 0xFF, 0);
    define_color!(blue, 0, 0, 0xFF);
}
