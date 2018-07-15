use reqwest::header::ContentType;
use reqwest::StatusCode;

const CARD_NOT_FOUND_RESPONSE: &'static str = "card with filter not found";
const CARD_NOT_FOUND_FILE_OUTPUT: &'static str = "Keine Karte gefunden";

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
    } else if status.is_strange_status() {
        panic!("K2 response status was outside HTTP RFC range")
    }
    let resp_body = resp.text().expect(&format!(
        "Unable to read K2 response body. Status: {:?}",
        status
    ));
    match status {
        StatusCode::Ok => panic!(
            "Response status was OK but had unexpected body: {:?}",
            resp_body
        ),
        StatusCode::NotFound => {
            if resp_body == CARD_NOT_FOUND_RESPONSE {
                println!("No card was found. This will be reflected in the output file.");
                let mut ret = K2Response {
                    ..Default::default()
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

pub fn request_egk_data(url: &str) -> K2Response {
    match ::reqwest::get(url) {
        Ok(ref mut resp) => {
            let headers = resp.headers().clone();
            if let Some(content_header) = headers.get::<ContentType>() {
                if format!("{}/{}", content_header.type_(), content_header.subtype())
                    == "application/json"
                {
                    match resp.json() {
                        Ok(json) => json,
                        Err(_) => panic!(
                            "Unable to deserialize JSON response. Status: {:?}",
                            resp.status()
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
