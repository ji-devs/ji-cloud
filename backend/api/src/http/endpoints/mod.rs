pub mod account;
pub mod additional_resource;
pub mod admin;
pub mod animation;
pub mod audio;
pub mod billing;
pub mod category;
pub mod circle;
pub mod course;
pub mod image;
pub mod jig;
pub mod locale;
pub mod media;
pub mod meta;
pub mod module;
pub mod pdf;
pub mod playlist;
pub mod resource;
pub mod scheduler;
pub mod search;
pub mod session;
pub mod user;

use actix_web::web::{Bytes, Payload};
use futures::StreamExt;
use shared::media::FileKind;

use crate::{error, service::storage};

async fn read_limited_payload(
    mut payload: Payload,
    file_kind: FileKind,
) -> Result<Vec<u8>, error::Upload> {
    let mut data = Vec::new();
    let limit = storage::Client::file_size_limit(&file_kind)
        .ok_or_else(|| anyhow::anyhow!("file type size limit undefined"))?;

    while let Some(chunk) = payload.next().await {
        let chunk: Bytes = chunk?;
        if data.len() + chunk.len() > limit {
            return Err(error::Upload::FileTooLarge);
        }
        data.extend_from_slice(&chunk);
    }

    Ok(data)
}
