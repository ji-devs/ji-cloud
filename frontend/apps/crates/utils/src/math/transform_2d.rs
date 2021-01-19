// 2d matrix math taken from glmatrix: http://glmatrix.net/docs/mat2d.js.html
pub fn translate_mut(transform:&mut [f64;6], x:f64, y: f64) {
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

pub fn scale_mut(transform:&mut [f64;6], x:f64, y: f64) {
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
