use anyhow::{Context, Result};
use fs2::FileExt;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

pub struct RepoLock {
    lock_file: File,
    lock_path: PathBuf,
}

impl RepoLock {
    pub fn new(repo: &crate::repository::Repository) -> Result<Self> {
        let lock_path = repo.vcs_dir.join("index.lock");

        // Try to open or create lock file
        let lock_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&lock_path)
            .with_context(|| format!("Failed to open lock file: {}", lock_path.display()))?;

        // Try to acquire exclusive lock (non-blocking)
        lock_file.try_lock_exclusive().with_context(|| {
            format!(
                "Repository is locked. Another VCS process may be running. \
                    If no other process is running, delete {} and try again.",
                lock_path.display()
            )
        })?;

        Ok(RepoLock {
            lock_file,
            lock_path,
        })
    }
}

impl Drop for RepoLock {
    fn drop(&mut self) {
        // Release lock and remove lock file
        let _ = self.lock_file.unlock();
        let _ = std::fs::remove_file(&self.lock_path);
    }
}
