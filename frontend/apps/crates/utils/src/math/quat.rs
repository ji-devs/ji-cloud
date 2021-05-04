const EPSILON:f64 = 0.000001;

pub fn get_axis_angle(q:&[f64]) -> ([f64;3], f64) {

    let rad = q[3].acos() * 2.0;
    let s = (rad / 2.0).sin();
  
    let out_axis = {
        if (s > EPSILON) {
            [
                q[0] / s,
                q[1] / s,
                q[2] / s,
            ]
        } else {
            [1.0, 0.0, 0.0]
        }
    };

    (out_axis, rad)
}
