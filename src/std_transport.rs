use super::*;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct StdTransport {
    base_url: String,
}

impl StdTransport {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    fn parse_url(&self, path: &str) -> Result<(String, u16, String)> {
        // Simple URL parser, for a production implementation you'd want to use a proper URL parser
        let url = format!("{}{}", self.base_url, path);

        let mut parts = url.splitn(3, ':');
        let protocol = parts.next().ok_or_else(|| Error::Custom("Invalid URL".into()))?;

        if protocol != "http" && protocol != "https" {
            return Err(Error::Custom("Only HTTP and HTTPS are supported".into()));
        }

        let domain_part = parts.next().ok_or_else(|| Error::Custom("Invalid URL".into()))?;
        let domain = domain_part.trim_start_matches("//");

        let port = if protocol == "http" { 80 } else { 443 };

        let path_part = parts.next().unwrap_or("/");

        Ok((domain.to_string(), port, path_part.to_string()))
    }
}

pub struct StdClientFactory;

impl RpcClientFactory for StdClientFactory {
    type Transport = StdTransport;

    fn create_transport(base_url: &str) -> Self::Transport {
        StdTransport::new(base_url)
    }
}

impl HttpTransport for StdTransport {
    fn request<M: RpcMethod>(
        &self,
        request: M::Request,
    ) -> Result<M::Response> {
        // Serialize the request to JSON
        let json = serde_json::to_string(&request)
            .map_err(|_| Error::SerializationError)?;

        // Parse the URL
        let (host, port, path) = self.parse_url(M::path())?;

        // Connect to the server
        let mut stream = TcpStream::connect(format!("{}:{}", host, port))
            .map_err(|_| Error::NetworkError)?;

        // Build the HTTP request
        let request = format!(
            "{} {} HTTP/1.1\r\n\
            Host: {}\r\n\
            Content-Type: application/json\r\n\
            Content-Length: {}\r\n\
            Connection: close\r\n\
            \r\n\
            {}",
            M::http_method(), path, host, json.len(), json
        );

        // Send the request
        stream.write_all(request.as_bytes())
            .map_err(|_| Error::NetworkError)?;

        // Read the response
        let mut response = Vec::new();
        stream.read_to_end(&mut response)
            .map_err(|_| Error::NetworkError)?;

        // Parse the HTTP response
        let response_str = String::from_utf8_lossy(&response);
        let parts: Vec<&str> = response_str.splitn(2, "\r\n\r\n").collect();

        if parts.len() != 2 {
            return Err(Error::NetworkError);
        }

        let headers = parts[0];
        let body = parts[1];

        // Check the status code
        let status_line = headers.lines().next().ok_or(Error::NetworkError)?;
        let status_parts: Vec<&str> = status_line.split(' ').collect();

        if status_parts.len() < 3 {
            return Err(Error::NetworkError);
        }

        let status_code = status_parts[1].parse::<u16>()
            .map_err(|_| Error::NetworkError)?;

        if status_code != 200 {
            return Err(Error::HttpError(status_code));
        }

        // Parse the JSON response
        let response: M::Response = serde_json::from_str(body)
            .map_err(|_| Error::DeserializationError)?;

        Ok(response)
    }
}
