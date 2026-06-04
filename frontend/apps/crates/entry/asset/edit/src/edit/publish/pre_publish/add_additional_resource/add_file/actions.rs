use std::rc::Rc;

use components::{audio, image, pdf};
use shared::domain::{
    additional_resource::ResourceContent,
    audio::AudioId,
    image::{ImageId, ImageSize},
    pdf::PdfId,
};
use utils::unwrap::UnwrapJiExt;
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
    image::upload::upload_user_image(ImageSize::Sticker, file, None)
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))
}

async fn upload_audio(file: &File) -> anyhow::Result<AudioId> {
    audio::upload::upload_audio(file, None)
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))
}

async fn upload_pdf(file: &File) -> Result<PdfId, anyhow::Error> {
    pdf::upload::upload_pdf(file, None)
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))
}
