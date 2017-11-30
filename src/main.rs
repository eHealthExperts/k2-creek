extern crate ini;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

use ini::Ini;
use std::fs::File;
use std::io::{Error, Write};

const DEFAULT_SCHEME : &'static str = "http";
const DEFAULT_HOST : &'static str = "localhost";
const DEFAULT_PORT : &'static str = "5000";
const DEFAULT_PATH : &'static str = "/k2/public/api/1/carddata";

#[derive(Deserialize)]
struct ProofOfTest {
    xml: String
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct GetEgkData {
    vd: String,
    gvd: String,
    pd: String,
    statusVd: String,
    kvkdata: Option<String>,
    pn: ProofOfTest
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct K2Response {
    cardType: String,
    iccsn: String,
    errorText: String,
    instruction: String,
    errorCode: Option<String>,
    geteGKData: GetEgkData
}

fn request_egk_data(url: &str) -> Result<K2Response, Error> {
    let resp : K2Response = reqwest::get(url).unwrap().json().unwrap();

    /*println!("vd: {},\ngvd: {},\npd: {},\nstatusVd: {},\nkvkdata: {:?},\npn.xml: {}",
             &resp.geteGKData.vd,
             &resp.geteGKData.gvd,
             &resp.geteGKData.pd,
             &resp.geteGKData.statusVd,
             &resp.geteGKData.kvkdata,
             &resp.geteGKData.pn.xml);*/
    Ok(resp)
}

fn write_string_to_file(string: &str, dest: &str) {
    let mut f = File::create(dest).expect("Unable to create file");
    f.write_all(string.as_bytes()).expect("Unable to write data");
}

#[allow(non_snake_case)]
fn create_result_xml_string(cardType: &str, iccsn: &str, errorText: &str, instruction: &str,
                            errorCode: &Option<String>) -> String {
    String::from(format!(
r#"<?xml version=\"1.0\"?>
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

fn dump_egk_data_to_file(url: &str) {
    let resp = request_egk_data(url);
    let resp_ref = resp.as_ref().unwrap();
    write_string_to_file(&resp_ref.geteGKData.vd, "eGK_allgemeineVersicherungsdaten.xml");
    write_string_to_file(&resp_ref.geteGKData.gvd, "eGK_geschuetzteVersichertendaten.xml");
    write_string_to_file(&resp_ref.geteGKData.pd, "eGK_PersoenlicheVersichertendaten.xml");
    write_string_to_file(&resp_ref.geteGKData.statusVd, "eGK_MFDF_HCA_EF_StatusVD.xml");
    if resp_ref.geteGKData.kvkdata.is_some() {
        let kvkdata_ref = resp_ref.geteGKData.kvkdata.as_ref().unwrap();
        write_string_to_file(&kvkdata_ref, "eGK_allgemeineVersicherungsdaten.xml");
    }
    write_string_to_file(&resp_ref.geteGKData.pn.xml, "eGK_Pruefungsnachweis.xml");
    write_string_to_file(&create_result_xml_string(&resp_ref.cardType,
                                                   &resp_ref.iccsn,
                                                   &resp_ref.errorText,
                                                   &resp_ref.instruction,
                                                   &resp_ref.errorCode),
                                                   "Result.xml");
    write_string_to_file(&create_mfefgdo_xml_string(&resp_ref.iccsn), "eGK_MFEFGDO.xml");
}

fn generate_url_from_config() -> String {
    let conf = Ini::load_from_file("config.ini").unwrap();
    let section = conf.section(Some("settings".to_owned())).unwrap();
    let def_scheme = DEFAULT_SCHEME.to_owned();
    let scheme = section.get("scheme").unwrap_or(&def_scheme);
    let def_host = DEFAULT_HOST.to_owned();
    let host = section.get("host").unwrap_or(&def_host);
    let def_port = DEFAULT_PORT.to_owned();
    let port = section.get("port").unwrap_or(&def_port);
    let def_path = DEFAULT_PATH.to_owned();
    let path = section.get("path").unwrap_or(&def_path);
    let url = format!("{}://{}:{}{}", scheme, host, port, path);
    url.to_owned()
}

fn main() {
    let url = generate_url_from_config();
    println!("Retrieving data from {}", &url);

    dump_egk_data_to_file(&url);
}
