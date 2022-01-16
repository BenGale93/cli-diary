use std::path::Path;

use git2::{Commit, Direction, ObjectType, Repository};

pub fn find_last_commit(repo: &Repository) -> Result<Option<Commit>, git2::Error> {
    let head = repo.head();

    let obj = match head {
        Ok(head_ref) => head_ref.resolve()?.peel(ObjectType::Commit)?,
        Err(_) => return Ok(None),
    };

    Ok(Some(obj.into_commit().map_err(|_| {
        git2::Error::from_str("Couldn't find commit") // uncovered.
    })?))
}

pub fn add_and_commit(
    repo: &Repository,
    file_path: &Path,
    message: &str,
) -> Result<(), git2::Error> {
    let mut index = repo.index()?;
    index.add_path(file_path)?;
    index.write()?;
    let oid = index.write_tree()?;
    let signature = Repository::signature(repo)?;
    let tree = repo.find_tree(oid)?;

    let last_commit = find_last_commit(repo)?;

    if let Some(reference) = last_commit {
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[&reference],
        )?;
    } else {
        repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[])?;
    }
    Ok(())
}

pub fn push_to_origin(repo: &Repository) -> Result<(), git2::Error> {
    let mut remote = repo.find_remote("origin")?;
    remote.connect(Direction::Push)?; // uncovered.
    remote.push(&["refs/heads/master:refs/heads/master"], None) // uncovered.
}

#[cfg(test)]
mod test {
    use git2::Repository;

    use super::find_last_commit;
    use crate::ops::testing;

    #[test]
    fn no_commit() {
        let dir = testing::temp_path();

        let repo = Repository::init(&dir).unwrap();

        let last_commit = find_last_commit(&repo).unwrap();

        assert!(last_commit.is_none())
    }
}
