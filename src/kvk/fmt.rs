use std::fmt::{Display, Formatter, Result};

macro_rules! fmt(
    ($bytes:expr) => ({
        String::from_utf8_lossy($bytes).chars().fold("".to_string(),|s,c| s + match c {
                    '@' => "§".to_string(),
                    '~' => "ß".to_string(),
                    '{' => "ä".to_string(),
                    '}' => "ü".to_string(),
                    '|' => "ö".to_string(),
                    '[' => "Ä".to_string(),
                    ']' => "Ü".to_string(),
                    '\\' => "Ö".to_string(),
                    _ => c.to_string()
                }.as_str())
    })
);

impl Display for super::Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(
            f,
            "KrankenKassenName:    {}\n\
             KrankenKassenNummer:  {}\n\
             VKNR:                 {}\n\
             VersichertenNummer:   {}\n\
             VersichertenStatus:   {}\n\
             StatusErgänzung:      {}",
            fmt!(&self.kkn),
            fmt!(&self.kknr),
            fmt!(&self.vknr),
            fmt!(&self.vnr),
            fmt!(&self.vs),
            fmt!(&self.se)
        )?;

        if let Some(t) = &self.t {
            writeln!(f, "Titel:                {}", fmt!(t))?;
        }

        writeln!(f, "VorName:              {}", fmt!(&self.v))?;

        if let Some(nz) = &self.nz {
            writeln!(f, "NamensZusatz:         {}", fmt!(nz))?;
        }

        writeln!(
            f,
            "FamilienName:         {}\n\
             GeburtsDatum:         {}",
            fmt!(&self.f),
            fmt!(&self.gd)
        )?;

        if let Some(sn) = &self.sn {
            writeln!(f, "Straßenname:          {}", fmt!(sn))?;
        }

        if let Some(wlc) = &self.wlc {
            writeln!(f, "WohnsitzLänderCode:   {}", fmt!(wlc))?;
        }

        write!(
            f,
            "Postleitzahl:         {}\n\
             Orstname:             {}\n\
             GültigkeitsDatum:     {}",
            fmt!(&self.plz),
            fmt!(&self.on),
            fmt!(&self.g)
        )
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn fmt_returns_error_string_if_invalid_bytes_given() {
        let buggy = vec![159, 146, 150];

        assert_eq!(r#"���"#, fmt!(&buggy))
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
}
