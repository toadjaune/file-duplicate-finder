use std::fs;
use std::collections::HashMap;
use std::ffi::{OsString, OsStr};


fn main() {

    let mut filenames = HashMap::new();
    let mut file_paths = vec![OsString::from("/a/toto.txt"), OsString::from("/b/toto.txt")];

    filenames.insert(OsString::from("toto.txt"), file_paths);

    scan_dir("/tmp", &mut filenames);
}

fn scan_dir(path: &str, map: &mut HashMap<OsString, std::vec::Vec<OsString>>) -> () {

    let dir_listing = fs::read_dir("./").unwrap();

    for file in dir_listing {
        // println!("Name: {}", path.unwrap().path().display());

        let file = file.unwrap();

        // TODO : can't we instead generate an enum and match on it ?
        // TODO : currently, we consider symlinks as normal files (even if symlinked towards a
        // directory)
        if file.file_type().unwrap().is_dir() {


        } else {

            let a = file.path();
            let b = a.file_name();
            let c = b.unwrap();
            let file_copy = c.clone();

            // let file_copy = file.path().file_name().unwrap().clone();
            // println!("{:?}", file.file_type().unwrap().is_dir());
            // println!("{:?}", file_copy);
            map.insert(OsString::from(file_copy), vec![OsString::from("a")]);

        }
    }
    println!("{:?}", map);

}
