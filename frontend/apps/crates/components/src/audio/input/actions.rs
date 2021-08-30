use super::{
    super::upload::{upload_audio, UploadError},
    state::{AudioInputMode, AudioInput}
};
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{audio::AudioId, jig::module::body::Audio, CreateResponse},
    error::EmptyError,
    media::MediaLibrary,
};
use std::rc::Rc;
use utils::prelude::*;
use web_sys::File;

impl AudioInput {
    //Internal only - when the audio is changed via recording/uploading
    //Will call the callbacks
    pub(super) fn set_audio(&self, audio: Option<Audio>) {
        //Change the mutable for affecting all DOM rendering stuff
        //with _eventual consistency_
        self.mode.set_neq(match audio.clone() {
            Some(audio) => AudioInputMode::Stopped(audio),
            None => AudioInputMode::Empty,
        });

        //Callbacks for immediate, unskipped updates
        match audio {
            Some(audio) => {
                if let Some(on_add) = &self.callbacks.on_add {
                    (on_add)(audio);
                }
            }
            None => {
                if let Some(on_delete) = &self.callbacks.on_delete {
                    (on_delete)();
                }
            }
        }
    }

    //Internal only - when the audio is changed via the external signal
    //Only changes state.
    //It's safe and idiomatic to set the external signal from callbacks too
    //(e.g. the external signal can be driven by a combo of history, current audio, and initial audio)
    pub(super) fn set_audio_ext(&self, audio: Option<Audio>) {
        self.mode.set_neq(match audio {
            Some(audio) => AudioInputMode::Stopped(audio),
            None => AudioInputMode::Empty,
        });
    }
}

pub async fn file_change(state: Rc<AudioInput>, file: File) {
    state.mode.set(AudioInputMode::Uploading);

    *state.aborter.borrow_mut() = AbortController::new();

    let lib = MediaLibrary::User;

    let err = {
        match api_with_auth_abortable::<CreateResponse<AudioId>, EmptyError, ()>(
            &endpoints::audio::user::Create::PATH,
            endpoints::audio::user::Create::METHOD,
            Some(&*state.aborter.borrow()),
            None,
        )
        .await {
            Ok(Ok(resp)) => {
                let CreateResponse { id } = resp;
                match upload_audio(id, lib, &file, Some(&*state.aborter.borrow())).await {
                    Err(err) => Some(err),
                    Ok(_) => {
                        state.set_audio(Some(Audio {
                            id,
                            lib
                        }));
                        None
                    }
                }
            }
            Err(true) => {
                // Not really and error, it's an abort 
                log::info!("Cancelled uploading audio file");
                Some(UploadError::Other(awsm_web::errors::Error::Empty))
            }
            _ => {
                log::error!("Error uploading audio file");
                Some(UploadError::Other(awsm_web::errors::Error::Empty))
            },
        }
    };

    if let Some(_) = err {
        state.mode.set(AudioInputMode::Empty);
    }
}


pub fn cancel_upload(state: Rc<AudioInput>) {
    state.aborter.borrow().abort();
}
