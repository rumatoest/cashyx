// use super::ZcsvFile;

use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Error as IoError, ErrorKind};
use std::path::PathBuf;
use zip;
use zip::write::FileOptions;

pub struct ZippedFile {
    path: PathBuf,
    file: File,
}

impl ZippedFile {
    pub fn from_path(path: &PathBuf) -> Result<ZippedFile, IoError> {
        if path.exists() && !path.is_file() {
            return Err(IoError::new(
                ErrorKind::InvalidInput,
                format!(
                    "Path exists and it is not a file {}",
                    path.to_string_lossy()
                ),
            ));
        }

        let mut file = OpenOptions::new().write(true).create(true).open(path)?;
        ZippedFile::check(&mut file)?;

        let ps = path.to_str().ok_or(IoError::new(
            ErrorKind::Other,
            "Zipped file path should convert to valid string at this point",
        ))?;
        let p = PathBuf::from(ps);

        Ok(ZippedFile {
            file: file,
            path: p,
        })
    }

    /// Check whether file could be read as ZIP
    fn check(file: &mut File) -> Result<usize, IoError> {
        if file.metadata()?.len() == 0 {
            return Ok(0);
        }

        let zreader = zip::ZipArchive::new(file)?;
        return Ok(zreader.len());
    }
}
