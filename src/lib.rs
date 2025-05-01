#![cfg_attr(not(feature = "std"))]
#![allow(unused_imports)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, string::String, vec::Vec};

use core::{fmt, marker::PhantomData};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod error;
pub use error::Error;

#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "std")]
mod std_transport;

pub type Result<T> = core::result::Result<T, Error>;

/// Core trait for RPC methods
pub trait RpcMethod {
    /// The request type for this method
    type Request: Serialize;

    /// The response type for this method
    type Response: DeserializeOwned;

    /// The path for this method
    fn path() -> &'static str;

    /// The HTTP method for this RPC call
    fn http_method() -> &'static str {
        "POST"
    }
}

/// Core trait for HTTP transport
pub trait HttpTransport {
    /// Send an HTTP request and receive a response
    fn request<M: RpcMethod>(
        &self,
        request: M::Request,
    ) -> Result<M::Response>;
}

/// Core RPC client that uses a transport to make requests
pub struct RpcClient<T> {
    transport: T,
}

impl<T: HttpTransport> RpcClient<T> {
    /// Create a new RPC client with the given transport
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    /// Call an RPC method
    pub fn call<M: RpcMethod>(&self, request: M::Request) -> Result<M::Response> {
        self.transport.request::<M>(request)
    }
}

/// Define a custom transport by implementing the HttpTransport trait
pub trait RpcClientFactory {
    type Transport: HttpTransport;

    fn create_transport(base_url: &str) -> Self::Transport;
}

#[cfg(feature = "wasm")]
pub use wasm::{WasmTransport, WasmClientFactory};

#[cfg(feature = "std")]
pub use std_transport::{StdTransport, StdClientFactory};

// Example usage for users of the library
#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    // Example RPC method
    struct GetUserMethod;

    impl RpcMethod for GetUserMethod {
        type Request = GetUserRequest;
        type Response = GetUserResponse;

        fn path() -> &'static str {
            "/api/users"
        }
    }

    #[derive(Serialize)]
    struct GetUserRequest {
        id: u64,
    }

    #[derive(Deserialize)]
    struct GetUserResponse {
        id: u64,
        name: String,
        email: String,
    }

    #[test]
    #[ignore] // This is just an example, not an actual test
    fn example_usage() {
        // Create a transport based on the environment
        #[cfg(feature = "std")]
        let transport = StdClientFactory::create_transport("https://api.example.com");

        #[cfg(feature = "wasm")]
        let transport = WasmClientFactory::create_transport("https://api.example.com");

        // Create a client with the transport
        let client = RpcClient::new(transport);

        // Make an RPC call
        let request = GetUserRequest { id: 123 };
        let response = client.call::<GetUserMethod>(request).unwrap();

        // Use the response
        println!("User: {} ({})", response.name, response.email);
    }
}
