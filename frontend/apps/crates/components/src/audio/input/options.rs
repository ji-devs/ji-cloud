use futures_signals::signal::Signal;
use shared::domain::jig::module::body::Audio;

pub struct AudioInputOptions<S: Signal<Item = Option<Audio>>> {
    /// This optional signal will cause the internal audio to change
    /// useful for both initial audio and keeping this component
    /// in sync with the history mechanism used elsehwere
    pub ext_audio_signal: Option<S>,
}

impl<S: Signal<Item = Option<Audio>>> AudioInputOptions<S> {
    pub fn new(ext_audio_signal: Option<S>) -> Self {
        Self { ext_audio_signal }
    }
}
