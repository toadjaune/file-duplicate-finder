use std::fs;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::error::Error;


fn main() -> Result<(), Box <dyn Error>> {

    let mut filenames = HashMap::new();

    scan_dir(PathBuf::from("./"), &mut filenames).unwrap();
    let filenames = remove_single_entries(filenames);
    text_output(&filenames);

    Ok(())
}


fn scan_dir(path: PathBuf, map: &mut HashMap<OsString, Vec<OsString>>) -> Result<(), Box<dyn Error>> {

    let dir_listing = fs::read_dir(path)?;

    for file in dir_listing {
        // println!("Name: {}", path.unwrap().path().display());

        let file = file?;

        // TODO : can't we instead generate an enum and match on it ?
        // TODO : currently, we consider symlinks as normal files (even if symlinked towards a
        // directory)
        if file.file_type()?.is_dir() {

            scan_dir(file.path(), map);

        } else {
            let files = map.entry(
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

fn remove_single_entries(map: HashMap<OsString, Vec<OsString>>) -> HashMap<OsString, Vec<OsString>> {
    map.into_iter()
        .filter( |entry|
            // NB : len() is constant-time
            entry.1.len() > 1
        ).collect()
}

fn text_output(map: &HashMap<OsString, Vec<OsString>>) {
    for (filename,filepaths) in map {

        // TODO : Find a way to directly handle OsString without conversion
        // TODO : Also, avoid cloning the string
        println!("Duplicate files named {} :", filename.clone().into_string().unwrap());
        for filepath in filepaths {
            println!("    {}", filepath.clone().into_string().unwrap());
        }
    }
}



