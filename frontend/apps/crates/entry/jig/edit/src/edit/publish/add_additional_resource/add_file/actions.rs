use std::rc::Rc;

use components::{audio, image, pdf};
use shared::{
    api::endpoints,
    domain::{
        additional_resource::ResourceContent,
        audio::{user::UserAudioCreatePath, AudioId},
        image::{
            user::{UserImageCreatePath, UserImageCreateRequest},
            ImageId, ImageSize,
        },
        pdf::{user::UserPdfCreatePath, PdfId},
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
    pub fn save(self: &Rc<Self>) {
        let state = Rc::clone(self);

        state.add_resources_state.active_popup.set(None);

        let file = self.file.get_cloned().unwrap_ji();
        let display_name = file.name();
        let resource_type_id = self.resource_type.get_cloned().unwrap_ji().id;

        self.add_resources_state.loader.load(async move {
            let resource_content = upload_file(&file).await.unwrap_ji();
            state
                .add_resources_state
                .save_additional_resource(resource_content, display_name, resource_type_id)
                .await;
        })
    }

    // fn allowed_file_type(mime_type: &str) -> bool {
    //     mime_type.starts_with(MIME_START_IMAGE)
    //     ||
    //     mime_type.starts_with(MIME_START_AUDIO)
    //     ||
    //     mime_type.starts_with(MIME_PDF)
    // }
}
pub async fn upload_file(file: &File) -> Result<ResourceContent, anyhow::Error> {
    let mime_type = Blob::type_(file);

    let value = if mime_type == MIME_PDF {
        let pdf_id = upload_pdf(file).await?;
        ResourceContent::PdfId(pdf_id)
    } else if mime_type.starts_with(MIME_START_IMAGE) {
        let image_id = upload_image(file).await?;
        ResourceContent::ImageId(image_id)
    } else if mime_type.starts_with(MIME_START_AUDIO) {
        let audio_id = upload_audio(file).await?;
        ResourceContent::AudioId(audio_id)
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
