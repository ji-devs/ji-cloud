//! Types for Jig short codes for sharing

use super::JigId;
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// Response for creating or fetching the code associated with a jig.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct JigCodeResponse {
    /// Short four digit code, which can be used with `GET /v1/jig/code/{index}` to
    /// get the Jig's ID.
    pub code: i16,
}

/// Request for getting the Jig ID from a code
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct JigIdFromCodeRequest {
    /// The code of the Jig
    pub code: i16,
}

/// Response for getting the Jig associated with a code.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct JigIdFromCodeResponse {
    /// The id of the Jig
    pub id: JigId,
}
