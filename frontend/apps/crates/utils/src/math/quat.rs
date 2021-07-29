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


pub fn invert(q:&[f64]) -> [f64;4] {
    let a0 = q[0];
    let a1 = q[1];
    let a2 = q[2];
    let a3 = q[3];
    let dot = a0 * a0 + a1 * a1 + a2 * a2 + a3 * a3;
    let inv_dot = if dot != 0.0 { 1.0 / dot } else { 0.0 };
    // TODO: Would be faster to return [0,0,0,0] immediately if dot == 0
    [
        -a0 * inv_dot,
        -a1 * inv_dot,
        -a2 * inv_dot,
        a3 * inv_dot,
    ]
}
