// taken from gl-matrix: https://glmatrix.net/docs/vec2.js.html#line478
pub fn angle(v1:&[f64], v2:&[f64]) -> f64 {
    let x1 = v1[0];
    let y1 = v1[1];
    let x2 = v2[0];
    let y2 = v2[1];


    // mag is the product of the magnitudes of a and b
    let mag = (x1 * x1 + y1 * y1).sqrt() * (x2 * x2 + y2 * y2).sqrt();

    if mag == 0.0 {
        f64::NAN
    } else {
        let cosine = (x1 * x2 + y1 * y2) / mag;
        //clamp it to -1, 1
        let cosine = cosine.max(-1.0).min(1.0);
        cosine.acos()
    }
}

//technically the cross product is supposed to return a vector
//but we're just interested in the value
pub fn cross_value(v1:&[f64], v2:&[f64]) -> f64 {
    v1[0] * v2[1] - v1[1] * v2[0]
}

pub fn len(v:&[f64]) -> f64 {
    v[0].hypot(v[1])
}

pub fn normalize(v:&[f64]) -> [f64;2] {
  let x = v[0];
  let y = v[1];
  let mut len = x * x + y * y;
  if len > 0.0 {
    len = 1.0 / len.sqrt(); 
  }

  [v[0] * len, v[1] * len]
}

pub fn rotate_by_quat(v:&[f64], q:&[f64]) -> [f64;2] {
    let (axis, rad) = super::quat::get_axis_angle(q);

    rotate(v, rad)
}
pub fn rotate(v:&[f64], rad:f64) -> [f64;2] {
    rotate_at_origin(v, &[0.0, 0.0], rad)
}

pub fn rotate_at_origin(v:&[f64], origin:&[f64], rad:f64) -> [f64;2] {
  //Translate point to the origin
  let p0 = v[0] - origin[0];
  let p1 = v[1] - origin[1];
  let sin_c = rad.sin();
  let cos_c = rad.cos();
  //perform rotation and translate to correct position
      [
      p0 * cos_c - p1 * sin_c + origin[0],
        p0 * sin_c + p1 * cos_c + origin[1]
      ]
}
