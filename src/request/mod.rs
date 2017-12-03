extern crate reqwest;

#[derive(Deserialize)]
pub struct ProofOfTest {
    pub xml: String
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct GetEgkData {
    pub vd: String,
    pub gvd: String,
    pub pd: String,
    pub statusVd: String,
    pub kvkdata: Option<String>,
    pub pn: ProofOfTest
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct K2Response {
    pub cardType: String,
    pub iccsn: String,
    pub errorText: String,
    pub instruction: String,
    pub errorCode: Option<String>,
    pub geteGKData: GetEgkData
}

pub fn request_egk_data(url: &str) -> Option<K2Response> {
    match reqwest::get(url) {
        Ok(ref mut resp) => match resp.json() {
            Ok(json) => Some(json),
            Err(e) => panic!("parsing response failed: {:?}", e)
        },
        Err(e) => panic!("request failed: {:?}", e)
    }
}
