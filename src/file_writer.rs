use encoding::{self, EncoderTrap, EncodingRef};
use encoding::label::encoding_from_whatwg_label;
use request::K2Response;
use std::fs::File;
use std::io::Write;
use std::str;
use treexml::Document;

macro_rules! unwrap_or_null {
    ($option: ident) => {
        $option.as_ref().unwrap_or(&String::from("null"))
    };
}

macro_rules! write_file_if_some {
    ($filename: expr, $option: expr) => {
        if let Some(ref field_var) = $option {
            write_string_to_file(&field_var, $filename);
        }
    };
}

macro_rules! determine_encoding {
    ($string: ident) => {
        match Document::parse($string.as_bytes()) {
            Ok(doc) => match encoding_from_whatwg_label(&doc.encoding) {
                Some(enc) => enc,
                None => encoding::all::ISO_8859_15 as EncodingRef,
            },
            Err(why) => panic!("Failed to parse string!\n{}", why),
        }
    };
}

fn write_string_to_file(string: &str, dest: &str) {
    let encoder = determine_encoding!(string);
    let encoded = match encoder.encode(string, EncoderTrap::Strict) {
        Ok(content) => content,
        Err(why) => panic!("Failed to encode content for {}:\n{}", dest, why),
    };

    let mut file = File::create(dest).expect("Unable to create file");
    file.write_all(&encoded[..]).expect("Unable to write data");
}

#[allow(non_snake_case)]
fn create_result_xml_string(
    cardType: &Option<String>,
    iccsn: &Option<String>,
    errorText: &Option<String>,
    instruction: &Option<String>,
    errorCode: &Option<String>,
) -> String {
    format!(
        r#"<?xml version="1.0"?>
<Results>
    <cardType>{}</cardType>
    <iccsn>{}</iccsn>
    <errorText>{}</errorText>
    <instruction>{}</instruction>
    <errorCode>{}</errorCode>
</Results>"#,
        unwrap_or_null!(cardType),
        unwrap_or_null!(iccsn),
        unwrap_or_null!(errorText),
        unwrap_or_null!(instruction),
        unwrap_or_null!(errorCode)
    )
}

fn create_mfefgdo_xml_string(iccsn: &Option<String>) -> String {
    format!(
        r#"<?xml version="1.0"?>
<eGK_MFEF_GDO_Hexadezimal>
    <MFEF_GDO_Tag_ICCSN>5A</MFEF_GDO_Tag_ICCSN>
    <MFEF_GDO_Length_ICCSN>0A</MFEF_GDO_Length_ICCSN>
    <MFEF_GDO_Value_ICCSN>{}</MFEF_GDO_Value_ICCSN>
</eGK_MFEF_GDO_Hexadezimal>"#,
        unwrap_or_null!(iccsn)
    )
}

#[allow(non_snake_case)]
pub fn dump_egk_data_to_files(resp: &K2Response) {
    if let Some(ref ged) = resp.eGKData {
        write_file_if_some!("eGK_allgemeineVersicherungsdaten.xml", ged.vd);
        write_file_if_some!("eGK_geschuetzteVersichertendaten.xml", ged.gvd);
        write_file_if_some!("eGK_PersoenlicheVersichertendaten.xml", ged.pd);
        write_file_if_some!("eGK_MFDF_HCA_EF_StatusVD.xml", ged.statusVd);
        if let Some(ref pn) = ged.pn {
            write_file_if_some!("eGK_Pruefungsnachweis.xml", pn.xml);
        }
    }

    write_string_to_file(
        &create_result_xml_string(
            &resp.cardType,
            &resp.iccsn,
            &resp.errorText,
            &resp.instruction,
            &resp.errorCode,
        ),
        "Result.xml",
    );

    write_string_to_file(&create_mfefgdo_xml_string(&resp.iccsn), "eGK_MFEFGDO.xml");

    if let Some(ref kvkdata) = resp.kvkData {
        let bytes = match ::base64::decode(kvkdata) {
            Ok(content) => content,
            Err(why) => panic!("Failed to decode kvkdata:\n{}", why),
        };

        let mut file = File::create("KVK_Daten.bin").expect("Unable to create file");
        file.write_all(&bytes[..]).expect("Unable to write data");
    }
}
