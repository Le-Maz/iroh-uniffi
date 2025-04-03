use uniffi::{export, Object};

#[derive(Object, Debug)]
pub struct Connection {
    pub(crate) connection: iroh::endpoint::Connection,
}

#[export]
impl Connection {
    pub fn alpn(&self) -> Option<Vec<u8>> {
        self.connection.alpn()
    }
}

impl From<iroh::endpoint::Connection> for Connection {
    fn from(connection: iroh::endpoint::Connection) -> Self {
        Self { connection }
    }
}
