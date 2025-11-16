use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct Config {
    repo_config_path: PathBuf,
    global_config_path: PathBuf,
}

impl Config {
    pub fn new(repo: &crate::repository::Repository) -> Self {
        let repo_config_path = repo.vcs_dir.join("config");
        // Use dirs crate for cross-platform home directory
        let global_config_path = dirs::home_dir()
            .map(|home| home.join(".vcsconfig"))
            .unwrap_or_else(|| PathBuf::from(".vcsconfig")); // Fallback to current dir

        Config {
            repo_config_path,
            global_config_path,
        }
    }

    pub fn global_only() -> Self {
        let repo_config_path = PathBuf::new(); // Empty path, won't be used
        let global_config_path = dirs::home_dir()
            .map(|home| home.join(".vcsconfig"))
            .unwrap_or_else(|| PathBuf::from(".vcsconfig"));

        Config {
            repo_config_path,
            global_config_path,
        }
    }

    pub fn get(&self, key: &str) -> Result<Option<String>> {
        // Check repo config first (if it exists)
        if !self.repo_config_path.as_os_str().is_empty() {
            if let Some(value) = self.read_config_file(&self.repo_config_path, key)? {
                return Ok(Some(value));
            }
        }

        // Then check global config
        self.read_config_file(&self.global_config_path, key)
    }

    pub fn set(&self, key: &str, value: &str, global: bool) -> Result<()> {
        let config_path = if global {
            &self.global_config_path
        } else {
            &self.repo_config_path
        };

        let mut config = self.read_all_config(config_path)?;
        config.insert(key.to_string(), value.to_string());
        self.write_config(config_path, &config)?;

        Ok(())
    }

    pub fn list(&self) -> Result<HashMap<String, String>> {
        let mut all_config = HashMap::new();

        // Read global config
        let global = self.read_all_config(&self.global_config_path)?;
        for (k, v) in global {
            all_config.insert(format!("global.{}", k), v);
        }

        // Read repo config (overrides global) if it exists
        if !self.repo_config_path.as_os_str().is_empty() {
            let repo = self.read_all_config(&self.repo_config_path)?;
            for (k, v) in repo {
                all_config.insert(k, v);
            }
        }

        Ok(all_config)
    }

    fn read_config_file(&self, path: &PathBuf, key: &str) -> Result<Option<String>> {
        if !path.exists() {
            return Ok(None);
        }

        let config = self.read_all_config(path)?;
        Ok(config.get(key).cloned())
    }

    fn read_all_config(&self, path: &PathBuf) -> Result<HashMap<String, String>> {
        if !path.exists() {
            return Ok(HashMap::new());
        }

        let content = fs::read_to_string(path)?;
        let mut config = HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((k, v)) = line.split_once('=') {
                config.insert(k.trim().to_string(), v.trim().to_string());
            }
        }

        Ok(config)
    }

    fn write_config(&self, path: &PathBuf, config: &HashMap<String, String>) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut content = String::new();
        for (key, value) in config {
            content.push_str(&format!("{}={}\n", key, value));
        }

        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_user_name(&self) -> String {
        self.get("user.name")
            .ok()
            .flatten()
            .unwrap_or_else(whoami::username)
    }

    pub fn get_user_email(&self) -> String {
        self.get("user.email")
            .ok()
            .flatten()
            .unwrap_or_else(|| "user@example.com".to_string())
    }
}
