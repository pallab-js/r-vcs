use anyhow::Result;
use glob::Pattern;
use std::fs;
use std::path::Path;

pub struct IgnoreRules {
    patterns: Vec<Pattern>,
}

impl IgnoreRules {
    pub fn load(repo: &crate::repository::Repository) -> Result<Self> {
        let ignore_path = repo.worktree.join(".vcsignore");
        let mut patterns = Vec::new();

        // Always ignore .vcs directory
        patterns.push(Pattern::new(".vcs/**")?);
        patterns.push(Pattern::new(".vcs")?);

        if ignore_path.exists() {
            let content = fs::read_to_string(&ignore_path)?;
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                // Convert gitignore-style patterns to glob
                let pattern = normalize_pattern(line);
                if let Ok(pat) = Pattern::new(&pattern) {
                    patterns.push(pat);
                }
            }
        }

        Ok(IgnoreRules { patterns })
    }

    pub fn should_ignore(&self, path: &Path, repo: &crate::repository::Repository) -> bool {
        // Get relative path from worktree
        let rel_path = match path.strip_prefix(&repo.worktree) {
            Ok(p) => p,
            Err(_) => return false,
        };

        let path_str = rel_path.to_string_lossy().replace('\\', "/");

        // Check against all patterns
        for pattern in &self.patterns {
            if pattern.matches(&path_str) || pattern.matches_path(rel_path) {
                return true;
            }
        }

        false
    }
}

fn normalize_pattern(pattern: &str) -> String {
    let mut normalized = pattern.to_string();

    // If pattern doesn't start with /, make it match anywhere
    if !normalized.starts_with('/') {
        normalized = format!("**/{}", normalized);
    } else {
        normalized = normalized
            .strip_prefix('/')
            .unwrap_or(&normalized)
            .to_string();
    }

    // If pattern ends with /, it's a directory
    if normalized.ends_with('/') {
        normalized = format!("{}**", normalized);
    }

    normalized
}
