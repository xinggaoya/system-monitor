# Contributing to System Monitor

Thank you for your interest in contributing to System Monitor! This document provides guidelines and information for contributors.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Style](#code-style)
- [Submitting Changes](#submitting-changes)
- [Issue Reporting](#issue-reporting)
- [Feature Requests](#feature-requests)

## Getting Started

### Prerequisites

- Node.js 18+
- Rust 1.70+
- pnpm (recommended) or npm
- Git

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/your-username/system-monitor.git
   cd system-monitor
   ```

2. **Install Dependencies**
   ```bash
   pnpm install
   ```

3. **Start Development Server**
   ```bash
   pnpm tauri dev
   ```

## Code Style

### Frontend (Vue/TypeScript)

- Use TypeScript for type safety
- Follow Vue 3 Composition API patterns
- Use meaningful variable and function names
- Add comments for complex logic

### Backend (Rust)

- Follow Rust idiomatic patterns
- Use `cargo fmt` for code formatting
- Use `cargo clippy` for linting
- Add documentation for public functions

### General Guidelines

- Keep commits small and focused
- Write clear commit messages
- Update documentation when needed
- Test your changes thoroughly

## Submitting Changes

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

- Implement your feature or fix
- Add tests if applicable
- Update documentation
- Ensure code passes linting

### 3. Test Your Changes

```bash
# Run frontend checks
pnpm type-check

# Run backend checks
cd src-tauri
cargo check
cargo clippy
cargo test
```

### 4. Commit Your Changes

```bash
git add .
git commit -m "feat: add your feature description"
```

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub with:
- Clear title and description
- Screenshots if applicable
- Testing instructions

## Issue Reporting

### Bug Reports

When reporting bugs, please include:

- **Environment**: OS, version, browser (if applicable)
- **Steps to Reproduce**: Detailed steps to reproduce the issue
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Screenshots**: If applicable
- **Additional Context**: Any other relevant information

### Security Issues

For security issues, please do not open a public issue. Instead, email the maintainers privately.

## Feature Requests

We welcome feature requests! Please:

1. Check existing issues to avoid duplicates
2. Provide a clear description of the feature
3. Explain the use case and why it's valuable
4. Consider if you can contribute the implementation

## Development Commands

```bash
# Development
pnpm tauri dev              # Start development server
pnpm tauri build            # Build for production

# Frontend
pnpm type-check            # TypeScript type checking
pnpm test                   # Run tests (if any)
pnpm build                  # Build frontend only

# Backend (in src-tauri directory)
cargo check                 # Check compilation
cargo test                  # Run tests
cargo clippy                # Run linter
cargo fmt                   # Format code
```

## Project Structure

```
system-monitor/
├── src/                    # Frontend source code
│   ├── components/         # Vue components
│   ├── composables/        # Composable functions
│   ├── stores/            # Pinia state management
│   └── assets/            # Static assets
├── src-tauri/              # Rust backend
│   ├── src/               # Rust source code
│   ├── icons/             # Application icons
│   └── tauri.conf.json    # Tauri configuration
├── .github/               # GitHub workflows
├── docs/                  # Documentation (if any)
└── scripts/               # Build and utility scripts
```

## Code Review Process

All submissions require review to maintain code quality:

1. **Automated Checks**: CI/CD pipeline runs tests and linting
2. **Peer Review**: At least one maintainer reviews changes
3. **Testing**: Changes must be tested on multiple platforms when applicable
4. **Documentation**: Updates to documentation are reviewed

## Release Process

Releases are automated through GitHub Actions:

1. **Tag Creation**: Create a new tag (e.g., `v1.0.0`)
2. **Release Draft**: GitHub Actions creates a draft release
3. **Build Process**: Automatic builds for all platforms
4. **Release Publishing**: Review and publish the release

## Getting Help

If you need help:

- Check existing documentation and issues
- Ask questions in GitHub Discussions
- Contact maintainers directly

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to System Monitor! 🎉