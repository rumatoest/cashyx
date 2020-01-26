extern crate zip;

pub mod file;

use std::io::{Error, Read, Write};

pub trait ZipStorage {
    fn exists(&self, name: &str, partitioned: bool) -> Result<bool, Error>;

    fn list(&self) -> Result<Vec<String>, Error>;

    fn list_partitioned(&self) -> Result<Vec<String>, Error>;

    fn read(&self, name: &str) -> Result<Vec<u8>, Error>;

    fn read_partitioned(&self, name: &str, partition: &str);

    fn write(&mut self, name: &str, content: &[u8]) -> Result<(), Error>;

    fn write_partitioned(&mut self, name: &str, partition: &str);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
