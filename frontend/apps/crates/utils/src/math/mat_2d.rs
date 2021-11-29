// 2d matrix math taken from glmatrix: http://glmatrix.net/docs/mat2d.js.html

//Broken...
// pub fn from_mat4(mat:&[f64;16]) -> [f64;6] {
//     let out:[f64;6] = [
//         mat[0],
//         mat[1],
//         mat[4],
//         mat[5],
//         mat[8],
//         mat[9],
//     ];

//     out
// }

pub fn translate_mut(transform: &mut [f64; 6], x: f64, y: f64) {
    let a = transform;
    let a0 = a[0];
    let a1 = a[1];
    let a2 = a[2];
    let a3 = a[3];
    let a4 = a[4];
    let a5 = a[5];
    let v0 = x;
    let v1 = y;
    a[0] = a0;
    a[1] = a1;
    a[2] = a2;
    a[3] = a3;
    a[4] = a0 * v0 + a2 * v1 + a4;
    a[5] = a1 * v0 + a3 * v1 + a5;
}

pub fn scale_mut(transform: &mut [f64; 6], x: f64, y: f64) {
    let a = transform;
    let a0 = a[0];
    let a1 = a[1];
    let a2 = a[2];
    let a3 = a[3];
    let a4 = a[4];
    let a5 = a[5];
    let v0 = x;
    let v1 = y;
    a[0] = a0 * v0;
    a[1] = a1 * v0;
    a[2] = a2 * v1;
    a[3] = a3 * v1;
    a[4] = a4;
    a[5] = a5;
}

pub fn mul_mut(transform: &mut [f64; 6], other: &[f64; 6]) {
    let a = transform;
    let a0 = a[0];
    let a1 = a[1];
    let a2 = a[2];
    let a3 = a[3];
    let a4 = a[4];
    let a5 = a[5];
    let b = other;
    let b0 = b[0];
    let b1 = b[1];
    let b2 = b[2];
    let b3 = b[3];
    let b4 = b[4];
    let b5 = b[5];
    a[0] = a0 * b0 + a2 * b1;
    a[1] = a1 * b0 + a3 * b1;
    a[2] = a0 * b2 + a2 * b3;
    a[3] = a1 * b2 + a3 * b3;
    a[4] = a0 * b4 + a2 * b5 + a4;
    a[5] = a1 * b4 + a3 * b5 + a5;
}
/*
export function multiply(out, a, b) {
  let a0 = a[0],
    a1 = a[1],
    a2 = a[2],
    a3 = a[3],
    a4 = a[4],
    a5 = a[5];
  let b0 = b[0],
    b1 = b[1],
    b2 = b[2],
    b3 = b[3],
    b4 = b[4],
    b5 = b[5];
  out[0] = a0 * b0 + a2 * b1;
  out[1] = a1 * b0 + a3 * b1;
  out[2] = a0 * b2 + a2 * b3;
  out[3] = a1 * b2 + a3 * b3;
  out[4] = a0 * b4 + a2 * b5 + a4;
  out[5] = a1 * b4 + a3 * b5 + a5;
  return out;
}
*/
