use web_sys::AudioContext;
use std::rc::Rc;
use shared::domain::jig::Jig;
use super::instance::*;
use awsm_web::audio::AudioPlayer as AwsmAudio;
use std::cell::RefCell;
use utils::{prelude::*, path::audio_lib_url};
use shared::{
    domain::audio::AudioId,
    media::MediaLibrary,
    domain::jig::module::body::Audio,
};
use beach_map::{BeachMap, DefaultVersion};
use std::rc::Weak;
use super::id::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};

pub(super) type InstanceLookup = Rc<RefCell<BeachMap<DefaultVersion, Weak<AudioInstance>>>>;

#[derive(Clone)]
pub struct AudioMixer {
    //All operations need to go through with_ctx
    //So that we can lazy-load it
    ctx: RefCell<Option<AudioContext>>,

    //the lookup holds weak references to the instances
    //when they're dropped, they'll clean up their entry
    pub(super) instance_lookup: InstanceLookup,
}

impl AudioMixer {
    pub fn new(ctx: Option<AudioContext>, jig: &Jig) -> Self {

        //TODO - populate jig-level effects
        Self {
            ctx: RefCell::new(ctx),
            instance_lookup: Rc::new(RefCell::new(BeachMap::new())),
        }
    }
    pub fn new_without_jig(ctx: Option<AudioContext>) -> Self {

        Self {
            ctx: RefCell::new(ctx),
            instance_lookup: Rc::new(RefCell::new(BeachMap::new())),
        }
    }

    pub fn suspend_ctx(&self) {
        self.with_ctx(|ctx| {
            let promise = ctx.suspend().unwrap_ji();
            spawn_local(async move {
                let _ = JsFuture::from(promise).await;
            });
        });
    }
    pub fn resume_ctx(&self) {
        self.with_ctx(|ctx| {
            let promise = ctx.resume().unwrap_ji();
            spawn_local(async move {
                let _ = JsFuture::from(promise).await;
            });
        });
    }

    //Lazy-creates the AudioContext as needed
    pub fn with_ctx<A>(&self, f: impl FnOnce(&AudioContext) -> A) -> A {
        let mut ctx = self.ctx.borrow_mut();
        if let Some(ctx) = ctx.as_ref() {
            f(ctx)
        } else {
            let new_ctx = AudioContext::new().unwrap_ji();
            let ret = f(&new_ctx);
            *ctx = Some(new_ctx);
            ret
        }
    }

    pub fn play_oneshot(&self, audio: Audio) -> Rc<AudioInstance> {
        self.play_oneshot_callback(audio, None::<fn()>)
    }

    pub fn play_oneshot_callback(&self, audio: Audio, on_ended: Option<impl FnMut() + 'static>) -> Rc<AudioInstance> {
        let url = audio_lib_url(audio.lib, audio.id);
        self.add_instance(AudioClip::OneShot(self.with_ctx(|ctx| AwsmAudio::play_oneshot_url(ctx, &url, on_ended).unwrap_ji())))

    }

    fn add_instance(&self, clip: AudioClip) -> Rc<AudioInstance> {
        let id = self.instance_lookup.borrow_mut().insert_with_id(move |id| {
            let instance = Rc::new(AudioInstance {
                id,
                instance_lookup: self.instance_lookup.clone(),
                clip
            });

            Rc::downgrade(&instance)
        });

        self.get_instance(id).unwrap_ji()
    }

    pub fn get_instance(&self, id: Id) -> Option<Rc<AudioInstance>> {
        self.instance_lookup.borrow().get(id)
            .and_then(|weak_ref| {
                weak_ref.upgrade()
            })
    }
}
