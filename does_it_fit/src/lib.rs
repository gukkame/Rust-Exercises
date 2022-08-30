
mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
	x: usize,
	y: usize,
	objects: areas_volumes::GeometricalShapes,
	times: usize,
	a: usize,
	b: usize,
) -> bool {
let area = x * y;
    if objects == areas_volumes::GeometricalShapes::Rectangle {
        let one = rectangle_area(a, b);
      let res = one * times;
      if res < area  {
          return true
      }else{
        return false
      }
    }
    if objects == areas_volumes::GeometricalShapes::Square {
        let one = square_area(a);
      let res = one * times;
      if res < area  {
          return true
      }else{
        return false
      }
    }
    if objects == areas_volumes::GeometricalShapes::Circle {
        let one = circle_area(a);
      let res = one * times as f64;
      if res < area as f64 {
          return true
      }else{
        return false
      }
    }
    if objects == areas_volumes::GeometricalShapes::Triangle {
        let one = triangle_area(a, b);
      let res = one * times as f64;
      if res < area as f64 {
          return true
      }else{
        return false
      }
    }
    println!("{:?}", objects);
false
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

    let area = x * y *z;
    if objects == areas_volumes::GeometricalVolumes::Cube {
        let one = cube_volume(a);
      let res = one * times;
      if res < area  {
          return true
      }else{
        return false
      }
    }
    if objects == areas_volumes::GeometricalVolumes::Sphere {
        let one = sphere_volume(a);
      let res = one * times as f64;
      if res < area as f64  {
          return true
      }else{
        return false
      }
    }
    if objects == areas_volumes::GeometricalVolumes::Cone {
        let one = cone_volume(a, b);
      let res = one * times as f64;
      if res < area as f64  {
          return true
      }else{
        return false
      }
    }
    if objects == areas_volumes::GeometricalVolumes::Pyramid {
        let one = triangular_pyramid_volume(a as f64, b);
      let res = one * times as f64;
      if res < area as f64  {
          return true
      }else{
        return false
      }
    }
    if objects == areas_volumes::GeometricalVolumes::Parallelepiped {
        let one = parallelepiped_volume(a,b, c);
      let res = one * times;
      if res < area  {
          return true
      }else{
        return false
      }
    }
false
}
