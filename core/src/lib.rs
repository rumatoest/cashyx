extern crate chrono;
extern crate csv;
extern crate serde;

pub mod zfile;
pub mod model;
pub mod service;
pub mod storage;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
