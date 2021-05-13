/*
 * There are some general math functions here
 * But the "normalization" is special for our needs
 * It uses a ResizeInfo to go back and forth
 * While "normalization" does map some values to the 0->1 range
 * It's more about having a screen-size-agnostic storage format
 *
 * The Transform itself shouldn't do any math, it's only a
 * storage format
 *
 * Heavy lifting of math should be done by getting the inner slices
 * or Matrix4, and handing that off to math utils (native or 3rd party)
 */

use crate::resize::ResizeInfo;
use shared::domain::jig::module::body::{Vec3, Vec4, Transform};
use super::mat4::Matrix4;

pub trait TransformExt {
    fn to_mat4(&self) -> Matrix4;

    /// Create a new Transform
    fn identity() -> Self;

    /// rotate around the z axis, in radians
    fn rotate_z(&mut self, angle:f64);


    fn get_translation_2d(&self) -> (f64, f64);
    fn set_translation_2d(&mut self, x: f64, y: f64);

    fn set_origin_2d(&mut self, x: f64, y: f64);
    
    fn get_scale_2d(&self) -> (f64, f64);
    fn set_scale_2d(&mut self, x: f64, y: f64);
    fn set_scale_x(&mut self, x: f64);
    fn set_scale_y(&mut self, y: f64);

    fn set_scale_identity(&mut self);
    fn set_translation_identity(&mut self);
    fn set_rotation_identity(&mut self);

    fn denormalize_translation(&mut self, resize_info: &ResizeInfo);

    fn map<A>(&self, f: impl FnOnce(&Self) -> A) -> A {
        f(&self)
    }
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

            //translation: Vec3([0.0, 0.0, 0.0]),
            //rotation: Vec4([0.0, 0.0, 0.0, 1.0]),
            //scale: Vec3([1.0, 1.0, 1.0]),
            //origin: Vec3([0.0, 0.0, 0.0]),


    /// Create a new Transform
    fn identity() -> Self {
        Self {
            translation: Vec3([0.0, 0.0, 0.0]),
            rotation: Vec4([0.0, 0.0, 0.0, 1.0]),
            scale: Vec3([1.0, 1.0, 1.0]),
            origin: Vec3([0.0, 0.0, 0.0]),
        }
    }

    fn set_scale_identity(&mut self) {
        self.scale.0 = [1.0, 1.0, 1.0];
    }
    fn set_translation_identity(&mut self) {
        self.translation.0 = [0.0, 0.0, 0.0];
    }
    fn set_rotation_identity(&mut self) {
        self.rotation.0 = [0.0, 0.0, 0.0, 1.0];
    }

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

    /// Takes into account that translation coordinates are normalized
    fn denormalize_translation(&mut self, resize_info: &ResizeInfo) {


        let mut translation = &mut self.translation.0;
        let mut rotation = &mut self.rotation.0;
        let mut scale = &mut self.scale.0;
        let mut origin = &mut self.origin.0;

        let (tx, ty) = resize_info.get_pos_denormalized(translation[0], translation[1]);


        translation[0] = tx; 
        translation[1] = ty; 

    }
    fn to_mat4(&self) -> Matrix4 {
        Matrix4::new_from_trs_origin(
            &self.translation.0, 
            &self.rotation.0, 
            &self.scale.0,
            &self.origin.0
        )
    }
}
