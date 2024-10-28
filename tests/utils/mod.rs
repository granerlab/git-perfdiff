use anyhow::{anyhow, Context, Result};
use git2::{Oid, Repository};
use git_perfdiff::git;
use std::path::Path;

pub struct TestContext(pub git::Context);

impl Drop for TestContext {
    fn drop(&mut self) {
        let Self(ctx) = self;
        let repo_path = &ctx.path;
        std::fs::remove_dir_all(repo_path)
            .unwrap_or_else(|_| panic!("Git repo at path {repo_path:#?} not deleted"));
    }
}

pub fn initial_commit(repo: &git2::Repository) -> Result<Oid> {
    let signature = repo.signature()?;
    let oid = repo.index()?.write_tree()?;
    let tree = repo.find_tree(oid)?;
    Ok(repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )?)
}

pub fn git_init(path: &Path) -> Result<TestContext> {
    // Check if directory already exists
    if path.try_exists().unwrap_or_default() {
        return Err(anyhow!(format!("Directory {path:#?} already exists!")));
    }

    // Attempt to create directory.
    std::fs::create_dir_all(path).with_context(|| "Failed to create directory {path:#?}")?;

    let repo =
        git2::Repository::init(path).with_context(|| "Failed to create repository at {path:#?}")?;
    initial_commit(&repo)?;

    Ok(TestContext(git::Context {
        repo,
        path: path.to_path_buf(),
    }))
}

pub fn git_add(repo: &Repository, paths: &[&Path]) -> Result<()> {
    let mut index = repo.index()?;
    for path in paths {
        index.add_path(path)?;
    }
    index.write()?;
    Ok(())
}

pub fn git_commit(repo: &Repository, message: &str) -> Result<Oid> {
    let mut index = repo.index()?;
    let oid = index.write_tree()?;
    let signature = repo.signature()?;
    let parent_commit = repo.head()?.peel_to_commit()?;
    let tree = repo.find_tree(oid)?;
    Ok(repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit],
    )?)
}
