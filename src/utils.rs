use std::env;
use std::path::Path;
use std::process::Command;

use scopeguard::defer;

use crate::data;

#[macro_export]
macro_rules! cd {
    ( $newdir:expr, $x:block ) => {
        {
            let old = env::current_dir().unwrap();
            {
                defer! {
                    env::set_current_dir(old).unwrap();
                };
                env::set_current_dir($newdir).unwrap();
                $x
            }
        }
    };
}

pub fn open_in_browser(image_id: &str) {
    let link = format!("https://www.pixiv.net/artworks/{}", image_id);
    Command::new("xdg-open").arg(&link).spawn();
    println!("Opened {} in browser!", link);
}

pub fn open_link_num(data: data::Gallery, number: i32) {
    open_in_browser(&data.image_id(number))
}

#[cfg(test)]
mod tests {
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
