use std::fs::File;
use std::io::{self, BufReader, Read, Seek, Write};
use std::path::{Path, PathBuf};

use uuid::Uuid;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

use std::cell::{OnceCell, RefCell};
use std::rc::Rc;

use crate::siq::SiqError;

#[derive(Clone, Debug)]
enum FileDepSrc {
    Plain {
        path: PathBuf,
    },
    Compressed {
        zip_path: Option<PathBuf>,
        path: PathBuf,
    },
}

impl FileDepSrc {
    pub fn is_plain(&self) -> bool {
        match self {
            Self::Plain { .. } => true,
            _ => false,
        }
    }

    pub fn is_compressed(&self) -> bool {
        match self {
            Self::Compressed { .. } => true,
            _ => false,
        }
    }

    pub fn is_binded(&self) -> bool {
        match self {
            Self::Compressed { zip_path: z, .. } => z.is_some(),
            _ => false,
        }
    }

    pub fn bind_zip_path<P: AsRef<Path>>(&mut self, zip_path: P) {
        match self {
            Self::Compressed { zip_path: z, .. } => {
                if z.is_some() {
                    panic!("Already binded");
                }

                *z = Some(zip_path.as_ref().to_path_buf())
            }
            _ => panic!("Only compressed filedepsrc can be binded"),
        }
    }

    pub fn get_path(&self) -> &Path {
        match self {
            Self::Compressed { path: p, .. } => p.as_path(),
            Self::Plain { path: p } => p.as_path(),
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum FileDepDstDir {
    Root,
    Images,
    Video,
    Audio,
}

impl FileDepDstDir {
    fn to_dir_name(&self) -> &Path {
        let res = match self {
            Self::Root => "",
            Self::Images => "Images",
            Self::Video => "Video",
            Self::Audio => "Audio",
        };

        Path::new(res)
    }
}

#[derive(Clone, Debug)]
pub struct FileDep {
    src: FileDepSrc,
    dst_dir: FileDepDstDir,

    dst_path: PathBuf,
}

fn get_dst_path<P: AsRef<Path>>(src: P, dst_dir: FileDepDstDir) -> PathBuf {
    let path = src.as_ref();

    let mut dst_filename = PathBuf::from(Uuid::new_v4().to_string());
    if let Some(ext) = path.extension() {
        dst_filename.set_extension(ext);
    }

    let dst = dst_dir.to_dir_name().join(dst_filename);

    dst.to_path_buf()
}

impl FileDep {
    pub fn new_plain<P: AsRef<Path>>(src: P, dst: FileDepDstDir) -> Self {
        let src = FileDepSrc::Plain {
            path: src.as_ref().to_path_buf(),
        };

        let src_path = src.get_path()
            .to_owned();

        Self { src: src, dst_dir: dst, dst_path: get_dst_path(src_path, dst) }
    }

    pub fn new_compressed<P: AsRef<Path>>(inner_path: P, dst: FileDepDstDir) -> Self {
        let inner_path = inner_path.as_ref();
        let src = FileDepSrc::Compressed {
            zip_path: None,
            path: inner_path.to_path_buf(),
        };

        assert!(inner_path.is_relative());

        let src_path = src.get_path()
            .to_owned();

        Self { src: src, dst_dir: dst, dst_path: get_dst_path(src_path, dst) }
    }

    pub fn get_dst_path(&self) -> &Path {
        &self.dst_path
    }

    pub fn is_plain(&self) -> bool {
        self.src.is_plain()
    }

    pub fn is_compressed(&self) -> bool {
        self.src.is_compressed()
    }

    pub fn is_binded(&self) -> bool {
        self.src.is_binded()
    }

    pub fn bind_zip_path<P: AsRef<Path>>(&mut self, zip_path: P) {
        self.src.bind_zip_path(zip_path)
    }

    pub fn get_src_path(&self) -> &Path {
        self.src.get_path()
    }

    pub fn pack<W: Write + Seek>(&self, dest: &mut ZipWriter<W>) -> Result<(), SiqError> {
        let dst = self.get_dst_path();

        match &self.src {
            FileDepSrc::Plain { path } => {
                let options = SimpleFileOptions::default();

                dest.start_file_from_path(dst, options)?;

                let mut src = File::open(path)?;

                io::copy(&mut src, dest);
            }
            FileDepSrc::Compressed { zip_path, path } => {
                let zip_path = zip_path.as_ref().expect("should be set");

                let zip = File::open(zip_path)?;
                let zip = BufReader::new(zip);
                let mut zip = ZipArchive::new(zip)?;

                let mut index = zip.index_for_path(&path);

                // file name could be urlencoded
                if index.is_none() {
                    let path_str = path.to_str()
                        .expect("UTF-8");

                    // It can be confusing, but this isn't filename. It's a path
                    for encoded_filename in zip.file_names() {
                        let decoded_filename = urlencoding::decode(encoded_filename)
                            .expect("UTF-8");

                        if decoded_filename == path_str {
                            index = zip.index_for_name(encoded_filename);
                            break;
                        }
                    }
                }


                let index = if let Some(i) = index {
                    i
                } else {
                    let err = io::Error::new(io::ErrorKind::NotFound, path.to_string_lossy());
                    return Err(SiqError::Io(err));
                };

                let src_file = zip.by_index_raw(index)?;

                dest.raw_copy_file_to_path(src_file, dst)?;
            }
        }

        Ok(())
    }
}
