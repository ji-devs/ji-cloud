use std::{fs::File, io::BufWriter, path::PathBuf};

use indicatif::ProgressBar;
use shared::error::{ApiError, EmptyError};

pub async fn run(
    output_file: PathBuf,
    endpoint: String,
    token: String,
    csrf: String,
    show_progress: bool,
) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/v0/admin/media", endpoint))
        .header("Cookie", &format!("X-JWT={}", token))
        .header("X-CSRF", csrf)
        .send()
        .await?;

    match response.error_for_status_ref() {
        Ok(_) => {}
        Err(_) => {
            let error_json = response.json::<ApiError<EmptyError>>().await?;

            anyhow::bail!(
                "request failed ({}): {}",
                error_json.code,
                error_json.message
            )
        }
    }

    let data = response
        .json::<shared::domain::admin::AdminListMediaResponse>()
        .await?;

    tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
        let writer = File::create(&output_file)?;
        let writer = BufWriter::new(writer);

        let pb = if show_progress {
            ProgressBar::new_spinner()
        } else {
            ProgressBar::hidden()
        };

        pb.set_message(&format!("writing output file: {:?}", output_file));
        serde_json::to_writer(pb.wrap_write(writer), &data)?;
        pb.finish();

        Ok(())
    })
    .await
    .unwrap()?;

    Ok(())
}
