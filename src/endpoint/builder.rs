use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use uniffi::{Object, export};

use crate::{iroh_error::IrohError, util::Apply};

use super::Endpoint;

#[derive(Object, Debug)]
pub struct EndpointBuilder(pub(crate) Mutex<Option<iroh::endpoint::Builder>>);

#[export]
pub fn new_endpoint_builder() -> EndpointBuilder {
    EndpointBuilder(Mutex::new(Some(iroh::Endpoint::builder())))
}

fn already_bound() -> IrohError {
    anyhow!("This endpoint builder has been bound").into()
}

#[export(async_runtime = "tokio")]
impl EndpointBuilder {
    pub fn bind_addr_v4(self: Arc<Self>, addr: &str) -> Result<Arc<Self>, IrohError> {
        let addr = addr.parse()?;
        self.0
            .lock()
            .unwrap()
            .apply(move |builder| builder.bind_addr_v4(addr));
        Ok(self)
    }

    pub fn bind_addr_v6(self: Arc<Self>, addr: &str) -> Result<Arc<Self>, IrohError> {
        let addr = addr.parse()?;
        self.0
            .lock()
            .unwrap()
            .apply(move |builder| builder.bind_addr_v6(addr));
        Ok(self)
    }

    pub fn secret_key(self: Arc<Self>, secret_key: &str) -> Result<Arc<Self>, IrohError> {
        let secret_key = secret_key.parse()?;
        self.0
            .lock()
            .unwrap()
            .apply(move |builder| builder.secret_key(secret_key));
        Ok(self)
    }

    pub fn alpns(self: Arc<Self>, alpn_protocols: Vec<Vec<u8>>) -> Result<Arc<Self>, IrohError> {
        self.0
            .lock()
            .unwrap()
            .apply(move |builder| builder.alpns(alpn_protocols));
        Ok(self)
    }

    pub fn discovery_n0(self: Arc<Self>) -> Result<Arc<Self>, IrohError> {
        self.0
            .lock()
            .unwrap()
            .apply(|builder| builder.discovery_n0());
        Ok(self)
    }

    pub fn tls_x509(self: Arc<Self>) -> Result<Arc<Self>, IrohError> {
        self.0.lock().unwrap().apply(|builder| builder.tls_x509());
        Ok(self)
    }

    pub fn tls_raw_public_keys(self: Arc<Self>) -> Result<Arc<Self>, IrohError> {
        self.0
            .lock()
            .unwrap()
            .apply(|builder| builder.tls_raw_public_keys());
        Ok(self)
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn discovery_dht(self: Arc<Self>) -> Result<Arc<Self>, IrohError> {
        self.0
            .lock()
            .unwrap()
            .apply(|builder| builder.discovery_dht());
        Ok(self)
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn discovery_local_network(self: Arc<Self>) -> Result<Arc<Self>, IrohError> {
        self.0
            .lock()
            .unwrap()
            .apply(|builder| builder.discovery_local_network());
        Ok(self)
    }

    pub fn proxy_url(self: Arc<Self>, url: &str) -> Result<Arc<Self>, IrohError> {
        let url = url.parse()?;
        self.0
            .lock()
            .unwrap()
            .apply(|builder| builder.proxy_url(url));
        Ok(self)
    }

    pub fn proxy_from_env(self: Arc<Self>) -> Result<Arc<Self>, IrohError> {
        self.0
            .lock()
            .unwrap()
            .apply(|builder| builder.proxy_from_env());
        Ok(self)
    }

    pub fn keylog(self: Arc<Self>, keylog: bool) -> Result<Arc<Self>, IrohError> {
        self.0
            .lock()
            .unwrap()
            .apply(|builder| builder.keylog(keylog));
        Ok(self)
    }

    pub async fn bind(self: Arc<Self>) -> Result<Endpoint, IrohError> {
        let builder = self.0.lock().unwrap().take().ok_or_else(already_bound)?;
        Ok(Endpoint(builder.bind().await?))
    }
}
