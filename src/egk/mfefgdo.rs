#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Default))]
#[serde(rename = "eGK_MFEF_GDO_Hexadezimal")]
#[allow(clippy::upper_case_acronyms)]
pub struct MFEFGDO {
    #[serde(rename = "MFEF_GDO_Tag_ICCSN")]
    pub tag: String,
    #[serde(rename = "MFEF_GDO_Length_ICCSN")]
    pub length: String,
    #[serde(rename = "MFEF_GDO_Value_ICCSN")]
    pub value: String,
}

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
impl MFEFGDO {
    pub fn write(iccsn: &str) -> anyhow::Result<()> {
        let mut buffer = Vec::new();
        serde_xml_rs::ser::to_writer(
            &mut buffer,
            &Self {
                length: "0A".to_string(),
                tag: "5A".to_string(),
                value: iccsn.to_string(),
            },
        )?;

        crate::writer::files::Files::EgkMFEFGDO.write(&String::from_utf8_lossy(&buffer))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::random_string;

    test! {
        name: valid_xml_layout,
        temp_dir: true,
        vars: [
            iccsn => &random_string(10)
        ],
        it: it,
        steps: {
            MFEFGDO::write(iccsn).unwrap();
        },
        assert: [
            "eGK_MFEFGDO.xml" => format!("<?xml version=\"1.0\" encoding=\"ISO-8859-15\" standalone=\"yes\"?>\
                                  <eGK_MFEF_GDO_Hexadezimal>\
                                  <MFEF_GDO_Tag_ICCSN>5A</MFEF_GDO_Tag_ICCSN>\
                                  <MFEF_GDO_Length_ICCSN>0A</MFEF_GDO_Length_ICCSN>\
                                  <MFEF_GDO_Value_ICCSN>{}</MFEF_GDO_Value_ICCSN>\
                                  </eGK_MFEF_GDO_Hexadezimal>", iccsn).as_bytes()
        ]
    }
}
