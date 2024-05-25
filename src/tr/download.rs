use super::hash::ChooseHash;
use crate::errors::error::Result;
use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use futures_util::StreamExt;
use indicatif::ProgressBar;

use crate::dictionary::pb_messages::PbMessages;
use crate::tr::load::Load;
use lazy_static::lazy_static;

lazy_static! {
    static ref DICT: PbMessages = PbMessages::load_sync().unwrap();
}

#[async_trait]
pub trait Download {
    async fn get_file(link: String, hash: ChooseHash, pb: &ProgressBar) -> Result<Bytes> {
        // make reqwest
        let response = reqwest::get(link).await?;
        // know size of file
        let size = response.content_length().unwrap_or(0);
        pb.set_length(size);

        pb.set_message(&DICT.download_file);

        let mut size_upload = 0_u64;
        let mut content = BytesMut::new();
        let mut a = response.bytes_stream();
        while let Some(chunk) = a.next().await {
            let chunk = chunk?;
            size_upload += chunk.len() as u64;
            pb.set_position(size_upload);
            content.extend_from_slice(&chunk)
        }
        pb.set_message(&DICT.calculate_hash);
        hash.calculate_hash(&*content).await?;
        // End
        pb.set_message(&DICT.file_downloaded);
        Ok(content.freeze())
    }
}

/// Make progress bar with template
///
/// ` Package:: {PackageName} >>>{spinner} {msg} > eta: {eta} `
#[macro_export]
macro_rules! pb {
    ($mpb:expr, $name:expr) => {{
        let name = $name.to_string();
        let pb = $mpb.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::with_template(
                "Package:: {prefix:.blue} >>>{spinner:.green} {msg:.blue} > eta: {eta:.blue}",
            )
            .unwrap(),
        );
        pb.set_prefix(name.clone());
        pb
    }};
}
