use crate::utils;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct Repository {
    pub worktree: PathBuf,
    pub vcs_dir: PathBuf,
}

impl Repository {
    pub fn new(path: PathBuf) -> Result<Self> {
        let vcs_dir = path.join(".vcs");
        Ok(Repository {
            worktree: path,
            vcs_dir,
        })
    }

    pub fn find() -> Result<Option<Self>> {
        let mut path = std::env::current_dir()?;
        let mut visited = std::collections::HashSet::new();

        loop {
            // Prevent infinite loops in case of symlinks or other filesystem issues
            if !visited.insert(path.clone()) {
                return Ok(None);
            }

            let vcs_dir = path.join(".vcs");
            if vcs_dir.exists() && vcs_dir.is_dir() {
                return Ok(Some(Repository::new(path)?));
            }
            match path.parent() {
                Some(parent) => path = parent.to_path_buf(),
                None => return Ok(None),
            }
        }
    }

    pub fn create() -> Result<Self> {
        let path = std::env::current_dir()?;
        let repo = Repository::new(path.clone())?;

        if repo.vcs_dir.exists() {
            anyhow::bail!("Repository already exists");
        }

        fs::create_dir_all(&repo.vcs_dir)?;
        fs::create_dir_all(repo.vcs_dir.join("objects"))?;
        fs::create_dir_all(repo.vcs_dir.join("refs").join("heads"))?;

        // Create HEAD file
        let head_path = repo.vcs_dir.join("HEAD");
        fs::write(head_path, "ref: refs/heads/master\n")?;

        // Create index file (staging area)
        let index_path = repo.vcs_dir.join("index");
        fs::write(index_path, "[]")?;

        println!(
            "Initialized empty VCS repository in {}",
            repo.vcs_dir.display()
        );
        Ok(repo)
    }

    pub fn get_repo() -> Result<Self> {
        Repository::find()?.context("Not a VCS repository (or any of the parent directories)")
    }

    pub fn object_path(&self, hash: &str) -> PathBuf {
        self.vcs_dir
            .join("objects")
            .join(&hash[..2])
            .join(&hash[2..])
    }

    pub fn write_object(&self, obj: &crate::objects::GitObject) -> Result<String> {
        let data = obj.serialize()?;
        let hash = utils::hash_object(&data);
        let path = self.object_path(&hash);

        if !path.exists() {
            let parent = path
                .parent()
                .with_context(|| format!("Object path has no parent: {}", path.display()))?;
            fs::create_dir_all(parent)?;
            fs::write(&path, data)?;
        }

        Ok(hash)
    }

    pub fn read_object(&self, hash: &str) -> Result<crate::objects::GitObject> {
        let path = self.object_path(hash);
        let data = fs::read(path).context("Object not found")?;
        crate::objects::GitObject::deserialize(&data)
    }

    pub fn get_index(&self) -> Result<Vec<crate::objects::IndexEntry>> {
        let index_path = self.vcs_dir.join("index");
        if !index_path.exists() {
            return Ok(vec![]);
        }
        let content = fs::read_to_string(&index_path)?;
        let entries: Vec<crate::objects::IndexEntry> =
            serde_json::from_str(&content).unwrap_or_else(|_| vec![]);

        // Migrate old index entries that don't have mode field
        let migrated: Vec<_> = entries
            .into_iter()
            .map(|mut entry| {
                if entry.mode.is_empty() {
                    entry.mode = "100644".to_string(); // Default to regular file
                }
                entry
            })
            .collect();

        Ok(migrated)
    }

    pub fn write_index(&self, entries: &[crate::objects::IndexEntry]) -> Result<()> {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let index_path = self.vcs_dir.join("index");
        let content = serde_json::to_string_pretty(entries)?;

        // Atomic write: write to temp file, then rename
        let mut temp_file = NamedTempFile::new_in(&self.vcs_dir)
            .context("Failed to create temporary index file")?;
        temp_file
            .write_all(content.as_bytes())
            .context("Failed to write index content")?;
        temp_file.flush().context("Failed to flush index content")?;

        // Atomic rename
        temp_file
            .persist(&index_path)
            .with_context(|| format!("Failed to persist index to {}", index_path.display()))?;

        Ok(())
    }

    pub fn get_head(&self) -> Result<Option<String>> {
        let head_path = self.vcs_dir.join("HEAD");
        if !head_path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(head_path)?;
        let content = content.trim();
        if content.starts_with("ref: ") {
            let ref_path = content
                .strip_prefix("ref: ")
                .ok_or_else(|| anyhow::anyhow!("Invalid HEAD format"))?
                .trim();
            let ref_file = self.vcs_dir.join(ref_path);
            if ref_file.exists() {
                Ok(Some(fs::read_to_string(ref_file)?.trim().to_string()))
            } else {
                Ok(None)
            }
        } else {
            Ok(Some(content.to_string()))
        }
    }

    pub fn set_head(&self, commit_hash: &str) -> Result<()> {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let head_path = self.vcs_dir.join("HEAD");
        let content = fs::read_to_string(&head_path)?;
        let content = content.trim();
        if content.starts_with("ref: ") {
            let ref_path = content
                .strip_prefix("ref: ")
                .ok_or_else(|| anyhow::anyhow!("Invalid HEAD format"))?
                .trim();
            let ref_file = self.vcs_dir.join(ref_path);
            let parent = ref_file
                .parent()
                .with_context(|| format!("Ref file has no parent: {}", ref_file.display()))?;
            fs::create_dir_all(parent)?;

            // Atomic write for ref file
            let mut temp_file =
                NamedTempFile::new_in(parent).context("Failed to create temporary ref file")?;
            temp_file.write_all(format!("{}\n", commit_hash).as_bytes())?;
            temp_file.flush()?;
            temp_file
                .persist(&ref_file)
                .with_context(|| format!("Failed to persist ref to {}", ref_file.display()))?;
        } else {
            // Atomic write for HEAD
            let mut temp_file = NamedTempFile::new_in(&self.vcs_dir)
                .context("Failed to create temporary HEAD file")?;
            temp_file.write_all(format!("{}\n", commit_hash).as_bytes())?;
            temp_file.flush()?;
            temp_file
                .persist(&head_path)
                .with_context(|| format!("Failed to persist HEAD to {}", head_path.display()))?;
        }
        Ok(())
    }
}
