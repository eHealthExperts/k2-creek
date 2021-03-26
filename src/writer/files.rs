use enum_iterator::IntoEnumIterator;

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[derive(IntoEnumIterator)]
pub enum Files {
    EgkAllgemein,
    EgkGeschuetzt,
    EgkPersoenlich,
    EgkStatusVD,
    EgkPruefungsnachweis,
    EgkMFEFGDO,
    EgkResult,
    KvkBinDaten,
    KvkDaten,
}

impl Files {
    pub fn cleanup() {
        Files::into_enum_iter().for_each(|f| f.delete());
    }

    pub fn delete(&self) {
        if self.exists() {
            match std::env::current_dir() {
                Err(why) => panic!("Failed to get current dir! No file was deleted!\n{}", why),
                Ok(path) => match std::fs::remove_file(path.join(self.filename())) {
                    Ok(_) => debug!("Deleted {:?}", self.filename()),
                    Err(why) => panic!("Failed to delete {:?}!\n{}", self.filename(), why),
                },
            }
        }
    }

    pub fn exists(&self) -> bool {
        match std::env::current_dir() {
            Ok(path) => path.join(self.filename()).exists(),
            Err(why) => panic!("Failed to access current dir!\n{}", why),
        }
    }

    pub fn filename(&self) -> &str {
        match self {
            Files::EgkAllgemein => "eGK_allgemeineVersicherungsdaten.xml",
            Files::EgkGeschuetzt => "eGK_geschuetzteVersichertendaten.xml",
            Files::EgkPersoenlich => "eGK_PersoenlicheVersichertendaten.xml",
            Files::EgkStatusVD => "eGK_MFDF_HCA_EF_StatusVD.xml",
            Files::EgkPruefungsnachweis => "eGK_Pruefungsnachweis.xml",
            Files::EgkMFEFGDO => "eGK_MFEFGDO.xml",
            Files::EgkResult => "Result.xml",
            Files::KvkBinDaten => "KVK_Daten.bin",
            Files::KvkDaten => "KVK.dat",
        }
    }

    pub fn present() -> bool {
        Files::into_enum_iter().any(|f| f.exists())
    }

    pub fn write(&self, content: &str) -> anyhow::Result<()> {
        let (encoding, label) = super::encode::determine_encoding(content)?;
        let xml = super::encode::ensure_xml_declaration(content.to_owned(), label);
        let encoded = encoding
            .encode(&xml, encoding::EncoderTrap::Strict)
            .map_err(anyhow::Error::msg)?;

        self.write_raw(&encoded)
    }

    pub fn write_raw(&self, bytes: &[u8]) -> anyhow::Result<()> {
        let output_dir = std::env::current_dir()?;
        let mut file = std::fs::File::create(output_dir.join(self.filename()))?;

        use std::io::Write;
        file.write_all(bytes)?;

        debug!("Wrote file {:?}", self.filename());
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn filenames() {
        assert_eq!(
            Files::EgkAllgemein.filename(),
            "eGK_allgemeineVersicherungsdaten.xml"
        );
        assert_eq!(
            Files::EgkGeschuetzt.filename(),
            "eGK_geschuetzteVersichertendaten.xml"
        );
        assert_eq!(
            Files::EgkPersoenlich.filename(),
            "eGK_PersoenlicheVersichertendaten.xml"
        );
        assert_eq!(
            Files::EgkStatusVD.filename(),
            "eGK_MFDF_HCA_EF_StatusVD.xml"
        );
        assert_eq!(
            Files::EgkPruefungsnachweis.filename(),
            "eGK_Pruefungsnachweis.xml"
        );
        assert_eq!(Files::EgkMFEFGDO.filename(), "eGK_MFEFGDO.xml");
        assert_eq!(Files::EgkResult.filename(), "Result.xml");
        assert_eq!(Files::KvkBinDaten.filename(), "KVK_Daten.bin");
        assert_eq!(Files::KvkDaten.filename(), "KVK.dat");
    }
}
