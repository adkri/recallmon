use crate::models::VectorRecord;
use anyhow::Result;
use aws_sdk_s3::{primitives::ByteStream, Client};

#[derive(Clone)]
pub struct WalAppender {
    pub bucket: String,
    pub client: Client,
}

impl WalAppender {
    pub async fn append(&self, rec: &VectorRecord) -> Result<()> {
        let data = serde_json::to_vec(rec)?;
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
