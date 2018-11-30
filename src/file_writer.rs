use encoding::label::encoding_from_whatwg_label;
use encoding::{self, EncoderTrap, EncodingRef};
use std::{fs::File, io::Write};
use treexml::Document;

macro_rules! write_file_if_some {
    ($filename:expr, $option:expr) => {
        if let Some(ref field_var) = $option {
            write_string_to_file(&field_var, $filename);
        }
    };
}

pub fn write_string_to_file(string: &str, dest: &str) {
    let encoding = determine_encoding(string);
    let encoded = match encoding.encode(string, EncoderTrap::Strict) {
        Ok(content) => content,
        Err(why) => panic!("Failed to encode content for {}:\n{}", dest, why),
    };

    let mut file = File::create(dest).expect("Unable to create file");
    file.write_all(&encoded[..]).expect("Unable to write data");
    println!("Wrote file {:?}", dest);
}

fn determine_encoding(data: &str) -> EncodingRef {
    match Document::parse(data.as_bytes()) {
        Ok(doc) => match encoding_from_whatwg_label(&doc.encoding) {
            Some(enc) => enc,
            None => encoding::all::ISO_8859_15 as EncodingRef,
        },
        Err(why) => panic!("Failed to parse string!\n{}", why),
    }
}
