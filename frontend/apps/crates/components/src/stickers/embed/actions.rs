use shared::domain::module::body::_groups::design::EmbedHost;

use super::state::Embed;

impl Embed {
    pub fn set_value(&self, host: EmbedHost) {
        self.host.set_neq(host);
    }
}
