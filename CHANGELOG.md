# Changelog

All notable changes to VCS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-11-09

### Added
- Enhanced status command with color-coded output showing staged, modified, untracked, and deleted files
- `.vcsignore` support for ignoring files and directories
- Configuration system for user name and email (repository and global)
- Enhanced log command with `--oneline` and `-n` options
- Reset command to unstage files from index
- File locking mechanism to prevent concurrent access corruption
- Atomic file operations for index, HEAD, and refs
- File permissions preservation on Unix systems
- Line ending normalization (CRLF to LF) for cross-platform consistency
- Cross-platform home directory support using `dirs` crate
- Colored terminal output for better user experience

### Changed
- Improved error handling - replaced all `unwrap()` calls with proper error handling
- Enhanced status output to show comprehensive file states
- Log output now supports one-line format and commit limiting
- Path normalization for consistent cross-platform behavior

### Fixed
- Status showing files as untracked after commit
- Files showing as both modified and untracked
- Path normalization issues causing status bugs
- Windows compatibility (HOME directory handling)
- Data integrity issues (atomic writes, file locking)

### Security
- Added file locking to prevent race conditions
- Atomic writes prevent data corruption
- Improved input validation

## [0.1.0] - 2025-11-09

### Added
- Initial release
- Repository initialization
- File staging and committing
- Basic status and log commands
- Object storage with SHA-1 hashing
- Commit history tracking
- Cat-file command for object inspection

[0.2.0]: https://github.com/YOUR_USERNAME/vcs/releases/tag/v0.2.0
[0.1.0]: https://github.com/YOUR_USERNAME/vcs/releases/tag/v0.1.0
