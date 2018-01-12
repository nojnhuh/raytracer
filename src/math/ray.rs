use math::Vector;

pub const TMIN: f64 = 0.000001;
pub const TMAX: f64 = 100000.;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub pos: Vector,
    pub dir: Vector,
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            pos: Vector::new(),
            dir: Vector::new(),
        }
    }

    // pub fn normalize(&mut self) {
    //     self.dir.normalize();
    // }

    pub fn find_point(&self, t: f64) -> Vector {
        self.pos + self.dir * t
    }
}
