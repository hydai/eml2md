---
description: How to commit changes after implementation
---

# Commit Workflow

Follow these steps after completing an implementation:

## 1. Stage and Commit Changes

// turbo
1. Stage all modified files:
   ```bash
   git add -A
   ```

2. Commit using **Conventional Commits** format with **Antigravity as co-author**:
   ```bash
   git commit -m "<type>(<scope>): <description>

   Co-authored-by: Antigravity <noreply@google.com>"
   ```

## Conventional Commits Format

Use one of these types:
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

```bash
git commit -m "feat(formatter): add HTML to Markdown conversion

Co-authored-by: Antigravity <noreply@google.com>"
```

```bash
git commit -m "fix(parser): handle empty EML files gracefully

Co-authored-by: Antigravity <noreply@google.com>"
```

```bash
git commit -m "docs: update README with installation instructions

Co-authored-by: Antigravity <noreply@google.com>"
```
