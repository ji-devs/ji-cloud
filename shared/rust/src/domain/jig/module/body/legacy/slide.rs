pub use super::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The body for [`Legacy`](crate::domain::jig::module::ModuleKind::Legacy) modules.
#[skip_serializing_none]
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Slide {
    /// Full Image
    pub image_full: String,

    /// Design layer  
    pub design: design::Design,

    /// The activity
    pub activity: Option<activity::Activity>,
}
