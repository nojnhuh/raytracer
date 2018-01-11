mod color;
mod vector;
mod scene;
mod ray;
mod light;
mod intersect;
pub mod raytracer;

pub use self::raytracer::run;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
