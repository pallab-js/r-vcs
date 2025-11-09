# Publishing to crates.io

This guide explains how to publish VCS to crates.io so users can install it with `cargo install vcs`.

## Prerequisites

1. **Crates.io Account**
   - Sign up at: https://crates.io
   - Verify your email address
   - Get your API token

2. **Cargo Login**
   - Login to crates.io from command line

3. **Package Requirements**
   - Valid `Cargo.toml` with required metadata
   - Unique crate name (check availability)
   - Proper version number
   - Valid README
   - License specified

---

## Step 1: Prepare Your Package

### Check Crate Name Availability

The name `vcs` might be taken. Check availability:

```bash
# Visit: https://crates.io/crates/vcs
# Or use cargo search
cargo search vcs
```

**If `vcs` is taken, consider alternatives:**
- `vcs-rs`
- `simple-vcs`
- `rust-vcs`
- `vcs-tool`
- `my-vcs` (your username prefix)

### Update Cargo.toml

Ensure your `Cargo.toml` has all required fields:

```toml
[package]
name = "vcs"  # Or your chosen name
version = "0.2.0"
edition = "2021"
description = "A production-ready version control system similar to Git"
authors = ["Your Name <your@email.com>"]  # Required!
license = "MIT"  # Required!
repository = "https://github.com/yourusername/vcs"
readme = "README.md"
keywords = ["vcs", "version-control", "git", "scm"]
categories = ["development-tools", "command-line-utilities"]
```

**Required fields:**
- ✅ `name` - Crate name
- ✅ `version` - Semantic version
- ✅ `authors` - At least one author with email
- ✅ `license` - License identifier (or `license-file`)
- ✅ `description` - Brief description

**Recommended fields:**
- `repository` - Source code URL
- `readme` - Path to README
- `keywords` - Search keywords
- `categories` - Crate categories

### Verify Package

```bash
# Check for common issues
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy

# Build to ensure it compiles
cargo build --release

# Test (if you have tests)
cargo test
```

### Prepare Documentation

Ensure you have:
- ✅ `README.md` - Will be displayed on crates.io
- ✅ `LICENSE` - License file
- ✅ All documentation files are in place

---

## Step 2: Create Crates.io Account

1. **Sign up**: https://crates.io/users/register
2. **Verify email**: Check your inbox
3. **Get API token**: 
   - Go to: https://crates.io/me
   - Click "New Token"
   - Name it (e.g., "vcs-publish")
   - Copy the token (you'll only see it once!)

---

## Step 3: Login to Cargo

```bash
# Login with your API token
cargo login <your-api-token>

# Example:
cargo login abc123xyz789...
```

This stores your token in `~/.cargo/credentials` (Unix) or `%USERPROFILE%\.cargo\credentials` (Windows).

---

## Step 4: Package Verification

Before publishing, verify your package:

```bash
# Create a package (dry-run, doesn't upload)
cargo package

# This will:
# - Check Cargo.toml
# - Create a .crate file in target/package/
# - Verify all files are included
# - Check for common issues
```

**Check the output** for any warnings or errors.

**Verify included files:**
```bash
# List what will be included
cargo package --list
```

Make sure:
- ✅ Source files are included
- ✅ README.md is included
- ✅ LICENSE is included
- ✅ `target/` is NOT included (should be in .gitignore)
- ✅ `.vcs/` directories are NOT included

---

## Step 5: Publish to Crates.io

```bash
# Publish the package
cargo publish

# This will:
# - Package your crate
# - Upload to crates.io
# - Make it available for installation
```

**First publish:**
- Takes a few minutes
- Creates the crate on crates.io
- Makes it searchable

**Subsequent publishes:**
- Must increment version number
- Faster (just updates)

---

## Step 6: Verify Publication

1. **Check crates.io**: https://crates.io/crates/vcs (or your crate name)
2. **Test installation**:
   ```bash
   cargo install vcs
   ```
3. **Verify it works**:
   ```bash
   vcs --help
   ```

---

## Updating Your Crate

### Version Bump

Follow [Semantic Versioning](https://semver.org/):
- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features, backward compatible
- **PATCH** (0.0.1): Bug fixes, backward compatible

**Update version in Cargo.toml:**
```toml
version = "0.2.1"  # Increment as needed
```

**Then publish:**
```bash
cargo publish
```

### Publishing Updates

```bash
# 1. Update version in Cargo.toml
# 2. Update CHANGELOG.md (if you have one)
# 3. Commit changes
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to 0.2.1"

# 4. Publish
cargo publish
```

---

## Important Notes

### Crate Name

- **Once published, the name is permanent**
- You cannot delete a crate (only yank it)
- Choose your name carefully
- Names are first-come-first-served

### Version Numbers

- **Cannot reuse version numbers**
- Each publish must have a new version
- Follow semantic versioning

### Yanking (Removing)

If you need to remove a version:

```bash
# Yank a version (marks as unusable, but doesn't delete)
cargo yank --version 0.2.0

# Un-yank (make it available again)
cargo yank --undo --version 0.2.0
```

**Note**: Yanking doesn't delete the crate, just marks it as unusable for new installations.

---

## Pre-Publishing Checklist

Before publishing, verify:

- [ ] Crate name is available and chosen
- [ ] `Cargo.toml` has all required fields
- [ ] `authors` field has at least one entry with email
- [ ] `license` is specified
- [ ] `description` is clear and concise
- [ ] `README.md` exists and is informative
- [ ] `LICENSE` file exists
- [ ] Code compiles: `cargo build --release`
- [ ] Code is formatted: `cargo fmt`
- [ ] No warnings: `cargo clippy`
- [ ] `.gitignore` excludes build artifacts
- [ ] Tested installation locally
- [ ] Documentation is complete

---

## Common Issues

### "crate name is already taken"

**Solution**: Choose a different name or add a suffix:
- `vcs-rs`
- `simple-vcs`
- `yourusername-vcs`

### "invalid license"

**Solution**: Use a valid SPDX license identifier:
- `MIT`
- `Apache-2.0`
- `MIT OR Apache-2.0`
- See: https://spdx.org/licenses/

### "missing field: authors"

**Solution**: Add authors to `Cargo.toml`:
```toml
authors = ["Your Name <your@email.com>"]
```

### "package contains path 'target'"

**Solution**: Ensure `target/` is in `.gitignore`:
```
/target
```

### "authentication failed"

**Solution**: 
- Re-login: `cargo login <token>`
- Check token is valid
- Verify email is confirmed on crates.io

---

## Post-Publishing

### Update README

Add installation instructions:

```markdown
## Installation

Install from crates.io:

```bash
cargo install vcs
```

Or build from source:
...
```

### Announcement

Consider announcing:
- Reddit: r/rust
- Rust user forums
- Your blog/social media
- GitHub releases

### Monitor

- Check download stats: https://crates.io/crates/vcs
- Monitor issues on GitHub
- Respond to user feedback

---

## Example: Complete Publishing Workflow

```bash
# 1. Check name availability
cargo search vcs

# 2. Update Cargo.toml with your chosen name
# Edit Cargo.toml: name = "vcs-rs" (if vcs is taken)

# 3. Verify package
cargo check
cargo fmt
cargo clippy
cargo build --release

# 4. Test package creation
cargo package

# 5. Login to crates.io
cargo login <your-api-token>

# 6. Publish
cargo publish

# 7. Verify
cargo install vcs-rs  # or your crate name
vcs-rs --help
```

---

## Publishing Checklist

### Before First Publish

- [ ] Create crates.io account
- [ ] Verify email
- [ ] Get API token
- [ ] Check crate name availability
- [ ] Update `Cargo.toml` with all required fields
- [ ] Add `authors` field with your email
- [ ] Ensure `LICENSE` file exists
- [ ] Ensure `README.md` exists
- [ ] Run `cargo package` successfully
- [ ] Test build: `cargo build --release`
- [ ] Format code: `cargo fmt`
- [ ] Check lints: `cargo clippy`

### Publishing

- [ ] Login: `cargo login <token>`
- [ ] Verify package: `cargo package`
- [ ] Publish: `cargo publish`
- [ ] Verify on crates.io
- [ ] Test installation: `cargo install <crate-name>`

### After Publishing

- [ ] Update README with installation instructions
- [ ] Create GitHub release
- [ ] Announce (optional)
- [ ] Monitor for issues

---

## Quick Reference

```bash
# Check if name is available
cargo search <name>

# Login
cargo login <api-token>

# Verify package
cargo package

# Publish
cargo publish

# Install your own crate
cargo install <crate-name>

# Update version and republish
# 1. Edit Cargo.toml: version = "0.2.1"
# 2. cargo publish

# Yank a version
cargo yank --version 0.2.0
```

---

## Resources

- **Crates.io**: https://crates.io
- **Cargo Book**: https://doc.rust-lang.org/cargo/
- **Publishing Guide**: https://doc.rust-lang.org/cargo/reference/publishing.html
- **Semantic Versioning**: https://semver.org/
- **SPDX Licenses**: https://spdx.org/licenses/

---

## Tips

1. **Start with a test crate**: Publish a small test crate first to learn the process
2. **Read the docs**: Check cargo publishing documentation
3. **Test locally**: Always test `cargo package` before publishing
4. **Version carefully**: Follow semantic versioning
5. **Document well**: Good README increases adoption
6. **Monitor feedback**: Check crates.io and GitHub for issues

---

Good luck with your publication! 🚀
