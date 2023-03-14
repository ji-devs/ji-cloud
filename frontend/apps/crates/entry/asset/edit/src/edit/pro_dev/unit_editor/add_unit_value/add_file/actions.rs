use std::rc::Rc;

use components::{audio, image, pdf};
use shared::{
    api::endpoints,
    domain::{
        audio::{user::UserAudioCreatePath, AudioId},
        image::{
            user::{UserImageCreatePath, UserImageCreateRequest},
            ImageId, ImageSize,
        },
        pdf::{user::UserPdfCreatePath, PdfId},
        pro_dev::unit::ProDevUnitValue,
    },
    media::MediaLibrary,
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};
use web_sys::{Blob, File};

use super::state::AddFile;

const MIME_START_IMAGE: &str = "image/";
const MIME_START_AUDIO: &str = "audio/";
const MIME_PDF: &str = "application/pdf";

impl AddFile {
    pub fn save(self: &Rc<Self>, file: File) {
        let state = Rc::clone(self);

        self.add_unit_value_state.loader.load(async move {
            let value = upload_file(&file).await.unwrap_ji();
            state
                .add_unit_value_state
                .unit_editor_state
                .value
                .set(Some(value))
        })
    }
}
pub async fn upload_file(file: &File) -> Result<ProDevUnitValue, anyhow::Error> {
    let mime_type = Blob::type_(file);

    let value = if mime_type == MIME_PDF {
        let pdf_id = upload_pdf(file).await?;
        ProDevUnitValue::PdfId(pdf_id)
    } else if mime_type.starts_with(MIME_START_IMAGE) {
        let image_id = upload_image(file).await?;
        ProDevUnitValue::ImageId(image_id)
    } else if mime_type.starts_with(MIME_START_AUDIO) {
        let audio_id = upload_audio(file).await?;
        ProDevUnitValue::AudioId(audio_id)
    } else {
        anyhow::bail!("We don't support {}", mime_type);
    };

    Ok(value)
}

async fn upload_image(file: &File) -> Result<ImageId, anyhow::Error> {
    let req = UserImageCreateRequest {
        size: ImageSize::Sticker,
    };

    let image_id = endpoints::image::user::Create::api_with_auth(UserImageCreatePath(), Some(req))
        .await
        .map_err(|_| anyhow::Error::msg("Error creating image in db"))?
        .id;

    image::upload::upload_image(image_id, MediaLibrary::User, file, None)
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    Ok(image_id)
}

async fn upload_audio(file: &File) -> anyhow::Result<AudioId> {
    let audio_id = endpoints::audio::user::Create::api_with_auth(UserAudioCreatePath(), None)
        .await
        .map_err(|_| anyhow::Error::msg("Error creating audio in db"))?
        .id;

    audio::upload::upload_audio(audio_id, MediaLibrary::User, file, None)
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    Ok(audio_id)
}

async fn upload_pdf(file: &File) -> Result<PdfId, anyhow::Error> {
    let pdf_id = endpoints::pdf::user::Create::api_with_auth(UserPdfCreatePath(), None)
        .await
        .map_err(|_| anyhow::Error::msg("Error creating pdf in db"))?
        .id;

    pdf::upload::upload_pdf(pdf_id, MediaLibrary::User, file, None)
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    Ok(pdf_id)
}
