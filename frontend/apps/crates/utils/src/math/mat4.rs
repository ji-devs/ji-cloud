use std::ops::{Mul, MulAssign, Deref, DerefMut};
use std::convert::{AsRef, TryInto};
use crate::unwrap::UnwrapJiExt;

#[derive(thiserror::Error, Debug)]
pub enum MatrixError {
    #[error("cannot invert the matrix")]
    Invert
}

#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct Matrix4 ([f64;16]);

const MATRIX_IDENTITY:[f64;16] = [
    1.0,0.0,0.0,0.0,
    0.0,1.0,0.0,0.0,
    0.0,0.0,1.0,0.0,
    0.0,0.0,0.0,1.0,
];

impl Matrix4 {
    pub fn identity() -> Self {
        MATRIX_IDENTITY.as_ref().into()
    }

    pub fn as_matrix_string(&self) -> String {
        let mat = self;

        format!("matrix3d({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
            mat[0],
            mat[1],
            mat[2],
            mat[3],
            mat[4],
            mat[5],
            mat[6],
            mat[7],
            mat[8],
            mat[9],
            mat[10],
            mat[11],
            mat[12],
            mat[13],
            mat[14],
            mat[15],
        )
    }

    pub fn reset_from_trs_origin(&mut self, translation:&[f64], rotation:&[f64], scale:&[f64], origin:&[f64]) {
        let values = &mut self.0;
        let x = rotation[0];
        let y = rotation[1];
        let z = rotation[2]; 
        let w = rotation[3];
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;
        let sx = scale[0];
        let sy = scale[1];
        let sz = scale[2];
        let ox = origin[0];
        let oy = origin[1];
        let oz = origin[2];
        let out0 = (1.0 - (yy + zz)) * sx;
        let out1 = (xy + wz) * sx;
        let out2 = (xz - wy) * sx;
        let out4 = (xy - wz) * sy;
        let out5 = (1.0 - (xx + zz)) * sy;
        let out6 = (yz + wx) * sy;
        let out8 = (xz + wy) * sz;
        let out9 = (yz - wx) * sz;
        let out10 = (1.0 - (xx + yy)) * sz;
        values[0] = out0;
        values[1] = out1;
        values[2] = out2;
        values[3] = 0.0;
        values[4] = out4;
        values[5] = out5;
        values[6] = out6;
        values[7] = 0.0;
        values[8] = out8;
        values[9] = out9;
        values[10] = out10;
        values[11] = 0.0;
        values[12] = translation[0] + ox - (out0 * ox + out4 * oy + out8 * oz);
        values[13] = translation[1] + oy - (out1 * ox + out5 * oy + out9 * oz);
        values[14] = translation[2] + oz - (out2 * ox + out6 * oy + out10 * oz);
        values[15] = 1.0;
    }

    pub fn mul_assign(&mut self, other:&Self) {
        *self *= other;
    }
    pub fn as_slice(&self) -> &[f64] {
        &self.0
    }

    pub fn as_slice_mut(&mut self) -> &mut [f64] {
        &mut self.0
    }

    pub fn new( a:f64, b:f64, c:f64, d:f64,
                e:f64, f:f64, g:f64, h:f64,  
                i:f64, j:f64, k:f64, l:f64,
                m:f64, n:f64, o:f64, p:f64,
        ) -> Self {
            Self([
                a,b,c,d,
                e,f,g,h,
                i,j,k,l,
                m,n,o,p,
            ])
    }
    pub fn reset(&mut self) {
        self.copy_from_slice(&MATRIX_IDENTITY);
    }
    //translation
    pub fn new_from_translation(translation: &[f64]) -> Self {
        let mut m = Self::identity();
        m.translate(translation);
        m
    }

    pub fn reset_from_translation(&mut self, translation:&[f64]) {
        self.reset();
        self.translate(translation);
    }

    pub fn translate(&mut self, translation:&[f64]) {
        let values = &mut self.0;
        values[12] = translation[0];
        values[13] = translation[1];
        values[14] = translation[2];
    }

    //rotation
    pub fn new_from_rotation(rotation: &[f64]) -> Self {
        let mut m = Self::identity();
        m.rotate(rotation);
        m
    }
    pub fn reset_from_rotation(&mut self, rotation:&[f64]) {
        self.reset();
        self.rotate(rotation);
    }
    pub fn rotate(&mut self, rotation:&[f64]) {
        let values = &mut self.0;
        let x = rotation[0];
        let y = rotation[1];
        let z = rotation[2];
        let w = rotation[3];
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let yx = y * x2;
        let yy = y * y2;
        let zx = z * x2;
        let zy = z * y2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;
        values[0] = 1.0 - yy - zz;
        values[1] = yx + wz;
        values[2] = zx - wy;
        //values[3 = 0.0;
        values[4] = yx - wz;
        values[5] = 1.0 - xx - zz;
        values[6] = zy + wx;
        //values[7 = 0.0;
        values[8] = zx + wy;
        values[9] = zy - wx;
        values[10] = 1.0 - xx - yy;
        //values[11 = 0.0;
        //values[12 = 0.0;
        //values[13 = 0.0;
        //values[14 = 0.0;
        //values[15 = 1.0;
    }

    //scale
    pub fn new_from_scale(scale:&[f64]) -> Self {
        let mut m = Self::identity();
        m.scale(scale);
        m
    }
    pub fn reset_from_scale(&mut self, scale:&[f64]) {
        self.reset();
        self.scale(scale);
    }

    pub fn scale(&mut self, scale:&[f64]) {
        let values = &mut self.0;
        values[0] = scale[0];
        values[5] = scale[1];
        values[10] = scale[2];
    }

    //translation, rotation, scale
    pub fn new_from_trs(translation:&[f64], rotation:&[f64], scale:&[f64]) -> Self {
        let mut m = Self::identity();
        m.set_trs(translation, rotation, scale);
        m
    }
    pub fn new_from_trs_origin(translation:&[f64], rotation:&[f64], scale:&[f64], origin:&[f64]) -> Self {
        let mut m = Self::identity();
        Self::reset_from_trs_origin(&mut m, translation, rotation, scale, origin);
        m
    }
    pub fn reset_from_trs(&mut self, translation:&[f64], rotation:&[f64], scale:&[f64]) {
        self.reset();
        self.set_trs(translation, rotation, scale);
    }
    pub fn set_trs(&mut self, translation:&[f64], rotation:&[f64], scale:&[f64]) {
        let values = &mut self.0;
        let x = rotation[0];
        let y = rotation[1]; 
        let z = rotation[2];
        let w = rotation[3];
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;
        let sx = scale[0];
        let sy = scale[1];
        let sz = scale[2];
        values[0] = (1.0 - (yy + zz)) * sx;
        values[1] = (xy + wz) * sx;
        values[2] = (xz - wy) * sx;
        values[3] = 0.0;
        values[4] = (xy - wz) * sy;
        values[5] = (1.0 - (xx + zz)) * sy;
        values[6] = (yz + wx) * sy;
        values[7] = 0.0;
        values[8] = (xz + wy) * sz;
        values[9] = (yz - wx) * sz;
        values[10] = (1.0 - (xx + yy)) * sz;
        values[11] = 0.0;
        values[12] = translation[0];
        values[13] = translation[1];
        values[14] = translation[2];
        values[15] = 1.0;
    }

    // arithmetic 

    /// returns true if it was able to invert, false otherwise
    pub fn invert_mut(&mut self) -> Result<(), MatrixError> {
        let values = &mut self.0;
        let a:&[f64] = values; 
        let a00 = a[0]; 
        let a01 = a[1]; 
        let a02 = a[2]; 
        let a03 = a[3];
        let a10 = a[4]; 
        let a11 = a[5]; 
        let a12 = a[6]; 
        let a13 = a[7];
        let a20 = a[8]; 
        let a21 = a[9]; 
        let a22 = a[10]; 
        let a23 = a[11];
        let a30 = a[12]; 
        let a31 = a[13]; 
        let a32 = a[14]; 
        let a33 = a[15];
        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;
        // Calculate the determinant
        let mut det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
        if det == 0.0 {
            Err(MatrixError::Invert)
        } else {
            det = 1.0 / det;
            values[0] = (a11 * b11 - a12 * b10 + a13 * b09) * det;
            values[1] = (a02 * b10 - a01 * b11 - a03 * b09) * det;
            values[2] = (a31 * b05 - a32 * b04 + a33 * b03) * det;
            values[3] = (a22 * b04 - a21 * b05 - a23 * b03) * det;
            values[4] = (a12 * b08 - a10 * b11 - a13 * b07) * det;
            values[5] = (a00 * b11 - a02 * b08 + a03 * b07) * det;
            values[6] = (a32 * b02 - a30 * b05 - a33 * b01) * det;
            values[7] = (a20 * b05 - a22 * b02 + a23 * b01) * det;
            values[8] = (a10 * b10 - a11 * b08 + a13 * b06) * det;
            values[9] = (a01 * b08 - a00 * b10 - a03 * b06) * det;
            values[10] = (a30 * b04 - a31 * b02 + a33 * b00) * det;
            values[11] = (a21 * b02 - a20 * b04 - a23 * b00) * det;
            values[12] = (a11 * b07 - a10 * b09 - a12 * b06) * det;
            values[13] = (a00 * b09 - a01 * b07 + a02 * b06) * det;
            values[14] = (a31 * b01 - a30 * b03 - a32 * b00) * det;
            values[15] = (a20 * b03 - a21 * b01 + a22 * b00) * det;
            Ok(())
        }
    }
    pub fn invert(&self) -> Result<Self, MatrixError> {
        let mut clone = self.clone();
        clone.invert_mut()?;
        Ok(clone)
    }

    pub fn transform_vec2(&self, v:&[f64]) -> [f64;2] {

        let m = self.0;

        let x = v[0];
        let y = v[1];
        [
            m[0] * x + m[4] * y + m[12],
            m[1] * x + m[5] * y + m[13]
        ]
    }

    pub fn transform_vec3(&self, v:&[f64]) -> [f64;3] {
        let m = self.0;
        let x = v[0];
        let y = v[1];
        let z = v[2];


        let w = m[3] * x + m[7] * y + m[11] * z + m[15];
        let w = if w == 0.0 { 1.0 } else { w };
        [
            (m[0] * x + m[4] * y + m[8] * z + m[12]) / w,
            (m[1] * x + m[5] * y + m[9] * z + m[13]) / w,
            (m[2] * x + m[6] * y + m[10] * z + m[14]) / w,
        ]
    }

}

impl From<&[f64]> for Matrix4 {
    fn from(values:&[f64]) -> Self {
        let data:[f64;16] = values.try_into().unwrap_ji();
        Self(data)
    }
}

impl Deref for Matrix4 { 
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Matrix4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Clone for Matrix4 {
    fn clone(&self) -> Self {
        self.as_slice().into()
    }
}

impl AsRef<Matrix4> for Matrix4 {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl <T: AsRef<Matrix4>> Mul<T> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: T) -> Self::Output {
        let mut clone = self.clone();
        clone *= rhs.as_ref();
        clone
    }
}

impl <T: AsRef<Matrix4>> Mul<T> for &Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: T) -> Self::Output {
        let mut clone = self.clone();
        clone *= rhs.as_ref();
        clone
    }
}

impl <T: AsRef<Matrix4>> MulAssign<T> for Matrix4 {

    fn mul_assign(&mut self, other:T) {
        let values = &mut self.0;
        let a:&[f64] = values; 
        let b:&[f64] = other.as_ref().as_slice();
        let a00 = a[0]; 
        let a01 = a[1]; 
        let a02 = a[2];
        let a03 = a[3];
        let a10 = a[4]; 
        let a11 = a[5];
        let a12 = a[6]; 
        let a13 = a[7];
        let a20 = a[8];
        let a21 = a[9];
        let a22 = a[10];
        let a23 = a[11];
        let a30 = a[12];
        let a31 = a[13];
        let a32 = a[14];
        let a33 = a[15];
        let mut b0  = b[0];
        let mut b1 = b[1];
        let mut b2 = b[2];
        let mut b3 = b[3];

        values[0] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        values[1] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        values[2] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        values[3] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
        b0 = b[4]; b1 = b[5]; b2 = b[6]; b3 = b[7];
        values[4] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        values[5] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        values[6] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        values[7] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
        b0 = b[8]; b1 = b[9]; b2 = b[10]; b3 = b[11];
        values[8] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        values[9] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        values[10] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        values[11] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
        b0 = b[12]; b1 = b[13]; b2 = b[14]; b3 = b[15];
        values[12] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        values[13] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        values[14] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        values[15] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
    }

    
}




/*
pub fn identity() -> [f64;16] {
    let out:[f64;16] = [
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0
    ];
    out
}

pub fn translate(out:&mut [f64], translation_vec: &[f64]) {
    let x = translation_vec[0];
    let y = translation_vec[1];
    let z = translation_vec[2];
    out[12] = out[0] * x + out[4] * y + out[8] * z + out[12];
    out[13] = out[1] * x + out[5] * y + out[9] * z + out[13];
    out[14] = out[2] * x + out[6] * y + out[10] * z + out[14];
    out[15] = out[3] * x + out[7] * y + out[11] * z + out[15];
}
pub fn scale(out:&mut [f64], scale_vec: &[f64]) {
    let x = scale_vec[0];
    let y = scale_vec[1];
    let z = scale_vec[2];
    out[0] *= x;
    out[1] *= x;
    out[2] *= x;
    out[3] *= x;
    out[4] *= y;
    out[5] *= y;
    out[6] *= y;
    out[7] *= y;
    out[8] *= z;
    out[9] *= z;
    out[10] *= z;
    out[11] *= z;
}

pub fn rotate(out:&mut [f64], rotation_quat: &[f64]) {
}
pub fn from_quat(quat:&[f64]) -> [f64;16] {

  let x = quat[0];
  let y = quat[1];
  let z = quat[2];
  let w = quat[3];
  let x2 = x + x;
  let y2 = y + y;
  let z2 = z + z;
  let xx = x * x2;
  let yx = y * x2;
  let yy = y * y2;
  let zx = z * x2;
  let zy = z * y2;
  let zz = z * z2;
  let wx = w * x2;
  let wy = w * y2;
  let wz = w * z2;

  let out:[f64;16] = [
  1.0 - yy - zz,
  yx + wz,
  zx - wy,
  0.0,
  yx - wz,
  1.0 - xx - zz,
  zy + wx,
  0.0,
  zx + wy,
  zy - wx,
  1.0 - xx - yy,
  0.0,
  0.0,
  0.0,
  0.0,
  1.0
  ];

  out

}
//"borrowed"from https://glmatrix.net/docs/mat4.js.html#line1354
pub fn trso_to_mat4_series( translation_vec: &[f64], rotation_quat: &[f64], scale_vec: &[f64], origin_vec: &[f64]) -> [f64;16] {
    let mut mat = identity();
    translate(&mut mat, translation_vec);
    scale(&mut mat, scale_vec);
    rotate(&mut mat, rotation_quat);
    /*
    
    origin[0] = -(image.nativeWidth)/2;
    origin[1] = -(image.nativeHeight)/2;


    mat2d.identity(matrix);
    mat2d.translate(matrix, matrix, translation as any);
    mat2d.scale(matrix, matrix, scale as any);
    mat2d.rotate(matrix, matrix, image.rotation);
    mat2d.translate(matrix, matrix, origin as any);

    */

    mat
}

//"borrowed"from https://glmatrix.net/docs/mat4.js.html#line1354
pub fn trso_to_mat4_direct( translation: &[f64], rotation: &[f64], scale: &[f64], origin: &[f64]) -> [f64;16] {
    let q = &rotation;
    let v = &translation;
    let s = &scale;
    let o = &origin;


    let mut out:[f64;16] = [0.0;16];

      let x = q[0];
      let y = q[1];
      let z = q[2];
      let w = q[3];
      let x2 = x + x;
      let y2 = y + y;
      let z2 = z + z;
      let xx = x * x2;
      let xy = x * y2;
      let xz = x * z2;
      let yy = y * y2;
      let yz = y * z2;
      let zz = z * z2;
      let wx = w * x2;
      let wy = w * y2;
      let wz = w * z2;
      let sx = s[0];
      let sy = s[1];
      let sz = s[2];
      let ox = o[0];
      let oy = o[1];
      let oz = o[2];
      let out0 = (1.0 - (yy + zz)) * sx;
      let out1 = (xy + wz) * sx;
      let out2 = (xz - wy) * sx;
      let out4 = (xy - wz) * sy;
      let out5 = (1.0 - (xx + zz)) * sy;
      let out6 = (yz + wx) * sy;
      let out8 = (xz + wy) * sz;
      let out9 = (yz - wx) * sz;
      let out10 = (1.0 - (xx + yy)) * sz;
      out[0] = out0;
      out[1] = out1;
      out[2] = out2;
      out[3] = 0.0;
      out[4] = out4;
      out[5] = out5;
      out[6] = out6;
      out[7] = 0.0;
      out[8] = out8;
      out[9] = out9;
      out[10] = out10;
      out[11] = 0.0;
      out[12] = v[0] + ox - (out0 * ox + out4 * oy + out8 * oz);
      out[13] = v[1] + oy - (out1 * ox + out5 * oy + out9 * oz);
      out[14] = v[2] + oz - (out2 * ox + out6 * oy + out10 * oz);
      out[15] = 1.0;

      out
}
*/
