#[cfg(feature = "wasm")]
mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use js_sys::{Promise, JSON};
    use web_sys::{Request, RequestInit, RequestMode, Response};

    pub struct WasmTransport {
        base_url: String,
    }

    impl WasmTransport {
        pub fn new(base_url: &str) -> Self {
            Self {
                base_url: base_url.to_string(),
            }
        }
    }

    pub struct WasmClientFactory;

    impl RpcClientFactory for WasmClientFactory {
        type Transport = WasmTransport;

        fn create_transport(base_url: &str) -> Self::Transport {
            WasmTransport::new(base_url)
        }
    }

    impl HttpTransport for WasmTransport {
        fn request<M: RpcMethod>(
            &self,
            request: M::Request,
        ) -> Result<M::Response> {
            // Create a future that will be executed by JavaScript
            let fut = async {
                let json = serde_json::to_string(&request)
                    .map_err(|_| Error::SerializationError)?;

                // Create a RequestInit object
                let mut opts = RequestInit::new();
                opts.method(M::http_method());
                opts.mode(RequestMode::Cors);
                opts.body(Some(&JsValue::from_str(&json)));

                // Create a Request object
                let url = format!("{}{}", self.base_url, M::path());
                let request = Request::new_with_str_and_init(&url, &opts)
                    .map_err(|_| Error::NetworkError)?;

                // Set the Content-Type header
                request.headers().set("Content-Type", "application/json")
                    .map_err(|_| Error::NetworkError)?;

                // Send the request
                let window = web_sys::window().ok_or(Error::NetworkError)?;
                let resp_value = JsFuture::from(window.fetch_with_request(&request))
                    .await
                    .map_err(|_| Error::NetworkError)?;

                // Convert the response to a Response object
                let resp: Response = resp_value.dyn_into()
                    .map_err(|_| Error::NetworkError)?;

                // Check if the response is successful
                if !resp.ok() {
                    return Err(Error::HttpError(resp.status()));
                }

                // Get the response text
                let text = JsFuture::from(resp.text().map_err(|_| Error::NetworkError)?)
                    .await
                    .map_err(|_| Error::NetworkError)?;

                let text_str = text.as_string().ok_or(Error::DeserializationError)?;

                // Parse the response JSON
                let response: M::Response = serde_json::from_str(&text_str)
                    .map_err(|_| Error::DeserializationError)?;

                Ok(response)
            };

            // Execute the future synchronously (this is a simplified approach)
            // In a real application, you'd need to properly handle futures in WASM
            // This is a simplification and would require additional code or a crate like wasm-bindgen-futures
            #[cfg(target_arch = "wasm32")]
            {
                wasm_bindgen_futures::spawn_local(async move {
                    // This is where you'd handle the future and return the result
                    // But for our example, we're simplifying
                });
            }

            // This is a placeholder - in reality you'd need to properly handle the async nature
            // of the request in your WASM environment
            Err(Error::Custom("Async execution not fully implemented in this example".into()))
        }
    }
}
