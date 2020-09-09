mod mfefgdo;

cfg_if! {
    if #[cfg(test)] {
        use mfefgdo::MockMFEFGDO as MFEFGDO;
    } else {
        use mfefgdo::MFEFGDO;
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct Pn {
    pub pnXML: String,
}

#[allow(non_snake_case)]
#[cfg_attr(test, derive(Default))]
#[derive(Clone, Debug, Deserialize)]
pub struct EgkData {
    pub gvd: Option<String>,
    pub statusVd: Option<String>,
    pub pd: Option<String>,
    pub pn: Option<Pn>,
    pub vd: Option<String>,
}

impl EgkData {
    pub fn write(&self, iccsn: &str) -> anyhow::Result<()> {
        use crate::writer::files::Files;

        MFEFGDO::write(iccsn)?;

        if let Some(pd) = &self.pd {
            Files::EgkPersoenlich.write(&pd)?;
        }

        if let Some(vd) = &self.vd {
            Files::EgkAllgemein.write(&vd)?;
        }

        if let Some(status_vd) = &self.statusVd {
            Files::EgkStatusVD.write(&status_vd)?;
        }

        if let Some(gvd) = &self.gvd {
            Files::EgkGeschuetzt.write(&gvd)?;
        }

        if let Some(pn) = &self.pn {
            Files::EgkPruefungsnachweis.write(&pn.pnXML)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use mockall::predicate::*;
    use test_server::helper;

    #[test]
    fn write_mfefgdo_ok() {
        let iccsn = test_server::helper::random_string(10);

        let ctx = MFEFGDO::write_context();
        ctx.expect()
            .with(str::similar(iccsn.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let egk_data = EgkData::default();

        assert!(egk_data.write(&iccsn).is_ok());
    }

    #[test]
    fn write_mfefgdo_failed() {
        let iccsn = test_server::helper::random_string(10);

        let ctx = MFEFGDO::write_context();
        ctx.expect()
            .with(str::similar(iccsn.clone()))
            .times(1)
            .returning(|_| bail!("ERROR"));

        let egk_data = EgkData::default();

        assert!(egk_data.write(&iccsn).is_err());
    }

    test! {
        name: pd_is_written,
        temp_dir: true,
        vars: [
            iccsn => helper::random_string(10),
            pd => "<a>PD</a>"
        ],
        steps: {
            let ctx = MFEFGDO::write_context();
            ctx.expect().returning(|_| Ok(()));

            let mut egk_data = EgkData::default();
            egk_data.pd = Some(pd.to_string());

            egk_data.write(&iccsn).unwrap();
        },
        assert: [
            "eGK_PersoenlicheVersichertendaten.xml" => format!("<?xml version=\"1.0\" encoding=\"ISO-8859-15\" standalone=\"yes\"?>{}", pd).as_bytes()
        ]
    }

    test! {
        name: status_vd_is_written,
        temp_dir: true,
        vars: [
            iccsn => helper::random_string(10),
            status_vd => "<a>STATUS_VD</a>"
        ],
        steps: {
            let ctx = MFEFGDO::write_context();
            ctx.expect().returning(|_| Ok(()));

            let mut egk_data = EgkData::default();
            egk_data.statusVd = Some(status_vd.to_string());

            egk_data.write(&iccsn).unwrap();
        },
        assert: [
            "eGK_MFDF_HCA_EF_StatusVD.xml" => format!("<?xml version=\"1.0\" encoding=\"ISO-8859-15\" standalone=\"yes\"?>{}", status_vd).as_bytes()
        ]
    }

    test! {
        name: vd_is_written,
        temp_dir: true,
        vars: [
            iccsn => helper::random_string(10),
            vd => "<a>VD</a>"
        ],
        steps: {
            let ctx = MFEFGDO::write_context();
            ctx.expect().returning(|_| Ok(()));

            let mut egk_data = EgkData::default();
            egk_data.vd = Some(vd.to_string());

            egk_data.write(&iccsn).unwrap();
        },
        assert: [
            "eGK_allgemeineVersicherungsdaten.xml" => format!("<?xml version=\"1.0\" encoding=\"ISO-8859-15\" standalone=\"yes\"?>{}", vd).as_bytes()
        ]
    }

    test! {
        name: gvd_is_written,
        temp_dir: true,
        vars: [
            iccsn => helper::random_string(10),
            gvd => "<a>GVD</a>"
        ],
        steps: {
            let ctx = MFEFGDO::write_context();
            ctx.expect().returning(|_| Ok(()));

            let mut egk_data = EgkData::default();
            egk_data.gvd = Some(gvd.to_string());

            egk_data.write(&iccsn).unwrap();
        },
        assert: [
            "eGK_geschuetzteVersichertendaten.xml" => format!("<?xml version=\"1.0\" encoding=\"ISO-8859-15\" standalone=\"yes\"?>{}", gvd).as_bytes()
        ]
    }

    test! {
        name: pn_is_written,
        temp_dir: true,
        vars: [
            iccsn => helper::random_string(10),
            pn => "<a>PN</a>"
        ],
        steps: {
            let ctx = MFEFGDO::write_context();
            ctx.expect().returning(|_| Ok(()));

            let mut egk_data = EgkData::default();
            egk_data.pn = Some(Pn {
                pnXML: pn.to_string()
            });

            egk_data.write(&iccsn).unwrap();
        },
        assert: [
            "eGK_Pruefungsnachweis.xml" => format!("<?xml version=\"1.0\" encoding=\"ISO-8859-15\" standalone=\"yes\"?>{}", pn).as_bytes()
        ]
    }

    test! {
        name: all_files_written,
        temp_dir: true,
        vars: [
            iccsn => helper::random_string(10),
            gvd => "<a>GVD</a>",
            vd => "<a>VD</a>",
            status_vd => "<a>STATUS_VD</a>",
            pd => "<a>PD</a>",
            pn => "<a>PN</a>"
        ],
        steps: {
            let ctx = MFEFGDO::write_context();
            ctx.expect().returning(|_| Ok(()));

            let egk_data = EgkData {
                vd: Some(vd.to_string()),
                gvd: Some(gvd.to_string()),
                pd: Some(pd.to_string()),
                statusVd: Some(status_vd.to_string()),
                pn: Some(Pn {
                    pnXML: pn.to_string()
                }),
            };

            egk_data.write(&iccsn).unwrap();
        },
        assert: [
            "eGK_allgemeineVersicherungsdaten.xml" => format!("<?xml version=\"1.0\" encoding=\"ISO-8859-15\" standalone=\"yes\"?>{}", vd).as_bytes(),
            "eGK_geschuetzteVersichertendaten.xml" => format!("<?xml version=\"1.0\" encoding=\"ISO-8859-15\" standalone=\"yes\"?>{}", gvd).as_bytes(),
            "eGK_MFDF_HCA_EF_StatusVD.xml" => format!("<?xml version=\"1.0\" encoding=\"ISO-8859-15\" standalone=\"yes\"?>{}", status_vd).as_bytes(),
            "eGK_PersoenlicheVersichertendaten.xml" => format!("<?xml version=\"1.0\" encoding=\"ISO-8859-15\" standalone=\"yes\"?>{}", pd).as_bytes(),
            "eGK_Pruefungsnachweis.xml" => format!("<?xml version=\"1.0\" encoding=\"ISO-8859-15\" standalone=\"yes\"?>{}", pn).as_bytes()
        ]
    }
}
