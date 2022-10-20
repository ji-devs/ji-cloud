struct Audio {
    active: HashMap<Handle, HtmlAudioElement>,
    inactive: Vec<HtmlAudioElement>,
    context: AudioContext,
}
impl Audio {
    fn new() {
        // init context
        // init pool
    }

    fn play() -> Handle {
        // move from inactive to active
        // return handle
    }

    fn pause_all() {
        // iter over active and pause each
    }

    fn play_all() {
        // iter over active and play each
    }

    fn context_available() -> bool {

    }
}

struct Handle (String);
impl Handle {
    fn play(&self) {
        
    }
    fn pause(&self) {
        
    }
}
impl Drop for Handle {
    fn drop() {
        // pause
        // move from active to inactive
    }
}