use super::Response;
use crate::CONFIG;
use reqwest::{blocking::get, header::CONTENT_TYPE};

fn url_from_config() -> String {
    format!(
        "{}://{}:{}{}",
        &CONFIG.read().k2.scheme,
        &CONFIG.read().k2.host,
        &CONFIG.read().k2.port,
        &CONFIG.read().k2.path
    )
}

pub fn request() -> Response {
    let url = url_from_config();
    match get(&url) {
        Ok(response) => {
            let headers = response.headers().clone();
            let status = response.status();
            if let Some(content_header) = headers.get(CONTENT_TYPE) {
                if content_header == "application/json" {
                    match response.json() {
                        Ok(json) => json,
                        Err(ref err) if err.is_redirect() => {
                            panic!("Redirect loop when attempting to get JSON from K2")
                        }
                        Err(err) => panic!(
                            "Unable to receive JSON from K2. Status: {:?} - Error: {:?}",
                            status, err
                        ),
                    }
                } else {
                    super::handle_response_failure(response)
                }
            } else {
                super::handle_response_failure(response)
            }
        }
        Err(err) => panic!("Request failed: {:?}", err),
    }
}
