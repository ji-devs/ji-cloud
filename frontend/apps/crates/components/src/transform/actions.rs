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
                    vec_to_point: self.center_to_point_vec(x, y)
                });
            },
            Action::Scale(from, _) => {


                let (vec_to_tp_x, vec_to_tp_y) = self.center_to_tp_vecs(from);

                *self.scale_stash.borrow_mut() = Some(InitScale {
                    vec_to_tp_x,
                    vec_to_tp_y,
                    vec_to_point: self.center_to_point_vec(x, y),
                    scale: self.transform.lock_ref().get_scale_2d()
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
                            //moving is just translating by the drag amount
                            //as normalized coordinates
                            let resize_info = get_resize_info();
                            let (pos_x, pos_y) = resize_info.get_pos_normalized(pos.x as f64, pos.y as f64);
                            let mut transform = self.transform.lock_mut();
                            transform.set_translation_2d(pos_x, pos_y);
                        },

                        Action::Rotate => {
                            //rotation is calculated by taking a vector from center to the current cursor
                            //and getting the angle between that and the same thing from last move 
                            let resize_info = get_resize_info();
                            let new_vec = self.center_to_point_vec(pos.x, pos.y);
                            let old_vec = self.rot_stash.borrow().as_ref().unwrap_ji().vec_to_point.clone();
                            let mut transform = self.transform.lock_mut();

                            let mut angle = math::vec2::angle(&old_vec, &new_vec);

                            if angle != f64::NAN {
                                if math::vec2::cross_value(&old_vec, &new_vec) < 0.0 {
                                    angle = -angle;
                                }
                                transform.rotate_z(angle);

                                *self.rot_stash.borrow_mut() = Some(InitRotation {
                                    vec_to_point: new_vec
                                });
                            }
                        },

                        //maintain isn't being used yet
                        //it's a placeholder for opting in/out of dual-side scaling
                        Action::Scale(from, maintain) => {

                            //hard coded for now
                            let free_form = match from {
                                ScaleFrom::Left | ScaleFrom::Right | ScaleFrom::Top | ScaleFrom::Bottom => {
                                    true 
                                },
                                _ => false 
                            };

                            //get a vector from the center to the cursor
                            let curr_point_vec = self.center_to_point_vec(pos.x, pos.y);

                            let get_curr_perc = |v: &[f64]| {
                                //get the amount that our cursor vector is 
                                //along the length of the comparison vector 
                                let proj_len = math::vec2::project(&curr_point_vec, v);
                                let orig_len = math::vec2::len(v);

                                //as a percentage
                                let perc = proj_len / orig_len;

                                let curr_len = math::vec2::len(&curr_point_vec);
                                log::info!("curr_len: {}, orig_len: {}, proj_len: {}, perc: {}", curr_len, orig_len, proj_len, perc);

                                perc
                            };

                            if free_form {
                                //when doing free-form transformations, 
                                //we can take an optimal and very clean approach of just comparing
                                //to the original basis vectors

                                //get a vector from the center through the transform point
                                let orig_vx = self.scale_stash.borrow().as_ref().unwrap_ji().vec_to_tp_x.clone();
                                let orig_vy = self.scale_stash.borrow().as_ref().unwrap_ji().vec_to_tp_y.clone();
                                //let orig_vec = self.center_to_tp_vec(*from);

                                let mut transform = self.transform.lock_mut();

                                match from {
                                    ScaleFrom::Left | ScaleFrom::Right => {
                                        let perc = get_curr_perc(&orig_vx);
                                        transform.set_scale_x(perc);
                                    },
                                    ScaleFrom::Top | ScaleFrom::Bottom => {
                                        let perc = get_curr_perc(&orig_vy);
                                        transform.set_scale_y(perc);
                                    },
                                    _ => {
                                        let perc_x = get_curr_perc(&orig_vx);
                                        let perc_y = get_curr_perc(&orig_vy);
                                        transform.set_scale_2d(perc_x, perc_y);
                                    }
                                }
                            } else {
                                match from {
                                    ScaleFrom::Left | ScaleFrom::Right => {
                                        //proportional scaling from side?
                                    },
                                    ScaleFrom::Top | ScaleFrom::Bottom => {
                                        //proportional scaling from top-bottom?
                                    },
                                    _ => {
                                        //For locked aspect ratio, we can't just compare to the
                                        //original basis vectors
                                        //rather, we need to dynamically track a new vector
                                        let scale_stash = self.scale_stash.borrow();
                                        let scale_stash = scale_stash.as_ref().unwrap_ji();

                                        let (init_x, init_y) = scale_stash.scale;
                                        let init_vec = &scale_stash.vec_to_point;

                                        let mut transform = self.transform.lock_mut();

                                        let perc = get_curr_perc(init_vec);

                                        transform.set_scale_2d(init_x * perc, init_y * perc);
                                    }
                                }
                            }

                        },
                    }
                }
                
            }
        }
    }

    fn center_to_tp_vecs(&self, from: ScaleFrom) -> ([f64;2], [f64;2]) {
        let resize_info = get_resize_info();

        let (width, height) = self.size.get_cloned().unwrap_ji();

        let (width, height) = (width * resize_info.scale, height * resize_info.scale);
       
        //first get the vector from the center to the unmodified coordinates
        let vx = match from {
            ScaleFrom::Right | ScaleFrom::TopRight | ScaleFrom::BottomRight => [width/2.0, 0.0],

            ScaleFrom::Left | ScaleFrom::TopLeft | ScaleFrom::BottomLeft => [-width/2.0, 0.0],
            _ => [0.0, 0.0]
        };

        let vy = match from {
            ScaleFrom::Top | ScaleFrom::TopRight | ScaleFrom::TopLeft => [0.0, -height/2.0],
            ScaleFrom::Bottom | ScaleFrom::BottomRight | ScaleFrom::BottomLeft => [0.0, height/2.0],
            _ => [0.0, 0.0]
        };

        let vd = match from {
            ScaleFrom::BottomRight => {
                [width/2.0, height/2.0]
            },
            _ => [0.0, 0.0]
        };
        //then modify it by the current transform
        let q = &self.transform.lock_ref().rotation.0;

        let vx = math::vec2::rotate_by_quat(&vx, q);
        let vy = math::vec2::rotate_by_quat(&vy, q);

        (vx, vy)
    }

    fn center_to_point_vec(&self, viewport_x: i32, viewport_y: i32) -> [f64;2] {

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

