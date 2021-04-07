use std::rc::Rc;

use dominator::{Dom, clone, html};
use futures_signals::signal::SignalExt;
use utils::events;
use shared::{domain::audio::AudioId, media::MediaLibrary};
use crate::audio_input::state::{State, AudioInputMode, AudioInputAddMethod};


pub const STR_OPTION_RECORD: &'static str = "Record";
pub const STR_OPTION_UPLOAD: &'static str = "Upload a file";


pub fn render<F: Fn(Option<AudioId>) + 'static>(state: Rc<State<F>>, add_method: AudioInputAddMethod) -> Dom {
    html!("label", {
        .property("slot", "options")
        .child(html!("input", {
            .property("type", "radio")
            .property("name", "type")
            .property("value", { match add_method {
                AudioInputAddMethod::Record => "record",
                AudioInputAddMethod::Upload => "upload",
            }})
            .property_signal("checked", state.add_method.signal_cloned().map(clone!(add_method => move |selected_add_method| {
                selected_add_method == add_method
            })))
        }))
        .text({
            match add_method {
                AudioInputAddMethod::Record => STR_OPTION_RECORD,
                AudioInputAddMethod::Upload => STR_OPTION_UPLOAD,
            }
        })
        .event(clone!(state => move |_: events::Change| {
            state.add_method.set(add_method.clone());
            match add_method {
                AudioInputAddMethod::Record => state.mode.set(AudioInputMode::Record),
                AudioInputAddMethod::Upload => state.mode.set(AudioInputMode::Upload),
            }
        }))
    })
}
