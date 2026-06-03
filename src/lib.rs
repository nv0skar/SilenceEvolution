// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

pub mod app_cx;
pub mod config;
pub mod endpoint;
pub mod panel;

pub use app_cx::*;
pub use panel::*;

use std::any::{Any, TypeId};
use std::convert::Infallible;
use std::env::current_dir;
use std::fs::{read, read_dir};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock};
use std::task::Poll;

use waveless_commons::{endpoint as wv_endpoint, *};
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
use rust_embed_for_web::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::{OnceCell, RwLock, RwLockReadGuard, RwLockWriteGuard};
use tower::Service;
use tracing::*;

pub static APP_CX: OnceCell<AppCx> = OnceCell::const_new();
