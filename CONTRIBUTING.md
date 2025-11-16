# Contributing to VCS

Thank you for your interest in contributing to VCS!

## Development Setup

1. **Fork and clone the repository**:
   ```bash
   git clone https://github.com/pallab-js/vcs.git
   cd vcs
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run tests** (when available):
   ```bash
   cargo test
   ```

4. **Format code**:
   ```bash
   cargo fmt
   ```

5. **Check for issues**:
   ```bash
   cargo clippy
   ```

## Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy: `cargo clippy`
- Ensure code compiles without warnings

## Pull Request Process

1. Create a feature branch
2. Make your changes
3. Test thoroughly
4. Update documentation if needed
5. Submit a pull request

## Areas for Contribution

- Bug fixes
- Performance improvements
- New features (see `ENHANCEMENTS.md`)
- Documentation
- Tests

## Questions?

Open an issue for discussion before starting major work.
