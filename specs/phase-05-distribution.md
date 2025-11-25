# Phase 5: Distribution & Tooling

**Goal:** Cross-platform builds, releases, and installation methods

**Timeline:** Week 6

**Milestone:** v1.0 release ready for distribution

---

## 5.1 Build Optimization

- [ ] Configure release profile in Cargo.toml:
  - Optimize for size (`opt-level = "z"`)
  - Enable LTO (link-time optimization)
  - Strip symbols
  - Single codegen unit
- [ ] Test release build on all platforms
- [ ] Verify binary size acceptable

---

## 5.2 Cross-Platform Compilation

### Target Platforms
- [ ] macOS (Apple Silicon): `aarch64-apple-darwin`
- [ ] macOS (Intel): `x86_64-apple-darwin`
- [ ] Linux (x86_64): `x86_64-unknown-linux-gnu`
- [ ] Linux (ARM64): `aarch64-unknown-linux-gnu`
- [ ] Windows (x86_64): `x86_64-pc-windows-gnu`

### Build Process
- [ ] Install all rustup targets
- [ ] Create build script for all platforms
- [ ] Test each binary on target platform
- [ ] Package binaries (tar.gz for Unix, zip for Windows)

---

## 5.3 GitHub Actions CI/CD

- [ ] Create `.github/workflows/release.yml`
- [ ] Trigger on version tags (`v*`)
- [ ] Build matrix for all platforms
- [ ] Run tests before building
- [ ] Upload artifacts
- [ ] Create GitHub release automatically
- [ ] Attach binaries to release

---

## 5.4 Installation Methods

### Method 1: Cargo
- [ ] Publish to crates.io
- [ ] Test: `cargo install ytdl`

### Method 2: Homebrew (macOS/Linux)
- [ ] Create Homebrew formula
- [ ] Create tap repository
- [ ] Test: `brew install ytdl`
- [ ] Add `yt-dlp` and `ffmpeg` as dependencies

### Method 3: Direct Download
- [ ] Provide install scripts
- [ ] Document manual installation
- [ ] Provide checksums (SHA256)

---

## 5.5 Shell Completions

- [ ] Add `clap_complete` dependency
- [ ] Generate completions for:
  - Bash
  - Zsh
  - Fish
- [ ] Provide via `ytdl --completions <shell>`
- [ ] Document installation in README

---

## 5.6 Documentation

### README.md
- [ ] Project description
- [ ] Features list
- [ ] Installation instructions (all methods)
- [ ] Usage examples
- [ ] Configuration guide
- [ ] Requirements (yt-dlp, ffmpeg)
- [ ] License
- [ ] Contributing guidelines

### CHANGELOG.md
- [ ] Document v1.0.0 features
- [ ] Breaking changes (if any)
- [ ] Known issues
- [ ] Future plans

### User Guide (optional)
- [ ] Advanced usage
- [ ] Configuration options
- [ ] Troubleshooting
- [ ] FAQ

---

## 5.7 Release Process

### Pre-Release Checklist
- [ ] All tests passing
- [ ] Documentation complete
- [ ] CHANGELOG updated
- [ ] Version bumped in Cargo.toml
- [ ] No critical bugs
- [ ] Performance acceptable

### Release Steps
1. [ ] Update version in Cargo.toml
2. [ ] Update CHANGELOG.md
3. [ ] Commit changes
4. [ ] Create git tag: `git tag -a v1.0.0 -m "Release v1.0.0"`
5. [ ] Push tag: `git push origin v1.0.0`
6. [ ] GitHub Actions builds and releases automatically
7. [ ] Publish to crates.io: `cargo publish`
8. [ ] Update Homebrew formula
9. [ ] Announce release

### Post-Release
- [ ] Monitor for issues
- [ ] Respond to feedback
- [ ] Plan v1.1 features

---

## 5.8 Versioning

- Follow [Semantic Versioning](https://semver.org/)
- MAJOR.MINOR.PATCH (e.g., 1.0.0)
- Breaking changes → MAJOR
- New features → MINOR
- Bug fixes → PATCH

---

## Testing

- [ ] Test builds on all platforms
- [ ] Test all installation methods
- [ ] Verify shell completions work
- [ ] Documentation complete and accurate
- [ ] Release process dry-run

---

## Success Criteria

- [ ] Builds successfully on all target platforms
- [ ] CI/CD pipeline working
- [ ] Multiple installation methods available
- [ ] Shell completions generated
- [ ] Complete documentation
- [ ] v1.0 released to GitHub
- [ ] Published to crates.io
- [ ] Homebrew formula working

---

## Deliverables

- [ ] Binaries for all platforms
- [ ] GitHub release with artifacts
- [ ] crates.io package
- [ ] Homebrew formula
- [ ] Shell completions
- [ ] Complete documentation
- [ ] Release announcement

---

## Project Complete

Congratulations! v1.0 is ready for users.
