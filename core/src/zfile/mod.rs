pub mod zfile;

use std::io::Error;

pub trait ZcsvFile {
    fn file_exist(uri: &str) -> Result<bool, Error>;

    fn list(dir_uri: &str, ext: &str);

    fn list_clustered(dir_uri: &str, ext: &str);

    fn list_all();

    fn write(file_uri: &str);
}
