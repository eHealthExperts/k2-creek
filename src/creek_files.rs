extern crate promptly;

use self::promptly::Promptable;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use CONFIG;

#[derive(PartialEq, Eq, Hash)]
pub enum CreekFileType {
    EgkAllgemein,
    EgkGeschuetzt,
    EgkPersoenlich,
    EgkMFDFHCAEF,
    EgkPruefungsnachweis,
    EgkMFEFGDO,
    EgkResult,
    KvkDaten,
}
lazy_static! {
    pub static ref FILENAMES: HashMap<CreekFileType, &'static str> = {
        let mut map = HashMap::new();
        map.insert(
            CreekFileType::EgkAllgemein,
            "eGK_allgemeineVersicherungsdaten.xml",
        );
        map.insert(
            CreekFileType::EgkGeschuetzt,
            "eGK_geschuetzteVersichertendaten.xml",
        );
        map.insert(
            CreekFileType::EgkPersoenlich,
            "eGK_PersoenlicheVersichertendaten.xml",
        );
        map.insert(CreekFileType::EgkMFDFHCAEF, "eGK_MFDF_HCA_EF_StatusVD.xml");
        map.insert(
            CreekFileType::EgkPruefungsnachweis,
            "eGK_Pruefungsnachweis.xml",
        );
        map.insert(CreekFileType::EgkMFEFGDO, "eGK_MFEFGDO.xml");
        map.insert(CreekFileType::EgkResult, "Result.xml");
        map.insert(CreekFileType::KvkDaten, "KVK_Daten.bin");
        map
    };
}

pub fn check_exists(path: &str) -> bool {
    Path::new(&format!("./{}", path)).exists()
}

pub fn handle_files_on_users_command() {
    let delete = CONFIG.is_force_delete()
        || bool::prompt_default(
            "WARNING - Old files found in output folder. Delete before proceeding?",
            false,
        );
    if delete {
        FILENAMES.values().for_each(|file| {
            if check_exists(file) {
                fs::remove_file(file)
                    .unwrap_or_else(|_| panic!("Unable to delete {}. Aborting...", file));
                println!("Deleted old {}", file);
            }
        });
    } else {
        println!("Continuing with file generation. You will probably end up with an inconsistent set of result files.");
    }
}
