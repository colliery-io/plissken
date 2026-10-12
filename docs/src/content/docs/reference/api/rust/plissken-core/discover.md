---
title: "plissken-core::discover"
description: "Python module auto-discovery"
editUrl: false
sidebar:
  order: 5
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Python module auto-discovery

Walks the filesystem to find Python modules, converting file paths
to dotted module names.

## Structs

### <span id="struct.DiscoveredModule"></span><code style="font-size: 0.85em">struct DiscoveredModule</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`

A discovered Python module

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | Dotted module name (e.g., "mypackage.utils.helpers") |
| `path` | `PathBuf` | Path to the Python file |
| `module_type` | `ModuleSourceType` | Detected module type (Python or PyO3) |

## Functions

### <span id="fn.discover_python_modules"></span><code style="font-size: 0.85em">fn discover_​python_​modules</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn discover_python_modules(
    source_dir: &Path,
    package_name: &str,
) -> Result<Vec<DiscoveredModule>, std::io::Error>
```

Discover Python modules by walking the filesystem.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `source_dir` | `&Path` | The directory to search for Python files |
| `package_name` | `&str` | The root package name for module path generation |

**Returns:**

A vector of discovered modules with their dotted names and paths.

<details>
<summary>Source</summary>

```rust
pub fn discover_python_modules(
    source_dir: &Path,
    package_name: &str,
) -> Result<Vec<DiscoveredModule>, std::io::Error> {
    let mut modules = Vec::new();

    if !source_dir.exists() {
        return Ok(modules);
    }

    for entry in WalkDir::new(source_dir)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !should_skip_entry(e))
    {
        let entry = entry?;
        let path = entry.path();

        // Only process .py files
        if path.extension().map(|e| e == "py").unwrap_or(false)
            && let Some(module) = path_to_module(path, source_dir, package_name)
        {
            modules.push(module);
        }
    }

    // Sort modules by name for consistent ordering
    modules.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(modules)
}
```

</details>

### <span id="fn.merge_modules"></span><code style="font-size: 0.85em">fn merge_​modules</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn merge_modules(
    discovered: Vec<DiscoveredModule>,
    explicit: &HashMap<String, ModuleSourceType>,
) -> HashMap<String, ModuleSourceType>
```

Merge discovered modules with explicitly configured modules.

Explicit modules take precedence over discovered ones.

<details>
<summary>Source</summary>

```rust
pub fn merge_modules(
    discovered: Vec<DiscoveredModule>,
    explicit: &HashMap<String, ModuleSourceType>,
) -> HashMap<String, ModuleSourceType> {
    let mut result: HashMap<String, ModuleSourceType> = discovered
        .into_iter()
        .map(|m| (m.name, m.module_type))
        .collect();

    // Explicit modules override discovered ones
    for (name, module_type) in explicit {
        result.insert(name.clone(), module_type.clone());
    }

    result
}
```

</details>
