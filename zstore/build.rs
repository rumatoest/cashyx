use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use zip;
use zip::write::FileOptions;

fn main() {
    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }

    let basedir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let csvdir = Path::new(&basedir).join("test");
    let targetfile = Path::new(&basedir).join("target").join("test.zstore");
    let mut file = File::create(&targetfile).unwrap();

    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);


    let dir_list = fs::read_dir(csvdir).expect("Test directory");
    for dentry in dir_list {
        let dir_entry = dentry.expect("path");
        let name = dir_entry.file_name().into_string().unwrap();

        if !dir_entry.path().is_file() {
            continue;
        }

        println!("Adding to ZIP package {}", name);
        let mut file = File::open(&dir_entry.path()).expect("file");
        let mut file_content = String::new();
        file.read_to_string(&mut file_content);

        zip.start_file(name, options).unwrap();
        zip.write_all(file_content.as_ref()).unwrap();
    }
    zip.finish().unwrap();
}
