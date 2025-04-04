use std::sync::Arc;

use uniffi::{export, Object, Record};

use crate::iroh_error::IrohError;

use super::{recv_stream::RecvStream, send_stream::SendStream};

#[derive(Object, Debug)]
pub struct Connection {
    pub(crate) connection: iroh::endpoint::Connection,
}

#[export(async_runtime = "tokio")]
impl Connection {
    pub fn alpn(&self) -> Option<Vec<u8>> {
        self.connection.alpn()
    }

    pub fn node_id(&self) -> Result<String, IrohError> {
        Ok(self.connection.remote_node_id()?.to_string())
    }

    pub fn node_id_short(&self) -> Result<String, IrohError> {
        Ok(self.connection.remote_node_id()?.fmt_short())
    }

    pub fn send_datagram(&self, data: Vec<u8>) -> Result<(), IrohError> {
        Ok(self.connection.send_datagram(data.into())?)
    }

    pub async fn read_datagram(&self) -> Result<Vec<u8>, IrohError> {
        Ok(self.connection.read_datagram().await?.into())
    }

    pub async fn open_bi(&self) -> Result<BiStream, IrohError> {
        let (send, recv) = self.connection.open_bi().await?;
        Ok(BiStream {
            send_stream: Arc::new(send.into()),
            recv_stream: Arc::new(recv.into()),
        })
    }

    pub async fn accept_bi(&self) -> Result<BiStream, IrohError> {
        let (send, recv) = self.connection.accept_bi().await?;
        Ok(BiStream {
            send_stream: Arc::new(send.into()),
            recv_stream: Arc::new(recv.into()),
        })
    }
}

impl From<iroh::endpoint::Connection> for Connection {
    fn from(connection: iroh::endpoint::Connection) -> Self {
        Self { connection }
    }
}

#[derive(Record, Debug)]
pub struct BiStream {
    pub send_stream: Arc<SendStream>,
    pub recv_stream: Arc<RecvStream>,
}
