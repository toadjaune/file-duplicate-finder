use std::fs;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::error::Error;

pub fn run(paths: Vec<&str>) -> Result<(), Box <dyn Error>> {

    let mut filenames = FilenameDuplicates::new();

    for path in paths {
        scan_dir(PathBuf::from(path), &mut filenames).unwrap();
    }
    let filenames = filenames.without_single_entries();
    filenames.text_output();

    Ok(())
}

pub struct FilenameDuplicates {
    map: HashMap<OsString, Vec<OsString>>
}

impl FilenameDuplicates {

    fn new() -> FilenameDuplicates {
        FilenameDuplicates {
            map: HashMap::new()
        }
    }

    fn text_output(&self) {
        for (filename,filepaths) in &self.map {

            // TODO : Find a way to directly handle OsString without conversion
            // TODO : Also, avoid cloning the string
            println!("Duplicate files named {} :", filename.clone().into_string().unwrap());
            for filepath in filepaths {
                println!("    {}", filepath.clone().into_string().unwrap());
            }
        }
    }

    // fn remove_single_entries(&mut self) {
    //     self.map = self.map.into_iter()
    //         .filter( |entry|
    //             // NB : len() is constant-time
    //             entry.1.len() > 1
    //         ).collect();
    // }

    fn without_single_entries(self) -> FilenameDuplicates {
        FilenameDuplicates {
            map: self.map.into_iter()
                .filter( |entry|
                    // NB : len() is constant-time
                    entry.1.len() > 1
                ).collect()
        }
    }

}

fn scan_dir(path: PathBuf, duplicates: &mut FilenameDuplicates) -> Result<(), Box<dyn Error>> {

    let dir_listing = fs::read_dir(path)?;

    for file in dir_listing {
        // println!("Name: {}", path.unwrap().path().display());

        let file = file?;

        // TODO : can't we instead generate an enum and match on it ?
        // TODO : currently, we consider symlinks as normal files (even if symlinked towards a
        // directory)
        if file.file_type()?.is_dir() {

            scan_dir(file.path(), duplicates);

        } else {
            let files = duplicates.map.entry(
                    OsString::from(
                        file.path().file_name().unwrap()
                    )
                ).or_insert(Vec::new());
            // NB : Canonicalize resolves all symlinks in the path
            files.push(OsString::from(file.path().canonicalize()?));
        }
    }
    Ok(())
}

