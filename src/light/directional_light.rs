use math::Vector;
use color::Color;
use light::Light;
use intersect::Intersectable;

#[derive(Copy, Clone, Debug)]
pub struct DirectionalLight {
    pub dir: Vector,
    pub intensity: Color,
}

impl Light for DirectionalLight {
    fn l(&self, _point_hit: Vector) -> Vector {
        self.dir.normalized() * -1.
    }

    fn compute_diffuse_component(
        &self,
        point_hit: Vector,
        v: Vector,
        shape_hit: &Intersectable,
    ) -> Color {
        let n = self.n(point_hit, v, shape_hit);
        let l = self.l(point_hit);
        let factor = n.dot(&l).max(0.);
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
        let factor = r.dot(&v).max(0.).powf(mat.ns);
        mat.spec * self.intensity * factor
    }
}
