use std::error::Error;
use std::fmt::Display;
use std::io;
use xsd_parser_types::quick_xml;
use zip::result::ZipError;

#[derive(Debug)]
pub enum SiqError {
    Zip(ZipError),
    Io(io::Error),
    BadContentXML(quick_xml::Error),
    FailedToConvert(&'static str),
}

impl From<ZipError> for SiqError {
    fn from(value: ZipError) -> Self {
        Self::Zip(value)
    }
}

impl From<io::Error> for SiqError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<quick_xml::Error> for SiqError {
    fn from(value: quick_xml::Error) -> Self {
        Self::BadContentXML(value)
    }
}

impl Display for SiqError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zip(e) => write!(f, "zip error: {e}"),
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::BadContentXML(e) => write!(f, "bad context.xml: {e}"),
            Self::FailedToConvert(s) => write!(f, "{s}"),
        }
    }
}

impl Error for SiqError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Zip(e) => Some(e),
            Self::Io(e) => Some(e),
            Self::BadContentXML(e) => Some(e),
            Self::FailedToConvert(_) => None,
        }
    }
}
