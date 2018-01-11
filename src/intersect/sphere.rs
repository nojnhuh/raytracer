use std::rc::Rc;

use intersect::{Hit, Intersect};
use vector::Vector;
use scene::Material;
use ray::{Ray, TMAX, TMIN};

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub pos: Vector,
    pub r: f64,
    pub mat: Material,
}

impl Sphere {}

impl Intersect for Sphere {
    fn get_ray_intersection(&self, ray: Ray) -> Hit {
        // println!("Checking ray {:?}", ray);
        let t = TMAX;
        let shape: Option<Rc<Intersect>> = Some(Rc::new(self.clone()));
        let hit = false;
        let mut hit = Hit { t, shape, hit, ray };

        if self.r == 0. {
            return hit;
        }

        let c_to_p = ray.pos - self.pos;

        let disc = ray.dir.dot(&c_to_p).powi(2)
            - ray.dir.dot(&ray.dir) * (c_to_p.dot(&c_to_p) - self.r.powi(2));

        if disc < 0. {
            return hit;
        }

        let a = -ray.dir.dot(&c_to_p);
        let t_plus = a + disc.sqrt();
        let t_minus = a - disc.sqrt();

        if t_plus > TMIN && t_minus > TMIN {
            hit.t = t_plus.min(t_minus);
        } else if t_plus > TMIN {
            hit.t = t_plus;
        } else if t_minus > TMIN {
            hit.t = t_minus
        }

        if t_plus > TMIN || t_minus > TMIN {
            hit.hit = true
        }

        hit
    }

    fn get_material(&self) -> Material {
        self.mat
    }

    fn surface_normal(&self, point: Vector, _v: Vector) -> Vector {
        (point - self.pos).normalized()
    }
}
