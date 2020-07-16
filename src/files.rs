use std::path::{Path, PathBuf};
use std::io::Read;
use std::fs::{self, DirEntry, File};

use crate::data::{Data, UserData};
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

pub fn read_invis(udata: UserData) -> i32 {
    let mut result = String::new();
    cd!(udata.download_path(), {
        let mut f = File::open(".koneko").unwrap();
        f.read_to_string(&mut result).unwrap();
    });
    result.parse::<i32>().unwrap()
}

pub fn remove_dir_if_exist(data: impl Data) {
    if data.download_path().exists() {
        fs::remove_dir_all(data.download_path()).unwrap()
    }
}

pub fn filter_history(path: PathBuf) -> Vec<String> {
    fs::read_dir(path)
        .unwrap()
        .map(|f| f.unwrap().file_name().to_str().unwrap().to_string())
        .filter(|f| f != "history")
        .collect()
}

fn dir_up_to_date(data: impl Data, dir: &[String]) -> bool {
    if dir.len() < data.all_names().len() {
        false
    } else {
        // This is so weird...
        let mut res = true;
        for (name, file) in data.all_names().iter().zip(dir) {
            if file.contains(&name.replace('/', "")) {
                res = false;
            }
        }
        res
    }
}

pub fn dir_not_empty(data: impl Data) -> bool {
    let mut dir: Vec<String> = fs::read_dir(data.download_path())
        .unwrap()
        .map(|r| r.unwrap().file_name().to_str().unwrap().to_string())
        .collect();

    if data.download_path().exists() && dir.iter().len() != 0 {
        //TODO: all_names: either data not updated, or request not sent
        dir.sort();
        if dir[0] == ".koneko" {
            dir_up_to_date(data, &dir[1..])
        } else {
            dir_up_to_date(data, &dir[..])
        }
    } else {
        false
    }
}
