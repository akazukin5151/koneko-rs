use std::io::{self, Read};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::fs::{self, DirEntry, File};

use crate::pure;
use crate::KONEKODIR;
use crate::data::{Data, UserData};

fn read_dir_to_string(d: io::Result<DirEntry>) -> String {
    d.unwrap().file_name().to_str().unwrap().to_string()
}

fn isdigit(entry: DirEntry) -> (DirEntry, bool) {
    let b = pure::str_is_digit(entry.file_name().to_str().unwrap());
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
        .map(|f| read_dir_to_string(f))
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
        .map(|r| read_dir_to_string(r))
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

pub fn filter_dir(modes: Vec<i32>) -> Vec<String> {
    let path = Path::new(KONEKODIR);
    let dirs = fs::read_dir(path).unwrap();
    let mut allowed_names = HashSet::new();

    if modes.iter().find(|&&x| x == 1).is_some() {
        allowed_names.insert("testgallery");
    }
    if modes.iter().find(|&&x| x == 3).is_some() {
        allowed_names.insert("following");
        allowed_names.insert("testuser");
    }
    if modes.iter().find(|&&x| x == 4).is_some() {
        allowed_names.insert("search");
    }
    if modes.iter().find(|&&x| x == 5).is_some() {
        allowed_names.insert("illustfollow");
    }

    let res = dirs.map(|x| read_dir_to_string(x));
    if modes.iter().find(|&&x| x == 1).is_some() {
        let predicate = |d: &str| pure::str_is_digit(d) || allowed_names.contains(d);
        res.filter(|x| predicate(x)).collect()
    } else if modes.iter().find(|&&x| x == 2).is_some() {
        let predicate = |d: &str| {
            find_mode2_dirs().iter().find(|&x| x == d).is_some() || allowed_names.contains(d)
        };
        res.filter(|x| predicate(x)).collect()
    } else {
        let predicate = |d: &str| allowed_names.contains(d);
        res.filter(|x| predicate(x)).collect()
    }
}
