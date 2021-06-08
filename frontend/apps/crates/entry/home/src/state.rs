use std::{
    rc::Rc,
    iter
};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::jig::Jig;
use super::search_state::{SearchOptions, SearchSelected};


pub struct State {
    pub loader: AsyncLoader,
    pub mode: Mutable<HomePageMode>,
    pub search_options: SearchOptions,
    pub search_selected: SearchSelected,
    pub quick_searches: Vec<QuickSearch>,
    pub whats_new: Vec<WhatsNewItem>,
    pub parents_testimonials: Vec<Testimonial>,
    pub teachers_testimonials: Vec<Testimonial>,
}

impl State {
    pub fn new() -> Self {
        Self {
            loader: AsyncLoader::new(),
            mode: Mutable::new(HomePageMode::Home),
            search_options: SearchOptions::new(),
            search_selected: SearchSelected::new(),
            quick_searches: Self::get_quick_searches(),
            whats_new: Self::get_whats_new(),
            parents_testimonials: Self::get_parents_testimonials(),
            teachers_testimonials: Self::get_teachers_testimonials(),
        }
    }

    fn get_quick_searches() -> Vec<QuickSearch> {
        iter::repeat(QuickSearch {
            image_id: String::from("??"),
            image_lib: String::from("mock"),
            search_term: String::from("Chanukah"),
            jigs_count: 355 as u32,
        }).take(5).collect()
    }

    fn get_whats_new() -> Vec<WhatsNewItem> {
        iter::repeat(WhatsNewItem {
            image_id: String::from("something.jpg"),
            image_lib: String::from("mock"),
            header: String::from("HOP TV - New Hebrew Series"),
            paragraph: String::from("Learning Hebrew with HOP Channel, Learning Hebrew with HOP Channel, Learning Hebrew with HOP Channel, Learning Hebrew with HOP Channel Learning Hebrew with HOP"),
            link: String::from(""),
        }).take(3).collect()
    }

    fn get_parents_testimonials() -> Vec<Testimonial> {
        iter::repeat(Testimonial {
            image_id: String::from("face-round.webp"),
            image_lib: String::from("mock"),
            header: String::from("Sarah Nazirah, Mexico"),
            paragraph: String::from("I want to tell you, because of JI, my children are learning Hebrew and English simultaneously. For my children, you are not only teaching two children, you are also saving their souls. I reaffirm that they have achieved educational rehabilitation, thanks to JI."),
        }).take(5).collect()
    }

    fn get_teachers_testimonials() -> Vec<Testimonial> {
        iter::repeat(Testimonial {
            image_id: String::from("face-round.webp"),
            image_lib: String::from("mock"),
            header: String::from("Sarah Nazirah, Mexico"),
            paragraph: String::from("I want to tell you, because of JI, my children are learning Hebrew and English simultaneously. For my children, you are not only teaching two children, you are also saving their souls. I reaffirm that they have achieved educational rehabilitation, thanks to JI."),
        }).take(5).collect()
    }

}

#[derive(Clone)]
pub enum HomePageMode {
    Home,
    Search(String, Rc<MutableVec<Jig>>)
}


#[derive(Clone)]
pub struct QuickSearch {
    pub image_id: String,
    pub image_lib: String, // is this always the same?
    pub search_term: String,
    pub jigs_count: u32,
}


#[derive(Clone)]
pub struct WhatsNewItem {
    pub image_id: String,
    pub image_lib: String, // is this always the same?
    pub header: String,
    pub paragraph: String,
    pub link: String,
}


#[derive(Clone)]
pub struct Testimonial {
    pub image_id: String,
    pub image_lib: String, // is this always the same?
    pub header: String,
    pub paragraph: String,
}
