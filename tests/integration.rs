extern crate encoding;
extern crate nv_xml;
extern crate reqwest;
extern crate serde_json;
extern crate test_server;

use encoding::all::ISO_8859_15;
use encoding::{DecoderTrap, Encoding};
use nv_xml::XmlParser;
use serde_json::Value;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::process::Command;
use test_server::{HttpResponse, TestServer};

const BIN_PATH: &'static str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/target/debug/",
    env!("CARGO_PKG_NAME")
);

const AVD: &'static str = "eGK_allgemeineVersicherungsdaten.xml";
const GVD: &'static str = "eGK_geschuetzteVersichertendaten.xml";
const STATUSVD: &'static str = "eGK_MFDF_HCA_EF_StatusVD.xml";
const PVD: &'static str = "eGK_PersoenlicheVersichertendaten.xml";
const PN: &'static str = "eGK_Pruefungsnachweis.xml";
const MFEFGDO: &'static str = "eGK_MFEFGDO.xml";
const DATEN: &'static str = "KVK_Daten.bin";
const RESULT: &'static str = "Result.xml";

macro_rules! first_child_data {
    ($element:ident) => {
        $element.first().unwrap().data().unwrap()
    };
}

fn read_file(file: &str) -> String {
    let mut file = File::open(file).unwrap();
    let mut ret = Vec::new();
    file.read_to_end(&mut ret).unwrap();
    ISO_8859_15.decode(&ret[..], DecoderTrap::Strict).unwrap()
}

fn delete_files() {
    for file in vec![AVD, GVD, STATUSVD, PVD, PN, MFEFGDO, DATEN, RESULT] {
        match fs::remove_file(file) {
            Ok(_) => {}
            _ => {}
        }
    }
}

#[test]
fn example_response() {
    delete_files();

    let _server = TestServer::new(8089, |_| {
        let content = read_file("tests/example_response.json");
        HttpResponse::Ok()
            .header("Content-Type", "application/json")
            .body(content)
            .into()
    });

    let _ = Command::new(BIN_PATH).output().unwrap();

    let contents = read_file("tests/example_response.json");
    let json: Value = serde_json::from_str(&contents).unwrap();

    assert!(Path::new(AVD).exists());
    let vd = read_file(AVD);
    assert_eq!(vd, json["eGKData"]["vd"]);

    assert!(Path::new(GVD).exists());
    let gvd = read_file(GVD);
    assert_eq!(gvd, json["eGKData"]["gvd"]);

    assert!(Path::new(STATUSVD).exists());
    let status_vd = read_file(STATUSVD);
    assert_eq!(status_vd, json["eGKData"]["statusVd"]);

    assert!(Path::new(PVD).exists());
    let pd = read_file(PVD);
    assert_eq!(pd, json["eGKData"]["pd"]);

    assert!(Path::new(PN).exists());
    let xml = read_file(PN);
    assert_eq!(xml, json["eGKData"]["pn"]["xml"]);

    assert!(Path::new(MFEFGDO).exists());
    let mfefgdo_xml = read_file(MFEFGDO);
    let mfefgdo = XmlParser::parse(&mfefgdo_xml).unwrap();
    let mfefgdo_iccsn = mfefgdo.children_with_name("MFEF_GDO_Value_ICCSN");
    assert_eq!(first_child_data!(mfefgdo_iccsn), json["iccsn"]);

    assert!(Path::new(RESULT).exists());
    let result_xml = read_file(RESULT);
    let result = XmlParser::parse(&result_xml).unwrap();
    let result_card_type = result.children_with_name("cardType");
    assert_eq!(first_child_data!(result_card_type), json["cardType"]);
    let result_iccsn = result.children_with_name("iccsn");
    assert_eq!(first_child_data!(result_iccsn), json["iccsn"]);
    let result_error_text = result.children_with_name("errorText");
    assert_eq!(first_child_data!(result_error_text), json["errorText"]);
    let result_instruction = result.children_with_name("instruction");
    assert_eq!(first_child_data!(result_instruction), json["instruction"]);
    let result_error_code = result.children_with_name("errorCode");
    assert_eq!(first_child_data!(result_error_code), "null");
    assert_eq!(None, json["errorCode"].as_str());

    assert!(Path::new(DATEN).exists());
    assert!(fs::metadata(DATEN).unwrap().len() > 0);
}

#[test]
#[cfg(not(target_os = "windows"))] // carllerche/mio#776
fn example_response_with_error_code() {
    delete_files();

    let _server = TestServer::new(8089, |_| {
        let content = read_file("tests/example_response_with_error_code.json");
        HttpResponse::Ok()
            .header("Content-Type", "application/json")
            .body(content)
            .into()
    });

    let _ = Command::new(BIN_PATH).output().unwrap();

    assert!(Path::new(RESULT).exists());
    let result_xml = read_file(RESULT);
    let result = XmlParser::parse(&result_xml).unwrap();
    let result_error_code = result.children_with_name("errorCode");
    assert_eq!(first_child_data!(result_error_code), "123");
}

#[test]
#[cfg(not(target_os = "windows"))] // carllerche/mio#776
fn example_response_with_many_nulls() {
    delete_files();

    let _server = TestServer::new(8089, |_| {
        let content = read_file("tests/example_response_with_many_nulls.json");
        HttpResponse::Ok()
            .header("Content-Type", "application/json")
            .body(content)
            .into()
    });

    let _ = Command::new(BIN_PATH).output().unwrap();

    let contents = read_file("tests/example_response_with_many_nulls.json");
    let json: Value = serde_json::from_str(&contents).unwrap();

    assert!(Path::new(AVD).exists());
    let vd = read_file(AVD);
    assert_eq!(vd, json["eGKData"]["vd"]);

    assert!(Path::new(GVD).exists());
    let gvd = read_file(GVD);
    assert_eq!(gvd, json["eGKData"]["gvd"]);

    assert!(Path::new(STATUSVD).exists());
    let status_vd = read_file(STATUSVD);
    assert_eq!(status_vd, json["eGKData"]["statusVd"]);

    assert!(Path::new(PVD).exists());
    let pd = read_file(PVD);
    assert_eq!(pd, json["eGKData"]["pd"]);

    assert_eq!(false, Path::new(PN).exists());

    assert_eq!(false, Path::new(DATEN).exists());

    assert!(Path::new(MFEFGDO).exists());
    let mfefgdo_xml = read_file(MFEFGDO);
    let mfefgdo = XmlParser::parse(&mfefgdo_xml).unwrap();
    let mfefgdo_iccsn = mfefgdo.children_with_name("MFEF_GDO_Value_ICCSN");
    assert_eq!(first_child_data!(mfefgdo_iccsn), json["iccsn"]);

    assert!(Path::new(RESULT).exists());
    let result_xml = read_file(RESULT);
    let result = XmlParser::parse(&result_xml).unwrap();
    let result_card_type = result.children_with_name("cardType");
    assert_eq!(first_child_data!(result_card_type), json["cardType"]);
    let result_iccsn = result.children_with_name("iccsn");
    assert_eq!(first_child_data!(result_iccsn), json["iccsn"]);
    let result_error_text = result.children_with_name("errorText");
    assert_eq!(first_child_data!(result_error_text), "null");
    assert_eq!(None, json["errorText"].as_str());
    let result_instruction = result.children_with_name("instruction");
    assert_eq!(first_child_data!(result_instruction), "null");
    assert_eq!(None, json["instruction"].as_str());
    let result_error_code = result.children_with_name("errorCode");
    assert_eq!(first_child_data!(result_error_code), "null");
    assert_eq!(None, json["errorCode"].as_str());
}

#[test]
#[cfg(not(target_os = "windows"))] // carllerche/mio#776
fn example_response_with_no_matching_card_filter() {
    delete_files();

    let _server = test_server::new(8089, |_| {
        HttpResponse::NotFound()
            .body(" card with filter not found ")
            .into()
    });

    let _ = Command::new(BIN_PATH).output().unwrap();

    assert_eq!(false, Path::new(AVD).exists());
    assert_eq!(false, Path::new(GVD).exists());
    assert_eq!(false, Path::new(STATUSVD).exists());
    assert_eq!(false, Path::new(PVD).exists());
    assert_eq!(false, Path::new(PN).exists());
    assert_eq!(false, Path::new(DATEN).exists());
    assert_eq!(false, Path::new(MFEFGDO).exists());

    assert!(Path::new(RESULT).exists());
    let result_xml = read_file(RESULT);
    let result = XmlParser::parse(&result_xml).unwrap();
    let result_card_type = result.children_with_name("cardType");
    assert_eq!(first_child_data!(result_card_type), "null");
    let result_iccsn = result.children_with_name("iccsn");
    assert_eq!(first_child_data!(result_iccsn), "null");
    let result_error_text = result.children_with_name("errorText");
    assert_eq!(first_child_data!(result_error_text), "Keine Karte gefunden");
    let result_instruction = result.children_with_name("instruction");
    assert_eq!(first_child_data!(result_instruction), "null");
    let result_error_code = result.children_with_name("errorCode");
    assert_eq!(first_child_data!(result_error_code), "null");
}
