#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct Results {
    pub cardType: String,
    pub errorCode: String,
    pub errorText: String,
    pub iccsn: String,
    pub instruction: String,
    pub status: String,
    pub terminalId: String,
}
