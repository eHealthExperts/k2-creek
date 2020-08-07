#[derive(Clone, Debug, Deserialize)]
pub struct Wrapper {
    pub data: crate::kvk::KvkData,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Carddata {
    pub cardType: String,
    pub ehcData: Option<crate::egk::EgkData>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub iccsn: Option<String>,
    pub kvkData: Option<Wrapper>,
    pub status: String,
    pub statusData: crate::api::StatusData,
    pub terminalId: String,
}

use crate::writer::results::Results;

impl Carddata {
    pub fn write(&self) -> anyhow::Result<()> {
        Results::write(
            &self.cardType,
            self.iccsn.clone(),
            Some(self.status.clone()),
            Some(self.statusData.clone()),
            &self.terminalId,
        )?;

        if let Some(wrapper) = self.kvkData.clone() {
            wrapper.data.write()
        } else {
            self.ehcData
                .clone()
                .expect("No EGK data for writing found!")
                .write(&self.iccsn.clone().expect("No ICCSN found!"))
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

        let file = std::fs::File::open("tests/api/v2/egk.json").unwrap();
        let result = serde_json::from_reader::<_, Carddata>(&file);

        assert!(result.is_ok());

        let carddata = result.unwrap();
        assert!(carddata.ehcData.is_some());
        assert!(carddata.kvkData.is_none());
    }

    #[test]
    #[serial(io)]
    fn deserialize_kvk_carddata() {
        crate::tests::init_logger();

        let file = std::fs::File::open("tests/api/v2/kvk.json").unwrap();
        let result = serde_json::from_reader::<_, Carddata>(&file);

        assert!(result.is_ok());

        let carddata = result.unwrap();
        assert!(carddata.ehcData.is_none());
        assert!(carddata.kvkData.is_some());
    }
}
