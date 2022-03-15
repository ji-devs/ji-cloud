use super::resize::*;
use dominator_helpers::{make_custom_event, make_custom_event_serde, temp_make_event};
use serde::Deserialize;
use shared::domain::jig::TextDirection;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub use dominator::events::*;
pub use dominator_helpers::events::{Load, Message};

temp_make_event!(TimeUpdate, "timeupdate" => web_sys::Event);
temp_make_event!(Ended, "ended" => web_sys::Event);
temp_make_event!(Ready, "ready" => web_sys::Event);

temp_make_event!(Open, "open" => web_sys::Event);
temp_make_event!(Close, "close" => web_sys::Event);

temp_make_event!(Submit, "submit" => web_sys::Event);

temp_make_event!(YoutubePlaying, "youtube-playing" => web_sys::Event);
temp_make_event!(YoutubePaused, "youtube-paused" => web_sys::Event);
temp_make_event!(YoutubeBuffering, "youtube-buffering" => web_sys::Event);
temp_make_event!(YoutubeEnded, "youtube-ended" => web_sys::Event);

temp_make_event!(Updated, "updated" => web_sys::Event);

temp_make_event!(Accept, "accept" => web_sys::Event);

temp_make_event!(Enter, "enter" => web_sys::Event);

temp_make_event!(Next, "next" => web_sys::Event);
temp_make_event!(Prev, "prev" => web_sys::Event);

temp_make_event!(Reset, "reset" => web_sys::Event);

temp_make_event!(ExpandAll, "expand-all" => web_sys::Event);
temp_make_event!(CollapseAll, "collapse-all" => web_sys::Event);

temp_make_event!(CustomBlur, "custom-blur" => web_sys::Event);

temp_make_event!(LoadedMetadata, "loadedmetadata" => web_sys::Event);

temp_make_event!(ScrollEnd, "scroll-end" => web_sys::Event);

temp_make_event!(FocusIn, "focusin" => web_sys::Event);
temp_make_event!(FocusOut, "focusout" => web_sys::Event);

make_custom_event_serde!("module-resize", ModuleResizeEvent, ResizeInfo);

macro_rules! make_pointer_event {
    ($name:ident, $type:literal => $event:path) => {
        temp_make_event!($name, $type => $event);

        impl $name {
            #[inline] pub fn x(&self) -> i32 { self.event.x() }
            #[inline] pub fn y(&self) -> i32 { self.event.y() }

            #[inline] pub fn pointer_id(&self) -> i32 { self.event.pointer_id() }
        }
    };
}

make_pointer_event!(PointerDown, "pointerdown" => web_sys::PointerEvent);
make_pointer_event!(PointerEnter, "pointerenter" => web_sys::PointerEvent);
make_pointer_event!(PointerLeave, "pointerleave" => web_sys::PointerEvent);
make_pointer_event!(PointerMove, "pointermove" => web_sys::PointerEvent);
make_pointer_event!(PointerUp, "pointerup" => web_sys::PointerEvent);
make_pointer_event!(PointerCancel, "pointercancel" => web_sys::PointerEvent);

// Custom Bounds
#[derive(Deserialize, Debug)]
pub struct CustomBoundsData {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

make_custom_event_serde!("custom-bounds", CustomBounds, CustomBoundsData);

// Custom Change
#[derive(Deserialize, Debug)]
pub struct CustomChangeData {
    pub value: String,
}

make_custom_event_serde!("custom-change", CustomChange, CustomChangeData);

impl CustomChange {
    pub fn value(&self) -> String {
        self.data().value
    }
}

// Custom Input
#[derive(Deserialize, Debug)]
pub struct CustomInputData {
    pub value: String,
}

make_custom_event_serde!("custom-input", CustomInput, CustomInputData);

impl CustomInput {
    pub fn value(&self) -> String {
        self.data().value
    }
}

// Custom Toggle
#[derive(Deserialize, Debug)]
pub struct CustomToggleData {
    pub value: bool,
}

make_custom_event_serde!("custom-toggle", CustomToggle, CustomToggleData);

impl CustomToggle {
    pub fn value(&self) -> bool {
        self.data().value
    }
}

// Custom Route
#[derive(Deserialize, Debug)]
pub struct CustomRouteData {
    pub route: String,
}

make_custom_event_serde!("custom-route", CustomRoute, CustomRouteData);

impl CustomRoute {
    pub fn route(&self) -> String {
        self.data().route
    }
}

// Custom String - USE SPARINGLY, AND ONLY FOR OPAQUE STRINGS!
#[derive(Deserialize, Debug)]
pub struct CustomStringData {
    pub value: String,
}

make_custom_event_serde!("custom-string", CustomString, CustomStringData);

impl CustomString {
    pub fn value(&self) -> String {
        self.data().value
    }
}

// Custom Search
#[derive(Deserialize, Debug)]
pub struct CustomSearchData {
    pub query: String,
}

make_custom_event_serde!("custom-search", CustomSearch, CustomSearchData);

impl CustomSearch {
    pub fn query(&self) -> String {
        self.data().query
    }
}

// Google Location
#[derive(Deserialize, Debug)]
pub struct GoogleLocationData {
    #[serde(rename = "rawJson")]
    pub raw_json: Option<String>,
    pub input: Option<String>,
    //not going to try and decode place
}

make_custom_event_serde!("google-location", GoogleLocation, GoogleLocationData);

impl GoogleLocation {
    pub fn raw_json(&self) -> Option<String> {
        self.data().raw_json
    }
}

make_custom_event!(CustomFile, "custom-file");

impl CustomFile {
    pub fn file(&self) -> web_sys::File {
        self.detail().unchecked_into()
    }
}

// Image Load  and Error
#[derive(Deserialize, Debug)]
pub struct ImageLoadData {
    pub width: f64,
    pub height: f64,
}

make_custom_event_serde!("image-load", ImageLoad, ImageLoadData);

impl ImageLoad {
    pub fn size(&self) -> (f64, f64) {
        let ImageLoadData { width, height } = self.data();
        (width, height)
    }
}

temp_make_event!(ImageError, "image-error" => web_sys::Event);

// Custom Direction
#[derive(Deserialize, Debug)]
pub struct CustomDirectionData {
    pub direction: TextDirection,
}

make_custom_event_serde!("custom-direction", CustomDirection, CustomDirectionData);

impl CustomDirection {
    pub fn direction(&self) -> TextDirection {
        self.data().direction
    }
}

// Custom Selected
#[derive(Deserialize, Debug)]
pub struct CustomSelectedChangeData {
    pub selected: bool,
}

make_custom_event_serde!(
    "custom-selected",
    CustomSelectedChange,
    CustomSelectedChangeData
);

impl CustomSelectedChange {
    pub fn selected(&self) -> bool {
        self.data().selected
    }
}

// Custom Confirm
#[derive(Deserialize, Debug)]
pub struct CustomConfirmData;

make_custom_event_serde!(
    "custom-confirm",
    CustomConfirm,
    CustomConfirmData
);

// Custom Cancel
#[derive(Deserialize, Debug)]
pub struct CustomCancelData;

make_custom_event_serde!(
    "custom-cancel",
    CustomCancel,
    CustomCancelData
);

// Custom Rating
#[derive(Deserialize, Debug)]
pub struct CustomRatingData {
    pub rating: Option<u8>,
}

make_custom_event_serde!(
    "custom-rating-change",
    CustomRatingChange,
    CustomRatingData
);

impl CustomRatingChange {
    pub fn rating(&self) -> Option<u8> {
        self.data().rating
    }
}
