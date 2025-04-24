#![allow(unused)] // TODO remove this when the code is complete

use core::fmt::Debug;
use alloc::vec::Vec;
use crate::rpc_client::RpcClient;
use crate::rpc_client::RpcError;

/// A trait representing an HTTP client in a no-std environment.
pub trait HttpClient {
    /// Sends an HTTP request and returns the response as a byte vector.
    fn send_request(&self, method: &str, url: &str, body: Option<&[u8]>) -> Result<Vec<u8>, HttpClientError>;
}

/// An error type for the HTTP client.
#[derive(Debug)]
pub enum HttpClientError {
    /// Represents a network error.
    NetworkError,
    /// Represents an invalid response error.
    InvalidResponse,
    /// Represents other errors with a custom message.
    Other(&'static str),
}

/// A basic implementation of the `RpcClient` trait using the `HttpClient`.
pub struct BasicHttpRpcClient<C: HttpClient> {
    http_client: C,
}

impl<C: HttpClient> BasicHttpRpcClient<C> {
    /// Creates a new `BasicHttpRpcClient` with the given HTTP client.
    pub fn new(http_client: C) -> Self {
        Self { http_client }
    }
}

impl<C: HttpClient> RpcClient for BasicHttpRpcClient<C> {
    fn send(&self, url: &str, payload: &[u8]) -> Result<Vec<u8>, RpcError> {
        // Template
        Ok(Vec::new())
    }
    fn receive(&self) -> Result<Vec<u8>, RpcError> {
        // Template
        Ok(Vec::new())
    }
}
