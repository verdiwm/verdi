#![allow(clippy::default_constructed_unit_structs)]

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod error;

pub mod actors;
pub mod keymap;
pub mod protocol;

pub use crate::{
    actors::{
        client::Client,
        compositor::{Compositor, CompositorHandle},
    },
    error::{Result, VerdiError},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Custom wayland socket path
    pub socket: Option<PathBuf>,
}
