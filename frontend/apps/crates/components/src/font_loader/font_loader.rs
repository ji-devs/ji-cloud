/*
 * FontLoader is intended to be created and then held at a top-level
 * calling the loader will concurrently load the fonts up to LOAD_BATCH_SIZE at a time
 * it's okay to call the loader with the same font multiple times, it'll just skip it the second
 * time (the queue is updated immediately, doesn't wait for loading to succeed/fail)
 *
 * error handling is non-existant atm
 */

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use std::collections::HashSet;
use futures::stream::{FuturesUnordered, StreamExt};
use wasm_bindgen_futures::JsFuture;
use utils::{path, prelude::*};

const LOAD_BATCH_SIZE:usize = 10;

const ALL_FONTS:[Font;5] = [
    Font::ShesekRegular,
    Font::FrankRuhlMedium,
    Font::ArchitectsDaughterRegular,
    Font::RobotoSlabRegular,
    Font::CaveatMedium,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Font {
    ShesekRegular,
    FrankRuhlMedium,
    ArchitectsDaughterRegular,
    RobotoSlabRegular,
    CaveatMedium,
}

struct LoaderInfo {
    pub name: &'static str,
    pub format: &'static str,
    pub url: String,
    pub unicode_range: Option<&'static str>,
}

impl Font {
    fn get_loader_info(self) -> LoaderInfo {
        let (name, filepath, format) = match self {
            Self::ShesekRegular => ("Shesek - Regular", "shesek-regular-fm.woff2", "woff2"),
            Self::FrankRuhlMedium => ("Frank Ruhl Libre - Medium", "Frank_Ruhl_Libre/FrankRuhlLibre-Medium.ttf", "truetype"),
            Self::ArchitectsDaughterRegular => ("Architects Daughter - Regular", "Architects_Daughter/ArchitectsDaughter-Regular.ttf", "truetype"),
            Self::RobotoSlabRegular => ("Roboto Slab - Regular", "Roboto_Slab/static/RobotoSlab-Regular.ttf", "truetype"),
            Self::CaveatMedium=> ("Caveat - Medium", "Caveat/static/Caveat-Medium.ttf", "truetype"),
        };

        let unicode_range = match self {
            Self::ShesekRegular | Self::FrankRuhlMedium => Some("todo"),
            _ => None
        };

        LoaderInfo {
            name,
            url: path::ui(&format!("fonts/{}", filepath)),
            format, 
            unicode_range
        }
    }
}

#[wasm_bindgen(inline_js=r#"
export function load_font(name, url, format, unicode_range) {

    console.log(`url(${url}) format('${format}')`);

    return new FontFace(name, `url(${url}) format('${format}')`)
        .load()
        .then(face => {
            console.log(face);
            console.log("loaded", `"${name}"`, "from", url);
	        document.fonts.add(loaded_face);
        })
}
"#)]

extern "C" {
    fn load_font(name: &str, url:String, format:&str, unicode_range: &str) -> Promise;
}

pub struct FontLoader {
    has_queued: HashSet<Font>
}

impl FontLoader {
    pub fn new() -> Self {
        Self {
            has_queued: HashSet::new()
        }
    }

    pub async fn load_all(&mut self) {
        self.load(&ALL_FONTS).await;
    }

    pub async fn load(&mut self, fonts:&[Font]) {

        let mut fonts:Vec<Font> = fonts
            .iter()
            .filter(|font| !self.has_queued.contains(&font))
            .map(|font| *font).collect();

        for font in fonts.iter() {
            self.has_queued.insert(*font);
        }
        let mut futures = FuturesUnordered::new();

        //See: https://users.rust-lang.org/t/awaiting-futuresunordered/49295/5
        //Idea is we try to have a saturated queue of futures
        while let Some(font) = fonts.pop() {
            while futures.len() >= LOAD_BATCH_SIZE {
                futures.next().await;
            }
            let LoaderInfo {name, url, unicode_range, format}  = font.get_loader_info();
            let next_future = JsFuture::from(load_font(name, url, format, unicode_range.unwrap_or_default()));
            futures.push(next_future);
        }
        while let Some(_) = futures.next().await {}
    }
}
