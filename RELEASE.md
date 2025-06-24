# Release Guide

This document outlines the process for releasing new versions of unicode-rs.

## Pre-Release Checklist

### 1. Code Quality
- [ ] All tests pass: `cargo test --all-targets`
- [ ] No clippy warnings: `cargo clippy --all-targets`
- [ ] Documentation builds: `cargo doc --no-deps`
- [ ] All examples run successfully
- [ ] Security tests pass: `cargo test security`

### 2. Documentation
- [ ] Update CHANGELOG.md with new features and changes
- [ ] Verify README.md is up to date
- [ ] Check that all public APIs have rustdoc comments
- [ ] Ensure examples are working and documented

### 3. Version Management
- [ ] Update version in `Cargo.toml`
- [ ] Update version references in README.md if needed
- [ ] Tag follows semantic versioning (MAJOR.MINOR.PATCH)

### 4. Security Review
- [ ] Review any new Unicode character additions
- [ ] Verify security module detects latest threat vectors
- [ ] Test with known attack vectors
- [ ] Validate sanitization functions

## Release Process

### Step 1: Prepare the Release

1. **Update Version Numbers**
   ```bash
   # Update Cargo.toml version
   sed -i 's/version = "0.1.0"/version = "0.2.0"/' Cargo.toml
   ```

2. **Update CHANGELOG.md**
   ```markdown
   ## [0.2.0] - 2024-XX-XX
   
   ### Added
   - New feature descriptions
   
   ### Changed
   - Modified functionality
   
   ### Fixed
   - Bug fixes
   
   ### Security
   - Security improvements
   ```

3. **Run Full Test Suite**
   ```bash
   cargo test --all-targets
   cargo clippy --all-targets -- -D warnings
   cargo doc --no-deps
   ```

### Step 2: Create Release Commit

```bash
git add .
git commit -m "Release v0.2.0

- Add new features
- Improve security detection
- Update documentation"
```

### Step 3: Create and Push Tag

```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin main
git push origin v0.2.0
```

### Step 4: Publish to Crates.io

```bash
# Dry run first
cargo publish --dry-run

# If everything looks good
cargo publish
```

### Step 5: Create GitHub Release

1. Go to GitHub repository
2. Click "Releases" → "Create a new release"
3. Select the tag you just created
4. Use this template:

```markdown
# unicode-rs v0.2.0

## 🎉 What's New

### ✨ New Features
- Feature 1 description
- Feature 2 description

### 🔒 Security Improvements
- Enhanced Unicode threat detection
- New attack vector coverage

### 🐛 Bug Fixes
- Fix 1 description
- Fix 2 description

### 📚 Documentation
- Improved examples
- Better API documentation

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
unicode-rs = "0.2.0"
```

## 🔗 Links

- [Documentation](https://docs.rs/unicode-rs/0.2.0)
- [Changelog](CHANGELOG.md)
- [Examples](examples/)

## 🙏 Contributors

Thanks to all contributors who made this release possible!
```

## Post-Release Tasks

### 1. Verify Release
- [ ] Check crates.io page updates correctly
- [ ] Verify docs.rs builds documentation
- [ ] Test installation: `cargo install unicode-rs`
- [ ] Confirm badges update in README

### 2. Announce Release
- [ ] Update any dependent projects
- [ ] Share on social media if appropriate
- [ ] Update any blog posts or documentation

### 3. Monitor
- [ ] Watch for any immediate issues
- [ ] Monitor download statistics
- [ ] Check for user feedback

## Hotfix Process

For critical security fixes or major bugs:

1. **Create hotfix branch**
   ```bash
   git checkout -b hotfix/v0.1.1
   ```

2. **Make minimal fix**
   - Only fix the critical issue
   - Update version to patch level (0.1.0 → 0.1.1)

3. **Fast-track release**
   ```bash
   cargo test
   git commit -m "Hotfix v0.1.1: Fix critical security issue"
   git tag v0.1.1
   git push origin hotfix/v0.1.1
   git push origin v0.1.1
   cargo publish
   ```

4. **Merge back to main**
   ```bash
   git checkout main
   git merge hotfix/v0.1.1
   git push origin main
   ```

## Version Strategy

### Semantic Versioning

- **MAJOR** (1.0.0): Breaking API changes
- **MINOR** (0.1.0): New features, backward compatible
- **PATCH** (0.0.1): Bug fixes, backward compatible

### Pre-release Versions

For testing major changes:
- **Alpha**: `0.2.0-alpha.1` - Early development
- **Beta**: `0.2.0-beta.1` - Feature complete, testing
- **RC**: `0.2.0-rc.1` - Release candidate

## Security Release Protocol

For security-related releases:

1. **Coordinate disclosure** if vulnerability was reported
2. **Prepare fix** in private branch
3. **Security advisory** on GitHub
4. **Expedited release** process
5. **Notification** to users via multiple channels

## Rollback Procedure

If a release has critical issues:

1. **Yank from crates.io** (if necessary)
   ```bash
   cargo yank --vers 0.2.0
   ```

2. **Create fixed version**
   ```bash
   # Fix issues and release 0.2.1
   cargo publish
   ```

3. **Communicate** the issue and resolution

## Release Automation

Consider setting up GitHub Actions for:
- Automated testing on tag creation
- Automatic crates.io publishing
- Release note generation
- Documentation deployment

## Contact

For release-related questions:
- Create an issue on GitHub
- Contact maintainers directly

---

**Remember**: Every release represents our commitment to quality and security. Take time to ensure each release meets our high standards.
