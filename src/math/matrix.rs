use math::Vector;

pub struct Matrix {
    pub v: [Vector; 3],
}

impl Matrix {
    pub fn det(&self) -> f64 {
        self.v[0].x * (self.v[1].y * self.v[2].z - self.v[2].y * self.v[1].z)
            - self.v[1].x * (self.v[0].y * self.v[2].z - self.v[2].y * self.v[0].z)
            + self.v[2].x * (self.v[0].y * self.v[1].z - self.v[1].y * self.v[0].z)
    }
}
