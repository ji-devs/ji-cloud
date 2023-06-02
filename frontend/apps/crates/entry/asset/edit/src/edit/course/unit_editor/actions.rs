use std::rc::Rc;

use crate::edit::sidebar::SidebarSpot;
use dominator::clone;
use shared::{
    api::endpoints::course::unit,
    domain::course::unit::{
        CourseUnit, CourseUnitCreateRequest, CourseUnitId, CourseUnitUpdateRequest,
        CourseUnitValue, CreateCourseUnitPath, UpdateCourseUnitPath,
    },
};
use utils::{
    prelude::ApiEndpointExt,
    routes::{AssetEditRoute, CourseEditRoute},
    unwrap::UnwrapJiExt,
};

use super::UnitEditor;

const STR_ADD_TO_PLAYLIST: &str = "Add to playlist";
const STR_UPDATE: &str = " Update unit ";

impl UnitEditor {
    pub fn load_unit(self: &Rc<Self>) {
        if let Some(unit_id) = self.unit_id {
            let units = self.editable_course.units.lock_ref();
            let unit = units.iter().find(|x| x.id == unit_id);

            match unit {
                Some(unit) => {
                    self.display_name.set(unit.display_name.clone());
                    self.description.set(unit.description.clone());
                    self.value.set(unit.value.clone().into());
                }
                None => {}
            }
        };
    }

    pub async fn create_async(self: &Rc<Self>) -> Result<CourseUnitId, String> {
        let state = Rc::clone(&self);

        let body = CourseUnitCreateRequest {
            display_name: self.display_name.lock_ref().clone(),
            description: self.description.lock_ref().clone(),
            value: CourseUnitValue::try_from(self.value.lock_ref().clone()).unwrap_ji(),
        };

        let res = unit::Create::api_with_auth(
            CreateCourseUnitPath(state.asset_edit_state.asset_id.unwrap_course().clone()),
            Some(body),
        )
        .await;

        //
        // Update index for CourseUnit after created
        //
        match res {
            Ok(resp) => {
                let id = resp.id;

                let target_index = state
                    .asset_edit_state
                    .target_index
                    .lock_ref()
                    .clone()
                    .map(|x| x as u16);

                let body = CourseUnitUpdateRequest {
                    display_name: None,
                    description: None,
                    value: None,
                    index: target_index,
                };

                let _ = unit::Update::api_with_auth_empty(
                    UpdateCourseUnitPath(
                        state.asset_edit_state.asset_id.unwrap_course().clone(),
                        id,
                    ),
                    Some(body),
                )
                .await;

                state.asset_edit_state.route.set(AssetEditRoute::Course(
                    state.asset_edit_state.asset_id.unwrap_course().clone(),
                    CourseEditRoute::Unit(Some(id)),
                ));

                Ok(id)
            }
            Err(e) => {
                log::error!("create_async(): create failed: {}", e);
                Err(format!("create failed: {}", e))
            }
        }
    }

    pub fn create_unit(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            match state.create_async().await {
                Ok(unit_id) => {
                    state.changed.set(false);
                    let mut units = state.editable_course.units.lock_mut();
                    let mut spots = state.asset_edit_state.sidebar_spots.lock_mut();

                    let unit_index = state.asset_edit_state.target_index.get();

                    if let Some(unit_index) = unit_index {
                        let unit = CourseUnit {
                            id: unit_id,
                            display_name: state.display_name.lock_ref().clone(),
                            description: state.description.lock_ref().clone(),
                            value: CourseUnitValue::try_from(state.value.lock_ref().clone()).unwrap_ji(),
                        };

                        units.insert_cloned(unit_index, unit.clone());

                        let spot_index = unit_index + 1;

                        spots.remove(spot_index);

                        spots.insert_cloned(spot_index, SidebarSpot::new_course_unit(unit.clone()));

                        state.asset_edit_state.route.set(AssetEditRoute::Course(
                            state.asset_edit_state.asset_id.unwrap_course().clone(),
                            CourseEditRoute::Unit(Some(unit_id)),
                        ));
                    }
                },
                Err(msg) => {
                    log::error!("{}", msg);
                }
            }
        }));
    }

    pub async fn update_async(&self) -> Result<(), String> {
        let state = self;

        let body = CourseUnitUpdateRequest {
            display_name: Some(self.display_name.lock_ref().clone()),
            description: Some(self.description.lock_ref().clone()),
            value: Some(CourseUnitValue::try_from(self.value.lock_ref().clone()).unwrap_ji()),
            index: None,
        };

        if let Some(unit_id) = self.unit_id {
            let res = unit::Update::api_with_auth_empty(
                UpdateCourseUnitPath(
                    state.asset_edit_state.asset_id.unwrap_course().clone(),
                    unit_id,
                ),
                Some(body),
            )
            .await;

            match res {
                Ok(_) => {
                    log::info!("update_async(): update successful");
                    Ok(())
                }
                Err(e) => {
                    log::error!("update_async(): update failed: {}", e);
                    Err(format!("update failed: {}", e))
                }
            }
        } else {
            Err(String::from("update_async(): unit ID is None"))
        }
    }

    pub fn update_unit(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            if let Err(msg) = state.update_async().await {
                log::error!("{}", msg);
            } else {
                // deactivate unit submit button request has completed
                state.changed.set(false);
                let mut units = state.editable_course.units.lock_mut();
                let mut spots = state.asset_edit_state.sidebar_spots.lock_mut();


                let unit_index = units.iter().position(|x| x.id == state.unit_id.unwrap_ji());


                if let Some(unit_index) = unit_index {
                    let mut unit = units.remove(unit_index);

                    unit.display_name = state.display_name.get_cloned();
                    unit.description = state.description.get_cloned();
                    unit.value = CourseUnitValue::try_from(state.value.get_cloned()).unwrap_ji();

                    // replace sidebar spot with new data
                    units.insert_cloned(unit_index, unit.clone());
                    let spot_index = unit_index + 1;

                    // replace sidebar spot with new data
                    spots.remove(spot_index);

                    spots.insert_cloned(spot_index, SidebarSpot:: new_course_unit(unit));
                }


                state.asset_edit_state.route.set(AssetEditRoute::Course(
                    state.asset_edit_state.asset_id.unwrap_course().clone(),
                    CourseEditRoute::Unit(state.unit_id),
                ));
            }
        }));

        log::info!("Success");
    }

    pub fn on_display_name_change(self: &Rc<Self>, new_name: &str) {
        self.changed.set(true);
        self.display_name.set(new_name.to_string());

        log::info!("display_name {:?}", self.display_name);
    }

    pub fn on_description_change(self: &Rc<Self>, new_description: &str) {
        self.changed.set(true);
        self.description.set(new_description.to_string());

        log::info!("description {:?}", self.description);
    }

    //
    //TODO: Delete previous file after new file is assigned to a unit
    //
    // pub async fn update_value(self: &Rc<Self>, ) {
    //     let state = self.clone();

    //     state.loader.load(clone!(state => async move {
    //             let _ = state.map(clone!(state => async move {
    //                 match self.value.signal_cloned() {
    //                     super::UnitValue::File(file) => {
    //                         match file {
    //                             Some(val) => match val {
    //                                 super::UnitValueFile::ImageId(id) => {
    //                                     delete_image(id).await;
    //                                 }
    //                                 super::UnitValueFile::AudioId(id) => {
    //                                     delete_audio(id).await;
    //                                 }
    //                                 super::UnitValueFile::PdfId(id) => {
    //                                     delete_pdf(id).await;
    //                                 }
    //                             },
    //                             None => todo!(),
    //                         }
    //                     },
    //                     super::UnitValue::Link(_) => todo!(),
    //                     super::UnitValue::Video(_) => todo!(),
    //                 }
    //             }));
    //     }));
    // }

    pub fn create_or_update_text(&self) -> String {
        match self.unit_id {
            Some(_) => STR_UPDATE.to_string(),
            None => STR_ADD_TO_PLAYLIST.to_string(),
        }
    }
}

// pub async fn delete_image(image_id: Option<ImageId>) -> Result<(), String> {
//     let id = match image_id {
//         Some(id) => id,
//         None => todo!(),
//     };
//     let res = endpoints::image::user::Delete::api_no_auth_empty(UserImageDeletePath(id), None)
//         .await;

//     match res {
//         Ok(_) => {
//             log::info!("update_async(): update successful");
//             Ok(())
//         }
//         Err(e) => {
//             log::error!("update_async(): update failed: {}", e);
//             Err(format!("update failed: {}", e))
//         }
//     }
// }

// pub async fn delete_audio(audio_id: Option<AudioId>) -> Result<(), String> {
//     let id = match audio_id {
//         Some(id) => id,
//         None => todo!(),
//     };

//     let res = endpoints::audio::user::Delete::api_no_auth_empty(UserAudioDeletePath(id), None)
//         .await;

//     match res {
//         Ok(_) => {
//             log::info!("update_async(): update successful");
//             Ok(())
//         }
//         Err(e) => {
//             log::error!("update_async(): update failed: {}", e);
//             Err(format!("update failed: {}", e))
//         }
//     }
// }

// pub async fn delete_pdf(pdf_id: Option<PdfId>) -> Result<(), String> {
//     let id = match pdf_id {
//         Some(id) => id,
//         None => todo!(),
//     };

//     let res = endpoints::pdf::user::Delete::api_no_auth_empty(UserPdfDeletePath(id), None)
//         .await;

//     match res {
//         Ok(_) => {
//             log::info!("update_async(): update successful");
//             Ok(())
//         }
//         Err(e) => {
//             log::error!("update_async(): update failed: {}", e);
//             Err(format!("update failed: {}", e))
//         }
//     }
// }
