use std::error;
use std::fmt::Display;
use std::io::{self, BufRead, Write};

use chrono::{DateTime, Utc};
use regex::Regex;

fn main() -> Result<(), Error> {
    filter(&mut io::stdin().lock(), &mut io::stdout())?;
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

pub fn filter<R, W>(r: &mut R, w: &mut W) -> Result<(), Error>
where
    R: BufRead,
    W: Write,
{
    let re = Regex::new(r"[\d-]+T[\d:]+(\.\d+)?([Zz]|[+-][\d:]+)")?;
    for line in r.lines() {
        let line = line?;
        let rep = re.replace_all(line.as_str(), |x: &regex::Captures| {
            let s = &x[0];
            match s.parse::<DateTime<Utc>>() {
                Ok(t) => t.with_timezone(&chrono::offset::Local).to_rfc3339(),
                Err(_) => s.to_string(),
            }
        });
        w.write_all(rep.as_bytes())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_plain_text() -> Result<(), Error> {
        let src = "moo";
        let mut dst = vec![];

        super::filter(&mut src.as_bytes(), &mut dst)?;
        assert_eq!(String::from_utf8(dst).unwrap(), "moo".to_string());
        Ok(())
    }

    #[test]
    fn filter_utc() -> Result<(), Error> {
        let src = "2021-01-02T03:04:05Z moo 2021-01-02T03:04:05.999Z";
        let mut dst = vec![];

        super::filter(&mut src.as_bytes(), &mut dst)?;
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

        super::filter(&mut src.as_bytes(), &mut dst)?;
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

        super::filter(&mut src.as_bytes(), &mut dst)?;
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

        super::filter(&mut src.as_bytes(), &mut dst)?;
        assert_eq!(
            String::from_utf8(dst).unwrap(),
            "2021-01-02T20:04:05+09:00".to_string()
        );
        Ok(())
    }
}
