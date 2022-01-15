use git2::{Commit, ObjectType, Repository};

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

#[cfg(test)]
mod test {
    use git2::Repository;
    use tempfile::tempdir;

    use super::find_last_commit;

    #[test]
    fn no_commit() {
        let dir = tempdir().unwrap().path().to_path_buf();

        let repo = Repository::init(&dir).unwrap();

        let last_commit = find_last_commit(&repo).unwrap();

        assert!(last_commit.is_none())
    }
}
