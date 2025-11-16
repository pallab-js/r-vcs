# VCS - Version Control System

A production-ready version control system similar to Git, built in Rust.

**Status**: ✅ Production-ready for small teams (2-5 developers)

## Quick Links

- 📖 **Installation** - See below for installation instructions
- 🚀 **[Features](#features)** - What VCS can do
- 📝 **[Usage](#usage)** - How to use VCS

---

## Features

- **Repository initialization**: Create a new VCS repository
- **Staging area**: Add files to the staging area before committing
- **Commits**: Create commits with messages to save snapshots of your project
- **Object storage**: Uses SHA-1 hashing to store blobs, trees, and commits
- **Enhanced status**: Shows staged, modified, untracked, and deleted files with color coding
- **Enhanced log**: One-line format, limit commits, color-coded output
- **Ignore files**: Support for `.vcsignore` patterns
- **Configuration**: Set user name and email (repo or global)
- **Reset**: Unstage files from the index
- **Production-ready**: Cross-platform, atomic operations, file locking
- **File permissions**: Preserved on Unix systems
- **Line ending normalization**: CRLF to LF for cross-platform consistency

---

## Installation

### Method 1: Install from crates.io (Easiest)

**Once published**, install with:

```bash
cargo install vcs
```

This will:
- Download and compile from crates.io
- Install to `~/.cargo/bin/vcs` (Unix) or `%USERPROFILE%\.cargo\bin\vcs.exe` (Windows)
- Make it available in your PATH (if Cargo bin is in PATH)

**Verify:**
```bash
vcs --help
```

### Method 2: Build from Source

**Prerequisites**: Rust 1.70+ ([install Rust](https://rustup.rs))

```bash
# Clone the repository
git clone https://github.com/pallab-js/vcs.git
cd vcs

# Build
cargo build --release

# Install (Linux/macOS)
sudo cp target/release/vcs /usr/local/bin/vcs

# Or install to user directory (no sudo)
mkdir -p ~/.local/bin
cp target/release/vcs ~/.local/bin/vcs
export PATH="$HOME/.local/bin:$PATH"  # Add to ~/.bashrc or ~/.zshrc
```

**Windows:**
```powershell
# Build
cargo build --release

# Install
copy target\release\vcs.exe "C:\Program Files\VCS\vcs.exe"
# Add C:\Program Files\VCS to PATH via System Properties
```

**Verify:**
```bash
vcs --help
```

📖 **Installation instructions are provided below**

---

## Usage

### Initialize a repository

```bash
vcs init
```

This creates a `.vcs` directory in the current folder with the repository structure.

### Add files to staging

```bash
vcs add file.txt
vcs add src/
vcs add .
```

Adds files or directories to the staging area. Files are stored as blobs in the object database.

### Commit changes

```bash
vcs commit -m "Initial commit"
vcs commit -m "Add new feature"
```

Creates a commit with the staged files. Each commit includes:
- A tree object pointing to the files
- Author information (from config)
- Timestamp
- Commit message
- Parent commit (if any)

### View status

```bash
vcs status
```

Shows comprehensive repository status:
- **Changes to be committed**: Files staged for commit (green)
- **Changes not staged for commit**: Modified files not yet staged (yellow)
- **Untracked files**: New files not yet added
- **Deleted files**: Files removed from working directory (red)

Color-coded output for better readability.

### View commit history

```bash
vcs log
vcs log --oneline    # One-line format
vcs log -n 10        # Show last 10 commits
```

Displays the commit history with color-coded output. Use `--oneline` for compact format or `-n` to limit the number of commits shown.

### Inspect objects

```bash
vcs cat-file <hash>
```

Shows the contents of a repository object (blob, tree, or commit).

### Configure settings

```bash
vcs config user.name "Your Name"
vcs config user.email "your@email.com"
vcs config --global user.name "Your Name"  # Global config
vcs config --list                           # List all settings
```

Configure user name and email for commits. Settings are stored in `.vcs/config` (repo) or `~/.vcsconfig` (global).

### Unstage files

```bash
vcs reset <file>     # Unstage specific file
vcs reset            # Unstage all files
```

Remove files from the staging area without losing changes.

### Ignore files

Create a `.vcsignore` file in your repository root to exclude files:

```
target/
*.log
.env
.DS_Store
```

Supports glob patterns. The `.vcs` directory is always ignored.

---

## Example Workflow

```bash
# Initialize repository
vcs init

# Configure your identity
vcs config user.name "John Doe"
vcs config user.email "john@example.com"

# Create some files
echo "Hello, World!" > hello.txt
echo "fn main() {}" > main.rs

# Stage files
vcs add hello.txt main.rs

# Commit
vcs commit -m "Initial commit with hello and main"

# Make changes
echo "Updated content" >> hello.txt

# Stage and commit again
vcs add hello.txt
vcs commit -m "Update hello.txt"

# View history
vcs log --oneline
```

---

## Architecture

The VCS stores data in the `.vcs` directory:

```
.vcs/
├── objects/          # Object database (blobs, trees, commits)
│   └── <hash>/       # Objects stored by hash (first 2 chars as directory)
├── refs/
│   └── heads/        # Branch references
├── HEAD              # Points to current branch/commit
├── index             # Staging area (JSON format)
├── config            # Repository configuration
└── index.lock        # Lock file (prevents concurrent access)
```

### Object Types

1. **Blob**: Stores file contents
2. **Tree**: Stores directory structure (references to blobs and other trees)
3. **Commit**: Stores commit metadata and references to a tree

All objects are stored with SHA-1 hashing for content-addressable storage.

---

## Recent Enhancements

- ✅ Enhanced status with color coding
- ✅ `.vcsignore` support for ignoring files
- ✅ Configuration system (user name/email)
- ✅ Enhanced log with options (`--oneline`, `-n`)
- ✅ Reset command to unstage files
- ✅ Colored terminal output
- ✅ **Production-ready**: Cross-platform, atomic operations, file locking
- ✅ **File permissions**: Preserved on Unix systems
- ✅ **Line ending normalization**: CRLF to LF for cross-platform consistency

---

## Platform Support

- ✅ **Linux** (x86_64, ARM64)
- ✅ **macOS** (Intel, Apple Silicon)
- ✅ **Windows** (10+, x86_64)

---

## Limitations

This is a simplified VCS implementation. It does not include:
- Branching and merging
- Remote repositories
- Conflict resolution
- Advanced diff operations
- Tagging system

---

## License

MIT License - see [LICENSE](LICENSE) file for details.

---

## Contributing

Contributions are welcome! Please feel free to:

- Report bugs and issues
- Suggest new features
- Submit pull requests
- Improve documentation

---

## Status

**Version**: 0.2.0  
**Status**: Production-ready for small teams (2-5 developers)  
**Platforms**: Linux, macOS, Windows

---

## Acknowledgments

Built with Rust and inspired by Git. Thanks to all contributors and the Rust community.

---

## Links

- **Issues**: Report bugs and request features
- **Discussions**: Ask questions and share ideas
- **Releases**: See version history and downloads
