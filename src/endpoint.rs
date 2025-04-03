pub mod builder;
pub mod connection;
pub mod recv_stream;
pub mod send_stream;

use std::{str::FromStr, sync::Arc};

use anyhow::anyhow;
use iroh::NodeId;
use uniffi::{Object, export};

use crate::{endpoint::connection::Connection, iroh_error::IrohError};

#[derive(Object, Debug)]
pub struct Endpoint(pub(crate) iroh::Endpoint);

#[export(async_runtime = "tokio")]
impl Endpoint {
    pub fn node_id(self: Arc<Self>) -> String {
        self.0.node_id().to_string()
    }

    pub async fn connect(
        self: Arc<Self>,
        node_addr: &str,
        alpn: &[u8],
    ) -> Result<Connection, IrohError> {
        let node_id = NodeId::from_str(node_addr)?;
        Ok(self.0.connect(node_id, alpn).await?.into())
    }

    pub async fn accept(self: Arc<Self>) -> Result<Connection, IrohError> {
        let incoming = self
            .0
            .accept()
            .await
            .ok_or_else(|| anyhow!("This endpoint has been closed"))?;
        let connection = incoming.accept()?.await?;
        Ok(connection.into())
    }
}
