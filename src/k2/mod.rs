pub use request::request;
pub use response::K2Response as Response;

mod request;
mod response;

const CARD_NOT_FOUND_RESPONSE: &str = "card with filter not found";
const CARD_NOT_FOUND_FILE_OUTPUT: &str = "Keine Karte gefunden";

pub fn handle_response_failure(resp: reqwest::blocking::Response) -> Response {
    let status = resp.status();
    if status.is_server_error() {
        panic!("K2 server ran into error state")
    }

    let resp_body = resp
        .text()
        .unwrap_or_else(|_| panic!("Unable to read K2 response body. Status: {:?}", status));

    match status {
        reqwest::StatusCode::OK => panic!(
            "Response status was OK but had unexpected body: {:?}",
            resp_body
        ),
        reqwest::StatusCode::NOT_FOUND => {
            if resp_body.trim() == CARD_NOT_FOUND_RESPONSE {
                println!("No card was found. This will be reflected in the output file.");
                let mut ret = Response {
                    ..Response::default()
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
