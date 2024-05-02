use crate::errors::error::Result;
use bytes::{Bytes, BytesMut};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

use super::hash::ChooseHash;
pub trait Download {
    fn get_file(
        link: String,
        hash: ChooseHash,
        pb: &ProgressBar,
    ) -> impl std::future::Future<Output = Result<Bytes>> + Send {
        async move {
            // make reqwest
            let response = reqwest::get(link).await?;
            // know size of file
            let size = response.content_length().unwrap_or(0);
            pb.set_length(size);

            pb.set_message("Download...");

            let mut size_upload = 0_u64;
            let mut content = BytesMut::new();
            let mut a = response.bytes_stream();
            while let Some(chunk) = a.next().await {
                let chunk = chunk?;
                size_upload += chunk.len() as u64;
                pb.set_position(size_upload);
                content.extend_from_slice(&chunk)
            }
            pb.set_message("Calculate hash...");
            hash.calculate_hash(&*content).await?;
            // End
            pb.set_message("Downloaded!");
            Ok(content.freeze())
        }
    }
}
