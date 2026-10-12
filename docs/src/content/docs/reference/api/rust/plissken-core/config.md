---
title: "plissken-core::config"
description: "Configuration for plissken projects"
editUrl: false
sidebar:
  order: 3
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Configuration for plissken projects

## Structs

### <span id="struct.ConfigWarning"></span><code style="font-size: 0.85em">struct ConfigWarning</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`

Configuration warning (non-fatal issue)

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `field` | `String` | The config field that triggered the warning |
| `message` | `String` | Human-readable warning message |
| `hint` | `Option<String>` | Optional hint for resolution |

#### Methods

##### <span id="ConfigWarning.new"></span><code>new</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn new(field: impl Into<String>, message: impl Into<String>) -> Self
```

Create a new warning

<details>
<summary>Source</summary>

```rust
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            hint: None,
        }
    }
```

</details>

##### <span id="ConfigWarning.with_hint"></span><code>with_​hint</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_hint(mut self, hint: impl Into<String>) -> Self
```

Add a hint to the warning

<details>
<summary>Source</summary>

```rust
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
```

</details>

### <span id="struct.ValidationResult"></span><code style="font-size: 0.85em">struct ValidationResult</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`

Result of configuration validation

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `valid` | `bool` | Whether validation passed (no errors) |
| `warnings` | `Vec<ConfigWarning>` | Validation warnings (non-fatal) |

### <span id="struct.Config"></span><code style="font-size: 0.85em">struct Config</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Root configuration from plissken.toml

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `project` | `ProjectConfig` |  |
| `output` | `OutputConfig` |  |
| `rust` | `Option<RustConfig>` |  |
| `python` | `Option<PythonConfig>` |  |
| `links` | `LinksConfig` |  |
| `quality` | `QualityConfig` |  |

#### Methods

##### <span id="Config.from_file"></span><code>from_​file</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn from_file(path: &std::path::Path) -> crate::error::Result<Self>
```

Load configuration from a plissken.toml file.

**Errors:**

Returns `PlisskenError::ConfigNotFound` if the file doesn't exist,
`PlisskenError::ConfigParse` if the TOML is invalid.

<details>
<summary>Source</summary>

```rust
    pub fn from_file(path: &std::path::Path) -> crate::error::Result<Self> {
        use crate::error::PlisskenError;

        let content = std::fs::read_to_string(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                PlisskenError::ConfigNotFound {
                    path: path.to_path_buf(),
                }
            } else {
                PlisskenError::Io {
                    context: format!("failed to read config file '{}'", path.display()),
                    source: e,
                }
            }
        })?;

        let config: Config = toml::from_str(&content).map_err(|e| PlisskenError::ConfigParse {
            message: e.to_string(),
            source: Some(e),
        })?;

        Ok(config)
    }
```

</details>

##### <span id="Config.with_inferred_defaults"></span><code>with_​inferred_​defaults</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn with_inferred_defaults(mut self, project_root: &Path) -> Self
```

Apply inferred defaults from manifest files (Cargo.toml, pyproject.toml).

Infers project metadata from existing manifest files and fills in missing
configuration values. Explicit configuration always takes precedence over
inferred values.

**Inferred values**

- `project.name` - From pyproject.toml [project].name or Cargo.toml [package].name
- `rust.crates` - From Cargo.toml [workspace].members or single crate root
- `rust.entry_point` - From Cargo.toml [package].name
- `python.package` - From pyproject.toml [project].name (with dash-to-underscore)
- `python.source` - From pyproject.toml [tool.maturin].python-source

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `project_root` | `&Path` | The directory containing manifest files |

<details>
<summary>Source</summary>

```rust
    pub fn with_inferred_defaults(mut self, project_root: &Path) -> Self {
        use crate::manifest::InferredConfig;

        let inferred = InferredConfig::from_directory(project_root);

        // Fill in project name if empty
        if self.project.name.is_empty()
            && let Some(name) = inferred.project_name
        {
            self.project.name = name;
        }

        // Fill in Rust config if present but incomplete
        if let Some(ref mut rust) = self.rust {
            // Fill in crates if empty
            if rust.crates.is_empty()
                && let Some(crates) = inferred.rust_crates
            {
                rust.crates = crates;
            }
            // Fill in entry_point if not set
            if rust.entry_point.is_none() {
                rust.entry_point = inferred.rust_entry_point;
            }
        }

        // Fill in Python config if present but incomplete
        if let Some(ref mut python) = self.python {
            // Fill in package name if empty
            if python.package.is_empty()
                && let Some(pkg) = inferred.python_package
            {
                python.package = pkg;
            }
            // Fill in source if not set
            if python.source.is_none() {
                python.source = inferred.python_source;
            }
        }

        self
    }
```

</details>

##### <span id="Config.validate"></span><code>validate</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn validate(
    &self,
    project_root: &Path,
) -> Result<ValidationResult, ConfigError>
```

Validate configuration semantically.

Performs validation beyond TOML parsing:
- At least one language section must be configured
- version_from source file must exist
- Configured paths must exist

Returns `Ok(ValidationResult)` with any warnings if validation passes,
or `Err(ConfigError)` if validation fails.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `project_root` | `&Path` | The directory containing the plissken.toml file |

<details>
<summary>Source</summary>

```rust
    pub fn validate(&self, project_root: &Path) -> Result<ValidationResult, ConfigError> {
        let mut warnings = Vec::new();

        // Must have at least one language configured
        if self.rust.is_none() && self.python.is_none() {
            return Err(ConfigError::NoLanguageConfigured);
        }

        // Validate version_from source exists
        self.validate_version_source(project_root)?;

        // Validate Rust configuration
        if let Some(ref rust_config) = self.rust {
            self.validate_rust_config(rust_config, project_root, &mut warnings)?;
        }

        // Validate Python configuration
        if let Some(ref python_config) = self.python {
            self.validate_python_config(python_config, project_root, &mut warnings)?;
        }

        Ok(ValidationResult {
            valid: true,
            warnings,
        })
    }
```

</details>

### <span id="struct.ProjectConfig"></span><code style="font-size: 0.85em">struct ProjectConfig</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Project metadata

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `version_from` | `VersionSource` |  |

### <span id="struct.OutputConfig"></span><code style="font-size: 0.85em">struct OutputConfig</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Output configuration

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `format` | `String` |  |
| `path` | `PathBuf` |  |
| `template` | `Option<String>` |  |

### <span id="struct.RustConfig"></span><code style="font-size: 0.85em">struct RustConfig</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Rust source configuration

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `crates` | `Vec<PathBuf>` |  |
| `entry_point` | `Option<String>` |  |
| `include_private` | `bool` | Document items that are not `pub` (default: only the public API; PyO3 bindings are documented either way) |

### <span id="struct.PythonConfig"></span><code style="font-size: 0.85em">struct PythonConfig</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Python source configuration

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `package` | `String` | The Python package name |
| `source` | `Option<PathBuf>` | Source directory containing Python files (defaults to package name) |
| `auto_discover` | `bool` | Automatically discover Python modules by walking the filesystem |
| `modules` | `HashMap<String, ModuleSourceType>` | Explicit module mappings (overrides auto-discovered modules) |
| `include_private` | `bool` | Document names that start with `_` (default: only public names; `__dunder__` methods are public) |

### <span id="struct.LinksConfig"></span><code style="font-size: 0.85em">struct LinksConfig</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`, `Default`

Linking configuration

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `dependencies` | `DependencySource` |  |
| `docs_rs_base` | `String` |  |

### <span id="struct.QualityConfig"></span><code style="font-size: 0.85em">struct QualityConfig</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`, `Default`

Quality/linting configuration

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `require_docstrings` | `bool` |  |
| `min_coverage` | `Option<f64>` |  |
| `fail_on_broken_links` | `bool` |  |

## Enums

### <span id="enum.ConfigError"></span><code style="font-size: 0.85em">enum ConfigError</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Configuration validation error

#### Variants

- **`NoLanguageConfigured`**
- **`VersionSourceNotFound`**
- **`RustCrateNotFound`**
- **`PythonSourceNotFound`**
- **`GitRepoNotFound`**

### <span id="enum.VersionSource"></span><code style="font-size: 0.85em">enum VersionSource</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Where to get version information

#### Variants

- **`Git`**
- **`Cargo`**
- **`Pyproject`**

### <span id="enum.ModuleSourceType"></span><code style="font-size: 0.85em">enum ModuleSourceType</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Source type for a Python module

#### Variants

- **`Pyo3`**
- **`Python`**

### <span id="enum.DependencySource"></span><code style="font-size: 0.85em">enum DependencySource</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Where to get dependency versions

#### Variants

- **`CargoLock`**
- **`CargoToml`**
- **`None`**

## Constants

| Name | Type | Value | Description |
|------|------|-------|-------------|
| <span id="const.DEFAULT_OUTPUT_FORMAT"></span>`DEFAULT_OUTPUT_FORMAT` | `&str` | `"markdown"` | Default output format for documentation |
| <span id="const.DEFAULT_OUTPUT_PATH"></span>`DEFAULT_OUTPUT_PATH` | `&str` | `"docs/api"` | Default output path for generated documentation |
| <span id="const.DEFAULT_DOCS_RS_URL"></span>`DEFAULT_DOCS_RS_URL` | `&str` | `"https://docs.rs"` | Default docs.rs base URL for external links |
| <span id="const.DEFAULT_TEMPLATE"></span>`DEFAULT_TEMPLATE` | `&str` | `"mkdocs-material"` | Default SSG template name |
| <span id="const.VERSION_SOURCE_CARGO"></span>`VERSION_SOURCE_CARGO` | `&str` | `"cargo"` | Version source identifier for Cargo.toml |
| <span id="const.VERSION_SOURCE_PYPROJECT"></span>`VERSION_SOURCE_PYPROJECT` | `&str` | `"pyproject"` | Version source identifier for pyproject.toml |
| <span id="const.CARGO_MANIFEST"></span>`CARGO_MANIFEST` | `&str` | `"Cargo.toml"` | Cargo manifest filename |
| <span id="const.PYPROJECT_MANIFEST"></span>`PYPROJECT_MANIFEST` | `&str` | `"pyproject.toml"` | Python project manifest filename |
| <span id="const.PLISSKEN_CONFIG"></span>`PLISSKEN_CONFIG` | `&str` | `"plissken.toml"` | Plissken configuration filename |
| <span id="const.TEMPLATE_MKDOCS_MATERIAL"></span>`TEMPLATE_MKDOCS_MATERIAL` | `&str` | `"mkdocs-material"` | MkDocs Material template name |
| <span id="const.TEMPLATE_MDBOOK"></span>`TEMPLATE_MDBOOK` | `&str` | `"mdbook"` | mdBook template name |
| <span id="const.TEMPLATE_STARLIGHT"></span>`TEMPLATE_STARLIGHT` | `&str` | `"starlight"` | Astro Starlight template name |
| <span id="const.DEFAULT_STARLIGHT_OUTPUT_PATH"></span>`DEFAULT_STARLIGHT_OUTPUT_PATH` | `&str` | `"docs/src/content/docs/reference/api"` | Default output path for the Starlight template: a folder of the Starlight content collection |
| <span id="const.DEFAULT_CRATES"></span>`DEFAULT_CRATES` | `&str` | `"."` | Default crates configuration value |
