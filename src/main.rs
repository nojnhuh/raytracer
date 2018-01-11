extern crate raytracer;

use raytracer::run;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut scene_file = &String::from("test.scn");
    if args.len() > 1 {
        scene_file = &args[1];
    }

    run(scene_file);
}
