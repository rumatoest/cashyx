use crate::ZipStorage;

use std::fs::{File, OpenOptions};
use std::io::{Error as IoError, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use zip::{result::ZipError, result::ZipResult, write::FileOptions, ZipArchive, ZipWriter};

#[derive(Debug)]
pub struct ZipFileStorage {
    path: PathBuf,
}

impl ZipFileStorage {
    pub fn new(path: &PathBuf) -> Result<ZipFileStorage, IoError> {
        if path.exists() && !path.is_file() {
            return Err(IoError::new(
                ErrorKind::InvalidInput,
                format!(
                    "Path exists and it is not a file {}",
                    path.to_string_lossy()
                ),
            ));
        }

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        ZipFileStorage::validate(&mut file)?;

        let ps = path.to_str().ok_or(IoError::new(
            ErrorKind::Other,
            "Zipped file path should convert to valid string at this point",
        ))?;
        let p = PathBuf::from(ps);

        Ok(ZipFileStorage { path: p })
    }

    /// Check whether file could be read as ZIP
    fn validate(file: &mut File) -> Result<usize, IoError> {
        if file.metadata()?.len() == 0 {
            return Ok(0);
        }
        let zreader = zip::ZipArchive::new(file)?;
        Ok(zreader.len())
    }

    fn archive_reader(&self) -> Result<ZipArchive<File>, IoError> {
        let f = self.store_file_read()?;
        ZipArchive::new(f).map_err(ZipFileStorage::zip_error_to_io_error)
    }

    fn archive_writer(&self) -> Result<ZipWriter<File>, IoError> {
        let f = self.store_file_write()?;
        Ok(ZipWriter::new(f))
    }

    fn store_file_read(&self) -> Result<File, IoError> {
        OpenOptions::new().read(true).create(false).open(&self.path)
    }

    fn store_file_write(&self) -> Result<File, IoError> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.path)
    }

    fn zip_error_to_io_error(e: ZipError) -> IoError {
        match e {
            ZipError::Io(err) => err,
            ZipError::InvalidArchive(msg) => IoError::new(ErrorKind::InvalidInput, msg),
            ZipError::UnsupportedArchive(msg) => IoError::new(ErrorKind::InvalidInput, msg),
            ZipError::FileNotFound => IoError::new(ErrorKind::NotFound, "File was not found"),
        }
    }

    fn check_file_exists(za: &mut ZipArchive<File>, name: &str) -> Result<bool, IoError> {
        return match za.by_name(name) {
            Ok(f) => Ok(f.is_file()),
            Err(e) => {
                if let ZipError::FileNotFound = e {
                    Ok(false)
                } else {
                    Err(ZipFileStorage::zip_error_to_io_error(e))
                }
            }
        };
    }
}

impl ZipStorage for ZipFileStorage {
    fn exists(&self, name: &str, partitioned: bool) -> Result<bool, IoError> {
        let mut ar = self.archive_reader()?;

        if partitioned {
        } else {
            return ZipFileStorage::check_file_exists(&mut ar, name);
        }

        /*
        for i in 0..ar.len() {
            let mut zf = ar
                .by_index(i)
                .map_err(ZipFileStorage::zip_error_to_io_error)?;

            if partitioned {
                let pb = PathBuf::from(&zf.name());
                if let Some(p) = pb.parent() {
                    if name == p {
                        return Ok(true);
                    }
                }
            } else {
                if name == zf.name() {
                    return Ok(true);
                }
            }
        }
        */
        Ok(false)
    }

    fn list(&self) -> Result<Vec<String>, IoError> {
        println!("Getting reader");
        let mut ar = self.archive_reader()?;
        println!("Reader acquired");
        let mut result = Vec::new();
        for i in 0..ar.len() {
            let zf = ar
                .by_index(i)
                .map_err(ZipFileStorage::zip_error_to_io_error)?;
            result.push(String::from(zf.name()));
        }
        Ok(result)
    }

    fn list_partitioned(&self) -> Result<Vec<String>, IoError> {
        unimplemented!();
    }

    fn read(&self, name: &str) -> Result<Vec<u8>, IoError> {
        let mut ar = self.archive_reader()?;

        let exists = ZipFileStorage::check_file_exists(&mut ar, name)?;
        if !exists {
            return Err(IoError::from(ErrorKind::NotFound));
        }
        let mut result = Vec::new();
        let mut f = ar.by_name(name)?;
        f.read_to_end(&mut result)?;
        Ok(result)
    }

    fn read_partitioned(&self, name: &str, partition: &str) {
        unimplemented!();
    }
    fn write(&mut self, name: &str, content: &[u8]) -> Result<(), IoError> {
        let mut aw = self.archive_writer()?;

        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        aw.start_file(name, options)?;
        aw.write_all(content)?;
        aw.finish()?;
        Ok(())
    }

    fn write_partitioned(&mut self, name: &str, partition: &str) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use hamcrest2::prelude::*;
    use rand;

    fn file_path() -> PathBuf {
        let basedir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let p = Path::new(&basedir).join("target").join("test.zstore");
        println!("Storage path: {}", p.to_str().unwrap());
        p
    }

    #[test]
    fn resources_check() {
        let path = file_path();
        let zsr = ZipFileStorage::new(&path);
        assert_that!(&zsr, ok());
        let zs = zsr.unwrap();

        let files = zs.list();

        assert_that!(&files, ok());
        assert_that!(
            &files.unwrap(),
            contains(vec!["file_1.txt".to_owned(), "file_2.csv".to_owned()]).exactly()
        );

        assert_eq!(zs.exists("file_1.txt", false).unwrap(), true);
        assert_eq!(zs.exists("file_1.txt", true).unwrap(), false);
        assert_eq!(zs.exists("file_2.csv", false).unwrap(), true);
        assert_eq!(zs.exists("file_NONE.csv", false).unwrap(), false);
        assert_eq!(zs.exists("file_NONE.csv", true).unwrap(), false);
    }

    #[test]
    fn resources_read() {
        // let r = |reader: &mut dyn Read| {
        //     let mut result = String::new();
        //     reader.read_to_string(&mut result)?;
        //     Ok(result)
        // };

        let path = file_path();
        let zsr = ZipFileStorage::new(&path);
        assert_that!(&zsr, ok());
        let zs = zsr.unwrap();

        let file_content = zs.read("file_1.txt");
        assert_that!(&file_content, ok());
        assert_eq!(
            file_content.unwrap().as_slice(),
            "file_1 content".as_bytes()
        );

        let file_content = zs.read("file_2.csv");
        assert_that!(&file_content, ok());
        assert_eq!(
            file_content.unwrap().as_slice(),
            "file_2,content".as_bytes()
        );

        let file_content = zs.read("file_NONE.csv");
        assert_that!(&file_content, err());
    }

    #[test]
    fn write_and_read() {
        let basedir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let p = Path::new(&basedir).join("target").join("test.write.zstore");
        let zsr = ZipFileStorage::new(&p);
        assert_that!(&zsr, ok());
        let mut zs = zsr.unwrap();

        write_read_validate(
            "file.write.test",
            format!("file content with seed: {}", rand::random::<usize>()).as_bytes(),
            &mut zs,
        );

        write_read_validate(
            "file.write.2.test",
            format!("file 2 content with seed: {}", rand::random::<usize>()).as_bytes(),
            &mut zs,
        );

                // Same file another seed
                write_read_validate(
                    "file.write.test",
                    format!("file new content with seed: {}", rand::random::<usize>()).as_bytes(),
                    &mut zs,
                );
    }

    fn write_read_validate(file_name: &str, content: &[u8], zs: &mut ZipFileStorage) {
        let wr = zs.write(file_name, &content);
        assert_that!(&wr, ok());

        let ex = zs.exists(file_name, false);
        assert_that!(&ex, ok());
        assert_eq!(ex.unwrap(), true);

        let read = zs.read(file_name);
        assert_that!(&read, ok());
        assert_eq!(read.unwrap(), content);
    }
}
