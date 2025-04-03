use bytes::Bytes;
use iroh::endpoint::VarInt;
use tokio::sync::Mutex;
use uniffi::{Object, export};

use crate::iroh_error::IrohError;

#[derive(Object, Debug)]
pub struct SendStream(Mutex<iroh::endpoint::SendStream>);

#[export(async_runtime = "tokio")]
impl SendStream {
    pub async fn priority(&self) -> Result<i32, IrohError> {
        Ok(self.0.lock().await.priority()?)
    }

    pub async fn set_priority(&self, priority: i32) -> Result<(), IrohError> {
        Ok(self.0.lock().await.set_priority(priority)?)
    }

    pub async fn write(&self, buf: &[u8]) -> Result<u64, IrohError> {
        Ok(self.0.lock().await.write(buf).await? as u64)
    }

    pub async fn write_all(&self, buf: &[u8]) -> Result<(), IrohError> {
        Ok(self.0.lock().await.write_all(buf).await?)
    }

    pub async fn write_chunk(&self, buf: Vec<u8>) -> Result<(), IrohError> {
        Ok(self.0.lock().await.write_chunk(buf.into()).await?)
    }

    pub async fn write_chunks(&self, bufs: Vec<Vec<u8>>) -> Result<Written, IrohError> {
        let mut bufs: Vec<Bytes> = bufs.into_iter().map(|buf| buf.into()).collect();
        Ok(self.0.lock().await.write_chunks(&mut bufs).await?.into())
    }

    pub async fn write_all_chunks(&self, bufs: Vec<Vec<u8>>) -> Result<(), IrohError> {
        let mut bufs: Vec<Bytes> = bufs.into_iter().map(|buf| buf.into()).collect();
        Ok(self.0.lock().await.write_all_chunks(&mut bufs).await?)
    }

    pub async fn finish(&self) -> Result<(), IrohError> {
        Ok(self.0.lock().await.finish()?)
    }

    pub async fn reset(&self, error_code: u64) -> Result<(), IrohError> {
        Ok(self.0.lock().await.reset(VarInt::from_u64(error_code)?)?)
    }

    pub async fn stopped(&self) -> Result<Option<u64>, IrohError> {
        Ok(self.0.lock().await.stopped().await?.map(Into::into))
    }
}

impl From<iroh::endpoint::SendStream> for SendStream {
    fn from(value: iroh::endpoint::SendStream) -> Self {
        Self(Mutex::new(value))
    }
}

#[derive(Object, Debug)]
pub struct Written {
    bytes: u64,
    chunks: u64,
}

#[export]
impl Written {
    pub fn bytes(&self) -> u64 {
        self.bytes
    }

    pub fn chunks(&self) -> u64 {
        self.chunks
    }
}

impl From<iroh::endpoint::Written> for Written {
    fn from(value: iroh::endpoint::Written) -> Self {
        Self {
            bytes: value.bytes as u64,
            chunks: value.chunks as u64,
        }
    }
}
