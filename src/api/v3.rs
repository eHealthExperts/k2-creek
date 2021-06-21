#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Wrapper {
    pub data: crate::kvk::KvkData,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Card {
    pub cardType: String,
    pub ehcData: Option<crate::egk::EgkData>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub iccsn: Option<String>,
    pub kvkData: Option<Wrapper>,
    pub status: Option<String>,
    pub statusData: Option<crate::api::StatusData>,
    pub terminalId: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Cardsdata {
    pub cards: Vec<Card>,
}

use crate::writer::results::Results;

impl Cardsdata {
    pub fn write(&self) -> anyhow::Result<()> {
        if let Some(card) = self
            .cards
            .iter()
            .find(|c| ["EHC", "KVK"].contains(&c.cardType.as_str()))
        {
            Results::write(
                &card.cardType,
                card.iccsn.clone(),
                card.status.clone(),
                card.statusData.clone(),
                &card.terminalId,
            )?;

            if card.cardType == "KVK" {
                card.kvkData
                    .as_ref()
                    .expect("No KVK data found!")
                    .data
                    .write()
            } else {
                card.ehcData
                    .as_ref()
                    .expect("No EGK data found!")
                    .write(card.iccsn.as_ref().expect("No ICCSN found!"))
            }
        } else {
            bail!("No card of type 'EHC' or 'KVK' found! No files written!")
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

        let file = std::fs::File::open("tests/api/v3/egk.json").unwrap();
        let result = serde_json::from_reader::<_, Cardsdata>(&file);

        assert!(result.is_ok());

        let cardsdata = result.unwrap();
        let card = cardsdata.cards.iter().find(|&c| c.cardType == "EHC");

        assert!(card.is_some());
        assert!(card.unwrap().ehcData.is_some());
        assert!(card.unwrap().kvkData.is_none());
    }

    #[test]
    #[serial(io)]
    fn deserialize_kvk_carddata() {
        crate::tests::init_logger();

        let file = std::fs::File::open("tests/api/v3/kvk.json").unwrap();
        let result = dbg!(serde_json::from_reader::<_, Cardsdata>(&file));

        assert!(result.is_ok());

        let cardsdata = result.unwrap();
        let card = cardsdata.cards.iter().find(|&c| c.cardType == "KVK");

        assert!(card.is_some());
        assert!(card.unwrap().ehcData.is_none());
        assert!(card.unwrap().kvkData.is_some());
    }
}
