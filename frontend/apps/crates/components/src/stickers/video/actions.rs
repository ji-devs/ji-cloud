use shared::domain::jig::module::body::_groups::design::VideoHost;

use super::state::Video;

impl Video {
    pub fn set_value(&self, host: VideoHost) {
        self.host.set_neq(host);
    }

}

