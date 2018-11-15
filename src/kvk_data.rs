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

macro_rules! parse_optional_value(
    ($i:expr, $fun:ident, $($args:expr),*) => (
        alt_complete!(
            $i,
            do_parse!(
                content: call!($fun, $($args),*) >>
                (
                    Some(content)
                )
            ) |
            do_parse!(
                _content: call!(parse_der_explicit_failed, 0) >>
                (
                    None
                )
            )
        )
    )
);

macro_rules! parse_value(
    ($i:expr, $tag:expr) => ({
        do_parse!(
            $i,
            hdr: der_read_element_header
                >> error_if!(hdr.tag != $tag as u8, ErrorKind::Custom(DER_TAG_ERROR))
                >> content: take!(hdr.len)
                >> (content)
        )
    });
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

fn parse_value(i: &[u8], tag: u8) -> IResult<&[u8], &[u8]> {
    parse_value!(i, tag)
}

fn parse_optional_value(i: &[u8], tag: u8) -> IResult<&[u8], Option<&[u8]>> {
    parse_optional_value!(i, parse_value, tag)
}
