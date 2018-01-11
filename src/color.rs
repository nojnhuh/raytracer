extern crate num;

use std::ops;
use std::cmp;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn to_u8_array(&self) -> [u8; 3] {
        [
            num::clamp(self.r * 255., 0., 255.) as u8,
            num::clamp(self.g * 255., 0., 255.) as u8,
            num::clamp(self.b * 255., 0., 255.) as u8,
        ]
    }

    pub fn is_not_black(&self) -> bool {
        self.r != 0. || self.g != 0. || self.b != 0.
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        *self = *self + other;
    }
}

impl ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl ops::SubAssign for Color {
    fn sub_assign(&mut self, other: Color) {
        *self = *self - other;
    }
}

impl ops::Neg for Color {
    type Output = Color;

    fn neg(self) -> Color {
        self * -1.
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}

impl ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Color {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Color {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

impl cmp::PartialEq for Color {
    fn eq(&self, rhs: &Color) -> bool {
        self.r == rhs.r && self.g == rhs.g && self.b == self.b
    }
}

impl cmp::Eq for Color {}
