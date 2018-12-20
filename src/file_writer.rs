use encoding::label::encoding_from_whatwg_label;
use encoding::{self, EncoderTrap, EncodingRef};
use std::{borrow::Cow, fs::File, io::Write};
use treexml::Document;

macro_rules! write_file_if_some {
    ($filename:expr, $option:expr) => {
        if let Some(ref field_var) = $option {
            write_string_to_file(&field_var, $filename);
        }
    };
}

pub fn write_string_to_file(string: &str, dest: &str) {
    let (encoding, label) = determine_encoding(string);
    let content = ensure_xml_declaration(string, &label);
    let encoded = match encoding.encode(&content, EncoderTrap::Strict) {
        Ok(content) => content,
        Err(why) => panic!("Failed to encode content for {}:\n{}", dest, why),
    };

    let mut file = File::create(dest).expect("Unable to create file");
    file.write_all(&encoded[..]).expect("Unable to write data");
    println!("Wrote file {:?}", dest);
}

fn determine_encoding<'a>(data: &str) -> (EncodingRef, Cow<'a, str>) {
    match Document::parse(data.as_bytes()) {
        Ok(doc) => match encoding_from_whatwg_label(&doc.encoding) {
            Some(enc) => (enc, Cow::Owned(doc.encoding)),
            None => (
                encoding::all::ISO_8859_15 as EncodingRef,
                Cow::Borrowed("iso-8859-15"),
            ),
        },
        Err(why) => panic!("Failed to parse string!\n{}", why),
    }
}

fn ensure_xml_declaration<'a>(content: &'a str, label: &str) -> Cow<'a, str> {
    if content.starts_with("<?xml") {
        return Cow::Borrowed(content);
    }

    Cow::Owned(format!(
        "<?xml version=\"1.0\" encoding=\"{}\" standalone=\"yes\"?>{}",
        label.to_uppercase(),
        content
    ))
}
