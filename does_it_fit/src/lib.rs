pub mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
    x: usize,
    y: usize,
    objects: areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let total_area = match objects {
        GeometricalShapes::Square => square_area(a) as f64 * times as f64,
        GeometricalShapes::Circle => circle_area(a) as f64 * times as f64,
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64 * times as f64,
        GeometricalShapes::Triangle => triangle_area(a, b) as f64 * times as f64,
    };

    total_area <= x as f64 * y as f64
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let total_volume = match objects {
        GeometricalVolumes::Cube => cube_volume(a) as f64 * times as f64,
        GeometricalVolumes::Sphere => sphere_volume(a) as f64 * times as f64,
        GeometricalVolumes::Cone => cone_volume(a, b) as f64 * times as f64,
        GeometricalVolumes::Pyramid => triangular_pyramid_volume(a as f64, b) as f64 * times as f64,
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64 * times as f64,
    };

    total_volume <= x as f64 * y as f64 * z as f64
}
