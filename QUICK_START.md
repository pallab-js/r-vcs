# Quick Start Guide

Get up and running with VCS in 5 minutes!

## Installation (One-Time Setup)

### Step 1: Install Rust

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
1. Download and run: https://rustup.rs/
2. Follow the installer
3. Restart your terminal

**Verify:**
```bash
rustc --version
# Should show: rustc 1.70.0 or later
```

### Step 2: Get VCS

**Option A: Clone (if you have git)**
```bash
git clone https://github.com/pallab-js/vcs.git
cd vcs
```

**Option B: Download ZIP**
1. Download repository as ZIP
2. Extract it
3. Open terminal in extracted folder

### Step 3: Build and Install

```bash
# Build
cargo build --release

# Install (Linux/macOS)
sudo cp target/release/vcs /usr/local/bin/vcs

# Or install to user directory (no sudo needed)
mkdir -p ~/.local/bin
cp target/release/vcs ~/.local/bin/vcs
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
source ~/.bashrc  # or restart terminal
```

**Windows:**
```powershell
# Build
cargo build --release

# Copy to a directory in PATH
mkdir C:\Program Files\VCS
copy target\release\vcs.exe "C:\Program Files\VCS\vcs.exe"

# Add to PATH via:
# System Properties > Environment Variables > Path > Edit > New
# Add: C:\Program Files\VCS
```

### Step 4: Verify

```bash
vcs --help
```

You should see the help message!

---

## Your First Repository (2 Minutes)

```bash
# 1. Create a test project
mkdir my-first-repo
cd my-first-repo

# 2. Initialize VCS
vcs init

# 3. Configure your identity
vcs config user.name "Your Name"
vcs config user.email "your@email.com"

# 4. Create a file
echo "Hello, VCS!" > hello.txt

# 5. Stage it
vcs add hello.txt

# 6. Commit it
vcs commit -m "My first commit"

# 7. View history
vcs log
```

**Congratulations!** You've created your first VCS repository! 🎉

---

## Daily Usage

### Basic Workflow

```bash
# Make changes to files
vim file.txt

# Check what changed
vcs status

# Stage changes
vcs add file.txt

# Commit
vcs commit -m "Update file.txt"

# View history
vcs log --oneline
```

### Common Commands

| Command | What it does |
|---------|--------------|
| `vcs init` | Initialize a new repository |
| `vcs add <file>` | Stage files for commit |
| `vcs commit -m "msg"` | Commit staged changes |
| `vcs status` | Show repository status |
| `vcs log` | View commit history |
| `vcs log --oneline` | Compact history view |
| `vcs reset <file>` | Unstage a file |
| `vcs config user.name "Name"` | Set your name |

---

## Next Steps

1. **Read the full guide**: [INSTALLATION.md](INSTALLATION.md)
2. **Explore features**: [README.md](README.md)
3. **See examples**: Try the workflows above
4. **Get help**: `vcs --help` or `vcs <command> --help`

---

## Troubleshooting

**"Command not found"**
- Make sure the binary is in your PATH
- Restart your terminal after adding to PATH

**"Not a VCS repository"**
- Run `vcs init` in your project directory

**Build errors**
- Update Rust: `rustup update`
- Check Rust version: `rustc --version` (need 1.70+)

For more help, see [INSTALLATION.md](INSTALLATION.md#troubleshooting)

---

That's it! You're ready to use VCS. Happy version controlling! 🚀
