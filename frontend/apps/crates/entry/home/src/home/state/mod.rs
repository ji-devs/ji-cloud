use std::{iter, rc::Rc};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::jig::{JigId, JigSearchQuery};

use components::page_header::state::PageLinks;

use super::search_results::SearchResults;
use strum_macros::Display;

mod search_state;
pub use search_state::*;

pub struct State {
    pub loader: AsyncLoader,
    pub mode: Mutable<HomePageMode>,
    pub is_logged_in: Mutable<bool>,
    pub search_options: Rc<SearchOptions>,
    pub search_selected: Rc<SearchSelected>,
    pub quick_searches: Vec<QuickSearch>,
    pub whats_new: Vec<WhatsNewItem>,
    pub parents_testimonials: Vec<Testimonial>,
    pub teachers_testimonials: Vec<Testimonial>,
    pub total_jigs_count: Mutable<u64>,
    pub play_jig: Mutable<Option<JigId>>,
}

impl Default for State {
    fn default() -> Self {
        Self::new_with_search_selected(SearchSelected::default())
    }
}

impl State {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn new_search(query_params: Option<JigSearchQuery>) -> Self {
        let search_selected = match query_params {
            Some(query_params) => SearchSelected::from_search_request(query_params),
            None => SearchSelected::default(),
        };
        Self::new_with_search_selected(search_selected)
    }
    fn new_with_search_selected(search_selected: SearchSelected) -> Self {
        Self {
            search_selected: Rc::new(search_selected),
            loader: AsyncLoader::new(),
            mode: Mutable::new(HomePageMode::Home),
            is_logged_in: Mutable::new(false),
            search_options: Rc::new(SearchOptions::default()),
            quick_searches: Self::get_quick_searches(),
            whats_new: Self::get_whats_new(),
            parents_testimonials: Self::get_parents_testimonials(),
            teachers_testimonials: Self::get_teachers_testimonials(),
            total_jigs_count: Mutable::new(0),
            play_jig: Mutable::new(None),
        }
    }

    fn get_quick_searches() -> Vec<QuickSearch> {
        vec![
            QuickSearch {
                search_term: String::from("Hebrew"),
            },
            QuickSearch {
                search_term: String::from("Tishrei"),
            },
            QuickSearch {
                search_term: String::from("Chanukah"),
            },
            QuickSearch {
                search_term: String::from("Israel"),
            },
        ]
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
        vec![
            Testimonial {
                image_id: String::from("orly-rachamim.jpg"),
                name: String::from("Orly Rachamim"),
                bio: String::from("Netivot HaTorah Day School, Ontario, Canada"),
                paragraph: String::from("Having the ability to search for and download games and activities in addition to creating your own and sharing it in the platform is a great crowd-sourcing opportunity. Using the rich creation packs as well as interactive layers helps enhance the learning experience for students by bringing the material to life."),
            },
            Testimonial {
                image_id: String::from("liat-walker.png"),
                name: String::from("Liat Walker"),
                bio: String::from("Jewish Studies Coordinator, Martin J Gottlieb Day School, FL, USA"),
                paragraph: String::from("I use Ji as a way to enrich the students’ Jewish knowledge and experience. The lessons and images include every contemporary subject in the Jewish world and Israel and is an excellent way for our students to feel connected to their Jewish identity. Before Ji this kind of information would be found in a book or on an Internet site, which is geared for adults. In my opinion, there is no kid-friendly space for our students to learn about Jewish contemporary topics other than Ji."),
            },
            Testimonial {
                image_id: String::from("jonathon-simons.png"),
                name: String::from("Jonathon Simons"),
                bio: String::from("Broughton Jewish, Manchester, UK"),
                paragraph: String::from("In the last three months, I have found that I have been able to get 100% engagement from students and have been able to improve their grades."),
            },
            Testimonial {
                image_id: String::from("dana-cappel.png"),
                name: String::from("Dana Cappel"),
                bio: String::from("Beit Issie Shapiro, Israel"),
                paragraph: String::from("Ji is a fantastic resource for our students.  The fact that I can create customized activities for my students means that I can create activities in exactly the way that they need them to be so that they can learn and participate to their maximum potential."),
            },
        ]
    }

    fn get_teachers_testimonials() -> Vec<Testimonial> {
        vec![
            Testimonial {
                image_id: String::from("rabbi-yakov-shafferman.png"),
                name: String::from("Rabbi Yakov Shafferman"),
                bio: String::from("Jesode Hatorah, Antwerp, Belgium"),
                paragraph: String::from("I think this tool is going to be very, very useful for our school for many different subjects. We’re teaching Hebrew and other traditional subjects like Chumash and Gemarah. We can use it for teaching itself and for assessing the students. I’m looking forward to enhancing Jewish learning in our school with Ji."),
            },
            Testimonial {
                image_id: String::from("rabbi-hiller.jpg"),
                name: String::from("Rabbi Hersh Hiller"),
                bio: String::from("Yeshiva Elementary, Milwaukee"),
                paragraph: String::from("Yesterday, I tried the Tu’Bishvat app with our iPads. It was amazing! The 17 kids were super-engaged. You have to imagine five students with their heads packed tightly against each other in a tight circle hovering over the glow of the iPad on the floor. Thank you for all the work that you put into to make an amazing program that I could use my classroom."),
            },
            Testimonial {
                image_id: String::from("adina-levin.png"),
                name: String::from("Adina Levin"),
                bio: String::from("Hillel Day School, Detroit, USA"),
                paragraph: String::from("I’m amazed with what is finally, finally available for the Jewish Studies teachers. I always was jealous of the English teachers, that have so much material, and so much sources, and we as the Judaic Studies teachers are always trying to create our own material and come up with innovations."),
            },
            Testimonial {
                image_id: String::from("rabbi-moshe-rosenberg.jpg"),
                name: String::from("Rabbi Moshe Rosenberg"),
                bio: String::from("SAR Academy, NY"),
                paragraph: String::from("What sets your products apart is that you do not compromise on either the substance or the style. You have both the truly professional look and true content."),
            },
        ]
    }
}

#[derive(Clone, Display)]
pub enum HomePageMode {
    #[strum(serialize = "home")]
    Home,
    #[strum(serialize = "results")]
    Search(Rc<SearchResults>),
}

impl From<&HomePageMode> for PageLinks {
    fn from(mode: &HomePageMode) -> PageLinks {
        match mode {
            HomePageMode::Home => PageLinks::Home,
            HomePageMode::Search(..) => PageLinks::Content,
        }
    }
}

#[derive(Clone)]
pub struct QuickSearch {
    pub search_term: String,
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
    pub name: String,
    pub bio: String,
    pub paragraph: String,
}
