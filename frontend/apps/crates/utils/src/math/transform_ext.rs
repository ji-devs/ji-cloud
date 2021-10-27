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

use super::mat4::Matrix4;
use crate::resize::ResizeInfo;
use shared::domain::jig::module::body::{Transform, Vec3, Vec4};

pub trait TransformExt: Clone {
    fn to_mat4(&self) -> Matrix4;

    /// Create a new Transform
    fn identity() -> Self;

    /// rotate around the z axis, in radians
    fn rotate_z(&mut self, angle: f64);

    fn get_translation_2d(&self) -> (f64, f64);
    fn set_translation_2d(&mut self, x: f64, y: f64);
    fn add_translation_2d(&mut self, x: f64, y: f64) {
        let (tx, ty) = self.get_translation_2d();
        self.set_translation_2d(tx + x, ty + y);
    }

    fn set_origin_2d(&mut self, x: f64, y: f64);

    fn get_scale_2d(&self) -> (f64, f64);
    fn set_scale_2d(&mut self, x: f64, y: f64);
    fn set_scale_x(&mut self, x: f64);
    fn set_scale_y(&mut self, y: f64);

    fn set_scale_identity(&mut self);
    fn set_translation_identity(&mut self);
    fn set_rotation_identity(&mut self);

    fn get_denormalized_translation_2d(&self, resize_info: &ResizeInfo) -> (f64, f64);

    fn denormalize(&mut self, resize_info: &ResizeInfo);

    fn denormalize_matrix_string(&self, resize_info: &ResizeInfo) -> String;
    fn scale_only(&self) -> Self;
    fn rotation_only(&self) -> Self;
    fn scale_matrix_string(&self) -> String;
    fn rotation_matrix_string(&self) -> String;
    fn invert_rotation_matrix_string(&self) -> String;

    fn nudge_for_duplicate(&mut self);

    fn map_offset(&self, offset_x: f64, offset_y: f64) -> Self {
        self.map(|t| {
            let mut t = t.clone();
            let (tx, ty) = t.get_translation_2d();
            t.set_translation_2d(tx + offset_x, ty + offset_y);
            t
        })
    }

    fn map<A>(&self, f: impl FnOnce(&Self) -> A) -> A {
        f(self)
    }
}

pub trait RenderableMatrix {
    fn as_matrix_string(&self) -> String;
}

impl RenderableMatrix for [f64; 16] {
    fn as_matrix_string(&self) -> String {
        let mat = self;

        format!(
            "matrix3d({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
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

    fn nudge_for_duplicate(&mut self) {
        let translation = &mut self.translation.0;

        //TODO - make the nudging random
        translation[0] += 0.01;
        translation[1] -= 0.01;
    }

    fn denormalize_matrix_string(&self, resize_info: &ResizeInfo) -> String {
        let mut t = self.clone();
        t.denormalize(resize_info);
        t.to_mat4().as_matrix_string()
    }

    //CSS requires the full 4x4 or 6-element 2d matrix, so we return the whole thing
    //but set the rotation and translation to identity
    fn scale_only(&self) -> Self {
        let mut t = self.clone();
        t.set_rotation_identity();
        t.set_translation_identity();
        t
    }

    fn scale_matrix_string(&self) -> String {
        self.scale_only().to_mat4().as_matrix_string()
    }
    //CSS requires the full 4x4 or 6-element 2d matrix, so we return the whole thing
    //but set the scale and translation to identity
    fn rotation_only(&self) -> Self {
        let mut t = self.clone();
        t.set_scale_identity();
        t.set_translation_identity();
        t
    }
    fn rotation_matrix_string(&self) -> String {
        self.rotation_only().to_mat4().as_matrix_string()
    }
    fn invert_rotation_matrix_string(&self) -> String {
        let mut t = self.clone();
        t.set_scale_identity();
        t.set_translation_identity();
        t.rotation.0 = super::quat::invert(&t.rotation.0);
        t.to_mat4().as_matrix_string()
    }
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

    fn rotate_z(&mut self, mut rad: f64) {
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

    fn get_denormalized_translation_2d(&self, resize_info: &ResizeInfo) -> (f64, f64) {
        resize_info.get_pos_denormalized(self.translation.0[0], self.translation.0[1])
    }
    /// Takes into account that translation coordinates are normalized
    fn denormalize(&mut self, resize_info: &ResizeInfo) {
        let translation = &mut self.translation.0;
        let _rotation = &mut self.rotation.0;
        let _scale = &mut self.scale.0;
        let _origin = &mut self.origin.0;

        let (tx, ty) = resize_info.get_pos_denormalized(translation[0], translation[1]);

        translation[0] = tx;
        translation[1] = ty;
    }
    fn to_mat4(&self) -> Matrix4 {
        Matrix4::new_from_trs_origin(
            &self.translation.0,
            &self.rotation.0,
            &self.scale.0,
            &self.origin.0,
        )
    }
}
