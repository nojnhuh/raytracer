use std::rc::Rc;

use intersect::Intersect;
use vector::Vector;
use color::Color;

pub use self::point_light::PointLight;

mod point_light;

pub trait Light {
    fn position(&self) -> Vector;

    fn v(&self, point_hit: Vector, camera_pos: Vector) -> Vector {
        (camera_pos - point_hit).normalized()
    }

    fn l(&self, point_hit: Vector) -> Vector {
        (self.position() - point_hit).normalized()
    }

    fn n(&self, point_hit: Vector, v: Vector, shape_hit: &Rc<Intersect>) -> Vector {
        shape_hit.surface_normal(point_hit, v)
    }

    fn r(&self, point_hit: Vector, v: Vector, shape_hit: &Rc<Intersect>) -> Vector {
        let l = self.l(point_hit);
        let n = self.n(point_hit, v, shape_hit);
        l.reflect(&n).normalized()
    }

    fn compute_diffuse_component(
        &self,
        point_hit: Vector,
        v: Vector,
        shape_hit: &Rc<Intersect>,
    ) -> Color;
    fn compute_specular_component(
        &self,
        point_hit: Vector,
        v: Vector,
        shape_hit: &Rc<Intersect>,
        camera_pos: Vector,
    ) -> Color;
}
