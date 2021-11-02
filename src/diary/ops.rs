use std::{fs::canonicalize, fs::create_dir_all, io::ErrorKind, path::PathBuf};

use crate::{errors::DiaryError, Config};

pub struct InitOptions {
    pub path: PathBuf,
}

pub fn init(opts: InitOptions, config: &Config) -> Result<(), DiaryError> {
    if config.diary_path != PathBuf::from("") {
        if config.diary_path.exists() {
            return Err(DiaryError {
                desc: String::from("Diary folder already exists somewhere."),
            });
        } else {
            create_dir_all(&config.diary_path).unwrap();
        }
    } else {
        let diary_path = opts.path.join("diary");
        if let Err(e) = create_dir_all(&diary_path) {
            if e.kind() == ErrorKind::AlreadyExists {
                return Err(DiaryError {
                    desc: String::from("Diary folder already exists here."),
                });
            } else {
                panic!("{:?}", e);
            }
        };
        let new_cfg = Config {
            diary_path: canonicalize(&diary_path).unwrap(),
        };
        confy::store("diary", new_cfg).unwrap();
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::Config;

    use super::{init, InitOptions};

    #[test]
    fn blank_config_valid_path() {
        let dir = tempdir().unwrap().path().to_path_buf();
        let diary_dir = dir.join("diary");
        let opts = InitOptions { path: dir };
        let config = Config::default();

        init(opts, &config).unwrap();

        assert!(diary_dir.exists());
    }
}
