mod color;
mod scene;
mod light;
mod intersect;
mod math;
mod bvh;
pub mod raytracer;

pub use self::raytracer::run;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
