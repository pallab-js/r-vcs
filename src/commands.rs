use crate::config::Config;
use crate::ignore::IgnoreRules;
use crate::objects::{read_file, Commit, GitObject, IndexEntry, TreeEntry};
use crate::repository::Repository;
use anyhow::{Context, Result};
use chrono::Utc;
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn init() -> Result<()> {
    Repository::create()?;
    Ok(())
}

pub fn add(paths: Vec<String>) -> Result<()> {
    let repo = Repository::get_repo()?;
    let _lock = crate::lock::RepoLock::new(&repo)?;
    let ignore_rules = IgnoreRules::load(&repo)?;
    let mut index = repo.get_index()?;

    for path_str in paths {
        let path = PathBuf::from(&path_str);
        let full_path = if path.is_absolute() {
            path
        } else {
            repo.worktree.join(path)
        };

        if !full_path.exists() {
            anyhow::bail!("Path does not exist: {}", path_str);
        }

        if full_path.is_file() {
            if !ignore_rules.should_ignore(&full_path, &repo) {
                add_file(&repo, &mut index, &full_path)?;
            }
        } else if full_path.is_dir() {
            for entry in WalkDir::new(&full_path) {
                let entry = entry?;
                let entry_path = entry.path();
                if entry_path.is_file() && !ignore_rules.should_ignore(entry_path, &repo) {
                    add_file(&repo, &mut index, entry_path)?;
                }
            }
        }
    }

    repo.write_index(&index)?;
    Ok(())
}

fn add_file(repo: &Repository, index: &mut Vec<IndexEntry>, path: &Path) -> Result<()> {
    let mut data = read_file(path)?;

    // Normalize line endings: CRLF -> LF for consistency across platforms
    // This prevents false "modified" files when collaborating across Windows/Unix
    if data.contains(&b'\r') {
        let mut normalized = Vec::with_capacity(data.len());
        let mut i = 0;
        while i < data.len() {
            if data[i] == b'\r' && i + 1 < data.len() && data[i + 1] == b'\n' {
                // CRLF -> LF
                normalized.push(b'\n');
                i += 2;
            } else if data[i] == b'\r' {
                // Standalone CR -> LF
                normalized.push(b'\n');
                i += 1;
            } else {
                normalized.push(data[i]);
                i += 1;
            }
        }
        data = normalized;
    }

    let blob = GitObject::Blob(data.clone());
    let hash = repo.write_object(&blob)?;

    let rel_path = path
        .strip_prefix(&repo.worktree)
        .with_context(|| {
            format!(
                "Path {} is not under worktree {}",
                path.display(),
                repo.worktree.display()
            )
        })?
        .to_string_lossy()
        .replace('\\', "/")
        .to_string();

    // Get file permissions
    let metadata = std::fs::metadata(path)?;
    let mode = if cfg!(unix) {
        use std::os::unix::fs::PermissionsExt;
        format!("{:o}", metadata.permissions().mode())
    } else {
        "100644".to_string() // Regular file on Windows
    };

    // Remove existing entry if any
    index.retain(|e| e.path != rel_path);

    let path_clone = rel_path.clone();
    index.push(IndexEntry {
        path: rel_path,
        hash: hash.clone(),
        size: data.len() as u64,
        mode,
    });

    println!("Added {}", path_clone);
    Ok(())
}

pub fn commit(message: &str) -> Result<()> {
    let repo = Repository::get_repo()?;
    let _lock = crate::lock::RepoLock::new(&repo)?;
    let index = repo.get_index()?;

    if index.is_empty() {
        anyhow::bail!("Nothing to commit (use 'vcs add' to stage files)");
    }

    // Create tree from index
    let tree_entries = create_tree_from_index(&repo, &index)?;
    let tree = GitObject::Tree(tree_entries);
    let tree_hash = repo.write_object(&tree)?;

    // Get parent commit
    let parent = repo.get_head()?;

    // Get author from config
    let config = Config::new(&repo);
    let author_name = config.get_user_name();
    let author_email = config.get_user_email();

    // Create commit
    let commit = Commit {
        tree: tree_hash,
        parent,
        author: format!("{} <{}>", author_name, author_email),
        message: message.to_string(),
        timestamp: Utc::now().timestamp(),
    };

    let commit_obj = GitObject::Commit(commit);
    let commit_hash = repo.write_object(&commit_obj)?;

    // Update HEAD
    repo.set_head(&commit_hash)?;

    // Clear index
    repo.write_index(&[])?;

    println!("Committed {}: {}", &commit_hash[..8], message);
    Ok(())
}

fn create_tree_from_index(repo: &Repository, index: &[IndexEntry]) -> Result<Vec<TreeEntry>> {
    create_tree_for_path(repo, index, "")
}

fn create_tree_for_path(
    repo: &Repository,
    all_entries: &[IndexEntry],
    base_path: &str,
) -> Result<Vec<TreeEntry>> {
    use std::collections::HashMap;

    let base = if base_path.is_empty() {
        Path::new("")
    } else {
        Path::new(base_path)
    };

    // Find entries that belong to this directory level
    let mut file_entries: Vec<&IndexEntry> = Vec::new();
    let mut subdirs: HashMap<String, Vec<&IndexEntry>> = HashMap::new();

    for entry in all_entries {
        let entry_path = Path::new(&entry.path);

        // Check if this entry belongs to the current directory level
        let relative = if base == Path::new("") {
            entry_path
        } else {
            match entry_path.strip_prefix(base) {
                Ok(rel) => rel,
                Err(_) => continue, // Entry doesn't belong to this directory
            }
        };

        // Count path components
        let components: Vec<_> = relative.components().collect();

        if components.len() == 1 {
            // This is a file in the current directory
            file_entries.push(entry);
        } else if components.len() > 1 {
            // This is in a subdirectory
            let first_component = components[0];
            if let std::path::Component::Normal(name) = first_component {
                let subdir_name = name.to_string_lossy().to_string();
                subdirs
                    .entry(subdir_name)
                    .or_insert_with(Vec::new)
                    .push(entry);
            }
        }
    }

    let mut tree_entries = Vec::new();

    // Add files in this directory
    for entry in file_entries {
        let path = Path::new(&entry.path);
        let name = if base == Path::new("") {
            path.file_name()
        } else {
            path.strip_prefix(base).ok().and_then(|p| p.file_name())
        }
        .and_then(|n| n.to_str())
        .context("Invalid file name")?
        .to_string();

        // Use mode from index entry
        tree_entries.push(TreeEntry {
            mode: entry.mode.clone(),
            name,
            hash: entry.hash.clone(),
        });
    }

    // Create subtrees for subdirectories
    let mut subdir_names: Vec<String> = subdirs.keys().cloned().collect();
    subdir_names.sort();

    for subdir_name in subdir_names {
        let _subdir_entries = &subdirs[&subdir_name];
        let subdir_path = if base == Path::new("") {
            subdir_name.clone()
        } else {
            format!("{}/{}", base_path, subdir_name)
        };

        let sub_tree_entries = create_tree_for_path(repo, all_entries, &subdir_path)?;
        let sub_tree = GitObject::Tree(sub_tree_entries);
        let sub_tree_hash = repo.write_object(&sub_tree)?;

        tree_entries.push(TreeEntry {
            mode: "40000".to_string(), // Directory
            name: subdir_name,
            hash: sub_tree_hash,
        });
    }

    Ok(tree_entries)
}

pub fn status() -> Result<()> {
    let repo = Repository::get_repo()?;
    let index = repo.get_index()?;
    let ignore_rules = IgnoreRules::load(&repo)?;

    // Get HEAD tree for comparison
    let head_tree = get_head_tree(&repo)?;
    let head_files = get_files_from_tree(&repo, &head_tree)?;

    // Get all files in working directory
    let mut working_files: HashMap<String, Vec<u8>> = HashMap::new();
    for entry in WalkDir::new(&repo.worktree) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && !ignore_rules.should_ignore(path, &repo) {
            if let Ok(rel_path) = path.strip_prefix(&repo.worktree) {
                let rel_str = rel_path.to_string_lossy().replace('\\', "/");
                if let Ok(data) = fs::read(path) {
                    working_files.insert(rel_str, data);
                }
            }
        }
    }

    // Categorize files
    let mut staged_new: Vec<String> = Vec::new();
    let mut staged_modified: Vec<String> = Vec::new();
    let mut staged_deleted: Vec<String> = Vec::new();
    let mut modified: Vec<String> = Vec::new();
    let mut deleted: Vec<String> = Vec::new();
    let mut untracked: Vec<String> = Vec::new();

    // Build index map
    let mut index_paths: HashMap<String, String> = HashMap::new();
    for entry in &index {
        index_paths.insert(entry.path.clone(), entry.hash.clone());
    }

    // Check all files in working directory
    for (path, working_data) in &working_files {
        let blob = GitObject::Blob(working_data.clone());
        let working_hash = crate::utils::hash_object(&blob.serialize()?);

        let in_index = index_paths.contains_key(path);
        let in_head = head_files.contains_key(path);

        if in_index {
            // File is in index (staged)
            let idx_hash = index_paths[path].clone();
            if working_hash == idx_hash {
                // Working directory matches index
                if in_head {
                    let hd_hash = head_files[path].clone();
                    if idx_hash == hd_hash {
                        // Matches both index and HEAD - shouldn't be staged, but if it is, ignore
                        // This shouldn't happen in normal operation
                    } else {
                        // Index differs from HEAD - staged modification
                        staged_modified.push(path.clone());
                    }
                } else {
                    // New file, staged
                    staged_new.push(path.clone());
                }
            } else {
                // Working directory differs from index
                // File is staged (index version) but also modified in working directory
                if in_head {
                    let hd_hash = head_files[path].clone();
                    if idx_hash == hd_hash {
                        // Index matches HEAD, working differs - just modified, not staged
                        modified.push(path.clone());
                    } else {
                        // Index differs from HEAD (staged), working differs from index (modified)
                        staged_modified.push(path.clone());
                        modified.push(path.clone());
                    }
                } else {
                    // New file staged, then modified in working directory
                    staged_new.push(path.clone());
                    modified.push(path.clone());
                }
            }
        } else if in_head {
            // File is tracked (in HEAD) but not in index
            let hd_hash = head_files[path].clone();
            if working_hash != hd_hash {
                // Modified from HEAD
                modified.push(path.clone());
            }
            // If it matches HEAD, it's clean - don't show it
        } else {
            // Not in index, not in HEAD - untracked
            untracked.push(path.clone());
        }
    }

    // Check for deleted files (in index or HEAD but not in working directory)
    for entry in &index {
        if !working_files.contains_key(&entry.path) {
            // File is staged but deleted from working directory
            if head_files.contains_key(&entry.path) {
                staged_deleted.push(entry.path.clone());
            } else {
                // Was staged as new, then deleted - remove from staged, show as untracked?
                // Actually, if it was just added and deleted, it shouldn't be committed
            }
        }
    }

    // Check for files in HEAD but deleted from working directory (not staged)
    for (path, _) in &head_files {
        if !working_files.contains_key(path) && !index_paths.contains_key(path) {
            deleted.push(path.clone());
        }
    }

    // Sort all lists
    staged_new.sort();
    staged_modified.sort();
    staged_deleted.sort();
    modified.sort();
    deleted.sort();
    untracked.sort();

    // Print status
    println!("{}", "On branch master".bright_white().bold());

    if !staged_new.is_empty() || !staged_modified.is_empty() || !staged_deleted.is_empty() {
        println!("\n{}", "Changes to be committed:".green());
        println!("  (use \"vcs reset <file>...\" to unstage)");
        for file in &staged_new {
            println!("        {} {}", "new file:".green(), file);
        }
        for file in &staged_modified {
            println!("        {} {}", "modified:".green(), file);
        }
        for file in &staged_deleted {
            println!("        {} {}", "deleted:".red(), file);
        }
    }

    if !modified.is_empty() {
        println!("\n{}", "Changes not staged for commit:".yellow());
        println!("  (use \"vcs add <file>...\" to update what will be committed)");
        for file in &modified {
            println!("        {} {}", "modified:".yellow(), file);
        }
    }

    if !untracked.is_empty() {
        println!("\n{}", "Untracked files:".bright_white());
        println!("  (use \"vcs add <file>...\" to include in what will be committed)");
        for file in &untracked {
            println!("        {}", file);
        }
    }

    if staged_new.is_empty()
        && staged_modified.is_empty()
        && staged_deleted.is_empty()
        && modified.is_empty()
        && deleted.is_empty()
        && untracked.is_empty()
    {
        println!("\n{}", "nothing to commit, working tree clean".green());
    }

    Ok(())
}

fn get_head_tree(repo: &Repository) -> Result<Option<String>> {
    if let Some(commit_hash) = repo.get_head()? {
        let obj = repo.read_object(&commit_hash)?;
        if let GitObject::Commit(commit) = obj {
            return Ok(Some(commit.tree));
        }
    }
    Ok(None)
}

fn get_files_from_tree(
    repo: &Repository,
    tree_hash: &Option<String>,
) -> Result<HashMap<String, String>> {
    let mut files = HashMap::new();

    if let Some(hash) = tree_hash {
        collect_files_from_tree(repo, hash, "", &mut files)?;
    }

    Ok(files)
}

fn collect_files_from_tree(
    repo: &Repository,
    tree_hash: &str,
    prefix: &str,
    files: &mut HashMap<String, String>,
) -> Result<()> {
    let obj = repo.read_object(tree_hash)?;
    if let GitObject::Tree(entries) = obj {
        for entry in entries {
            let path = if prefix.is_empty() {
                entry.name.clone()
            } else {
                format!("{}/{}", prefix, entry.name)
            };
            // Normalize path to use forward slashes
            let normalized_path = path.replace('\\', "/");

            if entry.mode == "40000" {
                // Directory - recurse
                collect_files_from_tree(repo, &entry.hash, &normalized_path, files)?;
            } else {
                // File
                files.insert(normalized_path, entry.hash);
            }
        }
    }
    Ok(())
}

pub fn log(oneline: bool, number: Option<usize>) -> Result<()> {
    let repo = Repository::get_repo()?;
    let mut commit_hash = repo.get_head()?;

    if commit_hash.is_none() {
        println!("No commits yet");
        return Ok(());
    }

    let mut count = 0;
    let limit = number.unwrap_or(usize::MAX);

    while let Some(hash) = commit_hash {
        if count >= limit {
            break;
        }

        let obj = repo.read_object(&hash)?;
        if let GitObject::Commit(commit) = obj {
            if oneline {
                let short_hash = &hash[..8];
                let first_line = commit.message.lines().next().unwrap_or("");
                println!("{} {}", short_hash.bright_yellow(), first_line);
            } else {
                println!("{}", format!("commit {}", hash).bright_yellow().bold());
                println!("Author: {}", commit.author.bright_white());
                println!(
                    "Date:   {}",
                    chrono::DateTime::from_timestamp(commit.timestamp, 0)
                        .ok_or_else(|| anyhow::anyhow!("Invalid timestamp"))?
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                        .bright_white()
                );
                println!();
                for line in commit.message.lines() {
                    println!("    {}", line);
                }
                println!();
            }

            commit_hash = commit.parent;
            count += 1;
        } else {
            break;
        }
    }

    Ok(())
}

pub fn cat_file(hash: &str) -> Result<()> {
    let repo = Repository::get_repo()?;
    let obj = repo.read_object(hash)?;

    match obj {
        GitObject::Blob(data) => {
            print!("{}", String::from_utf8_lossy(&data));
        }
        GitObject::Tree(entries) => {
            for entry in entries {
                println!("{} {} {}", entry.mode, entry.hash, entry.name);
            }
        }
        GitObject::Commit(commit) => {
            println!("tree {}", commit.tree);
            if let Some(parent) = commit.parent {
                println!("parent {}", parent);
            }
            println!("author {}", commit.author);
            println!("timestamp {}", commit.timestamp);
            println!();
            println!("{}", commit.message);
        }
    }

    Ok(())
}

pub fn config(key: Option<String>, value: Option<String>, global: bool, list: bool) -> Result<()> {
    // For global config, we don't need a repo
    let repo = if global {
        // Create a dummy repo path for global config
        Repository::new(std::env::current_dir()?)?
    } else {
        Repository::get_repo()?
    };
    let config = Config::new(&repo);

    if list {
        let all_config = config.list()?;
        for (k, v) in all_config {
            println!("{}={}", k, v);
        }
        return Ok(());
    }

    if let Some(key) = key {
        if let Some(value) = value {
            // Set config
            config.set(&key, &value, global)?;
            println!("Set {} = {}", key, value);
        } else {
            // Get config
            if let Some(val) = config.get(&key)? {
                println!("{}", val);
            } else {
                anyhow::bail!("Config key '{}' not found", key);
            }
        }
    } else {
        anyhow::bail!("Either specify a key to get, or key and value to set");
    }

    Ok(())
}

pub fn reset(paths: Vec<String>) -> Result<()> {
    let repo = Repository::get_repo()?;
    let _lock = crate::lock::RepoLock::new(&repo)?;
    let mut index = repo.get_index()?;

    if paths.is_empty() {
        // Reset all
        repo.write_index(&[])?;
        println!("Unstaged all files");
        return Ok(());
    }

    let mut removed = Vec::new();
    for path_str in paths {
        let path = PathBuf::from(&path_str);
        let rel_path = if path.is_absolute() {
            path.strip_prefix(&repo.worktree)
                .ok()
                .and_then(|p| p.to_str())
                .map(|s| s.replace('\\', "/"))
        } else {
            Some(path_str.replace('\\', "/"))
        };

        if let Some(rel) = rel_path {
            let before_len = index.len();
            index.retain(|e| e.path != rel);
            if index.len() < before_len {
                removed.push(rel);
            }
        }
    }

    repo.write_index(&index)?;

    if removed.is_empty() {
        println!("No files were unstaged");
    } else {
        for file in &removed {
            println!("Unstaged {}", file);
        }
    }

    Ok(())
}
