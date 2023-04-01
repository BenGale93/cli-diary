use chrono::prelude::*;
use git2::Repository;
use pathdiff;

use crate::{errors::DiaryError, utils::git, Diary};
pub struct CommitOptions {
    /// The date of the entry to open.
    pub entry_date: DateTime<Local>,
    pub message: String,
    pub push: bool,
}

pub fn commit(opts: &CommitOptions, diary: &Diary) -> Result<(), DiaryError> {
    let entry_path = diary.get_entry_path(&opts.entry_date);
    let relative_path = pathdiff::diff_paths(entry_path, diary.diary_path()).unwrap();

    let repo = Repository::open(diary.diary_path())?;

    git::add_and_commit(&repo, &relative_path, &opts.message)?;

    if opts.push {
        git::push_to_origin(&repo)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use chrono::prelude::*;
    use git2::Repository;

    use super::{commit, CommitOptions};
    use crate::{
        ops::{init, testing, InitOptions},
        utils::git,
        Diary,
    };

    #[test]
    fn commit_today() {
        let config = testing::temp_config();

        let init_opts = InitOptions {
            path: testing::temp_path(),
            prefix: None,
            git_repo: true,
        };
        init(&init_opts, config.diary_path()).unwrap();

        let entry_date = Local.with_ymd_and_hms(2022, 1, 13, 0, 0, 0).unwrap();
        testing::new_entry(&config, &entry_date);

        let opts = CommitOptions {
            entry_date,
            message: "Test message".to_string(),
            push: false,
        };
        let repo = Repository::open(config.diary_path()).unwrap();

        let diary = Diary::from_config(&config).unwrap();
        commit(&opts, &diary).unwrap();

        let last_commit = git::find_last_commit(&repo).unwrap();
        commit(&opts, &diary).unwrap();
        assert!(last_commit.is_some());

        let index = repo.index().unwrap();
        assert_eq!(index.len(), 1)
    }

    #[test]
    fn commit_multiple() {
        let config = testing::temp_config();

        let init_opts = InitOptions {
            path: testing::temp_path(),
            prefix: None,
            git_repo: true,
        };
        init(&init_opts, config.diary_path()).unwrap();

        let entry_date = Local.with_ymd_and_hms(2022, 1, 13, 0, 0, 0).unwrap();
        testing::new_entry(&config, &entry_date);

        let opts = CommitOptions {
            entry_date,
            message: "Test message".to_string(),
            push: false,
        };
        let repo = Repository::open(config.diary_path()).unwrap();

        let diary = Diary::from_config(&config).unwrap();
        commit(&opts, &diary).unwrap();

        let last_commit = git::find_last_commit(&repo).unwrap();
        assert!(last_commit.is_some());

        let entry_date = Local.with_ymd_and_hms(2022, 1, 14, 0, 0, 0).unwrap();
        testing::new_entry(&config, &entry_date);

        let opts = CommitOptions {
            entry_date,
            message: "Test message".to_string(),
            push: false,
        };
        let repo = Repository::open(config.diary_path()).unwrap();

        commit(&opts, &diary).unwrap();

        let last_commit = git::find_last_commit(&repo).unwrap();
        assert!(last_commit.is_some());

        let index = repo.index().unwrap();
        assert_eq!(index.len(), 2)
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn commit_no_entry() {
        let config = testing::temp_config();

        let init_opts = InitOptions {
            path: testing::temp_path(),
            prefix: None,
            git_repo: true,
        };
        init(&init_opts, config.diary_path()).unwrap();

        let entry_date = Local.with_ymd_and_hms(2022, 1, 13, 0, 0, 0).unwrap();

        let opts = CommitOptions {
            entry_date,
            message: "Test message".to_string(),
            push: false,
        };

        let diary = Diary::from_config(&config).unwrap();
        commit(&opts, &diary).unwrap();
    }

    #[test]
    #[should_panic(expected = "remote 'origin' does not exist")]
    fn commit_and_fail_to_push() {
        let config = testing::temp_config();

        let init_opts = InitOptions {
            path: testing::temp_path(),
            prefix: None,
            git_repo: true,
        };
        init(&init_opts, config.diary_path()).unwrap();

        let entry_date = Local.with_ymd_and_hms(2022, 1, 13, 0, 0, 0).unwrap();

        testing::new_entry(&config, &entry_date);

        let opts = CommitOptions {
            entry_date,
            message: "Test message".to_string(),
            push: true,
        };

        let diary = Diary::from_config(&config).unwrap();
        commit(&opts, &diary).unwrap();
    }
}
