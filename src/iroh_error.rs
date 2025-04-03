use std::fmt::Display;

use anyhow::Error;
use uniffi::{export, Object};

#[derive(Object, Debug)]
pub struct IrohError(Error);

#[export]
impl IrohError {
    pub fn message(&self) -> String {
        format!("{}", self)
    }
}

impl Display for IrohError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<E> From<E> for IrohError
where
    E: Into<Error>,
{
    fn from(value: E) -> Self {
        IrohError(value.into())
    }
}
