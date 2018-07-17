use creek_files::handle_files_on_users_command;
use creek_files::CreekFileType::{
    EgkAllgemein, EgkGeschuetzt, EgkMFDFHCAEF, EgkMFEFGDO, EgkPersoenlich, EgkPruefungsnachweis,
    EgkResult, KvkDaten,
};
use creek_files::*;
use encoding::label::encoding_from_whatwg_label;
use encoding::{self, EncoderTrap, EncodingRef};
use request::K2Response;
use std::fs::File;
use std::io::Write;
use treexml::Document;

macro_rules! unwrap_or_null {
    ($option:ident) => {
        $option.as_ref().unwrap_or(&String::from("null"))
    };
}

macro_rules! filename_by_type {
    ($xml_type:ident) => {
        FILENAMES
            .get(&$xml_type)
            .expect("No matching file type found in known FILENAMES")
    };
}

macro_rules! write_file_if_some {
    ($filename:expr, $option:expr) => {
        if let Some(ref field_var) = $option {
            write_string_to_file(&field_var, $filename);
        }
    };
}

macro_rules! determine_encoding {
    ($string:ident) => {
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
    let encoding = determine_encoding!(string);
    let encoded = match encoding.encode(string, EncoderTrap::Strict) {
        Ok(content) => content,
        Err(why) => panic!("Failed to encode content for {}:\n{}", dest, why),
    };

    let mut file = File::create(dest).expect("Unable to create file");
    file.write_all(&encoded[..]).expect("Unable to write data");
    println!("Wrote file {:?}", dest);
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

fn handle_leftovers() {
    if FILENAMES.values().any(|file| check_exists(file)) {
        handle_files_on_users_command();
    }
}

#[allow(non_snake_case)]
pub fn dump_egk_data_to_files(resp: &K2Response) {
    handle_leftovers();
    if let Some(ref ged) = resp.eGKData {
        write_file_if_some!(filename_by_type!(EgkAllgemein), ged.vd);
        write_file_if_some!(filename_by_type!(EgkGeschuetzt), ged.gvd);
        write_file_if_some!(filename_by_type!(EgkPersoenlich), ged.pd);
        write_file_if_some!(filename_by_type!(EgkMFDFHCAEF), ged.statusVd);
        if let Some(ref pn) = ged.pn {
            write_file_if_some!(filename_by_type!(EgkPruefungsnachweis), pn.xml);
        }
    }
    let error_code_opt = match resp.errorCode {
        Some(code) => Some(code.to_string()),
        None => None,
    };
    write_string_to_file(
        &create_result_xml_string(
            &resp.cardType,
            &resp.iccsn,
            &resp.errorText,
            &resp.instruction,
            &error_code_opt,
        ),
        filename_by_type!(EgkResult),
    );

    write_string_to_file(
        &create_mfefgdo_xml_string(&resp.iccsn),
        filename_by_type!(EgkMFEFGDO),
    );

    if let Some(ref kvkdata) = resp.kvkData {
        let bytes = match ::base64::decode(kvkdata) {
            Ok(content) => content,
            Err(why) => panic!("Failed to decode kvkdata:\n{}", why),
        };

        let mut file = File::create(filename_by_type!(KvkDaten)).expect("Unable to create file");
        file.write_all(&bytes[..]).expect("Unable to write data");
    }
}
