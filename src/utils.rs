#[cfg(unix)]
use std::io;
use std::path::Path;

#[cfg(unix)]
pub(crate) fn make_symlink(outpath: &Path, target: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(target, outpath)
}

#[cfg(windows)]
pub(crate) fn make_symlink(outpath: &Path, target: &Path) -> io::Result<()> {
    if target.is_dir() {
        std::os::windows::fs::symlink_dir(target, outpath)
    } else {
        std::os::windows::fs::symlink_file(target, outpath)
    }
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn make_symlink(outpath: &Path, target: &Path) -> io::Result<()> {
    use std::fs;

    if target.is_file() {
        fs::copy(target, outpath)?;
    } else {
        unimplemented!();
    }

    Ok(())
}
