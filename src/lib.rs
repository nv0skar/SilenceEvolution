// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

pub mod app_cx;
pub mod config;
pub mod internal_endpoints;
pub mod simple_endpoint;
pub mod static_service;

pub use app_cx::*;
pub use internal_endpoints::*;
pub use simple_endpoint::*;
pub use static_service::*;

use std::any::{Any, TypeId};
use std::convert::Infallible;
use std::env::current_dir;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock};
use std::task::Poll;

use waveless_commons::{endpoint::*, *};
use waveless_executor::*;

use rustyrosetta::*;

use anyhow::{Result, anyhow, bail};
use async_trait::*;
use compact_str::*;
use derive_builder::*;
use derive_more::Constructor;
use derive_more::Display;
use futures::future::BoxFuture;
use getset::*;
use hyper::{Response, StatusCode};
use itertools::*;
use rust_embed_for_web::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use struct_patch::*;
use tokio::{
    fs::{create_dir, read, read_dir, write},
    sync::{OnceCell, RwLock},
};
use tower::Service;
use tracing::*;

pub static APP_CX: OnceCell<AppCx> = OnceCell::const_new();
