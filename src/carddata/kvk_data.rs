use der_parser::{
    ber::{parse_ber_explicit_failed, BerTag},
    der::der_read_element_header,
    error::BerError,
    flat_take, parse_der_application,
};
use nom::{alt, call, complete, do_parse, take, verify, IResult};
use rusticata_macros::custom_check;
use std::{fmt, str};

macro_rules! fmt(
    ($bytes:expr) => (
        if let Ok(value) = str::from_utf8($bytes) {
            let chars: Vec<_> = value.chars().map(|c| {
                match c {
                    '@' => "§".to_string(),
                    '~' => "ß".to_string(),
                    '{' => "ä".to_string(),
                    '}' => "ü".to_string(),
                    '|' => "ö".to_string(),
                    '[' => "Ä".to_string(),
                    ']' => "Ü".to_string(),
                    '\\' => "Ö".to_string(),
                    _ => c.to_string()
                }
            }).collect();
            chars.join("")
        } else {
            String::from("!Fehler!")
        }
    )
);

#[derive(Debug, Deserialize)]
struct KvkData<'a> {
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

impl<'a> fmt::Display for KvkData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "KrankenKassenName:    {}\n\
             KrankenKassenNummer:  {}\n\
             VKNR:                 {}\n\
             VersichertenNummer:   {}\n\
             VersichertenStatus:   {}\n\
             StatusErgänzung:      {}",
            fmt!(self.kkn),
            fmt!(self.kknr),
            fmt!(self.vknr),
            fmt!(self.vnr),
            fmt!(self.vs),
            fmt!(self.se)
        )?;

        if let Some(t) = self.t {
            writeln!(f, "Titel:                {}", fmt!(t))?;
        }

        writeln!(f, "VorName:              {}", fmt!(self.v))?;

        if let Some(nz) = self.nz {
            writeln!(f, "NamensZusatz:         {}", fmt!(nz))?;
        }

        writeln!(
            f,
            "FamilienName:         {}\n\
             GeburtsDatum:         {}",
            fmt!(self.f),
            fmt!(self.gd)
        )?;

        if let Some(sn) = self.sn {
            writeln!(f, "Straßenname:          {}", fmt!(sn))?;
        }

        if let Some(wlc) = self.wlc {
            writeln!(f, "WohnsitzLänderCode:   {}", fmt!(wlc))?;
        }

        write!(
            f,
            "Postleitzahl:         {}\n\
             Orstname:             {}\n\
             GültigkeitsDatum:     {}",
            fmt!(self.plz),
            fmt!(self.on),
            fmt!(self.g)
        )
    }
}

fn parse_der_tagged(i: &[u8], tag: u32) -> IResult<&[u8], &[u8], BerError> {
    do_parse!(
        i,
        hdr: der_read_element_header
            >> custom_check!(hdr.tag != BerTag(tag), BerError::InvalidTag)
            >> content: take!(hdr.len)
            >> (content)
    )
}

fn parse_optional_der_tagged(i: &[u8], tag: u32) -> IResult<&[u8], Option<&[u8]>, BerError> {
    alt!(
        i,
        complete!(do_parse!(
            content: call!(parse_der_tagged, tag) >> (Some(content))
        )) | do_parse!(_content: call!(parse_ber_explicit_failed, BerTag(0)) >> (None))
    )
}

fn parse_app<'a>(data: &'a [u8]) -> IResult<&'a [u8], KvkData<'_>, BerError> {
    parse_der_application!(
        data,
        APPLICATION 0,
        kkn: call!(parse_der_tagged, 0) >>
        kknr: call!(parse_der_tagged, 1) >>
        vknr: call!(parse_der_tagged, 15) >>
        vnr: call!(parse_der_tagged, 2) >>
        vs: call!(parse_der_tagged, 3) >>
        se: call!(parse_der_tagged, 16) >>
        t: call!(parse_optional_der_tagged, 4) >>
        v: call!(parse_der_tagged, 5) >>
        nz: call!(parse_optional_der_tagged, 6) >>
        f: call!(parse_der_tagged, 7) >>
        gd: call!(parse_der_tagged, 8) >>
        sn: call!(parse_optional_der_tagged, 9) >>
        wlc: call!(parse_optional_der_tagged, 10) >>
        plz: call!(parse_der_tagged, 11) >>
        on: call!(parse_der_tagged, 12) >>
        g: call!(parse_der_tagged, 13) >>
        //ps: apply!(parse_der_tagged, 14) >>
        ( KvkData {
            kkn, kknr, vknr, vnr, vs, se, t, v, nz, f, gd, sn, wlc, plz, on, g } )
    )
    .map(|(rem, t)| (rem, t.1))
}

pub fn parse(data: &str) -> Option<String> {
    let bytes = match ::base64::decode(&data) {
        Ok(content) => content,
        Err(why) => panic!("Failed to decode kvkdata:\n{}", why),
    };

    match parse_app(&bytes) {
        Ok(kvkdata) => Some(kvkdata.1.to_string()),
        Err(why) => {
            println!("{}", why);
            None
        }
    }
}

#[test]
fn fmt_replaces_chars_according_to_din_66003() {
    let ascii = vec![64, 126, 123, 125, 124, 91, 93, 92];

    assert_eq!(b"@~{}|[]\\", &ascii[..]);
    assert_eq!("§ßäüöÄÜÖ", fmt!(&ascii));
}

#[test]
fn fmt_not_replaces_latin_alphabet_and_arabic_numbers() {
    let sequence = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    assert_eq!(sequence, fmt!(sequence.as_bytes()));
}

#[test]
fn parse_consume_base64_encoded_asn1_der_sequence_and_return_kvkdata_as_formatted_string() {
    let d = "YIGXgBpCdW5kZXNwb2xpemVpLUtyYW5rZW5rYXNzZYEHMzYwMDM0Mo8FMDAwMjCCDDEyMzQ1Njc4OTAxM4MEMTAwMJABMYUSRGFuaWVsIEd1c3RhdiBMdXR6hwZIfG5zY2iICDE3MDUxOTYxiRJDYXJsLVdvbGZmLVN0ci4gMTKKAUSLBTQ1Mjc5jAVFc3Nlbo0EMTAyMY4BiA==";
    let s = parse(d);

    assert_eq!(
        Some(String::from(
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
        )),
        s
    );
}
