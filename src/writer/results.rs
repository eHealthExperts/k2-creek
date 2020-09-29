#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct Results {
    pub cardType: String,
    pub errorCode: String,
    pub errorText: String,
    pub iccsn: String,
    pub instruction: String,
    pub status: String,
    pub terminalId: String,
}

macro_rules! unwrap_or_null {
    ($option:expr) => {
        $option.unwrap_or("null".to_string())
    };
}

impl Results {
    pub fn write(
        card_type: &str,
        iccsn: Option<String>,
        status: Option<String>,
        status_data: Option<crate::api::StatusData>,
        terminal_id: &str,
    ) -> anyhow::Result<()> {
        let status_data = status_data.unwrap_or_default();
        let mut buffer = Vec::new();

        serde_xml_rs::ser::to_writer(
            &mut buffer,
            &Self {
                cardType: card_type.to_string(),
                errorCode: unwrap_or_null!(status_data.errorCode),
                errorText: unwrap_or_null!(status_data.errorText),
                iccsn: unwrap_or_null!(iccsn),
                instruction: unwrap_or_null!(status_data.instruction),
                status: unwrap_or_null!(status),
                terminalId: terminal_id.to_string(),
            },
        )?;

        crate::writer::files::Files::EgkResult.write(&String::from_utf8_lossy(&buffer))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_server::helper;

    test! {
        name: optional_values_are_null,
        temp_dir: true,
        vars: [
            card_type => helper::random_string(10),
            terminal_id => helper::random_string(10)
        ],
        it: it,
        steps: {
            Results::write(&card_type, None, None, None, &terminal_id)?;
        },
        assert: [
            "Result.xml" => format!("<?xml version=\"1.0\" encoding=\"ISO-8859-15\" standalone=\"yes\"?>\
                             <Results>\
                             <cardType>{}</cardType>\
                             <errorCode>null</errorCode>\
                             <errorText>null</errorText>\
                             <iccsn>null</iccsn>\
                             <instruction>null</instruction>\
                             <status>null</status>\
                             <terminalId>{}</terminalId>\
                             </Results>", card_type, terminal_id).as_bytes()
        ]
    }
}
