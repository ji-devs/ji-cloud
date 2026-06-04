use std::rc::Rc;

use components::{audio, image, pdf};
use shared::domain::{
    audio::AudioId,
    course::unit::CourseUnitValue,
    image::{ImageId, ImageSize},
    pdf::PdfId,
};
use utils::unwrap::UnwrapJiExt;
use web_sys::{Blob, File};

use crate::edit::course::unit_editor::UnitValue;

use super::state::AddFile;

const MIME_START_IMAGE: &str = "image/";
const MIME_START_AUDIO: &str = "audio/";
const MIME_PDF: &str = "application/pdf";

impl AddFile {
    pub fn save(self: &Rc<Self>) {
        let state = Rc::clone(self);

        let file = self.file.get_cloned().unwrap_ji();
        let filename = file.name();
        state.filename.set(filename);

        self.add_unit_value_state.loader.load(async move {
            let value = upload_file(&file).await.unwrap_ji();

            state
                .add_unit_value_state
                .unit_editor_state
                .value
                .set(UnitValue::try_from(value).unwrap_ji());

            state
                .add_unit_value_state
                .unit_editor_state
                .changed
                .set(true)
        });
    }
}

pub async fn upload_file(file: &File) -> Result<CourseUnitValue, anyhow::Error> {
    let mime_type = Blob::type_(file);

    let value = if mime_type == MIME_PDF {
        let pdf_id = upload_pdf(file).await?;
        CourseUnitValue::PdfId(pdf_id)
    } else if mime_type.starts_with(MIME_START_IMAGE) {
        let image_id = upload_image(file).await?;
        CourseUnitValue::ImageId(image_id)
    } else if mime_type.starts_with(MIME_START_AUDIO) {
        let audio_id = upload_audio(file).await?;
        CourseUnitValue::AudioId(audio_id)
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
