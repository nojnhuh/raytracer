use scene::Material;
use math::{Ray, Vector, TMAX};

use std::sync::Arc;

pub use self::sphere::Sphere;
pub use self::triangle::Triangle;

mod sphere;
mod triangle;

pub struct Hit {
    pub hit: bool,
    pub shape: Option<Intersectable>,
    pub ray: Ray,
    pub t: f64,
}

impl Hit {
    pub fn new() -> Hit {
        Hit {
            t: TMAX,
            shape: None,
            hit: false,
            ray: Ray::new(),
        }
    }
}

pub trait Intersect {
    fn center(&self) -> Vector;
    fn get_ray_intersection(&self, ray: Ray) -> Hit;
    fn get_material(&self) -> Material {
        Material::new()
    }
    fn surface_normal(&self, _point: Vector, _v: Vector) -> Vector {
        Vector::new()
    }
    fn get_extents(&self) -> [f64; 6];
}

pub type Intersectable = Arc<Intersect>;
