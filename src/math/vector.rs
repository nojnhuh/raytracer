use std::ops;
use std::cmp;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new() -> Vector {
        Vector {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)).sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
    }

    pub fn normalized(&self) -> Vector {
        let mag = self.magnitude();
        Vector {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        let x = self.y * other.z - other.y * self.z;
        let y = self.z * other.x - other.z * self.x;
        let z = self.x * other.y - other.x * self.y;
        Vector { x, y, z }
    }

    // pub fn angle(&self, other: &Vector) -> f64 {
    //     (self.dot(other) / (self.magnitude() * other.magnitude())).acos()
    // }

    pub fn reflect(&self, other: &Vector) -> Vector {
        *other * 2. * self.dot(other) - *self
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        *self = *self + other;
    }
}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// impl<'a> ops::Sub for &'a Vector {
//     type Output = Vector;

//     fn sub(self, rhs: &Vector) -> Vector {
//         Vector {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//             z: self.z - rhs.z,
//         }
//     }
// }

impl ops::SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        *self = *self - other;
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        self * -1.
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

impl cmp::PartialEq for Vector {
    fn eq(&self, rhs: &Vector) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == self.z
    }
}

impl cmp::Eq for Vector {}
