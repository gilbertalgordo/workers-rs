#![allow(clippy::new_without_default)]
#![allow(clippy::or_fun_call)]
//! # Features
//! ## `d1`
//!
//! Allows the use of [D1 bindings](crate::d1) and [`query!`](crate::query) macro.
//!
//!
//! ## `queue`
//!
//! Enables `queue` event type in [`[event]`](worker_macros::event) macro.
//!
//! ```
//! // Consume messages from a queue
//! #[event(queue)]
//! pub async fn main(message_batch: MessageBatch<MyType>, env: Env, _ctx: Context) -> Result<()> {
//!     Ok(())
//! }
//! ```
//!
//! ## `http`
//! `worker` `0.0.21` introduced an `http` feature flag which starts to replace custom types with widely used types from the [`http`](https://docs.rs/http/latest/http/) crate.
//!
//! This makes it much easier to use crates which use these standard types such as [`axum`](axum).
//!
//! This currently does a few things:
//!
//! 1. Introduce [`Body`](worker::Body), which implements [`http_body::Body`](http_body::Body) and is a simple wrapper around [`web_sys::ReadableStream`](web_sys::ReadableStream).
//! 1. The `req` argument when using the [`[event(fetch)]`](worker_macros::event) macro becomes `http::Request<worker::Body>`.
//! 1. The expected return type for the fetch handler is `http::Response<B>` where `B` can be any [`http_body::Body<Data=Bytes>`](http_body::Body).
//! 1. The argument for [`Fetcher::fetch_request`](Fetcher::fetch_request) is `http::Request<worker::Body>`.
//! 1. The return type of [`Fetcher::fetch_request`](Fetcher::fetch_request) is `http::Response<worker::Body>`.
//!
//! The end result is being able to use frameworks like `axum` directly (see [example](./examples/axum)):
//!
//! ```rust
//! pub async fn root() -> &'static str {
//!     "Hello Axum!"
//! }
//!
//! fn router() -> Router {
//!     Router::new().route("/", get(root))
//! }
//!
//! #[event(fetch)]
//! async fn fetch(
//!     req: HttpRequest,
//!     _env: Env,
//!     _ctx: Context,
//! ) -> Result<http::Response<axum::body::Body>> {
//!     Ok(router().call(req).await?)
//! }
//! ```
//!
//! We also implement `try_from` between `worker::Request` and `http::Request<worker::Body>`, and between `worker::Response` and `http::Response<worker::Body>`. This allows you to convert your code incrementally if it is tightly coupled to the original types.

#[doc(hidden)]
use std::result::Result as StdResult;

#[doc(hidden)]
pub use async_trait;
#[doc(hidden)]
pub use js_sys;
pub use url::Url;
#[doc(hidden)]
pub use wasm_bindgen;
#[doc(hidden)]
pub use wasm_bindgen_futures;
pub use worker_kv as kv;

pub use cf::{Cf, TlsClientAuth};
pub use worker_macros::{durable_object, event};
#[doc(hidden)]
pub use worker_sys;
pub use worker_sys::{console_debug, console_error, console_log, console_warn};

pub use crate::abort::*;
pub use crate::cache::{Cache, CacheDeletionOutcome, CacheKey};
pub use crate::context::Context;
pub use crate::cors::Cors;
#[cfg(feature = "d1")]
pub use crate::d1::*;
pub use crate::date::{Date, DateInit};
pub use crate::delay::Delay;
pub use crate::durable::*;
pub use crate::dynamic_dispatch::*;
pub use crate::env::{Env, EnvBinding, Secret, Var};
pub use crate::error::Error;
pub use crate::fetcher::Fetcher;
pub use crate::formdata::*;
pub use crate::global::Fetch;
pub use crate::headers::Headers;
pub use crate::http::Method;
#[cfg(feature = "queue")]
pub use crate::queue::*;
pub use crate::r2::*;
pub use crate::request::Request;
pub use crate::request_init::*;
pub use crate::response::{Response, ResponseBody};
pub use crate::router::{RouteContext, RouteParams, Router};
pub use crate::schedule::*;
pub use crate::socket::*;
pub use crate::streams::*;
pub use crate::websocket::*;

mod abort;
mod cache;
mod cf;
mod context;
mod cors;
// Require pub module for macro export
#[cfg(feature = "d1")]
/// **Requires** `d1` feature.
pub mod d1;
mod date;
mod delay;
pub mod durable;
mod dynamic_dispatch;
mod env;
mod error;
mod fetcher;
mod formdata;
mod global;
mod headers;
mod http;
#[cfg(feature = "queue")]
mod queue;
mod r2;
mod request;
mod request_init;
mod response;
mod router;
mod schedule;
mod socket;
mod streams;
mod websocket;

pub type Result<T> = StdResult<T, error::Error>;

#[cfg(feature = "http")]
/// **Requires** `http` feature. A convenience Body type which wraps [`web_sys::ReadableStream`](web_sys::ReadableStream)
/// and implements [`http_body::Body`](http_body::Body)
pub use http::body::Body;
#[cfg(feature = "http")]
pub use http::{
    request::from_wasm as request_from_wasm, request::to_wasm as request_to_wasm,
    response::from_wasm as response_from_wasm, response::to_wasm as response_to_wasm,
};
#[cfg(feature = "http")]
/// **Requires** `http` feature. Type alias for `http::Request<worker::Body>`.
pub type HttpRequest = ::http::Request<http::body::Body>;
#[cfg(feature = "http")]
/// **Requires** `http` feature. Type alias for `http::Response<worker::Body>`.
pub type HttpResponse = ::http::Response<http::body::Body>;
