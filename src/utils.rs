use std::fs;
use std::path::Path;
use std::process::Command;

use crossterm::terminal;

use crate::data;
use crate::config;
use crate::KONEKODIR;

#[macro_export]
macro_rules! cd {
    ( $newdir:expr, $x:block ) => {{
        use std::env;
        use scopeguard::defer;

        let old = env::current_dir().unwrap();
        {
            defer! {
                env::set_current_dir(old).unwrap();
            };
            env::set_current_dir($newdir).unwrap();
            $x
        }
    }};
}

pub fn seq_coords_to_int(mut keyseqs: Vec<&str>) -> Option<i32> {
    let second_num: i32 = keyseqs.pop().unwrap().parse().unwrap();
    let first_num: i32 = keyseqs.pop().unwrap().parse().unwrap();
    find_number_map(first_num, second_num)
}

pub fn find_number_map(x: i32, y: i32) -> Option<i32> {
    let ncols = config::ncols_config();
    let nrows = (30f32 / ncols as f32).ceil() as i32;
    if 1 <= x && x <= ncols && 1 <= y && y <= nrows {
        Some(((x - 1).rem_euclid(ncols)) + (ncols * (y - 1)))
    } else {
        None
    }
}

pub fn term_width() -> u16 {
    terminal::size().unwrap().0
}

pub fn term_height() -> u16 {
    terminal::size().unwrap().1
}

pub fn open_in_browser(image_id: &str) {
    let link = format!("https://www.pixiv.net/artworks/{}", image_id);
    Command::new("xdg-open").arg(&link).spawn().unwrap_or_else(|_| panic!("xdg-open not installed!"));
    println!("Opened {} in browser!", link);
}

pub fn open_link_num(data: data::Gallery, number: i32) {
    open_in_browser(&data.image_id(number))
}

pub fn handle_missing_pics() {
    let basedir = Path::new(KONEKODIR).parent().unwrap().join("pics");
    if basedir.exists() {
        return;
    }
    println!("Please wait, downloading welcome image (this will only occur once)...");
    let baseurl = "https://raw.githubusercontent.com/twenty5151/koneko/master/pics/";
    fs::create_dir_all(&basedir).unwrap();
    for pic in ["71471144_p0.png", "79494300_p0.png"].iter() {
        Command::new("curl")
            .args(&[
                "-s",
                &format!("{}{}", baseurl, pic),
                "-o",
                &basedir.join(pic).to_str().unwrap(),
            ])
            .spawn();
    }
    Command::new("clear").spawn();
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_cd_simple() {
        let root = env::current_dir().unwrap();
        cd!(Path::new("src"), {
            assert_eq!(env::current_dir().unwrap(), root.join("src"))
        });
        assert_eq!(env::current_dir().unwrap(), root);
    }

    #[rstest]
    #[should_panic]
    fn test_cd_panic() {
        let root = env::current_dir().unwrap();
        cd!(Path::new("src"), {
            assert_eq!(env::current_dir().unwrap(), root.join("src"));
            panic!("Test")
        });
        assert_eq!(env::current_dir().unwrap(), root);
    }
}
