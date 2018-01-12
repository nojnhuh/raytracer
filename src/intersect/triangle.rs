use scene::Material;
use math::{Matrix, Ray, Vector, TMAX, TMIN};
use intersect::{Hit, Intersect, Intersectable};

use std::sync::Arc;

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub v1: Vector,
    pub v3: Vector,
    pub v2: Vector,
    pub mat: Material,
}

impl Intersect for Triangle {
    fn center(&self) -> Vector {
        (self.v1 + self.v2 + self.v3) / 3.
    }

    fn get_ray_intersection(&self, ray: Ray) -> Hit {
        let shape: Option<Intersectable> = Some(Arc::new(self.clone()));
        let mut hit = Hit {
            t: TMAX,
            shape,
            hit: false,
            ray,
        };

        let m = Matrix {
            v: [self.v1 - self.v2, self.v1 - self.v3, hit.ray.dir],
        };
        let det_a = m.det();

        let m = Matrix {
            v: [self.v1 - self.v2, self.v1 - self.v3, self.v1 - hit.ray.pos],
        };
        let t = m.det() / det_a;
        if t < TMIN || t > TMAX {
            return hit;
        }

        let m = Matrix {
            v: [self.v1 - self.v2, self.v1 - hit.ray.pos, hit.ray.dir],
        };
        let c = m.det() / det_a;
        if c < 0. || c > 1. {
            return hit;
        }

        let m = Matrix {
            v: [self.v1 - hit.ray.pos, self.v1 - self.v3, hit.ray.dir],
        };
        let b = m.det() / det_a;
        if b < 0. || b > 1. - c {
            return hit;
        }

        hit.t = t;
        hit.hit = true;
        hit
    }

    fn get_material(&self) -> Material {
        self.mat
    }

    fn surface_normal(&self, _point: Vector, v: Vector) -> Vector {
        let v31 = self.v3 - self.v1;
        let v21 = self.v2 - self.v1;
        let cp1 = (v21).cross(&v31);
        let cp2 = (v31).cross(&v21);

        if v.dot(&cp1) > 0. {
            cp1.normalized()
        } else {
            cp2.normalized()
        }
    }

    fn get_extents(&self) -> [f64; 6] {
        let x_min = self.v1.x.min(self.v2.x.min(self.v3.x));
        let x_max = self.v1.x.max(self.v2.x.max(self.v3.x));
        let y_min = self.v1.y.min(self.v2.y.min(self.v3.y));
        let y_max = self.v1.y.max(self.v2.y.max(self.v3.y));
        let z_min = self.v1.z.min(self.v2.z.min(self.v3.z));
        let z_max = self.v1.z.max(self.v2.z.max(self.v3.z));
        [x_min, x_max, y_min, y_max, z_min, z_max]
    }
}
