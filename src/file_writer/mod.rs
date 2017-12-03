use std::fs::File;
use std::io::Write;

use request::K2Response;

fn write_string_to_file(string: &str, dest: &str) {
    let mut f = File::create(dest).expect("Unable to create file");
    f.write_all(string.as_bytes()).expect("Unable to write data");
}

#[allow(non_snake_case)]
fn create_result_xml_string(cardType: &str, iccsn: &str, errorText: &str, instruction: &str,
    errorCode: &Option<String>) -> String {
    String::from(format!(
r#"<?xml version="1.0"?>
<Results>
    <cardType>{}</cardType>
    <iccsn>{}</iccsn>
    <errorText>{}</errorText>
    <instruction>{}</instruction>
    <errorCode>{}</errorCode>
</Results>"#,
    cardType,
    iccsn,
    errorText,
    instruction,
    errorCode.as_ref().unwrap_or(&String::from("null"))))
}

fn create_mfefgdo_xml_string(iccsn: &str) -> String {
    String::from(format!(
r#"<?xml version="1.0"?>
<eGK_MFEF_GDO_Hexadezimal>
    <MFEF_GDO_Tag_ICCSN>5A</MFEF_GDO_Tag_ICCSN>
    <MFEF_GDO_Length_ICCSN>0A</MFEF_GDO_Length_ICCSN>
    <MFEF_GDO_Value_ICCSN>{}</MFEF_GDO_Value_ICCSN>
</eGK_MFEF_GDO_Hexadezimal>"#,
    iccsn))
}

pub fn dump_egk_data_to_files(resp: &K2Response) {
    write_string_to_file(&resp.geteGKData.vd, "eGK_allgemeineVersicherungsdaten.xml");
    write_string_to_file(&resp.geteGKData.gvd, "eGK_geschuetzteVersichertendaten.xml");
    write_string_to_file(&resp.geteGKData.pd, "eGK_PersoenlicheVersichertendaten.xml");
    write_string_to_file(&resp.geteGKData.statusVd, "eGK_MFDF_HCA_EF_StatusVD.xml");
    if let Some(ref kvkdata) = resp.geteGKData.kvkdata {
        write_string_to_file(&kvkdata, "eGK_allgemeineVersicherungsdaten.xml");
    }
    write_string_to_file(&resp.geteGKData.pn.xml, "eGK_Pruefungsnachweis.xml");
    write_string_to_file(&create_result_xml_string(&resp.cardType,
                                                   &resp.iccsn,
                                                   &resp.errorText,
                                                   &resp.instruction,
                                                   &resp.errorCode),
                                                   "Result.xml");
    write_string_to_file(&create_mfefgdo_xml_string(&resp.iccsn), "eGK_MFEFGDO.xml");
}
