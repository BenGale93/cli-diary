use std::{fs::canonicalize, fs::create_dir, io::ErrorKind, path::PathBuf};

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
            create_dir(&config.diary_path).unwrap();
        }
    } else {
        let mut diary_path = opts.path;
        diary_path.push("diary");
        if let Err(e) = create_dir(&diary_path) {
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
