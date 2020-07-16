use std::fs;
use std::path::Path;
use std::process::Command;

use crate::data;
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

pub fn open_in_browser(image_id: &str) {
    let link = format!("https://www.pixiv.net/artworks/{}", image_id);
    Command::new("xdg-open").arg(&link).spawn();
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
