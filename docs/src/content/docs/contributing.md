---
title: Contributing
description: Development guidelines and contribution process
---

import { Tabs, TabItem, Aside, FileTree } from '@astrojs/starlight/components';

## Development Setup

### Prerequisites

- Rust 1.92+ ([Install via rustup](https://rustup.rs/))
- Git

### Building from Source

Clone the repository:

<Tabs>
  <TabItem label="HTTPS">

```bash
git clone https://github.com/alvgaona/ros2env.git
cd ros2env
```

  </TabItem>
  <TabItem label="SSH">

```bash
git clone git@github.com:alvgaona/ros2env.git
cd ros2env
```

  </TabItem>
</Tabs>

Build the project:

```bash
cargo build --release
```

The binary will be at `target/release/rosenv`.

## Testing

rosenv has 39 tests with ~54% line coverage and 78% function coverage.

### Run All Tests

```bash
cargo test
```

### Test Categories

- **Unit tests** (10 tests) - Core functions and helpers in `src/main.rs`
- **Edge case tests** (14 tests) - Filesystem operations in `tests/edge_cases.rs`
- **Integration tests** (15 tests) - CLI commands in `tests/integration_tests.rs`

### Run Specific Tests

```bash
# Run unit tests only
cargo test --bin rosenv

# Run integration tests only
cargo test --test integration_tests

# Run specific test
cargo test test_name
```

### Test with Output

```bash
cargo test -- --nocapture
```

### Generate Coverage Report

Requires `cargo-llvm-cov`:

```bash
cargo install cargo-llvm-cov
cargo llvm-cov --all-features --workspace
```

## Code Quality

### Formatting

All code must be formatted with rustfmt:

```bash
cargo fmt --all
```

### Linting

All code must pass clippy with no warnings:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

This runs in CI and will block PRs if it fails.

## Development Guidelines

### Code Style

1. **Naming Conventions**
   - Follow Rust standard naming (snake_case for functions, CamelCase for types)
   - Use descriptive names that indicate purpose

2. **Comments**
   - Use `//` for inline comments
   - Use `/* */` only for section headers
   - No decorative comment dividers (e.g., `// ========`)
   - Write comments explaining "why", not "what"

3. **Function Design**
   - Keep functions focused on single responsibility
   - Limit function length (prefer <50 lines)
   - Use `Result<T>` for error handling

4. **Error Handling**
   - Use `anyhow::Result` for application errors
   - Provide context with `.context()`
   - Write user-friendly error messages

### Testing Guidelines

1. **Test Coverage**
   - Add tests for new functionality
   - Update existing tests when behavior changes
   - Integration tests must not require `/opt/ros` setup

2. **Test Structure**
   - Unit tests in `#[cfg(test)]` modules
   - Integration tests in `tests/` directory
   - Edge case tests separate from happy path

3. **Test Naming**
   - Use `test_` prefix
   - Descriptive names: `test_activate_missing_distro`

### Commit Guidelines

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>: <description>

[optional body]

[optional footer]
```

**Types:**
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions or modifications
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks
- `ci:` - CI/CD changes

**Examples:**

```
feat: add support for ROS 2 Rolling distribution

fix: resolve symlink validation error on macOS

docs: update getting started guide with troubleshooting

test: add edge case tests for broken symlinks
```

**Rules:**
- Keep subject line ≤72 characters
- Use imperative mood ("add" not "added")
- No period at end of subject line
- Separate subject from body with blank line

## Pull Request Process

### Before Submitting

Run quality checks:

```bash
cargo fmt --all --check          # Check formatting
cargo clippy -- -D warnings      # Check linting
cargo test                       # Run all tests
```

### Submitting PR

1. Create feature branch from `main`:

```bash
git checkout -b feature/my-feature
```

2. Make changes and commit with conventional commits

3. Push to fork:

```bash
git push origin feature/my-feature
```

4. Open PR on GitHub with:
   - Clear title describing the change
   - Reference to related issues
   - Description of what changed and why
   - Test results if relevant

### PR Review

- Address review feedback
- Keep commits clean (squash if requested)
- Ensure CI remains green

## Release Process

Releases are automated via GitHub Actions.

### Creating a Release

1. Tag commit:

```bash
git tag v0.x.x
git push origin v0.x.x
```

2. GitHub Actions will:
   - Run CI tests
   - Build binaries (Linux x86_64, macOS ARM64)
   - Build conda packages (rattler-build)
   - Generate changelog (git-cliff)
   - Create GitHub release with artifacts

### Release Checklist

- [ ] All tests passing
- [ ] Version updated in `Cargo.toml`
- [ ] Documentation updated
- [ ] Tag follows semver (v0.x.x)

## Project Structure

<FileTree>
- src/
  - main.rs **849 lines - All application logic**
- tests/
  - integration_tests.rs **15 CLI integration tests**
  - edge_cases.rs **14 filesystem edge case tests**
- recipe/
  - recipe.yaml **Conda package recipe (rattler-build)**
- .github/workflows/
  - ci.yml **Run tests, clippy, formatting**
  - release.yml **Build and publish releases**
  - deploy-docs.yml **Deploy documentation site**
- docs/ **Documentation site (Astro Starlight)**
- Cargo.toml **Package metadata**
- README.md **Project overview**
- LICENSE **MIT license**
</FileTree>

## Documentation

Documentation is built with Astro Starlight.

### Local Development

```bash
cd docs
pnpm install
pnpm dev  # http://localhost:4321/ros2env/
```

### Build Documentation

```bash
pnpm build
```

Documentation deploys automatically when changes are pushed to `main`.

## Getting Help

- **Bug Reports:** [Open an issue](https://github.com/alvgaona/ros2env/issues/new)
- **Feature Requests:** [Open an issue](https://github.com/alvgaona/ros2env/issues/new)
- **Questions:** Check existing issues or open a new one

**Before opening an issue:**
- Search existing issues
- Include system information (OS, Rust version, rosenv version)
- Provide reproducible example if applicable

## Code of Conduct

- Be professional and respectful
- Focus on technical merit
- Provide constructive feedback
- Welcome newcomers

Thank you for contributing to rosenv!
