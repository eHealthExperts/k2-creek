use crate::file_writer::write_string_to_file;
use crate::files::{self, FileTypes::*, FILES};
use crate::k2::Response;
use crate::CONFIG;
use promptly::Promptable;
use serde_xml_rs::ser::to_writer;
use std::{fs::File, io::Write, str};

mod kvk_data;
mod mfefgdo;
mod results;

macro_rules! unwrap_or_null {
    ($option:expr) => {
        $option.unwrap_or(String::from("null"))
    };
}

pub fn write_carddata(data: &Response) {
    cleanup();

    if let Some(ref egk) = data.eGKData {
        write_file_if_some!(filename_by_type!(EgkAllgemein), egk.vd);
        write_file_if_some!(filename_by_type!(EgkGeschuetzt), egk.gvd);
        write_file_if_some!(filename_by_type!(EgkPersoenlich), egk.pd);
        write_file_if_some!(filename_by_type!(EgkMFDFHCAEF), egk.statusVd);

        if let Some(ref pn) = egk.pn {
            write_file_if_some!(filename_by_type!(EgkPruefungsnachweis), pn.xml);
        }
    }

    if let Some(iccsn) = &data.iccsn {
        let mut buffer = Vec::new();
        to_writer(&mut buffer, &mfefgdo::MFEFGDO::new(iccsn.to_string()))
            .expect("Failed to serialize 'eGK_MFEF_GDO_Hexadezimal'");

        write_string_to_file(
            str::from_utf8(&buffer)
                .expect("Failed to read slices of 'eGK_MFEF_GDO_Hexadezimal' as UTF-8"),
            filename_by_type!(EgkMFEFGDO),
        );
    }

    {
        // write Results.xml
        let error_code_opt = match data.errorCode {
            Some(code) => Some(code.to_string()),
            None => None,
        };

        let mut buffer = Vec::new();
        to_writer(
            &mut buffer,
            &results::Results {
                cardType: unwrap_or_null!(data.cardType.clone()),
                errorCode: unwrap_or_null!(error_code_opt),
                errorText: unwrap_or_null!(data.errorText.clone()),
                iccsn: unwrap_or_null!(data.iccsn.clone()),
                instruction: unwrap_or_null!(data.instruction.clone()),
                status: unwrap_or_null!(data.status.clone()),
                terminalId: unwrap_or_null!(data.terminalId.clone()),
            },
        )
        .expect("Failed to serialize 'Results'");

        write_string_to_file(
            str::from_utf8(&buffer).expect("Failed to read slices of 'Results' as UTF-8"),
            filename_by_type!(EgkResult),
        );
    }

    if let Some(ref kvkdata_der) = data.kvkData {
        match ::base64::decode(kvkdata_der) {
            Ok(content) => {
                let mut file =
                    File::create(filename_by_type!(KvkBinDaten)).expect("Unable to create file");
                file.write_all(&content[..]).expect("Unable to write data");
            }
            Err(why) => panic!("Failed to decode kvkdata:\n{}", why),
        }

        let kvkdata = kvk_data::parse(kvkdata_der).expect("Failed to parse kvkdata");
        let mut file = File::create(filename_by_type!(KvkDaten)).expect("Unable to create file");
        file.write_all(kvkdata.as_bytes())
            .expect("Unable to write data");
    }
}

fn cleanup() {
    if FILES.keys().any(files::FileTypes::exists) {
        let delete = CONFIG.read().output.force_delete
            || bool::prompt_default(
                "WARNING - Old files found in output folder. Delete before proceeding?",
                false,
            );

        if delete {
            FILES.keys().for_each(files::FileTypes::delete);
        } else {
            println!("Continuing with file generation. You will probably end up with an inconsistent set of result files.");
        }
    }
}
