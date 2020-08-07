use encoding::{self, label::encoding_from_whatwg_label, EncodingRef};

const DEFAULT_ENCODING: &str = "iso-8859-15";

pub fn determine_encoding(content: &str) -> anyhow::Result<(EncodingRef, String)> {
    let mut reader = xml::reader::EventReader::new(content.as_bytes());
    let encoding = match (content.starts_with("<?xml"), reader.next()?) {
        (true, xml::reader::XmlEvent::StartDocument { encoding, .. }) => encoding,
        _ => {
            debug!("No encoding found. Defaulting to {}", DEFAULT_ENCODING);
            DEFAULT_ENCODING.to_string()
        }
    };

    match encoding_from_whatwg_label(&encoding) {
        Some(enc) => Ok((enc, encoding)),
        None => bail!("Failed to determine encoding!"),
    }
}

pub fn ensure_xml_declaration(content: String, label: String) -> String {
    if content.starts_with("<?xml") {
        content
    } else {
        format!(
            "<?xml version=\"1.0\" encoding=\"{}\" standalone=\"yes\"?>{}",
            label.to_uppercase(),
            content
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_declaration_is_added() {
        let content = "<ab>cd</ab>";
        let expected =
            r##"<?xml version="1.0" encoding="ISO-8859-15" standalone="yes"?><ab>cd</ab>"##;

        assert_eq!(
            expected,
            ensure_xml_declaration(content.to_owned(), "iso-8859-15".to_owned())
        );
    }

    #[test]
    fn xml_declaration_is_not_added() {
        let expected = r##"<?xml version="1.0" encoding="utf-8" standalone="yes"?><ab>cd</ab>"##;

        assert_eq!(
            expected,
            ensure_xml_declaration(expected.to_owned(), "iso-8859-15".to_owned())
        );
    }

    #[test]
    fn encoding_from_xml_declaration_or_default() {
        let undefined_content = "<ab>cd</ab>";
        let iso_content =
            r##"<?xml version="1.0" encoding="iso-8859-15" standalone="yes"?><ab>cd</ab>"##;
        let utf_content = r##"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><ab>cd</ab>"##;

        {
            let r = determine_encoding(undefined_content).unwrap();
            assert_eq!(r.0.name(), "iso-8859-15");
            assert_eq!(r.1, "iso-8859-15");
        }
        {
            let r = determine_encoding(iso_content).unwrap();
            assert_eq!(r.0.name(), "iso-8859-15");
            assert_eq!(r.1, "iso-8859-15");
        }
        {
            let r = determine_encoding(utf_content).unwrap();
            assert_eq!(r.0.name(), "utf-8");
            assert_eq!(r.1, "UTF-8");
        }
    }
}
