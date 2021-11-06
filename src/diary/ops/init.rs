use std::{fs::create_dir_all, path::PathBuf};

use crate::{errors::DiaryError, Config};

pub struct InitOptions {
    pub path: PathBuf,
}

enum InitStatus {
    UseConfig(PathBuf),
    UseOpt(PathBuf),
}

fn establish_path(opts: &InitOptions, config: &Config) -> Result<InitStatus, DiaryError> {
    if config.diary_path() != &PathBuf::from("") {
        if config.diary_path().exists() {
            Err(DiaryError::ExistsElsewhere)
        } else {
            Ok(InitStatus::UseConfig(config.diary_path().clone()))
        }
    } else {
        let diary_path = opts.path.join("diary");
        if diary_path.exists() {
            return Err(DiaryError::ExistsHere);
        }
        Ok(InitStatus::UseOpt(diary_path))
    }
}

pub fn init(opts: &InitOptions, config: &Config) -> Result<PathBuf, DiaryError> {
    let init_status = establish_path(opts, config);
    let path = match init_status? {
        InitStatus::UseConfig(path) => {
            print!("It appears the config file already has a diary path set. ");
            println!("Creating a diary folder here: {:?}", path);
            path
        }
        InitStatus::UseOpt(path) => {
            println!("Creating a diary folder.");
            path
        }
    };
    match create_dir_all(&path) {
        Ok(_) => Ok(path),
        Err(e) => Err(DiaryError::from(e)),
    }
}

#[cfg(test)]
mod tests {
    use std::fs::create_dir_all;

    use tempfile::tempdir;

    use crate::Config;

    use super::{init, InitOptions};

    #[test]
    fn blank_config_valid_path() {
        let dir = tempdir().unwrap().path().to_path_buf();
        let diary_dir = dir.join("diary");
        let opts = InitOptions { path: dir };
        let config = Config::default();

        init(&opts, &config).unwrap();

        assert!(diary_dir.exists());
    }

    #[test]
    fn blank_config_invalid_path() {
        let dir = tempdir().unwrap().path().to_path_buf();
        let diary_dir = dir.join("diary");
        let opts = InitOptions { path: dir };
        let config = Config::default();
        create_dir_all(diary_dir).unwrap();

        init(&opts, &config).expect_err("No error produced.");
    }

    #[test]
    fn filled_config_non_existing_path() {
        let dir = tempdir().unwrap().path().to_path_buf();
        let diary_dir = dir.join("diary");
        let config = Config::new(diary_dir.clone(), String::from("diary"));

        let other_dir = tempdir().unwrap().path().to_path_buf();
        let opts = InitOptions { path: other_dir };

        init(&opts, &config).unwrap();

        assert!(diary_dir.exists());
    }

    #[test]
    fn filled_config_existing_path() {
        let dir = tempdir().unwrap().path().to_path_buf();
        let diary_dir = dir.join("diary");
        let config = Config::new(diary_dir.clone(), String::from("diary"));

        let other_dir = tempdir().unwrap().path().to_path_buf();
        let opts = InitOptions { path: other_dir };

        create_dir_all(diary_dir).unwrap();

        init(&opts, &config).expect_err("No error produced.");
    }
}
