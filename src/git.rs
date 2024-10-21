use crate::cli::Args as CliArgs;
use anyhow::{anyhow, Result};
use git2::Repository;
use std::path::PathBuf;

/// Git repository context. Wraps the `git2::Repository` type.
pub struct Context {
    /// The wrapped repository.
    pub repo: Repository,
    /// Path to the git repository in the file system.
    // TODO: Refactor to be a `&Path` to avoid cloning
    pub path: PathBuf,
}

impl Context {
    /// Checkout a git reference (SHA, branch name, tag).
    ///
    /// # Errors
    ///
    /// Forwards any errors arising from `git2`.
    pub fn checkout(&self, reference: &str) -> Result<()> {
        let repo = &self.repo;
        // We don't want to discard uncommitted files.
        if !repo.statuses(None)?.is_empty() {
            return Err(anyhow!("Repository contains uncommitted files"));
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

impl TryFrom<&CliArgs> for Context {
    type Error = git2::Error;
    fn try_from(value: &CliArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            repo: Repository::open(&value.path)?,
            path: value.path.clone(),
        })
    }
}

/// Reference targets for performance diffing.
pub struct DiffTargets<'a> {
    /// Base reference.
    pub base_ref: &'a str,
    /// Head reference.
    pub head_ref: &'a str,
}

impl<'a> From<&'a CliArgs> for DiffTargets<'a> {
    fn from(value: &'a CliArgs) -> Self {
        Self {
            // TODO: default to commit before branching, or the root commit.
            base_ref: value.base.as_ref().expect("Base ref must be specified"),
            // TODO: Default to HEAD:
            // `head_ref: value.head.as_ref().map_or("HEAD", |s| s.as_str())`
            // Requires binding to the current head using `Context`
            head_ref: value.head.as_ref().expect("Head ref must be specified"),
        }
    }
}
