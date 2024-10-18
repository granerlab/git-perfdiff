use git2::Repository;
use git_perfdiff::git;
use std::path::{Path, PathBuf};

pub struct TestContext(pub git::Context);

impl Drop for TestContext {
    fn drop(&mut self) {
        let Self(ctx) = self;
        let repo_path = &ctx.path;
        std::fs::remove_dir_all(repo_path)
            .unwrap_or_else(|_| panic!("Git repo at path {repo_path:#?} not deleted"));
    }
}

pub fn initial_commit(repo: &git2::Repository) {
    let signature = repo.signature().unwrap();
    let oid = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )
    .unwrap();
}

pub fn git_init(path: &str) -> TestContext {
    let path = PathBuf::from(path);
    // Check if directory already exists
    if path.try_exists().unwrap_or_default() {
        panic!("Directory {path:#?} already exists!");
    } else {
        // Attempt to create directory.
        std::fs::create_dir_all(&path)
            .unwrap_or_else(|_| panic!("Failed to create directory {path:#?}"));
    };

    let repo = git2::Repository::init(&path)
        .unwrap_or_else(|_| panic!("Failed to create repository at {path:#?}"));
    initial_commit(&repo);
    let ctx = git::Context { repo, path };

    TestContext(ctx)
}

pub fn git_add(repo: &Repository, path: &Path) {
    let mut index = repo.index().unwrap();
    index.add_path(path).unwrap();
    index.write().unwrap();
}

pub fn git_commit(repo: &Repository, message: &str) -> git2::Oid {
    let mut index = repo.index().unwrap();
    let oid = index.write_tree().unwrap();
    let signature = repo.signature().unwrap();
    let parent_commit = repo.head().unwrap().peel_to_commit().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit],
    )
    .unwrap()
}
