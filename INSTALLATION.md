# Installation and Usage Guide

This guide explains how to install and use VCS on your system.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation Methods](#installation-methods)
3. [Post-Installation Setup](#post-installation-setup)
4. [Basic Usage](#basic-usage)
5. [Advanced Usage](#advanced-usage)
6. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required
- **Rust** (version 1.70 or later)
  - Install from: https://www.rust-lang.org/tools/install
  - Verify: `rustc --version`

### Optional
- **Git** (for cloning the repository)
- **Cargo** (comes with Rust)

---

## Installation Methods

### Method 1: Build from Source (Recommended)

This is the most flexible method and works on all platforms.

#### Step 1: Get the Source Code

**Option A: Clone from Git Repository**
```bash
git clone https://github.com/YOUR_USERNAME/vcs.git
cd vcs
```

**Option B: Download ZIP**
1. Download the repository as ZIP
2. Extract it
3. Navigate to the extracted directory

#### Step 2: Build the Project

```bash
# Build in release mode (optimized)
cargo build --release
```

This will:
- Download and compile all dependencies
- Build the optimized binary
- Place it in `target/release/vcs` (or `target\release\vcs.exe` on Windows)

**Build time**: 2-5 minutes (first time), 10-30 seconds (subsequent builds)

#### Step 3: Install the Binary

**Linux/macOS:**
```bash
# Option 1: Copy to a directory in your PATH
sudo cp target/release/vcs /usr/local/bin/vcs

# Option 2: Add to your local bin directory
mkdir -p ~/.local/bin
cp target/release/vcs ~/.local/bin/vcs

# Add to PATH (add to ~/.bashrc, ~/.zshrc, or ~/.profile)
export PATH="$HOME/.local/bin:$PATH"
```

**Windows:**
```powershell
# Option 1: Copy to a directory in your PATH
copy target\release\vcs.exe C:\Program Files\VCS\vcs.exe

# Option 2: Add to your user directory
mkdir %USERPROFILE%\bin
copy target\release\vcs.exe %USERPROFILE%\bin\vcs.exe

# Add to PATH (via System Properties > Environment Variables)
# Add: %USERPROFILE%\bin
```

**Verify Installation:**
```bash
vcs --help
```

---

### Method 2: Using Cargo Install (If Published)

If the project is published to crates.io:

```bash
cargo install vcs
```

This will:
- Download and compile from crates.io
- Install to `~/.cargo/bin/vcs` (Unix) or `%USERPROFILE%\.cargo\bin\vcs.exe` (Windows)
- Add to PATH automatically (if Cargo bin is in PATH)

**Note**: Make sure `~/.cargo/bin` (or `%USERPROFILE%\.cargo\bin`) is in your PATH.

---

### Method 3: Pre-built Binaries (If Available)

If pre-built binaries are provided:

**Linux:**
```bash
# Download the binary
wget https://github.com/YOUR_USERNAME/vcs/releases/latest/download/vcs-linux-x86_64

# Make it executable
chmod +x vcs-linux-x86_64

# Move to PATH
sudo mv vcs-linux-x86_64 /usr/local/bin/vcs
```

**macOS:**
```bash
# Download the binary
curl -LO https://github.com/yourusername/vcs/releases/latest/download/vcs-macos-x86_64

# Make it executable
chmod +x vcs-macos-x86_64

# Move to PATH
sudo mv vcs-macos-x86_64 /usr/local/bin/vcs
```

**Windows:**
1. Download `vcs-windows-x86_64.exe` from releases
2. Rename to `vcs.exe`
3. Place in a directory in your PATH (e.g., `C:\Program Files\VCS\`)
4. Or add the directory to your PATH

---

## Post-Installation Setup

### 1. Configure Your Identity

Set your name and email for commits:

```bash
# Repository-specific (recommended for projects)
vcs config user.name "Your Name"
vcs config user.email "your.email@example.com"

# Or set globally (for all repositories)
vcs config --global user.name "Your Name"
vcs config --global user.email "your.email@example.com"
```

**Verify:**
```bash
vcs config --list
```

### 2. Verify Installation

```bash
# Check version (if implemented)
vcs --version

# See all available commands
vcs --help

# Test basic functionality
mkdir test-vcs
cd test-vcs
vcs init
echo "Hello, VCS!" > hello.txt
vcs add hello.txt
vcs commit -m "Test commit"
vcs log
```

---

## Basic Usage

### Your First Repository

```bash
# 1. Navigate to your project directory
cd ~/my-project

# 2. Initialize a VCS repository
vcs init

# 3. Create some files
echo "Hello, World!" > hello.txt
echo "fn main() {}" > main.rs

# 4. Stage files
vcs add hello.txt main.rs

# 5. Check what's staged
vcs status

# 6. Commit your changes
vcs commit -m "Initial commit"

# 7. View commit history
vcs log
```

### Daily Workflow

```bash
# Make changes to files
vim file.txt

# Check status
vcs status

# Stage modified files
vcs add file.txt

# Or stage all changes
vcs add .

# Commit
vcs commit -m "Update file.txt"

# View history
vcs log --oneline
```

### Ignoring Files

Create a `.vcsignore` file in your repository root:

```bash
# .vcsignore
target/
*.log
.env
.DS_Store
node_modules/
*.tmp
```

Files matching these patterns will be automatically ignored.

---

## Advanced Usage

### Enhanced Status

```bash
# See detailed status with colors
vcs status

# Output shows:
# - Changes to be committed (staged, green)
# - Changes not staged (modified, yellow)
# - Untracked files
# - Deleted files (red)
```

### Enhanced Log

```bash
# One-line format
vcs log --oneline

# Limit number of commits
vcs log -n 10

# Full format (default)
vcs log
```

### Unstaging Files

```bash
# Unstage a specific file
vcs reset file.txt

# Unstage all files
vcs reset
```

### Inspecting Objects

```bash
# View a commit
vcs cat-file <commit-hash>

# View a tree
vcs cat-file <tree-hash>

# View a file (blob)
vcs cat-file <blob-hash>
```

---

## Platform-Specific Notes

### Linux

**Installation:**
- Works on all major distributions
- May need to install `build-essential` or equivalent for compilation
- Binary works on x86_64 and ARM64

**Permissions:**
- File permissions (executable bits) are preserved
- Scripts maintain executable permissions after checkout

### macOS

**Installation:**
- Works on Intel and Apple Silicon (M1/M2/M3)
- May need Xcode Command Line Tools: `xcode-select --install`
- Binary works on both architectures

**Permissions:**
- File permissions preserved
- Case-insensitive filesystem by default (see limitations)

### Windows

**Installation:**
- Requires Rust toolchain (install from rustup.rs)
- May need Visual Studio Build Tools or MSVC
- Binary works on x86_64 Windows

**Configuration:**
- Global config stored in `%USERPROFILE%\.vcsconfig`
- Repository config in `.vcs\config`

**Line Endings:**
- CRLF files are normalized to LF on add
- Prevents false "modified" files when collaborating

**Limitations:**
- File permissions not fully preserved (Windows limitation)
- Case-insensitive filesystem (see known issues)

---

## Common Workflows

### Starting a New Project

```bash
# 1. Create project directory
mkdir my-new-project
cd my-new-project

# 2. Initialize VCS
vcs init

# 3. Configure (optional but recommended)
vcs config user.name "Your Name"
vcs config user.email "your@email.com"

# 4. Create initial files
# ... create your files ...

# 5. Stage and commit
vcs add .
vcs commit -m "Initial project setup"
```

### Working with Existing Project

```bash
# 1. Navigate to project
cd existing-project

# 2. Check status
vcs status

# 3. Make changes
# ... edit files ...

# 4. Stage changes
vcs add <modified-files>

# 5. Commit
vcs commit -m "Description of changes"

# 6. View history
vcs log --oneline
```

### Undoing Changes

```bash
# Unstage a file (keep changes)
vcs reset file.txt

# Check what changed
vcs status
```

---

## Troubleshooting

### "Command not found: vcs"

**Problem**: The binary is not in your PATH.

**Solution**:
```bash
# Find where you installed it
which vcs  # or: where vcs (Windows)

# Add to PATH:
# Linux/macOS: Add to ~/.bashrc, ~/.zshrc, or ~/.profile
export PATH="$HOME/.local/bin:$PATH"

# Windows: Add via System Properties > Environment Variables
```

### "Not a VCS repository"

**Problem**: You're not in a VCS repository directory.

**Solution**:
```bash
# Initialize a repository
vcs init

# Or navigate to an existing repository
cd /path/to/repository
```

### "Repository is locked"

**Problem**: Another VCS process is running or a previous process crashed.

**Solution**:
```bash
# Remove the lock file
rm .vcs/index.lock

# If on Windows:
del .vcs\index.lock
```

### Build Errors

**Problem**: Compilation fails.

**Solutions**:
1. **Update Rust**: `rustup update`
2. **Clean build**: `cargo clean && cargo build --release`
3. **Check Rust version**: `rustc --version` (need 1.70+)

### Permission Denied (Linux/macOS)

**Problem**: Can't write to installation directory.

**Solution**:
```bash
# Use sudo for system-wide installation
sudo cp target/release/vcs /usr/local/bin/vcs

# Or install to user directory (no sudo needed)
cp target/release/vcs ~/.local/bin/vcs
```

### Windows: "Unable to find vcs.exe"

**Problem**: Binary not found or PATH not updated.

**Solution**:
1. Check if file exists: `dir C:\path\to\vcs.exe`
2. Add directory to PATH via System Properties
3. Restart terminal/command prompt

---

## Quick Reference

### Essential Commands

```bash
vcs init                    # Initialize repository
vcs add <files>             # Stage files
vcs commit -m "message"     # Commit changes
vcs status                  # Check status
vcs log                     # View history
vcs log --oneline           # Compact history
vcs reset <file>            # Unstage file
vcs config user.name "Name" # Set user name
vcs config user.email "..." # Set user email
```

### Getting Help

```bash
vcs --help                  # General help
vcs <command> --help       # Command-specific help
```

---

## Next Steps

1. **Read the README**: `README.md` for detailed documentation
2. **Check Enhancements**: `ENHANCEMENTS.md` for upcoming features
3. **Review Examples**: Try the example workflows above
4. **Explore Advanced Features**: See `README.md` for all commands

---

## Support

- **Issues**: Report bugs on GitHub Issues
- **Documentation**: See `README.md` and other `.md` files
- **Contributing**: See `CONTRIBUTING.md`

---

## System Requirements

### Minimum Requirements
- **RAM**: 512 MB free
- **Disk**: 100 MB for installation, varies for repositories
- **OS**: 
  - Linux (x86_64, ARM64)
  - macOS (10.15+, Intel/Apple Silicon)
  - Windows (10+, x86_64)

### Recommended
- **RAM**: 1 GB+ free
- **Disk**: 500 MB+ for larger repositories
- **Network**: For cloning from remote repositories (future feature)

---

## Uninstallation

### Remove Binary

**Linux/macOS:**
```bash
# If installed to /usr/local/bin
sudo rm /usr/local/bin/vcs

# If installed to ~/.local/bin
rm ~/.local/bin/vcs
```

**Windows:**
```powershell
# Remove from installation directory
del C:\Program Files\VCS\vcs.exe
# Or
del %USERPROFILE%\bin\vcs.exe
```

### Remove Configuration

**Linux/macOS:**
```bash
rm ~/.vcsconfig
```

**Windows:**
```powershell
del %USERPROFILE%\.vcsconfig
```

### Remove Cargo Installation

```bash
cargo uninstall vcs
```

---

## Examples by Use Case

### Personal Projects

```bash
# Quick setup
cd ~/projects/my-app
vcs init
vcs config user.name "John Doe"
vcs config user.email "john@example.com"
vcs add .
vcs commit -m "Initial commit"
```

### Team Projects

```bash
# Each team member:
cd ~/team-project
vcs init  # Or clone if shared
vcs config user.name "Your Name"
vcs config user.email "your@company.com"

# Work on features
vcs add feature-file.txt
vcs commit -m "Add new feature"
vcs log --oneline
```

### Scripts and Configs

```bash
# Track your dotfiles
cd ~/.config
vcs init
vcs add .bashrc .vimrc .gitconfig
vcs commit -m "Backup dotfiles"
```

---

That's it! You're ready to use VCS. Start with `vcs init` in your project directory and begin tracking your files.
