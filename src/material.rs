#[derive(Clone)]
pub enum Material {
    Lambertian,
    Hemispherical,
    Metal,
    Dielectric,
}