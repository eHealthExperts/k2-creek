use serde::{de::Error, Deserialize, Deserializer};

mod fmt;
mod parser;

#[derive(Debug)]
pub struct Model<'a> {
    kkn: &'a [u8],
    kknr: &'a [u8],
    vknr: &'a [u8],
    vnr: &'a [u8],
    vs: &'a [u8],
    se: &'a [u8],
    t: Option<&'a [u8]>,
    v: &'a [u8],
    nz: Option<&'a [u8]>,
    f: &'a [u8],
    gd: &'a [u8],
    sn: Option<&'a [u8]>,
    wlc: Option<&'a [u8]>,
    plz: &'a [u8],
    on: &'a [u8],
    g: &'a [u8],
}

#[cfg_attr(test, derive(Default))]
#[derive(Clone, Debug)]
pub struct KvkData {
    pub pretty: String,
    pub raw: Vec<u8>,
}

impl KvkData {
    pub fn parse(data: &str) -> anyhow::Result<Self> {
        trace!("Decoded: {}", data);

        let bytes = ::base64::decode(&data)?;
        trace!("Encoded: {:?}", bytes);

        parser::parse_app(&bytes)
            .map_err(anyhow::Error::from)
            .map(|(_, model)| {
                trace!("ASN.1 decoded:\n{}", model);
                Self {
                    raw: bytes.clone(),
                    pretty: model.to_string(),
                }
            })
    }

    pub fn write(&self) -> anyhow::Result<()> {
        crate::writer::files::Files::KvkDaten.write_raw(&self.pretty.as_bytes())?;
        crate::writer::files::Files::KvkBinDaten.write_raw(&self.raw)
    }
}

impl<'de> Deserialize<'de> for KvkData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::parse(s.as_str()).map_err(D::Error::custom)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn decode_base64_encoded_asn1_data() {
        crate::tests::init_logger();

        let data = "YIGXgBpCdW5kZXNwb2xpemVpLUtyYW5rZW5rYXNzZYEHMzYwMDM0Mo8FMDAwMjCCDDEyMzQ1Njc4OTAxM4MEMTAwMJABMYUSRGFuaWVsIEd1c3RhdiBMdXR6hwZIfG5zY2iICDE3MDUxOTYxiRJDYXJsLVdvbGZmLVN0ci4gMTKKAUSLBTQ1Mjc5jAVFc3Nlbo0EMTAyMY4BiA==";

        assert_eq!(
            String::from(
                "\
                KrankenKassenName:    Bundespolizei-Krankenkasse\n\
                KrankenKassenNummer:  3600342\n\
                VKNR:                 00020\n\
                VersichertenNummer:   123456789013\n\
                VersichertenStatus:   1000\n\
                StatusErgänzung:      1\n\
                VorName:              Daniel Gustav Lutz\n\
                FamilienName:         Hönsch\n\
                GeburtsDatum:         17051961\n\
                Straßenname:          Carl-Wolff-Str. 12\n\
                WohnsitzLänderCode:   D\n\
                Postleitzahl:         45279\n\
                Orstname:             Essen\n\
                GültigkeitsDatum:     1021\
                "
            ),
            super::KvkData::parse(data).unwrap().pretty
        );
    }
}
