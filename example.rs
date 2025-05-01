use minimal_http_rpc::{RpcClient, RpcMethod, StdClientFactory, RpcClientFactory};
use serde::{Deserialize, Serialize};

// Define your RPC method
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

// Use the client
fn main() {
    // Create a transport
    let transport = StdClientFactory::create_transport("https://api.example.com");

    // Create a client with the transport
    let client = RpcClient::new(transport);

    // Make an RPC call
    let request = GetUserRequest { id: 123 };
    match client.call::<GetUserMethod>(request) {
        Ok(response) => {
            println!("User: {} ({})", response.name, response.email);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
