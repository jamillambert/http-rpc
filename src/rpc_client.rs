use alloc::vec::Vec;

pub trait RpcClient {
    fn send(&self, url: &str, payload: &[u8]) -> Result<Vec<u8>, RpcError>;
    fn receive(&self) -> Result<Vec<u8>, RpcError>;
}

#[derive(Debug)]
pub enum RpcError {
    ConnectionFailed,
    Timeout,
    ResponseError,
}
