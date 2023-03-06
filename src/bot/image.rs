use std::path::PathBuf;

use megalodon::entities::{Attachment, UploadMedia};

use crate::{
    config::Config,
    errors::{Error as PError, Result as PResult},
};

/// Get the image id from the image path
pub async fn get_image_id(image_path: PathBuf, config: &Config) -> PResult<String> {
    log::info!("Uploading image: {}", image_path.display());
    let client = megalodon::generator(
        config.sns(),
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
        UploadMedia::Attachment(media) => Ok(media.id),
        UploadMedia::AsyncAttachment(media) => {
            match wait_image_upload(client.as_ref(), &media.id).await {
                Ok(media) => Ok(media.id),
                Err(e) => {
                    log::error!("Error uploading image: {}", e);
                    Err(e)
                }
            }
        }
    }
}

/// Wait the image to be uploaded to mastodon.
pub async fn wait_image_upload(
    client: &(dyn megalodon::Megalodon + Send + Sync),
    image_id: &str,
) -> PResult<Attachment> {
    log::info!("Waiting for image to be uploaded. Image ID: {}", image_id);
    let mut tries = 0;
    loop {
        tries += 1;
        log::debug!("Trying number: {}", tries);
        let res = client.get_media(image_id.to_owned()).await;
        if tries > 5 {
            log::error!("Image upload timeout. Image ID: {}", image_id);
            return Err(PError::ImageTimeout(image_id.to_owned()));
        }
        match res {
            Ok(media) => {
                log::info!("Image uploaded successfully. Image ID: {}", image_id);
                return Ok(media.json());
            }
            Err(err) => match err {
                megalodon::error::Error::OwnError(ref own_err) => match own_err.kind {
                    megalodon::error::Kind::HTTPPartialContentError => continue,
                    _ => return Err(err.into()),
                },
                _ => return Err(err.into()),
            },
        }
    }
}
