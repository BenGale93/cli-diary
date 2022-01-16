use std::{ffi::OsStr, path::PathBuf};

use chrono::prelude::*;
use git2::Repository;

use crate::{config::Config, entry::Entry, errors::DiaryError, utils::git};
pub struct CommitOptions {
    /// The date of the entry to open.
    pub entry_date: Date<Local>,
    pub message: String,
    pub push: bool,
}

pub fn commit(opts: &CommitOptions, config: &Config) -> Result<(), DiaryError> {
    config.initialised()?;

    let repo = Repository::open(config.diary_path())?;

    let diary_entry = Entry::from_config(config)?;
    let entry_path = diary_entry.get_entry_path(&opts.entry_date);

    let mut relative_path = PathBuf::new();

    let mut visited = false;
    for comp in entry_path.iter() {
        if visited {
            relative_path.push(comp);
        };
        if Some(comp) == Some(OsStr::new("diary")) {
            visited = true;
        }
    }

    let mut index = repo.index()?;
    index.add_path(&relative_path)?;
    index.write()?;
    let oid = index.write_tree()?;
    let signature = Repository::signature(&repo)?;
    let tree = repo.find_tree(oid)?;

    let last_commit = git::find_last_commit(&repo)?;

    match last_commit {
        Some(reference) => repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &opts.message,
            &tree,
            &[&reference],
        )?,

        None => repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &opts.message,
            &tree,
            &[],
        )?,
    };

    Ok(())
}

#[cfg(test)]

mod test {
    use chrono::prelude::*;
    use git2::Repository;
    use tempfile::tempdir;

    use super::{commit, CommitOptions};
    use crate::{
        config::Config,
        ops::{
            init,
            new::{new, NewOptions},
            InitOptions,
        },
        utils::{editing::test::test_string_getter, git},
    };

    #[test]
    fn commit_today() {
        let dir = tempdir().unwrap().path().to_path_buf();
        let diary_dir = dir.join("diary");
        let config = Config::builder().diary_path(diary_dir).build();

        let other_dir = tempdir().unwrap().path().to_path_buf();
        let init_opts = InitOptions {
            path: other_dir,
            prefix: None,
            git_repo: true,
        };

        init(&init_opts, &config).unwrap();

        let entry_date = Local.ymd(2022, 1, 13);

        let new_opts = NewOptions { open: false };
        new(&new_opts, &config, &entry_date, test_string_getter).unwrap();

        let opts = CommitOptions {
            entry_date,
            message: "Test message".to_string(),
            push: false,
        };
        let repo = Repository::open(config.diary_path()).unwrap();

        commit(&opts, &config).unwrap();

        let last_commit = git::find_last_commit(&repo).unwrap();
        assert!(last_commit.is_some())
    }
    #[test]
    fn commit_multiple() {
        let dir = tempdir().unwrap().path().to_path_buf();
        let diary_dir = dir.join("diary");
        let config = Config::builder().diary_path(diary_dir).build();

        let other_dir = tempdir().unwrap().path().to_path_buf();
        let init_opts = InitOptions {
            path: other_dir,
            prefix: None,
            git_repo: true,
        };

        init(&init_opts, &config).unwrap();

        let entry_date = Local.ymd(2022, 1, 13);
        let new_opts = NewOptions { open: false };
        new(&new_opts, &config, &entry_date, test_string_getter).unwrap();

        let opts = CommitOptions {
            entry_date,
            message: "Test message".to_string(),
            push: false,
        };
        let repo = Repository::open(config.diary_path()).unwrap();

        commit(&opts, &config).unwrap();

        let last_commit = git::find_last_commit(&repo).unwrap();
        assert!(last_commit.is_some());

        let entry_date = Local.ymd(2022, 1, 14);
        let new_opts = NewOptions { open: false };
        new(&new_opts, &config, &entry_date, test_string_getter).unwrap();

        let opts = CommitOptions {
            entry_date,
            message: "Test message".to_string(),
            push: false,
        };
        let repo = Repository::open(config.diary_path()).unwrap();

        commit(&opts, &config).unwrap();

        let last_commit = git::find_last_commit(&repo).unwrap();
        assert!(last_commit.is_some())
    }
}
