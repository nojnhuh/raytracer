use std::f64::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::rc::Rc;

use intersect::{Intersect, Sphere};
use color::Color;
use vector::Vector;
use light::{Light, PointLight};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub filename: String,
    pub background: Color,
    pub camera: Camera,
    pub canvas: Canvas,
    pub shapes: Vec<Rc<Intersect>>,
    pub lights: Vec<Rc<Light>>,
    pub max_depth: u32,
}

pub struct Camera {
    pub pos: Vector,
    pub dir: Vector,
    pub up: Vector,
    pub right: Vector,
    pub ha: f64,
}

#[derive(Debug)]
pub struct Canvas {
    pub left: f64,
    pub bottom: f64,
    pub depth: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub amb: Color,
    pub dif: Color,
    pub spec: Color,
    pub ns: f64,
    pub trs: Color,
    pub ior: f64,
}

impl Scene {
    pub fn new(scene_file: &String) -> Scene {
        // Define default values
        let mut width = 640;
        let mut height = 480;
        let mut filename = String::from("raytraced.png");
        let mut background = Color {
            r: 0.,
            g: 0.,
            b: 0.,
        };
        let mut camera = Camera {
            pos: Vector {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            dir: Vector {
                x: 0.,
                y: 0.,
                z: -1.,
            },
            up: Vector {
                x: 0.,
                y: 1.,
                z: 0.,
            },
            right: Vector {
                x: 1.,
                y: 0.,
                z: 0.,
            },
            ha: PI / 4.,
        };
        let mut shapes: Vec<Rc<Intersect>> = Vec::new();
        let mut lights: Vec<Rc<Light>> = Vec::new();
        let mut max_depth = 5;

        let mut current_material = Material::new();

        let f = File::open(scene_file).expect("error");
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = line.unwrap();
            let l = String::from(l);
            let mut l_iter = l.split_whitespace();
            match l_iter.next().unwrap_or("") {
                "resolution" => {
                    width = l_iter.next().unwrap().parse().unwrap();
                    height = l_iter.next().unwrap().parse().unwrap();
                    // println!("Resolution is now {} by {}", width, height);
                }
                "filename" => {
                    filename = String::from(l_iter.next().unwrap());
                    // println!("Output file is {}", filename);
                }
                "sphere" => {
                    let x: f64 = l_iter.next().unwrap().parse().unwrap();
                    let y: f64 = l_iter.next().unwrap().parse().unwrap();
                    let z: f64 = l_iter.next().unwrap().parse().unwrap();
                    let r: f64 = l_iter.next().unwrap().parse().unwrap();
                    let s = Sphere {
                        pos: Vector { x, y, z },
                        r,
                        mat: current_material,
                    };
                    // println!(
                    //     "Made new sphere at {} {} {} with radius {}",
                    //     s.pos.x, s.pos.y, s.pos.z, s.r
                    // );
                    shapes.push(Rc::new(s));
                }
                "background" => {
                    let r: f64 = l_iter.next().unwrap().parse().unwrap();
                    let g: f64 = l_iter.next().unwrap().parse().unwrap();
                    let b: f64 = l_iter.next().unwrap().parse().unwrap();
                    background = Color::new(r, g, b);
                    // println!("Background is now {:?}", background);
                }
                "camera" => {
                    let pos_x: f64 = l_iter.next().unwrap().parse().unwrap();
                    let pos_y: f64 = l_iter.next().unwrap().parse().unwrap();
                    let pos_z: f64 = l_iter.next().unwrap().parse().unwrap();
                    let dir_x: f64 = l_iter.next().unwrap().parse().unwrap();
                    let dir_y: f64 = l_iter.next().unwrap().parse().unwrap();
                    let dir_z: f64 = l_iter.next().unwrap().parse().unwrap();
                    let up_x: f64 = l_iter.next().unwrap().parse().unwrap();
                    let up_y: f64 = l_iter.next().unwrap().parse().unwrap();
                    let up_z: f64 = l_iter.next().unwrap().parse().unwrap();
                    camera.ha = l_iter.next().unwrap().parse().unwrap();
                    camera.ha *= PI / 180.;
                    let pos = Vector {
                        x: pos_x,
                        y: pos_y,
                        z: pos_z,
                    };
                    let mut dir = Vector {
                        x: dir_x,
                        y: dir_y,
                        z: dir_z,
                    } * -1.;
                    let mut up = Vector {
                        x: up_x,
                        y: up_y,
                        z: up_z,
                    };
                    let mut right = dir.cross(&up);
                    up = dir.cross(&right);
                    dir.normalize();
                    up.normalize();
                    right.normalize();

                    // println!("Camera position: {:?}", pos);
                    // println!("Camera look dir: {:?}", dir);
                    // println!("Camera up vectr: {:?}", up);
                    // println!("Camera right dr: {:?}", right);
                    // println!("Camera field ov: {}", camera.ha);

                    camera = Camera {
                        pos,
                        dir,
                        up,
                        right,
                        ha: camera.ha,
                    };
                }
                "max_depth" => {
                    max_depth = l_iter.next().unwrap().parse().unwrap();
                    // println!("Max recursion depth is {}", max_depth);
                }
                "material" => {
                    let ar = l_iter.next().unwrap().parse().unwrap();
                    let ag = l_iter.next().unwrap().parse().unwrap();
                    let ab = l_iter.next().unwrap().parse().unwrap();
                    let dr = l_iter.next().unwrap().parse().unwrap();
                    let dg = l_iter.next().unwrap().parse().unwrap();
                    let db = l_iter.next().unwrap().parse().unwrap();
                    let sr = l_iter.next().unwrap().parse().unwrap();
                    let sg = l_iter.next().unwrap().parse().unwrap();
                    let sb = l_iter.next().unwrap().parse().unwrap();
                    let ns = l_iter.next().unwrap().parse().unwrap();
                    let tr = l_iter.next().unwrap().parse().unwrap();
                    let tg = l_iter.next().unwrap().parse().unwrap();
                    let tb = l_iter.next().unwrap().parse().unwrap();
                    let ior = l_iter.next().unwrap().parse().unwrap();
                    current_material = Material {
                        amb: Color {
                            r: ar,
                            g: ag,
                            b: ab,
                        },
                        dif: Color {
                            r: dr,
                            g: dg,
                            b: db,
                        },
                        spec: Color {
                            r: sr,
                            g: sg,
                            b: sb,
                        },
                        ns: ns,
                        trs: Color {
                            r: tr,
                            g: tg,
                            b: tb,
                        },
                        ior: ior,
                    }
                }
                "point_light" => {
                    let r = l_iter.next().unwrap().parse().unwrap();
                    let g = l_iter.next().unwrap().parse().unwrap();
                    let b = l_iter.next().unwrap().parse().unwrap();
                    let x = l_iter.next().unwrap().parse().unwrap();
                    let y = l_iter.next().unwrap().parse().unwrap();
                    let z = l_iter.next().unwrap().parse().unwrap();
                    let l = PointLight {
                        pos: Vector { x, y, z },
                        intensity: Color { r, g, b },
                    };
                    lights.push(Rc::new(l));
                    // println!("Added point light at ({}, {}, {}) with color ({}, {}, {})", x, y, z, r, g, b);
                }
                _ => continue,
            }
        }
        let canvas = Canvas {
            left: -(width as f64) / 2.,
            bottom: -(height as f64) / 2.,
            depth: (height as f64) / (2. * camera.ha.tan()),
        };
        Scene {
            width,
            height,
            filename,
            background,
            camera,
            canvas,
            shapes,
            lights,
            max_depth,
        }
    }
}

impl Material {
    pub fn new() -> Material {
        Material {
            amb: Color {
                r: 1.,
                g: 0.,
                b: 0.,
            },
            dif: Color {
                r: 1.,
                g: 0.,
                b: 0.,
            },
            spec: Color {
                r: 1.,
                g: 1.,
                b: 1.,
            },
            ns: 16.,
            trs: Color {
                r: 1.,
                g: 1.,
                b: 1.,
            },
            ior: 1.,
        }
    }
}
