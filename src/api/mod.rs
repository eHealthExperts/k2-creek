pub(crate) mod v1;
pub(crate) mod v2;
pub(crate) mod v3;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Api {
    V1(v1::Carddata),
    V2(v2::Carddata),
    V3(v3::Cardsdata),
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, Deserialize)]
pub struct StatusData {
    pub errorCode: Option<String>,
    pub errorText: Option<String>,
    pub instruction: Option<String>,
}

#[cfg(test)]
mod tests {

    use super::Api;
    use std::fs::File;

    #[test]
    fn deserialize_egk_carddata_v1() {
        crate::tests::init_logger();

        let file = File::open("tests/api/v1/egk.json").unwrap();
        let result = serde_json::from_reader::<_, Api>(&file);

        match result {
            Ok(Api::V1(_)) => (),
            _ => panic!("Wrong version!"),
        }
    }

    #[test]
    fn deserialize_kvk_carddata_v1() {
        crate::tests::init_logger();

        let file = File::open("tests/api/v1/kvk.json").unwrap();
        let result = serde_json::from_reader::<_, Api>(&file);

        match result {
            Ok(Api::V1(_)) => (),
            _ => panic!("Wrong version!"),
        }
    }

    #[test]
    fn deserialize_egk_carddata_v2() {
        crate::tests::init_logger();

        let file = File::open("tests/api/v2/egk.json").unwrap();
        let result = serde_json::from_reader::<_, Api>(&file);

        match result {
            Ok(Api::V2(_)) => (),
            _ => panic!("Wrong version!"),
        }
    }

    #[test]
    fn deserialize_kvk_carddata_v2() {
        crate::tests::init_logger();

        let file = File::open("tests/api/v2/kvk.json").unwrap();
        let result = serde_json::from_reader::<_, Api>(&file);

        match result {
            Ok(Api::V2(_)) => (),
            _ => panic!("Wrong version!"),
        }
    }

    #[test]
    fn deserialize_egk_carddata_v3() {
        crate::tests::init_logger();

        let file = File::open("tests/api/v3/egk.json").unwrap();
        let result = serde_json::from_reader::<_, Api>(&file);

        match result {
            Ok(Api::V3(_)) => (),
            _ => panic!("Wrong version!"),
        }
    }

    #[test]
    fn deserialize_kvk_carddata_v3() {
        crate::tests::init_logger();

        let file = File::open("tests/api/v3/kvk.json").unwrap();
        let result = serde_json::from_reader::<_, Api>(&file);

        match result {
            Ok(Api::V3(_)) => (),
            _ => panic!("Wrong version!"),
        }
    }
}
