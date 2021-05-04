/*
 * There are some general math functions here
 * But the "normalization" is special for our needs
 * It uses a ResizeInfo to go back and forth
 * In this sense, while "normalization" does map some values to the 0->1 range
 * It's more about having a screen-size-agnostic storage format
 */

use crate::resize::ResizeInfo;
use shared::domain::jig::module::body::{Vec3, Vec4, Transform};
use super::mat4::Matrix4;

pub trait TransformExt {
    // everything from this point onwards is totally screen-space-agnostic
    fn to_screen_mat4(&self, resize_info: &ResizeInfo) -> Matrix4; 

    /// rotate around the z axis, in radians
    fn rotate_z(&mut self, angle:f64);


    fn get_translation_2d(&self) -> (f64, f64);
    fn set_translation_2d(&mut self, x: f64, y: f64);

    fn set_origin_2d(&mut self, x: f64, y: f64);
    
    fn get_scale_2d(&self) -> (f64, f64);
    fn set_scale_2d(&mut self, x: f64, y: f64);
    fn set_scale_x(&mut self, x: f64);
    fn set_scale_y(&mut self, y: f64);

}

pub trait RenderableMatrix {
    fn as_matrix_string(&self) -> String;
}

impl RenderableMatrix for [f64;16] {
    fn as_matrix_string(&self) -> String {
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
}
impl TransformExt for Transform {
    fn rotate_z(&mut self, mut rad:f64) {
        let quat = &mut self.rotation.0;
        rad *= 0.5;
        let ax = quat[0];
        let ay = quat[1];
        let az = quat[2];
        let aw = quat[3];
        let bz = rad.sin(); 
        let bw = rad.cos();
        quat[0] = ax * bw + ay * bz;
        quat[1] = ay * bw - ax * bz;
        quat[2] = az * bw + aw * bz;
        quat[3] = aw * bw - az * bz;
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

    fn get_scale_2d(&self) -> (f64, f64) {
        let scale = &self.scale.0;
        (scale[0], scale[1])
    }
    fn set_origin_2d(&mut self, x: f64, y: f64) {
        let origin = &mut self.origin.0;
        origin[0] = x;
        origin[1] = y;
    }

    fn set_scale_x(&mut self, x: f64) {
        let scale = &mut self.scale.0;
        scale[0] = x;
    }
    fn set_scale_y(&mut self, y: f64) {
        let scale = &mut self.scale.0;
        scale[1] = y;
    }
    fn set_scale_2d(&mut self, x: f64, y: f64) {
        let scale = &mut self.scale.0;
        scale[0] = x;
        scale[1] = y;
    }

    /// Return the Transform as a 4x4 Matrix
    /// Takes into account that coordinates are normalized
    fn to_screen_mat4(&self, resize_info: &ResizeInfo) -> Matrix4 {


        let mut translation = self.translation.0;
        let mut rotation = self.rotation.0;
        let mut scale = self.scale.0;
        let mut origin = self.origin.0;

        let (tx, ty) = resize_info.get_pos_denormalized(translation[0], translation[1]);

        translation[0] = tx; 
        translation[1] = ty; 

        //scale[0] *= resize_info.scale;
        //scale[1] *= resize_info.scale;

        Matrix4::new_from_trs_origin(&translation, &rotation, &scale, &origin)
    }
}
