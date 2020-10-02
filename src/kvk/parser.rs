use crate::kvk::Model;
use der_parser::{
    ber::{parse_ber_explicit_failed, BerTag},
    der::der_read_element_header,
    error::BerError as Error,
    flat_take, parse_der_application,
};
use nom::{alt, call, complete, do_parse, take, verify, IResult};
use rusticata_macros::custom_check;

pub(crate) fn parse_app(data: &'_ [u8]) -> IResult<&'_ [u8], Model<'_>, Error> {
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
        ( Model {
            kkn, kknr, vknr, vnr, vs, se, t, v, nz, f, gd, sn, wlc, plz, on, g } )
    )
    .map(|(rem, t)| (rem, t.1))
}

fn parse_der_tagged(i: &[u8], tag: u32) -> IResult<&[u8], &[u8], Error> {
    do_parse!(
        i,
        hdr: der_read_element_header
            >> custom_check!(hdr.tag != BerTag(tag), Error::InvalidTag)
            >> content: take!(hdr.len)
            >> (content)
    )
}

fn parse_optional_der_tagged(i: &[u8], tag: u32) -> IResult<&[u8], Option<&[u8]>, Error> {
    alt!(
        i,
        complete!(do_parse!(
            content: call!(parse_der_tagged, tag) >> (Some(content))
        )) | do_parse!(_content: call!(parse_ber_explicit_failed, BerTag(0)) >> (None))
    )
}
