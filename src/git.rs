use git2::Repository;
use std::path::PathBuf;

/// Git repository context. Wraps the `git2::Repository` type.
pub struct Context {
    /// The wrapped repository.
    pub repo: Repository,
    /// Path to the git repository in the file system.
    pub path: PathBuf,
}

impl Context {
    /// Checkout a git reference (SHA, branch name, tag).
    ///
    /// # Errors
    ///
    /// Forwards any errors arising from `git2`.
    pub fn checkout(&self, reference: &str) -> Result<(), git2::Error> {
        let repo = &self.repo;
        // We don't want to discard uncommitted files.
        if !repo.statuses(None)?.is_empty() {
            return Err(git2::Error::from_str(
                "Repository contains uncommitted files",
            ));
        }
        let (object, git_reference) = repo.revparse_ext(reference)?;
        repo.checkout_tree(&object, None)?;
        git_reference.map_or_else(
            || repo.set_head_detached(object.id()),
            |gref| {
                let ref_name = gref
                    .name()
                    .ok_or_else(|| git2::Error::from_str("Reference name is not valid UTF-8"))?;
                repo.set_head(ref_name)
            },
        )?;
        Ok(())
    }
}
