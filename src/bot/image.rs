use std::path::PathBuf;

use megalodon::SNS;

use crate::{config::Config, errors::Result as PResult};

/// Get the image id from the image path
pub async fn get_image_id(image_path: PathBuf, config: &Config) -> PResult<String> {
    log::info!("Uploading image: {}", image_path.display());
    let client = megalodon::generator(
        SNS::Pleroma,
        config
            .base_url
            .to_string()
            .trim_end_matches('/')
            .to_string(),
        Some(config.bot_token.clone()),
        None,
    );
    let res = client
        .upload_media(image_path.display().to_string(), None)
        .await?;
    match res.json() {
        megalodon::entities::UploadMedia::Attachment(media) => Ok(media.id),
        _ => unreachable!("Unexpected async upload"),
    }
}
