use encoding::{EncoderTrap, Encoding};
use encoding::all::ISO_8859_15;
use request::K2Response;
use std::fs::File;
use std::io::Write;

macro_rules! unwrap_or_null {
    ($option:ident) => (
        $option.as_ref().unwrap_or(&String::from("null"))
    )
}

macro_rules! write_file_if_some {
    ($filename:expr,$option:expr) => (
        if let Some(ref field_var) = $option {
            write_string_to_file(&field_var, $filename);
        }
    )
}

fn write_string_to_file(string: &str, dest: &str) {
    if let Ok(encoded) = ISO_8859_15.encode(string, EncoderTrap::Strict) {
        let mut f = File::create(dest).expect("Unable to create file");
        f.write_all(&encoded[..]).expect("Unable to write data");
    } else {
        panic!("Failed to encode content for {}", dest)
    }
}

#[allow(non_snake_case)]
fn create_result_xml_string(
    cardType: &Option<String>,
    iccsn: &Option<String>,
    errorText: &Option<String>,
    instruction: &Option<String>,
    errorCode: &Option<String>,
) -> String {
    String::from(format!(
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
    ))
}

fn create_mfefgdo_xml_string(iccsn: &Option<String>) -> String {
    String::from(format!(
        r#"<?xml version="1.0"?>
<eGK_MFEF_GDO_Hexadezimal>
    <MFEF_GDO_Tag_ICCSN>5A</MFEF_GDO_Tag_ICCSN>
    <MFEF_GDO_Length_ICCSN>0A</MFEF_GDO_Length_ICCSN>
    <MFEF_GDO_Value_ICCSN>{}</MFEF_GDO_Value_ICCSN>
</eGK_MFEF_GDO_Hexadezimal>"#,
        unwrap_or_null!(iccsn)
    ))
}

#[allow(non_snake_case)]
pub fn dump_egk_data_to_files(resp: &K2Response) {
    if let Some(ref ged) = resp.eGKData {
        write_file_if_some!("eGK_allgemeineVersicherungsdaten.xml", ged.vd);
        write_file_if_some!("eGK_geschuetzteVersichertendaten.xml", ged.gvd);
        write_file_if_some!("eGK_PersoenlicheVersichertendaten.xml", ged.pd);
        write_file_if_some!("eGK_MFDF_HCA_EF_StatusVD.xml", ged.statusVd);
        write_file_if_some!("KVK_Daten.xml", ged.kvkdata);
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
}
