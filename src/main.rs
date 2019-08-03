use std::fs;
use std::collections::HashMap;
use std::ffi::{OsString, OsStr};
use std::path::PathBuf;


fn main() {

    let mut filenames = HashMap::new();

    scan_dir(PathBuf::from("./"), &mut filenames);
}

fn scan_dir(path: PathBuf, map: &mut HashMap<OsString, std::vec::Vec<OsString>>) -> () {

    let dir_listing = fs::read_dir(path).unwrap();

    for file in dir_listing {
        // println!("Name: {}", path.unwrap().path().display());

        let file = file.unwrap();

        // TODO : can't we instead generate an enum and match on it ?
        // TODO : currently, we consider symlinks as normal files (even if symlinked towards a
        // directory)
        if file.file_type().unwrap().is_dir() {

            scan_dir(file.path(), map);

        } else {
            let files = map.entry(OsString::from(file.path().file_name().unwrap()))
                .or_insert(Vec::new());
            // NB : Canonicalize resolves all symlinks in the path
            files.push(OsString::from(file.path().canonicalize().unwrap()));
        }
    }
    println!("{:?}", map);

}
