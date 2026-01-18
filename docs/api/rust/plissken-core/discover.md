# discover <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Python module auto-discovery

Walks the filesystem to find Python modules, converting file paths
to dotted module names.

## Structs

### `struct DiscoveredModule`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`

A discovered Python module

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | Dotted module name (e.g., "mypackage.utils.helpers") |
| `path` | `PathBuf` | Path to the Python file |
| `module_type` | `ModuleSourceType` | Detected module type (Python or PyO3) |



## Functions

### `fn discover_python_modules`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn discover_python_modules (source_dir : & Path , package_name : & str ,) -> Result < Vec < DiscoveredModule > , std :: io :: Error >
```

Discover Python modules by walking the filesystem.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `source_dir` | `-` | The directory to search for Python files |
| `package_name` | `-` | The root package name for module path generation |


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
        if path.extension().map(|e| e == "py").unwrap_or(false) {
            if let Some(module) = path_to_module(path, source_dir, package_name) {
                modules.push(module);
            }
        }
    }

    // Sort modules by name for consistent ordering
    modules.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(modules)
}
```

</details>



### `fn should_skip_entry`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn should_skip_entry (entry : & walkdir :: DirEntry) -> bool
```

Check if an entry should be skipped during directory traversal.

<details>
<summary>Source</summary>

```rust
fn should_skip_entry(entry: &walkdir::DirEntry) -> bool {
    let file_name = entry.file_name().to_string_lossy();

    // Skip hidden files/directories (except the source dir itself)
    if file_name.starts_with('.') && entry.depth() > 0 {
        return true;
    }

    // Skip known non-module directories
    if entry.file_type().is_dir() {
        if SKIP_DIRS.iter().any(|&skip| file_name == skip) {
            return true;
        }
        // Skip directories ending in .egg-info
        if file_name.ends_with(".egg-info") {
            return true;
        }
    }

    false
}
```

</details>



### `fn path_to_module`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn path_to_module (file_path : & Path , source_dir : & Path , package_name : & str ,) -> Option < DiscoveredModule >
```

Convert a file path to a Python module.

<details>
<summary>Source</summary>

```rust
fn path_to_module(
    file_path: &Path,
    source_dir: &Path,
    package_name: &str,
) -> Option<DiscoveredModule> {
    // Get relative path from source directory
    let relative = file_path.strip_prefix(source_dir).ok()?;

    // Convert path to module name
    let module_name = path_to_module_name(relative, package_name)?;

    // Detect module type by scanning file content
    let module_type = detect_module_type(file_path);

    Some(DiscoveredModule {
        name: module_name,
        path: file_path.to_owned(),
        module_type,
    })
}
```

</details>



### `fn path_to_module_name`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn path_to_module_name (relative_path : & Path , package_name : & str) -> Option < String >
```

Convert a relative file path to a dotted module name.

Examples:
- `mypackage/__init__.py` → `mypackage`
- `mypackage/utils.py` → `mypackage.utils`
- `mypackage/sub/helpers.py` → `mypackage.sub.helpers`

<details>
<summary>Source</summary>

```rust
fn path_to_module_name(relative_path: &Path, package_name: &str) -> Option<String> {
    let mut components: Vec<&str> = Vec::new();

    for component in relative_path.components() {
        if let std::path::Component::Normal(name) = component {
            let name_str = name.to_str()?;
            components.push(name_str);
        }
    }

    if components.is_empty() {
        return None;
    }

    // Remove .py extension from the last component
    let last_idx = components.len() - 1;
    let last = components[last_idx];
    let last_without_ext = last.strip_suffix(".py")?;

    // Handle __init__.py - represents the package itself
    if last_without_ext == "__init__" {
        if components.len() == 1 {
            // Root __init__.py
            return Some(package_name.to_string());
        }
        // Sub-package __init__.py - remove the __init__ part
        components.pop();
    } else {
        components[last_idx] = last_without_ext;
    }

    if components.is_empty() {
        return Some(package_name.to_string());
    }

    // Check if the first component matches the package name
    // If source dir already contains the package, don't duplicate
    if components[0] == package_name {
        Some(components.join("."))
    } else {
        // Prepend package name
        Some(format!("{}.{}", package_name, components.join(".")))
    }
}
```

</details>



### `fn detect_module_type`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn detect_module_type (file_path : & Path) -> ModuleSourceType
```

Detect if a Python file is a PyO3 stub module.

Looks for markers that indicate the module imports from a native extension:
- Import from a module with underscore prefix (e.g., `from ._native import`)
- Comment marker `# pyo3` or `# pyo3-stub`

<details>
<summary>Source</summary>

```rust
fn detect_module_type(file_path: &Path) -> ModuleSourceType {
    // Read the first part of the file to check for markers
    if let Ok(content) = std::fs::read_to_string(file_path) {
        // Only check the first ~2KB for performance
        let preview = if content.len() > 2048 {
            &content[..2048]
        } else {
            &content
        };

        // Check for PyO3 markers
        if preview.contains("# pyo3")
            || preview.contains("#pyo3")
            || preview.contains("# type: ignore[import]")  // Common in PyO3 stubs
        {
            return ModuleSourceType::Pyo3;
        }

        // Check for imports from native modules (underscore prefix convention)
        for line in preview.lines() {
            let line = line.trim();
            if (line.starts_with("from ._") || line.starts_with("from _"))
                && line.contains(" import ")
            {
                return ModuleSourceType::Pyo3;
            }
        }
    }

    ModuleSourceType::Python
}
```

</details>



### `fn merge_modules`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn merge_modules (discovered : Vec < DiscoveredModule > , explicit : & HashMap < String , ModuleSourceType > ,) -> HashMap < String , ModuleSourceType >
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



