extern crate image;

use std::path::Path;
// use std::thread;

use color::Color;
use scene::Scene;
use math::Ray;
use intersect::{Hit, Intersect};

pub fn run(scene_file: &String) {
    let rt = Raytracer::new(scene_file);
    rt.run()
}

struct Raytracer {
    scene: Scene,
}

impl Raytracer {
    fn new(scene_file: &String) -> Raytracer {
        Raytracer {
            scene: Scene::new(scene_file),
        }
    }

    fn run(&self) {
        let mut i = image::RgbImage::new(self.scene.width, self.scene.height);
        // let mut colors: [Color; ]

        for (x, y, pixel) in i.enumerate_pixels_mut() {
            // println!("Image x, y = {}, {}", x, y);
            // thread::spawn(move || {
            *pixel = image::Rgb(self.get_color_for_pixel(x as f64, y as f64).to_u8_array());
            // });
        }

        i.save(Path::new(&self.scene.filename))
            .expect("Error saving image");
    }

    fn get_color_for_pixel(&self, x: f64, y: f64) -> Color {
        let r = self.get_ray_through_canvas(x, y);
        self.evaluate_ray_tree(r, 0)
    }

    fn get_ray_through_canvas(&self, x: f64, y: f64) -> Ray {
        let u = self.scene.canvas.left + x + 0.5;
        let v = self.scene.canvas.bottom + y + 0.5;

        let mut dir = self.scene.camera.right * u + self.scene.camera.up * v
            - self.scene.camera.dir * self.scene.canvas.depth;
        dir.normalize();

        let ray = Ray {
            pos: self.scene.camera.pos,
            dir,
        };

        // println!("Ray is {:?}", ray);

        ray
    }

    fn evaluate_ray_tree(&self, ray: Ray, current_depth: u32) -> Color {
        let current_depth = current_depth + 1;
        let hit = self.get_ray_intersection(ray);
        if hit.hit && current_depth <= self.scene.max_depth + 1 {
            // println!("Ray hit something");
            self.apply_lighting_model(hit, current_depth)
        } else {
            // println!("Ray missed");
            self.scene.background
        }
    }

    fn apply_lighting_model(&self, hit: Hit, current_depth: u32) -> Color {
        let mut color = Color::new(0., 0., 0.);
        let shape = hit.shape.unwrap();
        let mat = shape.get_material();

        color += shape.get_material().amb * self.scene.ambient_light;

        let point_hit = hit.ray.find_point(hit.t);
        let v = (hit.ray.pos - point_hit).normalized();
        let mut n = shape.surface_normal(point_hit, v);

        for light in self.scene.lights.iter() {
            let l = light.l(point_hit);
            let diff = light.compute_diffuse_component(point_hit, v, &shape);
            // println!("Diffuse component is {:?}", diff);
            let spec =
                light.compute_specular_component(point_hit, v, &shape, self.scene.camera.pos);

            // Check shadow
            let shadow = Ray {
                pos: point_hit,
                dir: l,
            };
            let shadow_hit = self.get_ray_intersection(shadow);
            if shadow_hit.hit {
                let p = shadow_hit.ray.find_point(shadow_hit.t);
                if (p - point_hit).magnitude() < (light.position() - point_hit).magnitude() {
                    // printf("Shadowing\n");
                    continue;
                }
            }

            color += diff + spec;
        }

        if mat.spec.is_not_black() {
            let reflect = Ray {
                pos: point_hit,
                dir: v.reflect(&n),
            };
            color += mat.spec * self.evaluate_ray_tree(reflect, current_depth);
        }

        if mat.trs.is_not_black() {
            let d = v * -1.;
            let ior_i: f64;
            let ior_r: f64;
            if v.dot(&n) > 0. {
                // Going into solid
                ior_i = 1.;
                ior_r = mat.ior;
            } else {
                // Going out of solid
                ior_i = mat.ior;
                ior_r = 1.;
                n = n * -1.;
            }

            let refract_v = ((d - n * d.dot(&n)) * ior_i / ior_r)
                - n * (1. - (ior_i.powi(2) * (1. - d.dot(&n).powi(2))) / ior_r.powi(2)).sqrt();
            let refract = Ray {
                pos: point_hit,
                dir: refract_v,
            };
            color += mat.trs * self.evaluate_ray_tree(refract, current_depth);
        }

        color
    }

    fn get_ray_intersection(&self, ray: Ray) -> Hit {
        self.scene.bvhroot.get_ray_intersection(ray)
    }
}
