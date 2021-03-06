use math::Vector;
use color::Color;
use intersect::Intersectable;
use light::Light;

#[derive(Copy, Clone, Debug)]
pub struct PointLight {
    pub pos: Vector,
    pub intensity: Color,
}

impl Light for PointLight {
    fn position(&self) -> Vector {
        self.pos
    }

    fn compute_diffuse_component(
        &self,
        point_hit: Vector,
        v: Vector,
        shape_hit: &Intersectable,
    ) -> Color {
        let n = self.n(point_hit, v, shape_hit);
        let l = self.l(point_hit);
        let distance_factor = (self.position() - point_hit).magnitude().powi(2);
        // println!("Distance factor is {}", distance_factor);
        let factor = n.dot(&l).max(0.) / distance_factor;
        // println!("factor is {}", factor);
        shape_hit.get_material().dif * self.intensity * factor
    }

    fn compute_specular_component(
        &self,
        point_hit: Vector,
        v: Vector,
        shape_hit: &Intersectable,
        camera_pos: Vector,
    ) -> Color {
        let r = self.r(point_hit, v, shape_hit);
        let v = self.v(point_hit, camera_pos);
        let mat = shape_hit.get_material();
        let distance_factor = (self.position() - point_hit).magnitude().powi(2);
        let factor = r.dot(&v).max(0.).powf(mat.ns) / distance_factor;
        mat.spec * self.intensity * factor
    }
}
