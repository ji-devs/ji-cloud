use super::state::*;
use utils::drag::Drag;

use utils::{math, prelude::*, resize::get_resize_info};

//really this should be a min size, depends on the artwork
//but this will do for now
//the idea is we can't let them scale down so much that the controls look weird / dissapear
const MIN_SCALE_PERC: f64 = 0.1;

impl TransformState {
    pub fn set_to_center(&self) {
        let transform = &mut self.transform.lock_mut();
        let resize_info = get_resize_info();
        let (width, height) = self.size.get_cloned().unwrap_ji();
        let (width, height) = resize_info.get_size_normalized(width, height);

        let x = 0.5 - (width / 2.0);
        let y = 0.5 - (height / 2.0);

        transform.set_translation_2d(x, y);
    }

    pub fn stop_tracking_action(&self, _x: i32, _y: i32) {
        if let Some(_drag) = self.drag.replace(None) {
            if self.action.borrow().is_some() {
                if let Some(on_action_finished) = &self.callbacks.on_action_finished {
                    on_action_finished(self.transform.get_cloned());
                }
            }
            *self.action.borrow_mut() = None;
            self.is_transforming.set_neq(false);
        }
    }
    pub fn start_tracking_action(&self, action: Action, x: i32, y: i32) {
        *self.action.borrow_mut() = Some(action);

        self.is_transforming.set_neq(true);
        let (anchor_x, anchor_y) = match action {
            Action::Move => {
                let resize_info = get_resize_info();
                let (screen_x, screen_y) = resize_info.get_pos_px(x as f64, y as f64);

                let (tx, ty) = self.transform.lock_ref().get_translation_2d();
                let (tx, ty) = resize_info.get_pos_denormalized(tx, ty);

                (screen_x - tx, screen_y - ty)
            }
            _ => (0.0, 0.0),
        };

        match action {
            Action::Rotate => {
                *self.rot_stash.borrow_mut() = Some(InitRotation {
                    vec_to_point: self.center_to_point_vec(x, y),
                });
            }
            Action::Scale(from, _) => {
                let (basis_vec_x, basis_vec_y) = self.get_basis_vectors(from);

                *self.scale_stash.borrow_mut() = Some(InitScale {
                    basis_vec_x,
                    basis_vec_y,
                    vec_to_point: self.center_to_point_vec(x, y),
                    scale: self.transform.lock_ref().get_scale_2d(),
                });
            }
            _ => {}
        }

        self.drag
            .set(Some(Drag::new(x, y, anchor_x, anchor_y, false)));
    }

    pub fn mouse_move(&self, x: i32, y: i32) {
        if let Some(drag) = &*self.drag.lock_ref() {
            if let Some((pos, _diff)) = drag.update(x, y) {
                if let Some(action) = self.action.borrow().as_ref() {
                    match action {
                        Action::Move => {
                            //moving is just translating by the drag amount
                            //as normalized coordinates
                            let resize_info = get_resize_info();
                            let (pos_x, pos_y) =
                                resize_info.get_pos_normalized(pos.x as f64, pos.y as f64);
                            let mut transform = self.transform.lock_mut();
                            transform.set_translation_2d(pos_x, pos_y);
                        }

                        Action::Rotate => {
                            //rotation is calculated by taking a vector from center to the current cursor
                            //and getting the angle between that and the same thing from last move
                            let _resize_info = get_resize_info();
                            let new_vec = self.center_to_point_vec(pos.x, pos.y);
                            let old_vec = self.rot_stash.borrow().as_ref().unwrap_ji().vec_to_point;
                            let mut transform = self.transform.lock_mut();

                            let mut angle = math::vec2::angle(&old_vec, &new_vec);

                            if angle != f64::NAN {
                                if math::vec2::cross_value(&old_vec, &new_vec) < 0.0 {
                                    angle = -angle;
                                }
                                transform.rotate_z(angle);

                                *self.rot_stash.borrow_mut() = Some(InitRotation {
                                    vec_to_point: new_vec,
                                });
                            }
                        }

                        Action::Scale(from, lock_aspect) => {
                            //get a vector from the center to the cursor
                            let curr_point_vec = self.center_to_point_vec(pos.x, pos.y);

                            let scale_stash = self.scale_stash.borrow();
                            let scale_stash = scale_stash.as_ref().unwrap_ji();

                            let (init_x, init_y) = scale_stash.scale;
                            let init_vec = &scale_stash.vec_to_point;

                            let mut transform = self.transform.lock_mut();

                            let get_curr_perc = |v: &[f64]| {
                                //get the amount that our cursor vector is
                                //along the length of the comparison vector
                                let proj_len = math::vec2::project(&curr_point_vec, v);
                                let orig_len = math::vec2::len(v);

                                //as a percentage
                                proj_len / orig_len
                            };

                            let perc = get_curr_perc(init_vec);
                            let scale_x = init_x * perc;
                            let scale_y = init_y * perc;

                            match from {
                                ScaleFrom::Left | ScaleFrom::Right => {
                                    if perc >= MIN_SCALE_PERC {
                                        transform.set_scale_x(scale_x);
                                    }
                                }
                                ScaleFrom::Top | ScaleFrom::Bottom => {
                                    if perc >= MIN_SCALE_PERC {
                                        transform.set_scale_y(scale_y);
                                    }
                                }
                                _ => {
                                    if !*lock_aspect {
                                        //when doing free-form transformations,
                                        //we need to compare to the original basis vectors
                                        let basis_vec_x = self
                                            .scale_stash
                                            .borrow()
                                            .as_ref()
                                            .unwrap_ji()
                                            .basis_vec_x;
                                        let basis_vec_y = self
                                            .scale_stash
                                            .borrow()
                                            .as_ref()
                                            .unwrap_ji()
                                            .basis_vec_y;
                                        let perc_x = get_curr_perc(&basis_vec_x);
                                        let perc_y = get_curr_perc(&basis_vec_y);
                                        if perc_x >= MIN_SCALE_PERC && perc_y >= MIN_SCALE_PERC {
                                            transform.set_scale_2d(perc_x, perc_y);
                                        }
                                    } else if scale_x >= MIN_SCALE_PERC && scale_y >= MIN_SCALE_PERC
                                    {
                                        transform.set_scale_2d(scale_x, scale_y);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn close_menu(&self) {
        self.menu_pos.set(None);
    }

    fn get_basis_vectors(&self, from: ScaleFrom) -> ([f64; 2], [f64; 2]) {
        let resize_info = get_resize_info();

        let (width, height) = self.size.get_cloned().unwrap_ji();

        let (width, height) = (width * resize_info.scale, height * resize_info.scale);

        //first get the vector from the center to the unmodified coordinates
        let vx = match from {
            ScaleFrom::Right | ScaleFrom::TopRight | ScaleFrom::BottomRight => [width / 2.0, 0.0],

            ScaleFrom::Left | ScaleFrom::TopLeft | ScaleFrom::BottomLeft => [-width / 2.0, 0.0],
            _ => [0.0, 0.0],
        };

        let vy = match from {
            ScaleFrom::Top | ScaleFrom::TopRight | ScaleFrom::TopLeft => [0.0, -height / 2.0],
            ScaleFrom::Bottom | ScaleFrom::BottomRight | ScaleFrom::BottomLeft => {
                [0.0, height / 2.0]
            }
            _ => [0.0, 0.0],
        };

        //then modify it by the current transform
        let q = &self.transform.lock_ref().rotation.0;

        let vx = math::vec2::rotate_by_quat(&vx, q);
        let vy = math::vec2::rotate_by_quat(&vy, q);

        (vx, vy)
    }

    fn center_to_point_vec(&self, viewport_x: i32, viewport_y: i32) -> [f64; 2] {
        let resize_info = get_resize_info();
        let (center_x, center_y) = self.get_center(&resize_info);
        let (pos_x, pos_y) = resize_info.get_pos_px(viewport_x as f64, viewport_y as f64);

        [pos_x - center_x, pos_y - center_y]
    }
}
