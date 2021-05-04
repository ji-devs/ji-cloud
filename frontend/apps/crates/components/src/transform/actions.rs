use super::state::*;
use utils::drag::Drag;
use std::rc::Rc;
use utils::{prelude::*, resize::get_resize_info, math};

impl TransformState {
    pub fn start_tracking_action(&self, action: Action, x: i32, y:i32) {
        *self.action.borrow_mut() = Some(action);

        let (anchor_x, anchor_y) = match action {
            Action::Move => {
                let resize_info = get_resize_info();
                let (screen_x, screen_y) = resize_info.get_pos_px(x as f64, y as f64);

                let (tx, ty) = self.transform.lock_ref().get_translation_2d();
                let (tx, ty) = resize_info.get_pos_denormalized(tx, ty);

                (screen_x - tx, screen_y - ty)
            },
            _ => (0.0, 0.0)
        };

        match action {
            Action::Rotate => {
                *self.rot_stash.borrow_mut() = Some(InitRotation {
                    vec_to_center: self.calc_vec_to_center(x, y)
                });
            },
            Action::Scale(_, _) => {


                *self.scale_stash.borrow_mut() = Some(InitScale {
                    vec_to_center: self.calc_vec_to_center(x, y),
                    transform: self.transform.get_cloned(),
                });
            }
            _ => {}
        }
        

        self.drag.set(Some(Drag::new(x, y, anchor_x, anchor_y)));
    }

    pub fn mouse_move(&self, x: i32, y:i32) {
        if let Some(drag) = &*self.drag.lock_ref() {
            if let Some((pos, diff)) = drag.update(x, y) {
                if let Some(action) = self.action.borrow().as_ref() {
                    match action {
                        Action::Move => {
                            let resize_info = get_resize_info();
                            let (pos_x, pos_y) = resize_info.get_pos_normalized(pos.x as f64, pos.y as f64);
                            //log::info!("{} {} -> {} {}", pos.x, pos.y, pos_x, pos_y);
                            let mut transform = self.transform.lock_mut();
                            transform.set_translation_2d(pos_x, pos_y);
                        },

                        Action::Rotate => {
                            let resize_info = get_resize_info();
                            let new_vec = self.calc_vec_to_center(pos.x, pos.y);
                            let old_vec = self.rot_stash.borrow().as_ref().unwrap_ji().vec_to_center.clone();
                            let mut transform = self.transform.lock_mut();

                            let mut angle = math::vec2::angle(&old_vec, &new_vec);

                            if angle != f64::NAN {
                                if math::vec2::cross_value(&old_vec, &new_vec) < 0.0 {
                                    angle = -angle;
                                }
                                transform.rotate_z(angle);

                                *self.rot_stash.borrow_mut() = Some(InitRotation {
                                    vec_to_center: new_vec
                                });
                            }
                        },

                        Action::Scale(from, maintain) => {

                            /*
                             *
                             * TODO - scratch all this and rewrite by some math
                             * that takes into account the vector through the transform point
                             *
                             * current system breaks when:
                             *
                             * 1. Doing a stretch and then corner scale
                             * 2. Doing a rotation and then a stretch
                             */

                            let new_vec = self.calc_vec_to_center(pos.x, pos.y);
                            let orig_vec = self.orig_tp_to_center(*from);

                            let mut transform = self.transform.lock_mut();

                            match from {
                                ScaleFrom::Left | ScaleFrom::Right => {
                                    let perc_x = (new_vec[0] - orig_vec[0]) / orig_vec[0];
                                    transform.set_scale_x(1.0 + perc_x);
                                },
                                ScaleFrom::Top | ScaleFrom::Bottom => {
                                    let perc_y = (new_vec[1] - orig_vec[1]) / orig_vec[1];
                                    transform.set_scale_y(1.0 + perc_y);
                                },
                                _ => {
                                    let new_len = math::vec2::len(&new_vec);
                                    let orig_len = math::vec2::len(&orig_vec);

                                    let perc = ((new_len - orig_len) / orig_len);
                                    
                                    let init = self.scale_stash.borrow_mut();
                                    let init = init.as_ref().unwrap_ji();

                                    let init_scale = init.transform.scale.0;

                                    transform.set_scale_2d(1.0 + perc, 1.0 + perc);

                                }
                            }

                        },
                    }
                }
                
            }
        }
    }

    fn orig_tp_to_center(&self, from: ScaleFrom) -> [f64;2] {
        let resize_info = get_resize_info();

        let (width, height) = self.size.get_cloned().unwrap_ji();

        let (width, height) = (width * resize_info.scale, height * resize_info.scale);
        
        let v = match from {
            ScaleFrom::Right => [width/2.0, 0.0],
            ScaleFrom::Left => [-width/2.0, 0.0],
            ScaleFrom::Top => [0.0, -height/2.0],
            ScaleFrom::Bottom => [0.0, height/2.0],
            ScaleFrom::TopLeft=> [-width/2.0, -height/2.0],
            ScaleFrom::TopRight=> [width/2.0, -height/2.0],
            ScaleFrom::BottomLeft=> [-width/2.0, height/2.0],
            ScaleFrom::BottomRight=> [width/2.0, height/2.0],
        };

        let q = &self.transform.lock_ref().rotation.0;

        math::vec2::rotate_by_quat(&v, q)
    }

    fn calc_vec_to_center(&self, viewport_x: i32, viewport_y: i32) -> [f64;2] {

        let resize_info = get_resize_info();
        let (center_x, center_y) = self.get_center(&resize_info);
        let (pos_x, pos_y) = resize_info.get_pos_px(viewport_x as f64, viewport_y as f64);

        [pos_x - center_x, pos_y - center_y]

    }

    pub fn mouse_up(&self, x: i32, y:i32) {
        if let Some(drag) = self.drag.replace(None) {
            
        }
    }
}

