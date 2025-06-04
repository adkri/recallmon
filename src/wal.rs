use aws_sdk_s3::{Client, types::ByteStream};
use thiserror::Error;
use crate::models::VectorRecord;

#[derive(Error, Debug)]
pub enum WalError {
    #[error("s3 error: {0}")]
    S3(#[from] aws_sdk_s3::Error),
}

#[derive(Clone)]
pub struct WalAppender {
    pub bucket: String,
    pub client: Client,
}

impl WalAppender {
    pub async fn append(&self, rec: &VectorRecord) -> Result<(), WalError> {
        let data = serde_json::to_vec(rec).expect("serialize");
        let body = ByteStream::from(data);
        let key = format!("wal/{}.wal", uuid::Uuid::new_v4());
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .send()
            .await?;
        Ok(())
    }
}
