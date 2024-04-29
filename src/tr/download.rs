use bytes::Bytes;

use crate::errors::error::Result;

use super::hash::ChooseHash;
pub trait Download {
    fn get_file(
        &self,
        link: String,
        hash: ChooseHash,
    ) -> impl std::future::Future<Output = Result<Bytes>> + Send {
        async move {
            let response = reqwest::get(link).await?;
            let content = response.bytes().await?;
            hash.calculate_hash(&*content).await?;
            Ok(content)
        }
    }
}
