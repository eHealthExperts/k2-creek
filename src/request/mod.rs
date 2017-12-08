extern crate reqwest;

#[derive(Deserialize)]
pub struct ProofOfTest {
    pub xml: Option<String>
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct EgkData {
    pub vd: Option<String>,
    pub gvd: Option<String>,
    pub pd: Option<String>,
    pub statusVd: Option<String>,
    pub kvkdata: Option<String>,
    pub pn: Option<ProofOfTest>
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct K2Response {
    pub cardType: Option<String>,
    pub iccsn: Option<String>,
    pub errorText: Option<String>,
    pub instruction: Option<String>,
    pub errorCode: Option<String>,
    pub eGKData: Option<EgkData>
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
