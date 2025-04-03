#![feature(iterator_try_collect)]

uniffi::setup_scaffolding!();

pub mod iroh_error;
pub mod endpoint;
pub mod connection;
pub mod util;
