use super::transform_2d;
use crate::resize::ResizeInfo;
use shared::domain::jig::module::body::{Vec3, Vec4, Transform};

pub trait TransformExt {
    fn to_mat4(&self) -> [f64;16];

    fn denormalize_2d(&self, resize_info: &ResizeInfo) -> Self;

    fn get_translation_2d(&self) -> (f64, f64);
    fn set_translation_2d(&mut self, x: f64, y: f64);

    fn set_scale_2d(&mut self, x: f64, y: f64);

    fn to_matrix_string(&self) -> String {
        let mat = self.to_mat4();

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
}

impl TransformExt for Transform {
    fn denormalize_2d(&self, resize_info: &ResizeInfo) -> Self {

        let mut translation = self.translation.0;
        let mut rotation = self.rotation.0;
        let mut scale = self.scale.0;
        let mut origin = self.origin.0;

        let (tx, ty) = resize_info.get_pos_denormalized(translation[0], translation[1]);

        translation[0] = tx; 
        translation[1] = ty; 

        Self {
            translation: Vec3(translation),
            rotation: Vec4(rotation),
            scale: Vec3(scale),
            origin: Vec3(origin),
        }

    }
    fn get_translation_2d(&self) -> (f64, f64) {
        let translation = &self.translation.0;
        (translation[0], translation[1])
    }
    fn set_translation_2d(&mut self, x: f64, y: f64) {
        let translation = &mut self.translation.0;
        translation[0] = x;
        translation[1] = y;
    }

    fn set_scale_2d(&mut self, x: f64, y: f64) {
        let scale = &mut self.scale.0;
        scale[0] = x;
        scale[1] = y;
    }

    /// Return the Transform as a 4x4 Matrix
    /// This is merely a convenience.
    /// Heavier lifting should take it from here via third-party libraries
    /// Or extension methods
    fn to_mat4(&self) -> [f64;16] {
        let q = self.rotation.0;
        let v = self.translation.0;
        let s = self.scale.0;
        let o = self.origin.0;

        let mut out:[f64;16] = [0.0;16];

        //"borrowed" from https://glmatrix.net/docs/mat4.js.html#line1354
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
}
