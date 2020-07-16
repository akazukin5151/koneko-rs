use std::path::Path;
use std::fs::{self, DirEntry};

use crate::KONEKODIR;

fn isdigit(entry: DirEntry) -> (DirEntry, bool) {
    let b = entry
        .file_name()
        .to_str()
        .unwrap()
        .chars()
        .all(|s| s.is_digit(10));
    (entry, b)
}

fn has_individual(entry: DirEntry) -> (DirEntry, bool) {
    let b = fs::read_dir(Path::new(KONEKODIR).join(entry.path()))
        .unwrap()
        .position(|x| x.unwrap().path().to_str() == Some("individual"))
        .is_none();
    (entry, b)
}

pub fn find_mode2_dirs() -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for entry in fs::read_dir(KONEKODIR).unwrap() {
        let (entry, isdigit) = isdigit(entry.unwrap());
        let (entry, has_individual_dir) = has_individual(entry);

        if isdigit && has_individual_dir {
            result.push(entry.path().to_str().unwrap().to_string());
        }
    }
    result
}
