use std::{
    error::Error,
    ffi::OsStr,
    fmt::Display,
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Seek, Write},
    path::{Path, PathBuf},
};

use hex::ToHex;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use tempfile::{TempDir, tempdir};
use xsd_parser_types::quick_xml::{self, DeserializeSync, IoReader, SerializeSync, Writer};
use zip::{
    ZipArchive,
    result::{ZipError, ZipResult},
};

#[allow(unused)]
pub mod types;

pub mod error;

use types::mstns::Package;
use error::SiqError;
use crate::utils;

pub struct Siq {
    pub package: Package,
    resourses_dir: TempDir,
}

fn unpack_siq<R: Read + Seek>(reader: R) -> Result<(TempDir, HashMap<String, String>), SiqError> {
    let dir = tempdir()?;

    let mut ziparch = ZipArchive::new(reader)?;
    let mut replaces = HashMap::new();

    for i in 0..ziparch.len() {
        let mut file = ziparch.by_index(i)?;

        let mut outpath = dir.path().to_owned().join(
            file.enclosed_name()
                .expect("siq files should have normal paths"),
        );

        // Because file names can be really long we need to make them shorter
        // SHA1 hash used, because it isn't so slow and the digest takes only 40 chars, so it
        // fits the 255 chars limit of Unix
        if (file.is_file() || file.is_symlink())
            && file.mangled_name().file_name().unwrap() != "content.xml"
        {
            let mut sha1 = Sha1::new();

            let old_file_name: String = outpath.file_name()
                .expect("We just checked that it is a usual file and should has name")
                .to_string_lossy() // We can neglect strange paths
                .to_string();

            let old_file_stem: String = outpath.file_stem()
                .expect("We just checked that it is a usual file and should has stem")
                .to_string_lossy() // We can neglect strange paths
                .to_string();

            let file_extension = outpath.extension()
                .unwrap_or_else(|| OsStr::new(""))
                .to_string_lossy() // We can neglect strange paths
                .to_string();

            sha1.update(old_file_name.as_bytes());
            let new_file_stem: String = sha1.finalize().encode_hex();
            let new_file_name = format!("{new_file_stem}.{file_extension}");

            outpath.set_file_name(&new_file_name);

            // Names are url encoded, but content.xml uses url decoded ones
            let old_file_name = urlencoding::decode(&old_file_name)
                .expect("UTF-8")
                .to_string();

            //Save our file name replacement, we will use it to patch content.xml
            replaces.insert(old_file_name, new_file_name);
        }

        if file.is_symlink() {
            // Get symlink value
            let mut target = Vec::with_capacity(file.size() as usize);
            file.read_to_end(&mut target)?;
            drop(file);

            if let Ok(target) = String::from_utf8(target) {
                let target = Path::new(&target);
                utils::make_symlink(&outpath, target)?;
            }
            continue;
        } else if file.is_dir() {
            fs::create_dir_all(&outpath)?;
            continue;
        }

        if let Some(parent) = outpath.parent() {
            let _ = fs::create_dir_all(parent); // Just create all needed directories
        }

        let mut outfile = File::create(&outpath)?;
        io::copy(&mut file, &mut outfile)?;
    }

    Ok((dir, replaces))
}

fn patch_string(string: &mut String, replaces: &HashMap<String, String>) {
    if let Some(val) = replaces.get(string) {
        *string = val.clone();
    }
}

fn patch_source(source: &mut String, replaces: &HashMap<String, String>) {
    // skip @ in the beggining of the source
    let mut file_path = source[1..].to_string();

    patch_string(&mut file_path, replaces);

    *source = format!("@{file_path}");
}

macro_rules! patch_siq_obj_info {
    ($id:ident, &$repls:ident) => {
        if let Some(info) = $id.info.as_mut() {
            if let Some(sources) = info.sources.as_mut() {
                for source in sources.source.iter_mut() {
                    patch_source(source, $repls)
                }
            }
        }
    };
}

fn patch_package(pack: &mut Package, replaces: &HashMap<String, String>) {
    patch_siq_obj_info!(pack, &replaces);

    if let Some(logo) = pack.logo.as_mut() {
        patch_source(logo, replaces);
    }


    for round in pack.rounds.as_mut().unwrap().round.iter_mut() {
        patch_siq_obj_info!(round, &replaces);
        for theme in round.themes.as_mut().unwrap().theme.iter_mut() {
            patch_siq_obj_info!(theme, &replaces);
            for question in theme.questions.as_mut().unwrap().question.iter_mut() {
                patch_siq_obj_info!(question, &replaces);

                for param_type in question.params.as_mut().unwrap().param.iter_mut() {
                    for item in param_type.item.iter_mut() {
                        if let Some(is_ref) = item.is_ref.as_mut() {
                            if is_ref == "True" {
                                patch_string(&mut item.content, replaces);
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Siq {
    pub fn try_new<R: Read + Seek>(reader: R) -> Result<Siq, SiqError> {
        let (dir, replaces) = unpack_siq(reader)?;

        let content = File::open((dir.path().join("content.xml")))?;
        let content = BufReader::new(content);
        let mut content = IoReader::new(content);
        let mut package = Package::deserialize(&mut content)?;

        patch_package(&mut package, &replaces);

        let res = Self {
            package: package,
            resourses_dir: dir,
        };

        Ok(res)
    }

    pub fn pack<W: Write>(&self, writer: W) -> Result<(), SiqError> {
        let content = File::open((self.resourses_dir.path().join("content.xml")))?;
        let content = BufWriter::new(content);
        let mut content = Writer::new(content);

        self.package.serialize("package", &mut content)?;

        unimplemented!();

        Ok(())
    }
}
