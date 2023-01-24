use std::rc::Rc;

use dominator::clone;
use futures::future::join_all;
use wasm_bindgen_futures::spawn_local;

use crate::image_table::editable_image::EditableImage;

use super::{MassEditing, Mode};

impl MassEditing {
    pub fn save_changes(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            let selected = state.get_selected();
            state.apply_changes_on_images(&selected);
            state.save_all_images(&selected).await;
            state.images_state.mass_editing.set(false);
            state.images_state.clear_selected();
        }));
    }

    fn get_selected(self: &Rc<Self>) -> Vec<Rc<EditableImage>> {
        let selected_images = self.images_state.selected_images.lock_ref();
        self.images_state
            .images
            .lock_ref()
            .iter()
            .filter(|image| selected_images.contains(&image.id))
            .map(|image| Rc::clone(image))
            .collect()
    }

    fn apply_changes_on_images(self: &Rc<Self>, selected: &Vec<Rc<EditableImage>>) {
        match &*self.mode.lock_ref() {
            Mode::Add => {
                for image in selected {
                    let mut styles = image.styles.lock_mut();
                    for style in self.styles.lock_ref().iter() {
                        styles.insert(*style);
                    }

                    let mut tags = image.tags.lock_mut();
                    for tag in self.tags.lock_ref().iter() {
                        tags.insert(*tag);
                    }

                    let mut age_ranges = image.age_ranges.lock_mut();
                    for age in self.ages.lock_ref().iter() {
                        age_ranges.insert(*age);
                    }

                    let mut affiliations = image.affiliations.lock_mut();
                    for affiliation in self.affiliations.lock_ref().iter() {
                        affiliations.insert(*affiliation);
                    }

                    // let mut categories = image.categories.lock_mut();
                    // for category in self.categories.lock_ref().iter() {
                    //     categories.insert(*category);
                    // }
                }
            }
            Mode::Remove => {
                for image in selected {
                    let mut styles = image.styles.lock_mut();
                    for style in self.styles.lock_ref().iter() {
                        styles.remove(style);
                    }

                    let mut tags = image.tags.lock_mut();
                    for tag in self.tags.lock_ref().iter() {
                        tags.remove(tag);
                    }

                    let mut age_ranges = image.age_ranges.lock_mut();
                    for age in self.ages.lock_ref().iter() {
                        age_ranges.remove(age);
                    }

                    let mut affiliations = image.affiliations.lock_mut();
                    for affiliation in self.affiliations.lock_ref().iter() {
                        affiliations.remove(affiliation);
                    }

                    // let mut categories = image.categories.lock_mut();
                    // for category in self.categories.lock_ref().iter() {
                    //     categories.remove(category);
                    // }
                }
            }
        };
    }

    async fn save_all_images(self: &Rc<Self>, selected: &Vec<Rc<EditableImage>>) {
        join_all(selected.iter().map(|image| image.save())).await;
    }
}
