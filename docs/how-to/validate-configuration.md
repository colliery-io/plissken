# How To: Validate Configuration

## Basic Validation

```bash
plissken check
```

This validates your `plissken.toml` without generating any output. It checks:

- TOML syntax is valid
- At least one language section (`[rust]` or `[python]`) is configured
- The `version_from` source file exists (Cargo.toml, pyproject.toml, or .git)
- Configured Rust crate paths exist on disk
- Configured Python source directories exist on disk

## JSON Output

For CI or programmatic use:

```bash
plissken check --format json
```

Output:

```json
{
  "valid": true,
  "config_path": "/path/to/plissken.toml",
  "issues": []
}
```

Or with issues:

```json
{
  "valid": false,
  "config_path": "/path/to/plissken.toml",
  "issues": [
    {
      "severity": "error",
      "message": "rust crate path 'nonexistent' does not exist",
      "hint": null
    },
    {
      "severity": "warning",
      "message": "no __init__.py in 'mypackage' - may not be a proper Python package",
      "hint": "add __init__.py to make it a Python package"
    }
  ]
}
```

## Specify a Different Path

If your config file is not in the current directory:

```bash
plissken check /path/to/project
```

Or point directly to the config file:

```bash
plissken check /path/to/plissken.toml
```

## Verbose Output

Add `-v` for progress details or `-vv` for debug output:

```bash
plissken check -v
plissken check -vv
```

## Common Validation Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `no language configured` | Neither `[rust]` nor `[python]` section exists | Add at least one language section |
| `version_from is 'cargo' but Cargo.toml not found` | `version_from = "cargo"` but no Cargo.toml | Change `version_from` or add Cargo.toml |
| `rust crate path 'X' does not exist` | Path in `rust.crates` doesn't exist | Fix the path or remove the entry |
| `python source path 'X' does not exist` | Python package directory missing | Fix the path or create the directory |

## Common Warnings

Warnings don't prevent documentation generation but indicate potential issues:

| Warning | Meaning |
|---------|---------|
| `no Cargo.toml found in crate 'X'` | Crate path exists but has no Cargo.toml |
| `no src/ directory in crate 'X'` | Crate exists but has no source directory |
| `no __init__.py in 'X'` | Python package directory lacks __init__.py |
| `no modules listed` | No explicit modules and auto_discover not enabled |

## Use in CI

```yaml
# .github/workflows/docs.yml
- name: Validate plissken config
  run: plissken check --format json
```

The command exits with code 0 on success and non-zero on validation errors.
