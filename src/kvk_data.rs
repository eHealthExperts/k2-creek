use der_parser::{
    der_read_element_header, flat_take, parse_der_application, parse_der_explicit_failed,
    DER_TAG_ERROR,
};
use nom::{ErrorKind, IResult};
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

#[derive(Debug, PartialEq)]
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

pub fn parse(data: &str) -> Option<String> {
    let bytes = match ::base64::decode(&data) {
        Ok(content) => content,
        Err(why) => panic!("Failed to decode kvkdata:\n{}", why),
    };

    match parse_as_kvkdata(&bytes) {
        Ok(kvkdata) => Some(kvkdata.1.to_string()),
        Err(_) => None,
    }
}

fn parse_as_kvkdata(i: &[u8]) -> IResult<&[u8], KvkData> {
    parse_der_application!(
        i,
        APPLICATION 0,
        kkn: apply!(parse_value, 0) >>
        kknr: apply!(parse_value, 1) >>
        vknr: apply!(parse_value, 15) >>
        vnr: apply!(parse_value, 2) >>
        vs: apply!(parse_value, 3) >>
        se: apply!(parse_value, 16) >>
        t: apply!(parse_optional_value, 4) >>
        v: apply!(parse_value, 5) >>
        nz: apply!(parse_optional_value, 6) >>
        f: apply!(parse_value, 7) >>
        gd: apply!(parse_value, 8) >>
        sn: apply!(parse_optional_value, 9) >>
        wlc: apply!(parse_optional_value, 10) >>
        plz: apply!(parse_value, 11) >>
        on: apply!(parse_value, 12) >>
        g: apply!(parse_value, 13) >>
        _ps: apply!(parse_value, 14) >>
        ( KvkData {
            kkn, kknr, vknr, vnr, vs, se, t, v, nz, f, gd, sn, wlc, plz, on, g } )
    )
    .map(|(rem, t)| (rem, t.1))
}

fn parse_value(i: &[u8], tag: u8) -> IResult<&[u8], &[u8]> {
    do_parse!(
        i,
        hdr: der_read_element_header
            >> error_if!(hdr.tag != tag as u8, ErrorKind::Custom(DER_TAG_ERROR))
            >> content: take!(hdr.len)
            >> (content)
    )
}

fn parse_optional_value(i: &[u8], tag: u8) -> IResult<&[u8], Option<&[u8]>> {
    alt_complete!(
        i,
        do_parse!(content: call!(parse_value, tag) >> (Some(content)))
            | do_parse!(_content: call!(parse_der_explicit_failed, 0) >> (None))
    )
}

#[test]
fn fmt_returns_error_string_if_invalid_bytes_given() {
    let buggy = vec![0, 159, 146, 150];

    assert_eq!("!Fehler!", fmt!(&buggy))
}

#[test]
fn fmt_replaces_chars_according_to_din_66003() {
    let ascii = vec![64, 126, 123, 125, 124, 91, 93, 92];

    assert_eq!("@~{}|[]\\".as_bytes(), &ascii[..]);
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
