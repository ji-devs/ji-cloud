pub use super::*;
use serde::{Deserialize, Serialize};

/// The body for [`Legacy`](crate::domain::jig::module::ModuleKind::Legacy) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Slide {
    /// Design layer  
    pub design: design::Design,

    /// The activity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<activity::Activity>,
}
