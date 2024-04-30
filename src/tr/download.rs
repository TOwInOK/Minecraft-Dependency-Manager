use crate::errors::error::Result;
use bytes::{Bytes, BytesMut};
use futures_util::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};

use super::hash::ChooseHash;
pub trait Download {
    fn get_file(
        name: String,
        link: String,
        hash: ChooseHash,
        mpb: &MultiProgress,
    ) -> impl std::future::Future<Output = Result<Bytes>> + Send {
        async move {
            // make reqwest
            let response = reqwest::get(link).await?;
            // know size of file
            let size = response.content_length().unwrap_or(0);
            // make key for pb
            let name_closure = move |_: &ProgressState, f: &mut dyn std::fmt::Write| {
                f.write_str(&name).unwrap();
            };
            // PB style, init
            let pb = mpb.add(ProgressBar::new(size));
            pb.set_style(
                ProgressStyle::with_template(
                    "Package:: {name:.blue} >>> {spinner:.green} {msg:.blue} | {bytes:.blue}/{total_bytes:.blue} -> eta:{eta:.blue}, {bytes_per_sec:.blue} | ",
                )
                .unwrap()
                .with_key("name", name_closure),
            );
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
            pb.finish_with_message("Downloaded!");
            Ok(content.freeze())
        }
    }
}
