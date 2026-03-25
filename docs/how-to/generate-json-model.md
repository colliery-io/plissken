# How To: Generate the JSON Doc Model

## Generate to stdout

```bash
plissken generate
```

Pipe to a file or another tool:

```bash
plissken generate > model.json
plissken generate | jq '.metadata'
```

## Generate to a File

```bash
plissken generate --output model.json
```

## Pretty-Print

```bash
plissken generate --pretty
```

## Inspect the Output

The JSON contains `metadata`, `rust_modules`, `python_modules`, and
`cross_refs`. Extract specific sections with `jq`:

```bash
# List all Python classes
plissken generate | jq '[.python_modules[].items[] | select(.kind == "Class") | .name]'

# Show all cross-references
plissken generate | jq '.cross_refs'

# Get project metadata
plissken generate | jq '.metadata'
```

For the complete schema of every field, see the
[Data Model Reference](../reference/data-model.md).

## Specify a Different Project

```bash
plissken generate /path/to/project
plissken generate /path/to/plissken.toml
```
