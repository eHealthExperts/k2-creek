#[derive(Debug, Serialize)]
#[serde(rename = "eGK_MFEF_GDO_Hexadezimal")]
pub struct MFEFGDO {
    #[serde(rename = "MFEF_GDO_Tag_ICCSN")]
    pub tag: String,
    #[serde(rename = "MFEF_GDO_Length_ICCSN")]
    pub length: String,
    #[serde(rename = "MFEF_GDO_Value_ICCSN")]
    pub value: String,
}

impl MFEFGDO {
    pub fn new(iccsn: String) -> MFEFGDO {
        MFEFGDO {
            length: String::from("0A"),
            tag: String::from("5A"),
            value: iccsn,
        }
    }
}
