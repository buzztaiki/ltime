use std::error;
use std::fmt::Display;
use std::io::{self, BufRead, Write};

use chrono::{offset::TimeZone, DateTime, Utc};
use regex::Regex;

pub fn filter<R, W, TZ>(r: &mut R, w: &mut W, tz: &TZ) -> Result<(), Error>
where
    R: BufRead,
    W: Write,
    TZ: TimeZone,
    TZ::Offset: Display,
{
    let re = Regex::new(r"[\d-]+T[\d:]+(\.\d+)?([Zz]|[+-][\d:]+)")?;
    let mut buf = String::new();
    while r.read_line(&mut buf)? > 0 {
        let rep = re.replace_all(buf.as_str(), |x: &regex::Captures| {
            let s = x[0].to_string();
            match s.parse::<DateTime<Utc>>() {
                Ok(t) => t.with_timezone(tz).to_rfc3339(),
                Err(_) => s,
            }
        });
        w.write_all(rep.as_bytes())?;
        buf.clear();
    }
    Ok(())
}

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    RegexError(regex::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(x) => x.fmt(f),
            Error::RegexError(x) => x.fmt(f),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(x: io::Error) -> Self {
        Error::IoError(x)
    }
}

impl From<regex::Error> for Error {
    fn from(x: regex::Error) -> Self {
        Error::RegexError(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{offset::FixedOffset, Duration};

    fn jst() -> FixedOffset {
        FixedOffset::east_opt(Duration::hours(9).num_seconds() as i32).unwrap()
    }

    #[test]
    fn filter_plain_text() -> Result<(), Error> {
        let src = "moo\nwoo";
        let mut dst = vec![];

        filter(&mut src.as_bytes(), &mut dst, &jst())?;
        assert_eq!(String::from_utf8(dst).unwrap(), "moo\nwoo".to_string());
        Ok(())
    }

    #[test]
    fn filter_utc() -> Result<(), Error> {
        let src = "2021-01-02T03:04:05Z moo 2021-01-02T03:04:05.999Z";
        let mut dst = vec![];

        filter(&mut src.as_bytes(), &mut dst, &jst())?;
        assert_eq!(
            String::from_utf8(dst).unwrap(),
            "2021-01-02T12:04:05+09:00 moo 2021-01-02T12:04:05.999+09:00".to_string()
        );
        Ok(())
    }

    #[test]
    fn filter_utc_small_letter_z() -> Result<(), Error> {
        let src = "2021-01-02T03:04:05z";
        let mut dst = vec![];

        filter(&mut src.as_bytes(), &mut dst, &jst())?;
        assert_eq!(
            String::from_utf8(dst).unwrap(),
            "2021-01-02T12:04:05+09:00".to_string()
        );
        Ok(())
    }

    #[test]
    fn filter_utc_numeric() -> Result<(), Error> {
        let src = "2021-01-02T03:04:05+00:00";
        let mut dst = vec![];

        filter(&mut src.as_bytes(), &mut dst, &jst())?;
        assert_eq!(
            String::from_utf8(dst).unwrap(),
            "2021-01-02T12:04:05+09:00".to_string()
        );
        Ok(())
    }

    #[test]
    fn filter_nonutc() -> Result<(), Error> {
        let src = "2021-01-02T03:04:05-08:00";
        let mut dst = vec![];

        filter(&mut src.as_bytes(), &mut dst, &jst())?;
        assert_eq!(
            String::from_utf8(dst).unwrap(),
            "2021-01-02T20:04:05+09:00".to_string()
        );
        Ok(())
    }
}
