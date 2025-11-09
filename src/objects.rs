use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub path: String,
    pub hash: String,
    pub size: u64,
    pub mode: String, // File permissions/mode
}

#[derive(Debug, Clone)]
pub enum GitObject {
    Blob(Vec<u8>),
    Tree(Vec<TreeEntry>),
    Commit(Commit),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeEntry {
    pub mode: String,
    pub name: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub tree: String,
    pub parent: Option<String>,
    pub author: String,
    pub message: String,
    pub timestamp: i64,
}

impl GitObject {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        match self {
            GitObject::Blob(data) => {
                let header = format!("blob {}\0", data.len());
                let mut result = header.into_bytes();
                result.extend_from_slice(data);
                Ok(result)
            }
            GitObject::Tree(entries) => {
                let mut data = Vec::new();
                for entry in entries {
                    data.extend_from_slice(entry.mode.as_bytes());
                    data.push(b' ');
                    data.extend_from_slice(entry.name.as_bytes());
                    data.push(0);
                    let hash_bytes = hex::decode(&entry.hash)?;
                    data.extend_from_slice(&hash_bytes);
                }
                let header = format!("tree {}\0", data.len());
                let mut result = header.into_bytes();
                result.extend_from_slice(&data);
                Ok(result)
            }
            GitObject::Commit(commit) => {
                let mut data = Vec::new();
                data.extend_from_slice(format!("tree {}\n", commit.tree).as_bytes());
                if let Some(ref parent) = commit.parent {
                    data.extend_from_slice(format!("parent {}\n", parent).as_bytes());
                }
                data.extend_from_slice(format!("author {}\n", commit.author).as_bytes());
                data.extend_from_slice(format!("timestamp {}\n", commit.timestamp).as_bytes());
                data.push(b'\n');
                data.extend_from_slice(commit.message.as_bytes());

                let header = format!("commit {}\0", data.len());
                let mut result = header.into_bytes();
                result.extend_from_slice(&data);
                Ok(result)
            }
        }
    }

    pub fn deserialize(data: &[u8]) -> Result<Self> {
        let null_pos = data
            .iter()
            .position(|&b| b == 0)
            .context("Invalid object format")?;

        let header = std::str::from_utf8(&data[..null_pos])?;
        let parts: Vec<&str> = header.split_whitespace().collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid object header");
        }

        let obj_type = parts[0];
        let _size: usize = parts[1].parse()?;
        let content = &data[null_pos + 1..];

        match obj_type {
            "blob" => Ok(GitObject::Blob(content.to_vec())),
            "tree" => {
                let mut entries = Vec::new();
                let mut pos = 0;

                while pos < content.len() {
                    // Read mode
                    let mode_end = content[pos..]
                        .iter()
                        .position(|&b| b == b' ')
                        .context("Invalid tree format")?;
                    let mode = std::str::from_utf8(&content[pos..pos + mode_end])?.to_string();
                    pos += mode_end + 1;

                    // Read name
                    let name_end = content[pos..]
                        .iter()
                        .position(|&b| b == 0)
                        .context("Invalid tree format")?;
                    let name = std::str::from_utf8(&content[pos..pos + name_end])?.to_string();
                    pos += name_end + 1;

                    // Read hash (20 bytes)
                    if pos + 20 > content.len() {
                        anyhow::bail!("Invalid tree format: hash too short");
                    }
                    let hash_bytes = &content[pos..pos + 20];
                    let hash = hex::encode(hash_bytes);
                    pos += 20;

                    entries.push(TreeEntry { mode, name, hash });
                }

                Ok(GitObject::Tree(entries))
            }
            "commit" => {
                let content_str = std::str::from_utf8(content)?;
                let mut tree = None;
                let mut parent = None;
                let mut author = None;
                let mut timestamp = None;
                let mut message = String::new();

                let mut lines = content_str.lines();
                let mut in_message = false;

                for line in lines.by_ref() {
                    if in_message {
                        if !message.is_empty() {
                            message.push('\n');
                        }
                        message.push_str(line);
                        continue;
                    }

                    if line.is_empty() {
                        in_message = true;
                        continue;
                    }

                    if let Some(t) = line.strip_prefix("tree ") {
                        tree = Some(t.to_string());
                    } else if let Some(p) = line.strip_prefix("parent ") {
                        parent = Some(p.to_string());
                    } else if let Some(a) = line.strip_prefix("author ") {
                        author = Some(a.to_string());
                    } else if let Some(ts) = line.strip_prefix("timestamp ") {
                        timestamp = Some(ts.parse()?);
                    }
                }

                Ok(GitObject::Commit(Commit {
                    tree: tree.context("Missing tree in commit")?,
                    parent,
                    author: author.context("Missing author in commit")?,
                    message,
                    timestamp: timestamp.context("Missing timestamp in commit")?,
                }))
            }
            _ => anyhow::bail!("Unknown object type: {}", obj_type),
        }
    }
}

pub fn read_file(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).with_context(|| format!("Failed to read file: {}", path.display()))
}
