use iroh::endpoint::VarInt;
use tokio::sync::Mutex;
use uniffi::{export, Object, Record};

use crate::iroh_error::IrohError;

#[derive(Object, Debug)]
pub struct RecvStream(Mutex<iroh::endpoint::RecvStream>);

#[export(async_runtime = "tokio")]
impl RecvStream {
    pub async fn is_0rtt(&self) -> bool {
        self.0.lock().await.is_0rtt()
    }

    pub async fn read_chunk(
        &self,
        max_length: u64,
        ordered: bool,
    ) -> Result<Option<Chunk>, IrohError> {
        Ok(self
            .0
            .lock()
            .await
            .read_chunk(max_length as usize, ordered)
            .await?
            .map(Into::into))
    }

    pub async fn read_exact(&self, bufsize: u64) -> Result<Vec<u8>, IrohError> {
        let mut buf = Vec::with_capacity(bufsize as usize);
        self.0.lock().await.read_exact(&mut buf).await?;
        Ok(buf)
    }

    pub async fn read_to_end(&self, size_limit: u64) -> Result<Vec<u8>, IrohError> {
        Ok(self.0.lock().await.read_to_end(size_limit as usize).await?)
    }

    pub async fn received_reset(&self) -> Result<Option<u64>, IrohError> {
        Ok(self.0.lock().await.received_reset().await?.map(Into::into))
    }

    pub async fn stop(&self, error_code: u64) -> Result<(), IrohError> {
        self.0.lock().await.stop(VarInt::from_u64(error_code)?)?;
        Ok(())
    }
}

impl From<iroh::endpoint::RecvStream> for RecvStream {
    fn from(value: iroh::endpoint::RecvStream) -> Self {
        Self(Mutex::new(value))
    }
}

#[derive(Record, Debug)]
pub struct Chunk {
    offset: u64,
    bytes: Vec<u8>,
}

impl From<iroh::endpoint::Chunk> for Chunk {
    fn from(value: iroh::endpoint::Chunk) -> Self {
        Self {
            offset: value.offset,
            bytes: value.bytes.into(),
        }
    }
}
