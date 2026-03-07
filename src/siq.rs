use std::{
    collections::{HashSet, VecDeque},
    ffi::OsStr,
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Seek, Write},
    path::{Path, PathBuf},
};

use xsd_parser_types::quick_xml::{DeserializeSync, IoReader, SerializeSync, Writer};
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

#[allow(unused)]
pub mod types;

pub mod error;

use error::SiqError;
use types::{Package, SiqFacageElement};

pub const CONTENT_FILE_NAME: &str = "content.xml";
pub const AUTHORS_FILE_NAME: &str = "authors.xml";
pub const SOURCES_FILE_NAME: &str = "sources.xml";
pub const QUALITY_MARKER_FILE_NAME: &str = "quality.marker";
const ROOT_FILES: [&str; 4] = [
    CONTENT_FILE_NAME,
    AUTHORS_FILE_NAME,
    SOURCES_FILE_NAME,
    QUALITY_MARKER_FILE_NAME,
];

#[derive(Debug)]
pub struct Siq {
    pub content: Package,
}

impl From<Package> for Siq {
    fn from(value: Package) -> Self {
        Self {
            content: value,
        }
    }
}

impl Siq {
    pub fn try_new<P: AsRef<Path>>(path: P) -> Result<Siq, SiqError> {
        let file = File::open(path.as_ref())?;
        let file = BufReader::new(file);
        let mut file = ZipArchive::new(file)?;

        let content_file = file.by_name(CONTENT_FILE_NAME)?;
        let content_file = BufReader::new(content_file);
        let mut content_file = IoReader::new(content_file);
        let mut package = Package::deserialize(&mut content_file)?;

        package.bind_zip(path.as_ref());

        let res = Self{
            content: package,
        };

        Ok(res)
    }

    pub fn pack<W: Write + Seek>(&self, writer: &mut W) -> Result<(), SiqError> {
        let mut zip = ZipWriter::new(writer);

        let options = SimpleFileOptions::default();
        zip.start_file(CONTENT_FILE_NAME, options)?;

        let mut writer = Writer::new(zip);
        self.content.serialize(&mut writer)?;
        let mut zip = writer.into_inner();

        self.content.pack(&mut zip)?;

        zip.finish()?;

        Ok(())
    }
}
