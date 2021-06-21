use crate::{
    api::StatusData,
    egk::EgkData,
    kvk::KvkData,
    writer::{results::Results, Write},
};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Carddata {
    pub cardType: String,
    pub eGKData: EgkData,
    pub errorCode: Option<String>,
    pub errorDetail: Option<String>,
    pub errorText: Option<String>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub iccsn: Option<String>,
    pub instruction: Option<String>,
    pub kvkData: Option<KvkData>,
    pub status: String,
    pub terminalId: String,
}

impl Write for Carddata {
    fn write(&self) -> anyhow::Result<()> {
        Results::write(
            &self.cardType,
            self.iccsn.clone(),
            Some(self.status.clone()),
            Some(StatusData {
                errorCode: self.errorCode.clone(),
                errorText: self.errorText.clone(),
                instruction: self.instruction.clone(),
            }),
            &self.terminalId,
        )?;

        match (&self.kvkData, &self.iccsn) {
            (Some(kvk_data), None) => kvk_data.write(),
            (None, Some(iccsn)) => self.eGKData.write(iccsn),
            _ => bail!("Neither an ICCSN nor KVK data found!"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[serial(io)]
    fn deserialize_egk_carddata() {
        crate::tests::init_logger();

        debug!("\n\nDIR\n {:?} \n\n", std::env::current_dir());

        let file = std::fs::File::open("tests/api/v1/egk.json").unwrap();
        let result = serde_json::from_reader::<_, Carddata>(&file);

        assert!(result.is_ok());

        let carddata = result.unwrap();
        assert!(carddata.eGKData.gvd.is_some());
        assert!(carddata.kvkData.is_none());
    }

    #[test]
    #[serial(io)]
    fn deserialize_kvk_carddata() {
        crate::tests::init_logger();

        let file = std::fs::File::open("tests/api/v1/kvk.json").unwrap();
        let result = serde_json::from_reader::<_, Carddata>(&file);

        assert!(result.is_ok());

        let carddata = result.unwrap();
        assert!(carddata.eGKData.gvd.is_none());
        assert!(carddata.kvkData.is_some());
    }
}
