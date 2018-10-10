use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;

const CARD_NOT_FOUND_RESPONSE: &str = "card with filter not found";
const CARD_NOT_FOUND_FILE_OUTPUT: &str = "Keine Karte gefunden";

#[derive(Deserialize)]
pub struct ProofOfTest {
    pub xml: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct EgkData {
    pub vd: Option<String>,
    pub gvd: Option<String>,
    pub pd: Option<String>,
    pub statusVd: Option<String>,
    pub pn: Option<ProofOfTest>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Default)]
pub struct K2Response {
    pub cardType: Option<String>,
    pub iccsn: Option<String>,
    pub errorText: Option<String>,
    pub instruction: Option<String>,
    pub errorCode: Option<i32>,
    pub eGKData: Option<EgkData>,
    pub kvkData: Option<String>,
}

fn handle_response_failure_cases(resp: &mut ::reqwest::Response) -> K2Response {
    let status = resp.status();
    if status.is_server_error() {
        panic!("K2 server ran into error state")
    }
    let resp_body = resp
        .text()
        .unwrap_or_else(|_| panic!("Unable to read K2 response body. Status: {:?}", status));
    match status {
        StatusCode::OK => panic!(
            "Response status was OK but had unexpected body: {:?}",
            resp_body
        ),
        StatusCode::NOT_FOUND => {
            if resp_body.trim() == CARD_NOT_FOUND_RESPONSE {
                println!("No card was found. This will be reflected in the output file.");
                let mut ret = K2Response {
                    ..K2Response::default()
                };
                ret.errorText = Some(CARD_NOT_FOUND_FILE_OUTPUT.to_string());
                ret
            } else {
                panic!("Received 404 with unknown body from K2. Are we calling the right URL?")
            }
        }
        _ => panic!(
            "Failed to get expected response from K2. Status: {:?}",
            status
        ),
    }
}

pub fn fetch_egk_data(url: &str) -> K2Response {
    match ::reqwest::get(url) {
        Ok(ref mut resp) => {
            let headers = resp.headers().clone();
            if let Some(content_header) = headers.get(CONTENT_TYPE) {
                if content_header == "application/json" {
                    match resp.json() {
                        Ok(json) => json,
                        Err(ref e) if e.is_redirect() => {
                            panic!("Redirect loop when attempting to get JSON from K2")
                        }
                        Err(ref e) if e.is_serialization() => {
                            if let Some(serde_error) = e.get_ref() {
                                panic!(
                                    "Unable to deserialize payload to JSON. Status: {:?} - Error: {:?}",
                                    resp.status(), serde_error
                                )
                            } else {
                                panic!(
                                    "Unable to deserialize payload for unknown reason. Status: {:?}",
                                    resp.status()
                                )
                            }
                        }
                        Err(e) => panic!(
                            "Unable to receive JSON from K2. Status: {:?} - Error: {:?}",
                            resp.status(),
                            e
                        ),
                    }
                } else {
                    handle_response_failure_cases(resp)
                }
            } else {
                handle_response_failure_cases(resp)
            }
        }
        Err(e) => panic!("Request failed: {:?}", e),
    }
}
