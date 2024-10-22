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
pub struct DiffTargets {
    /// Base reference.
    pub base_ref: git2::Oid,
    /// Head reference.
    pub head_ref: git2::Oid,
}

impl<'a> TryFrom<(&'a CliArgs, &'a Context)> for DiffTargets {
    type Error = anyhow::Error;
    fn try_from((args, ctx): (&'a CliArgs, &'a Context)) -> Result<Self, Self::Error> {
        /// Default branch name
        // TODO: Get this from config instead.
        const DEFAULT_BRANCH: &str = "main";
        let [base, head] = [(&args.base, DEFAULT_BRANCH), (&args.head, "HEAD")]
            .map(|(git_ref, default)| git_ref.as_ref().map_or(default, |v| v));
        Ok(Self {
            base_ref: ctx.resolve_ref(base)?,
            head_ref: ctx.resolve_ref(head)?,
        })
    }
}
