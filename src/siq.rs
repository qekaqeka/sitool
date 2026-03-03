use std::{
    collections::{HashSet, VecDeque}, ffi::OsStr, fs::{self, File}, io::{self, BufReader, BufWriter, Read, Seek, Write}, path::{Path, PathBuf}
};

use hex::ToHex;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use tempfile::{TempDir, tempdir};
use xsd_parser_types::quick_xml::{DeserializeSync, IoReader, SerializeSync, Writer};
use zip::{
    ZipArchive, ZipWriter, write::SimpleFileOptions
};

#[allow(unused)]
pub mod types;

pub mod error;

use types::Package;
use error::SiqError;
use crate::{siq::types::{Param, ParamItem, Question, Round, Theme}, utils};

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

pub struct Siq {
    pub content: Package,
    replaces: HashMap<String, String>,
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
            && file.mangled_name().file_name().unwrap() != CONTENT_FILE_NAME
        {
            let mut sha1 = Sha1::new();

            let old_file_name: String = outpath.file_name()
                .expect("We just checked that it is a usual file and should has name")
                .to_string_lossy() // We can neglect strange paths
                .to_string();

            let file_extension = outpath.extension()
                .unwrap_or_else(|| OsStr::new(""))
                .to_string_lossy() // We can neglect strange paths
                .to_string();

            sha1.update(old_file_name.as_bytes());
            let mangled_file_stem: String = sha1.finalize().encode_hex();
            let mangled_file_name = format!("{mangled_file_stem}.{file_extension}");

            outpath.set_file_name(&mangled_file_name);

            // Names are url encoded, but content.xml uses url decoded ones
            let old_file_name = urlencoding::decode(&old_file_name)
                .expect("UTF-8")
                .to_string();

            //Save our file name replacement, we will use it to patch content.xml
            replaces.insert(mangled_file_name, old_file_name);
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

fn param_item_type_to_dirname(type_: &String) -> &str {
    match type_.as_str() {
        "video" => "Video",
        "image" => "Images",
        "audio" => "Audio",
        _ => unreachable!("There isn't another param type for file"),
    }
}

fn get_param_item_used_files(item: &ParamItem, used_files: &mut HashSet<String>) {
    if item.is_ref.as_ref().is_some_and(|v| v == "True") {
        let file_name = item.content.clone();

        let mut source_path= PathBuf::new();
        if let Some(type_) = &item.type_ {
            source_path.push(param_item_type_to_dirname(type_));
        }

        source_path.push(file_name);

        let source_path = source_path.to_str()
            .expect("UTF-8")
            .to_string();

        used_files.insert(source_path);
    }
}

fn get_param_used_files(param: &Param, used_files: &mut HashSet<String>) {
    let mut params: VecDeque<&Param> = VecDeque::new();
    params.push_back(param);

    while let Some(prm) = params.pop_front() {
        params.extend(prm.param.iter().map(|p| &p.value));

        for item in prm.item.iter() {
            get_param_item_used_files(&item.value, used_files);
        }
    }
}

fn get_question_used_files(question: &Question, used_files: &mut HashSet<String>) {
    if let Some(params) = &question.params {
        for param in params.param.iter() {
            get_param_used_files(param, used_files);
        }
    }

    if let Some(_scenario) = &question.scenario {
        // Scenario can use files too
        unimplemented!();
    }
}

fn get_theme_used_files(theme: &Theme, used_files: &mut HashSet<String>) {
    if let Some(questions) = &theme.questions {
        for question in questions.question.iter() {
            get_question_used_files(question, used_files);
        }
    }
}

fn get_round_used_files(round: &Round, used_files: &mut HashSet<String>) {
    if let Some(themes) = &round.themes {
        for theme in themes.theme.iter() {
            get_theme_used_files(theme, used_files);
        }
    }
}

fn get_package_used_files(content: &Package, used_files: &mut HashSet<String>) {
    // Root files should be add explicitly
    for root_file in ROOT_FILES {
        used_files.insert(root_file.to_string());
    }

    if let Some(logo_link) = &content.logo {
        if logo_link.chars().nth(0).is_some_and(|c| c == '@') {
            // skip @ in the beggining of string to extract file_name 
            let logo_file_name = &logo_link[1..];
            let logo_path_str = PathBuf::from("Images")
                .join(logo_file_name)
                .to_str()
                .expect("UTF-8")
                .to_string();

            used_files.insert(logo_path_str);
        }
    }

    if let Some(rounds) = &content.rounds {
        for round in rounds.round.iter() {
            get_round_used_files(&round, used_files);
        }
    }
}

impl Siq {
    pub fn try_new<R: Read + Seek>(reader: R) -> Result<Siq, SiqError> {
        let (dir, replaces) = unpack_siq(reader)?;

        let content = File::open(dir.path().join(CONTENT_FILE_NAME))?;
        let content = BufReader::new(content);
        let mut content = IoReader::new(content);
        let package = Package::deserialize(&mut content)?;

        let res = Self {
            content: package,
            resourses_dir: dir,
            replaces: replaces,
        };

        Ok(res)
    }

    pub fn pack<W: Write + Seek>(&self, writer: &mut W) -> Result<(), SiqError> {
        let content = File::create(self.resourses_dir.path().join(CONTENT_FILE_NAME))?;
        let content = BufWriter::new(content);
        let mut content = Writer::new(content);

        self.content.serialize("package", &mut content)?;
        drop(content);

        let mut used_files = HashSet::new();
        get_package_used_files(&self.content, &mut used_files);
        dbg!(&used_files);

        let mut zip = ZipWriter::new(writer);

        let mut dir_entries: VecDeque<Result<fs::DirEntry, io::Error>> = fs::read_dir(self.resourses_dir.path())?.into_iter()
            .collect();

        while let Some(entry) = dir_entries.pop_front() {
            let path = entry?.path();

            if path.is_dir() {
                let sub_dir_entries = fs::read_dir(path)?;
                dir_entries.extend(sub_dir_entries.into_iter());
            } else if path.is_file() {
                let filename = path.file_name()
                    .expect("file")
                    .to_str()
                    .expect("UTF-8");

                // path filename could be mangled in the unpack_siq function
                let demangled_filename = self.replaces.get(filename)
                    .map_or(filename, |flnm| flnm.as_str());

                let demangled_path_full = path.with_file_name(demangled_filename);
                let demangled_path = demangled_path_full.strip_prefix(self.resourses_dir.path())
                    .expect("all pathches should be in the resourse dir");
                let demangled_path_str = demangled_path.to_str()
                    .expect("UTF-8");

                // Skip unused files
                if used_files.get(demangled_path_str).is_none() {
                    continue;
                }

                let file = File::open(&path)?;
                let mut file = BufReader::new(file);

                let options = SimpleFileOptions::default()
                    .compression_method(zip::CompressionMethod::Stored);

                // We need to use old name in the archive, because content.xml references it
                zip.start_file_from_path(demangled_path, options)?;
                io::copy(&mut file,&mut zip)?;
            } else if path.is_symlink() {
                let link_target = fs::read_link(&path)?;

                let options = SimpleFileOptions::default()
                    .compression_method(zip::CompressionMethod::Stored);
                zip.add_symlink_from_path(path, link_target, options)?;
            } else {
                unreachable!("There isn't another dir entry type")
            }
        }

        Ok(())
    }
}
