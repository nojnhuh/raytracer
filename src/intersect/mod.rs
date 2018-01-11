use scene::Material;
use ray::Ray;
use vector::Vector;
use std::rc::Rc;

pub use self::sphere::Sphere;

mod sphere;

pub struct Hit {
    pub hit: bool,
    pub shape: Option<Rc<Intersect>>,
    pub ray: Ray,
    pub t: f64,
}

pub trait Intersect {
    fn get_ray_intersection(&self, ray: Ray) -> Hit;
    fn get_material(&self) -> Material;
    fn surface_normal(&self, point: Vector, v: Vector) -> Vector;
}
