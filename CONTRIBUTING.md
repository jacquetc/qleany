# Contributing to Qleany

Thank you for your interest in contributing to Qleany! This document provides guidelines and information for contributors.

## Code of Conduct

Please be respectful and constructive in all interactions. We aim to maintain a welcoming environment for everyone.

## How to Contribute

### Reporting Issues

- Check existing issues before creating a new one
- Provide a clear description of the problem
- Include steps to reproduce, expected behavior, and actual behavior
- Mention your environment (OS, Rust version, etc.)

### Suggesting Features

- Open an issue describing the feature and its use case
- Explain why this would be valuable for Qleany users
- Be open to discussion about alternative approaches

### Submitting Code

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes
4. Ensure your code follows the project's style
5. Test your changes
6. Submit a pull request

## Developer Certificate of Origin

This project uses the [Developer Certificate of Origin (DCO)](DCO.md).

By contributing to this repository, you agree to the DCO. You **must sign off your commits** to indicate your agreement:

```bash
git commit -s -m "Your commit message"
```

This adds a `Signed-off-by: Your Name <your.email@example.com>` line to your commit, certifying that you wrote or have the right to submit the code under the project's license (MPL-2.0).

### Setting up automatic sign-off

You can configure Git to always sign off your commits for this repository:

```bash
git config user.name "Your Name"
git config user.email "your.email@example.com"
```

Then use `git commit -s` for each commit, or create a Git alias:

```bash
git config --global alias.cs "commit -s"
```

### What if I forgot to sign off?

You can amend your last commit:

```bash
git commit --amend -s
```

For multiple commits, you may need to rebase:

```bash
git rebase --signoff HEAD~N
```

(Replace `N` with the number of commits to sign off)

## License

By contributing to Qleany, you agree that your contributions will be licensed under the [Mozilla Public License 2.0](LICENSE).

## Questions?

If you have questions about contributing, feel free to open an issue for discussion.
