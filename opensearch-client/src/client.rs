#![allow(dead_code)]

use std::{
  fmt::Display,
  ops::{Deref, DerefMut},
};

use bytes::Bytes;
use futures::Stream;
use reqwest::RequestBuilder;
use serde::{
  de::{value, DeserializeOwned},
  Serialize,
};
use thiserror::Error;

#[cfg(not(target_arch = "wasm32"))]
type InnerByteStream = std::pin::Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>> + Send + Sync>>;

#[cfg(target_arch = "wasm32")]
type InnerByteStream = std::pin::Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>>>>;

/// Untyped byte stream used for both success and error responses.
pub struct ByteStream(InnerByteStream);

impl ByteStream {
  /// Creates a new ByteStream
  ///
  /// Useful for generating test fixtures.
  pub fn new(inner: InnerByteStream) -> Self {
    Self(inner)
  }

  /// Consumes the [`ByteStream`] and return its inner [`Stream`].
  pub fn into_inner(self) -> InnerByteStream {
    self.0
  }
}

impl Deref for ByteStream {
  type Target = InnerByteStream;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for ByteStream {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// Typed value returned by generated client methods.
///
/// This is used for successful responses and may appear in error responses
/// generated from the server (see [`Error::ErrorResponse`])
pub struct ResponseValue<T> {
  inner: T,
  status: reqwest::StatusCode,
  headers: reqwest::header::HeaderMap,
  // TODO cookies?
}

impl<T: DeserializeOwned> ResponseValue<T> {
  #[doc(hidden)]
  pub async fn from_response(response: reqwest::Response) -> Result<Self, Error> {
    let status = response.status();
    let headers = response.headers().clone();
    let inner = response.json().await.map_err(Error::InvalidResponsePayload)?;

    Ok(Self { inner, status, headers })
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl ResponseValue<reqwest::Upgraded> {
  #[doc(hidden)]
  pub async fn upgrade<E: std::fmt::Debug>(response: reqwest::Response) -> Result<Self, Error> {
    let status = response.status();
    let headers = response.headers().clone();
    if status == reqwest::StatusCode::SWITCHING_PROTOCOLS {
      let inner = response.upgrade().await.map_err(Error::InvalidResponsePayload)?;

      Ok(Self { inner, status, headers })
    } else {
      Err(Error::UnexpectedResponse(
        ReqwestResponse::from_response(response).await,
      ))
    }
  }
}

impl ResponseValue<ByteStream> {
  #[doc(hidden)]
  pub fn stream(response: reqwest::Response) -> Self {
    let status = response.status();
    let headers = response.headers().clone();
    Self {
      inner: ByteStream(Box::pin(response.bytes_stream())),
      status,
      headers,
    }
  }
}

impl ResponseValue<()> {
  #[doc(hidden)]
  pub fn empty(response: reqwest::Response) -> Self {
    let status = response.status();
    let headers = response.headers().clone();
    // TODO is there anything we want to do to confirm that there is no
    // content?
    Self {
      inner: (),
      status,
      headers,
    }
  }
}

impl ResponseValue<String> {
  #[doc(hidden)]
  pub async fn text(response: reqwest::Response) -> Result<Self, Error> {
    let status = response.status();
    let headers = response.headers().clone();
    let inner = response.text().await.map_err(Error::InvalidResponsePayload)?;
    // TODO is there anything we want to do to confirm that there is no
    // content?
    Ok(Self { inner, status, headers })
  }
}

impl<T> ResponseValue<T> {
  /// Creates a [`ResponseValue`] from the inner type, status, and headers.
  ///
  /// Useful for generating test fixtures.
  pub fn new(inner: T, status: reqwest::StatusCode, headers: reqwest::header::HeaderMap) -> Self {
    Self { inner, status, headers }
  }

  /// Consumes the ResponseValue, returning the wrapped value.
  pub fn into_inner(self) -> T {
    self.inner
  }

  /// Gets the status from this response.
  pub fn status(&self) -> reqwest::StatusCode {
    self.status
  }

  /// Gets the headers from this response.
  pub fn headers(&self) -> &reqwest::header::HeaderMap {
    &self.headers
  }

  /// Gets the parsed value of the Content-Length header, if present and
  /// valid.
  pub fn content_length(&self) -> Option<u64> {
    self
      .headers
      .get(reqwest::header::CONTENT_LENGTH)?
      .to_str()
      .ok()?
      .parse::<u64>()
      .ok()
  }

  #[doc(hidden)]
  pub fn map<U: std::fmt::Debug, F, E>(self, f: F) -> Result<ResponseValue<U>, E>
  where
    F: FnOnce(T) -> U, {
    let Self { inner, status, headers } = self;

    Ok(ResponseValue {
      inner: f(inner),
      status,
      headers,
    })
  }
}

impl ResponseValue<ByteStream> {
  /// Consumes the `ResponseValue`, returning the wrapped [`Stream`].
  pub fn into_inner_stream(self) -> InnerByteStream {
    self.into_inner().into_inner()
  }
}

impl<T> Deref for ResponseValue<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<T> DerefMut for ResponseValue<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}

impl<T: std::fmt::Debug> std::fmt::Debug for ResponseValue<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.inner.fmt(f)
  }
}

#[derive(Debug)]
pub struct DocumentedResponseValue {}
impl DocumentedResponseValue {
  fn new() -> Self {
    Self {}
  }
}

impl Display for DocumentedResponseValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("DocumentedResponseValue")
  }
}

#[derive(Debug)]
pub struct ReqwestResponse {
  pub status: reqwest::StatusCode,
  pub headers: reqwest::header::HeaderMap,
  pub value: String,
}
impl ReqwestResponse {
  pub fn new() -> Self {
    Self {
      status: reqwest::StatusCode::OK,
      headers: reqwest::header::HeaderMap::new(),
      value: "".to_string(),
    }
  }

  pub async fn from_response(response: reqwest::Response) -> ReqwestResponse {
    let status = response.status();
    let headers = response.headers().clone();
    let value = response.text().await.unwrap_or("".to_string());
    ReqwestResponse { status, headers, value }
  }
}

impl Display for ReqwestResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("ReqwestResponse")
  }
}

/// Error produced by generated client methods.
///
/// The type parameter may be a struct if there's a single expected error type
/// or an enum if there are multiple valid error types. It can be the unit type
/// if there are no structured returns expected.
#[derive(Error, Debug)]
pub enum Error {
  /// The request did not conform to API requirements.
  #[error("Invalid request: {0}")]
  InvalidRequest(String),
  /// The response  did not conform to API requirements.
  #[error("Invalid response: {0}")]
  InvalidResponse(String),
  /// An invalid URL was provided.
  #[error(transparent)]
  UrlParseError(#[from] url::ParseError),
  /// There is an error in provided credentials.
  #[error("Credential error: {0}")]
  CredentialsConfigError(String),
  /// The request did not conform to API requirements.
  #[error(transparent)]
  JsonExceptionError(#[from] serde_json::Error),

  /// A server error either due to the data, or with the connection.
  // #[error(transparent)]
  // CommunicationError(#[from] reqwest::Error),

  /// A server error either due to the data, or with the connection.
  #[error(transparent)]
  CommunicationError(#[from] reqwest_middleware::Error),

  /// A documented, expected error response.
  #[error("Document Error: {0}")]
  ErrorResponse(DocumentedResponseValue),

  /// An expected response code whose deserialization failed.
  #[error(transparent)]
  InvalidResponsePayload(#[from] reqwest::Error),

  /// A response not listed in the API description. This may represent a
  /// success or failure response; check `status().is_success()`.
  #[error("UnexpectedResponse: {0}")]
  UnexpectedResponse(ReqwestResponse),
}

trait ErrorFormat {
  fn fmt_info(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<E> ErrorFormat for ResponseValue<E>
where
  E: std::fmt::Debug,
{
  fn fmt_info(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "status: {}; headers: {:?}; value: {:?}",
      self.status, self.headers, self.inner,
    )
  }
}

impl ErrorFormat for ResponseValue<ByteStream> {
  fn fmt_info(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "status: {}; headers: {:?}; value: <stream>",
      self.status, self.headers,
    )
  }
}

// See https://url.spec.whatwg.org/#url-path-segment-string
const PATH_SET: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
  .add(b' ')
  .add(b'"')
  .add(b'#')
  .add(b'<')
  .add(b'>')
  .add(b'?')
  .add(b'`')
  .add(b'{')
  .add(b'}')
  .add(b'/')
  .add(b'%');

#[doc(hidden)]
pub fn encode_path(pc: &str) -> String {
  percent_encoding::utf8_percent_encode(pc, PATH_SET).to_string()
}
#[doc(hidden)]
pub fn encode_path_option_vec_string(pc: &Option<Vec<String>>) -> String {
  match pc {
    Some(v) => percent_encoding::utf8_percent_encode(&v.join(","), PATH_SET).to_string(),
    None => "".to_string(),
  }
}

#[doc(hidden)]
pub trait RequestBuilderExt<E> {
  fn form_urlencoded<T: Serialize + ?Sized>(self, body: &T) -> Result<RequestBuilder, Error>;
}

impl<E> RequestBuilderExt<E> for RequestBuilder {
  fn form_urlencoded<T: Serialize + ?Sized>(self, body: &T) -> Result<Self, Error> {
    Ok(
      self
        .header(
          reqwest::header::CONTENT_TYPE,
          reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded"),
        )
        .body(
          serde_urlencoded::to_string(body)
            .map_err(|_| Error::InvalidRequest("failed to serialize body".to_string()))?,
        ),
    )
  }
}
