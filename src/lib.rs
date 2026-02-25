// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

pub mod app_cx;
pub mod config;
pub mod endpoint;

pub use app_cx::*;

use std::env::current_dir;
use std::fs::{read, read_dir};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock};

use rustyrosetta::*;

use anyhow::{Result, anyhow, bail};
use compact_str::*;
use derive_more::Constructor;
use getset::*;
use serde::{Deserialize, Serialize};
use tokio::sync::{OnceCell, RwLock};
use tracing::*;

pub static APP_CX: OnceCell<AppCx> = OnceCell::const_new();
