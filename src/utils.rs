use std::env;
use std::path::Path;

use scopeguard::defer;

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
