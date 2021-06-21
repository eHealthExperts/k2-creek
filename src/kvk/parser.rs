use crate::kvk::Model;
use yasna::{ASN1Error, ASN1ErrorKind, ASN1Result, Tag};

macro_rules! next {
    ($reader:expr, $tag:tt) => {
        match $reader.next().read_tagged_der() {
            Ok(v) if v.tag().tag_number == $tag => Ok(v.value().to_vec()),
            _ => Err(ASN1Error::new(ASN1ErrorKind::Invalid)),
        }?
    };
}

macro_rules! next_opt {
    ($reader:expr, $tag:tt) => {{
        let reader = $reader.next();
        match reader.lookahead_tag() {
            Ok(tag) if tag.tag_number == $tag => Some(reader.read_tagged_der()?.value().to_vec()),
            _ => None,
        }
    }};
}

pub(crate) fn parse_app(data: &'_ [u8]) -> ASN1Result<Model> {
    yasna::parse_der(data, |reader| {
        reader.read_tagged_implicit(Tag::application(0), |reader| {
            reader.read_sequence(|reader| {
                let kkn = next!(reader, 0);
                trace!("kkn {:x?}", kkn);
                let kknr = next!(reader, 1);
                trace!("kknr {:x?}", kknr);
                let vknr = next!(reader, 15);
                trace!("vknr {:x?}", vknr);
                let vnr = next!(reader, 2);
                trace!("vnr {:x?}", vnr);
                let vs = next!(reader, 3);
                trace!("vs {:x?}", vs);
                let se = next!(reader, 16);
                trace!("se {:x?}", se);
                let t = next_opt!(reader, 4);
                trace!("t {:x?}", t);
                let v = next!(reader, 5);
                trace!("v {:x?}", v);
                let nz = next_opt!(reader, 6);
                trace!("nz {:x?}", nz);
                let f = next!(reader, 7);
                trace!("f {:x?}", f);
                let gd = next!(reader, 8);
                trace!("gd {:x?}", gd);
                let sn = next_opt!(reader, 9);
                trace!("sn {:x?}", sn);
                let wlc = next_opt!(reader, 10);
                trace!("wlc {:x?}", wlc);
                let plz = next!(reader, 11);
                trace!("plz {:x?}", plz);
                let on = next!(reader, 12);
                trace!("on {:x?}", on);
                let g = next!(reader, 13);
                trace!("g {:x?}", g);
                let ps = next!(reader, 14);
                trace!("ps {:x?}", ps);

                let model = Model {
                    kkn,
                    kknr,
                    vknr,
                    vnr,
                    vs,
                    se,
                    t,
                    v,
                    nz,
                    f,
                    gd,
                    sn,
                    wlc,
                    plz,
                    on,
                    g,
                };

                Ok(model)
            })
        })
    })
}
