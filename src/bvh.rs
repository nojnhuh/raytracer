use math::{Ray, Vector, TMIN, TMAX};
use intersect::{Hit, Intersect, Intersectable};

use bvh::Axis::{XAxis, YAxis, ZAxis};

use std::f64::INFINITY;
use std::sync::Arc;

#[derive(Copy, Clone, Debug)]
struct BoundingBox {
    v: [f64; 6],
}

impl BoundingBox {
    fn from_shape(shape: &Intersectable) -> BoundingBox {
        BoundingBox { v: shape.get_extents() }
    }

    fn combine(b1: BoundingBox, b2: BoundingBox) -> BoundingBox {
        let e1 = b1.get_extents();
        let e2 = b2.get_extents();
        let x_min = e1[0].min(e2[0]);
        let x_max = e1[1].max(e2[1]);
        let y_min = e1[2].min(e2[2]);
        let y_max = e1[3].max(e2[3]);
        let z_min = e1[4].min(e2[4]);
        let z_max = e1[5].max(e2[5]);

        BoundingBox {
            v: [x_min, x_max, y_min, y_max, z_min, z_max],
        }
    }

    fn x_min(&self) -> f64 {
        self.v[0]
    }

    fn x_max(&self) -> f64 {
        self.v[1]
    }

    fn y_min(&self) -> f64 {
        self.v[2]
    }

    fn y_max(&self) -> f64 {
        self.v[3]
    }

    fn z_min(&self) -> f64 {
        self.v[4]
    }

    fn z_max(&self) -> f64 {
        self.v[5]
    }
}

impl Intersect for BoundingBox {
    fn center(&self) -> Vector {
        Vector {
            x: (self.v[0] + self.v[1]) / 2.,
            y: (self.v[2] + self.v[3]) / 2.,
            z: (self.v[4] + self.v[5]) / 2.,
        }
    }

    fn get_ray_intersection(&self, ray: Ray) -> Hit {
        let shape: Option<Intersectable> = Some(Arc::new(self.clone()));
        let mut hit = Hit {
            t: TMAX,
            shape,
            hit: false,
            ray,
        };
        let divx = 1. / ray.dir.x;
        let mut t_min = (self.x_min() - ray.pos.x) * divx;
        let mut t_max = (self.x_max() - ray.pos.x) * divx;
        if ray.dir.x <= 0. {
            let temp = t_min;
            t_min = t_max;
            t_max = temp;
        }

        let divy = 1. / ray.dir.y;
        let mut t_ymin = (self.y_min() - ray.pos.y) * divy;
        let mut t_ymax = (self.y_max() - ray.pos.y) * divy;
        if ray.dir.y <= 0. {
            let temp = t_ymin;
            t_ymin = t_ymax;
            t_ymax = temp;
        }

        if t_min > t_ymax || t_ymin > t_max {
            return hit;
        }

        let divz = 1. / ray.dir.z;
        let mut t_zmin = (self.z_min() - ray.pos.z) * divz;
        let mut t_zmax = (self.z_max() - ray.pos.z) * divz;
        if ray.dir.z <= 0. {
            let temp = t_zmin;
            t_zmin = t_zmax;
            t_zmax = temp;
        }

        if t_min > t_zmax || t_zmin > t_max {
            return hit;
        }

        if t_ymin > t_min {
            t_min = t_ymin;
        }
        if t_ymax < t_max {
            t_max = t_ymax;
        }

        if t_zmin > t_min {
            t_min = t_zmin;
        }
        if t_zmax < t_max {
            t_max = t_zmax;
        }

        if !((t_min < TMAX) && (t_max > TMIN)) {
            return hit;
        }

        hit.hit = true;
        hit
    }

    fn get_extents(&self) -> [f64; 6] {
        self.v
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Axis {
    XAxis,
    YAxis,
    ZAxis,
}

pub struct BVHNode {
    bbox: BoundingBox,
    left: Option<Intersectable>,
    right: Option<Intersectable>,
}

impl BVHNode {
    pub fn new(shapes: &Vec<Intersectable>, axis: Axis) -> BVHNode {
        let n = shapes.len();
        let left: Option<Intersectable>;
        let right: Option<Intersectable>;
        let mut bbox: BoundingBox;
        if n == 1 {
            left = Some(Arc::clone(&shapes[0]));
            right = None;
            bbox = BoundingBox::from_shape(&Arc::clone(&shapes[0]));
        } else if n == 2 {
            left = Some(Arc::clone(&shapes[0]));
            right = Some(Arc::clone(&shapes[1]));
            bbox = BoundingBox::combine(
                BoundingBox::from_shape(&Arc::clone(&shapes[0])),
                BoundingBox::from_shape(&Arc::clone(&shapes[1])),
            );
        } else {
            bbox = BoundingBox {
                v: [
                    INFINITY, -INFINITY, INFINITY, -INFINITY, INFINITY, -INFINITY
                ],
            };
            let mut m = 0.;
            for s in shapes.iter() {
                let temp = BoundingBox::from_shape(&s);
                bbox = BoundingBox::combine(bbox, temp);

                // Find partition point
                match axis {
                    XAxis => m += s.center().x,
                    YAxis => m += s.center().y,
                    ZAxis => m += s.center().z,
                }
            }
            m /= n as f64;

            // partition shapes
            let mut left_shapes: Vec<Intersectable> = Vec::new();
            let mut right_shapes: Vec<Intersectable> = Vec::new();
            for s in shapes.iter() {
                match axis {
                    XAxis => {
                        if s.center().x < m {
                            left_shapes.push(Arc::clone(s));
                        } else {
                            right_shapes.push(Arc::clone(s));
                        }
                    }
                    YAxis => {
                        if s.center().y < m {
                            left_shapes.push(Arc::clone(s));
                        } else {
                            right_shapes.push(Arc::clone(s));
                        }
                    }
                    ZAxis => {
                        if s.center().z < m {
                            left_shapes.push(Arc::clone(s));
                        } else {
                            right_shapes.push(Arc::clone(s));
                        }
                    }
                }
            }

            let next_axis: Axis;
            match axis {
                XAxis => next_axis = YAxis,
                YAxis => next_axis = ZAxis,
                ZAxis => next_axis = XAxis,
            }

            // Create new nodes
            if left_shapes.len() > 0 {
                left = Some(Arc::new(BVHNode::new(&left_shapes, next_axis)));
            } else {
                left = None;
            }
            if right_shapes.len() > 0 {
                right = Some(Arc::new(BVHNode::new(&right_shapes, next_axis)));
            } else {
                right = None;
            }
        }
        BVHNode { left, right, bbox }
    }
}

impl Intersect for BVHNode {
    fn center(&self) -> Vector {
        self.bbox.center()
    }

    fn get_ray_intersection(&self, ray: Ray) -> Hit {
        let shape: Option<Intersectable> = None;
        let hit = Hit {
            t: TMAX,
            shape,
            hit: false,
            ray,
        };

        if self.bbox.get_ray_intersection(ray).hit {
            let mut hit_left = Hit::new();
            let mut hit_right = Hit::new();
            hit_left.hit = false;
            hit_right.hit = false;
            if let Some(ref s) = self.left {
                hit_left = s.get_ray_intersection(ray);
            }
            if let Some(ref s) = self.right {
                hit_right = s.get_ray_intersection(ray);
            }
            if hit_left.hit && hit_right.hit {
                if hit_left.t < hit_right.t {
                    return hit_left;
                } else {
                    return hit_right;
                }
            }
            if hit_left.hit {
                return hit_left;
            }
            if hit_right.hit {
                return hit_right;
            }
            return hit;
        }
        hit
    }

    fn get_extents(&self) -> [f64; 6] {
        self.bbox.get_extents()
    }
}
