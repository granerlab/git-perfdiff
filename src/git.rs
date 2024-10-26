use anyhow::{anyhow, Result};
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
    pub fn checkout(&self, reference: impl AsRef<str>) -> Result<()> {
        let repo = &self.repo;
        // We don't want to discard uncommitted files.
        if !repo.statuses(None)?.is_empty() {
            return Err(anyhow!("Repository contains uncommitted files"));
        }
        let (object, git_reference) = repo.revparse_ext(reference.as_ref())?;
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

    /// Resolve a git reference to an object ID.
    fn resolve_ref(&self, reference: impl AsRef<str>) -> Result<git2::Oid> {
        Ok(self
            .repo
            .revparse_single(reference.as_ref())?
            .peel_to_commit()?
            .id())
    }
}

impl TryFrom<PathBuf> for Context {
    type Error = git2::Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self {
            repo: Repository::open(&value)?,
            path: value,
        })
    }
}

/// Reference targets for performance diffing.
pub struct DiffTargets {
    /// Base reference.
    pub base_ref: git2::Oid,
    /// Head reference.
    pub head_ref: git2::Oid,
}

impl DiffTargets {
    /// Use git context to resolve e.g. branch names or tags.
    pub(crate) fn from_string_refs(ctx: &Context, base: &str, head: &str) -> Result<Self> {
        let head_ref = ctx.resolve_ref(head)?;
        let base_ref = ctx.resolve_ref(base)?;
        Ok(Self { base_ref, head_ref })
    }
}
