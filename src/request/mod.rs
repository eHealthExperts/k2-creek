extern crate reqwest;

use std::io::Error;

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

pub fn request_egk_data(url: &str) -> Result<K2Response, Error> {
    let resp : K2Response = reqwest::get(url).unwrap().json().unwrap();

    /*println!("vd: {},\ngvd: {},\npd: {},\nstatusVd: {},\nkvkdata: {:?},\npn.xml: {}",
    &resp.geteGKData.vd,
    &resp.geteGKData.gvd,
    &resp.geteGKData.pd,
    &resp.geteGKData.statusVd,
    &resp.geteGKData.kvkdata,
    &resp.geteGKData.pn.xml);*/
    Ok(resp)
}
