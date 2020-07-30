#[derive(Deserialize, Debug)]
pub struct Pn {
    pub xml: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct EgkData {
    pub vd: Option<String>,
    pub gvd: Option<String>,
    pub pd: Option<String>,
    pub statusVd: Option<String>,
    pub pn: Option<Pn>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Default, Debug)]
pub struct K2Response {
    pub cardType: Option<String>,
    pub eGKData: Option<EgkData>,
    pub errorCode: Option<usize>,
    pub errorText: Option<String>,
    pub iccsn: Option<String>,
    pub instruction: Option<String>,
    pub kvkData: Option<String>,
    pub status: Option<String>,
    pub terminalId: Option<String>,
}

#[cfg(test)]
mod tests {

    use super::K2Response as Response;
    use std::fs::File;

    #[test]
    fn deserialize_response() {
        let file = File::open("tests/k2/response.json").unwrap();
        let response = serde_json::from_reader::<_, Response>(&file);

        assert!(response.is_ok());
    }
}
