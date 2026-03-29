# plissken-core::config <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Configuration for plissken projects

## Structs

### `plissken-core::config::ConfigWarning`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`

Configuration warning (non-fatal issue)

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `field` | `String` | The config field that triggered the warning |
| `message` | `String` | Human-readable warning message |
| `hint` | `Option < String >` | Optional hint for resolution |

#### Methods

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn new (field : impl Into < String > , message : impl Into < String >) -> Self
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



##### `with_hint` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_hint (mut self , hint : impl Into < String >) -> Self
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





### `plissken-core::config::ValidationResult`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`

Result of configuration validation

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `valid` | `bool` | Whether validation passed (no errors) |
| `warnings` | `Vec < ConfigWarning >` | Validation warnings (non-fatal) |



### `plissken-core::config::Config`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Root configuration from plissken.toml

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `project` | `ProjectConfig` |  |
| `output` | `OutputConfig` |  |
| `rust` | `Option < RustConfig >` |  |
| `python` | `Option < PythonConfig >` |  |
| `links` | `LinksConfig` |  |
| `quality` | `QualityConfig` |  |

#### Methods

##### `from_file` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn from_file (path : & std :: path :: Path) -> crate :: error :: Result < Self >
```

Load configuration from a plissken.toml file.

**Raises:**

| Exception | Description |
|-----------|-------------|
| `Error` | Returns `PlisskenError::ConfigNotFound` if the file doesn't exist, `PlisskenError::ConfigParse` if the TOML is invalid. |


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



##### `with_inferred_defaults` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn with_inferred_defaults (mut self , project_root : & Path) -> Self
```

Apply inferred defaults from manifest files (Cargo.toml, pyproject.toml).

Infers project metadata from existing manifest files and fills in missing
configuration values. Explicit configuration always takes precedence over
inferred values.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `project_root` | `-` | The directory containing manifest files |


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



##### `validate` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn validate (& self , project_root : & Path) -> Result < ValidationResult , ConfigError >
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
| `project_root` | `-` | The directory containing the plissken.toml file |


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



##### `validate_version_source` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn validate_version_source (& self , project_root : & Path) -> Result < () , ConfigError >
```

<details>
<summary>Source</summary>

```rust
    fn validate_version_source(&self, project_root: &Path) -> Result<(), ConfigError> {
        match self.project.version_from {
            VersionSource::Cargo => {
                let cargo_toml = project_root.join(CARGO_MANIFEST);
                if !cargo_toml.exists() {
                    return Err(ConfigError::VersionSourceNotFound(
                        VERSION_SOURCE_CARGO.to_string(),
                        CARGO_MANIFEST.to_string(),
                    ));
                }
            }
            VersionSource::Pyproject => {
                let pyproject = project_root.join(PYPROJECT_MANIFEST);
                if !pyproject.exists() {
                    return Err(ConfigError::VersionSourceNotFound(
                        VERSION_SOURCE_PYPROJECT.to_string(),
                        PYPROJECT_MANIFEST.to_string(),
                    ));
                }
            }
            VersionSource::Git => {
                // Check if we're in a git repository
                let git_check = std::process::Command::new("git")
                    .args(["rev-parse", "--git-dir"])
                    .current_dir(project_root)
                    .output();

                match git_check {
                    Ok(output) if output.status.success() => {}
                    _ => return Err(ConfigError::GitRepoNotFound),
                }
            }
        }
        Ok(())
    }
```

</details>



##### `validate_rust_config` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn validate_rust_config (& self , rust_config : & RustConfig , project_root : & Path , warnings : & mut Vec < ConfigWarning > ,) -> Result < () , ConfigError >
```

<details>
<summary>Source</summary>

```rust
    fn validate_rust_config(
        &self,
        rust_config: &RustConfig,
        project_root: &Path,
        warnings: &mut Vec<ConfigWarning>,
    ) -> Result<(), ConfigError> {
        if rust_config.crates.is_empty() {
            warnings.push(
                ConfigWarning::new(
                    "rust.crates",
                    "no crates configured; no Rust docs will be generated",
                )
                .with_hint("add crate paths to the crates array"),
            );
            return Ok(());
        }

        for crate_path in &rust_config.crates {
            let crate_dir = project_root.join(crate_path);

            if !crate_dir.exists() {
                return Err(ConfigError::RustCrateNotFound(crate_path.clone()));
            }

            // Check for Cargo.toml in crate directory (warning, not error)
            let cargo_toml = crate_dir.join(CARGO_MANIFEST);
            if !cargo_toml.exists() && crate_path.as_os_str() != DEFAULT_CRATES {
                warnings.push(ConfigWarning::new(
                    "rust.crates",
                    format!("no Cargo.toml found in crate '{}'", crate_path.display()),
                ));
            }

            // Check for src directory (warning, not error)
            let src_dir = crate_dir.join("src");
            if !src_dir.exists() {
                warnings.push(
                    ConfigWarning::new(
                        "rust.crates",
                        format!("no src/ directory in crate '{}'", crate_path.display()),
                    )
                    .with_hint("Rust source files are typically in a src/ directory"),
                );
            }
        }

        Ok(())
    }
```

</details>



##### `validate_python_config` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn validate_python_config (& self , python_config : & PythonConfig , project_root : & Path , warnings : & mut Vec < ConfigWarning > ,) -> Result < () , ConfigError >
```

<details>
<summary>Source</summary>

```rust
    fn validate_python_config(
        &self,
        python_config: &PythonConfig,
        project_root: &Path,
        warnings: &mut Vec<ConfigWarning>,
    ) -> Result<(), ConfigError> {
        // Determine Python source directory
        let python_dir = if let Some(ref source) = python_config.source {
            project_root.join(source)
        } else {
            project_root.join(&python_config.package)
        };

        if !python_dir.exists() {
            return Err(ConfigError::PythonSourceNotFound(python_dir));
        }

        // Check for __init__.py (warning, not error)
        let init_py = python_dir.join("__init__.py");
        if !init_py.exists() {
            warnings.push(
                ConfigWarning::new(
                    "python.package",
                    format!(
                        "no __init__.py in '{}' - may not be a proper Python package",
                        python_dir.display()
                    ),
                )
                .with_hint("add __init__.py to make it a Python package"),
            );
        }

        // Check for empty modules list (warning)
        if python_config.modules.is_empty() {
            warnings.push(
                ConfigWarning::new(
                    "python.modules",
                    "no modules listed; consider using auto_discover or listing modules explicitly",
                )
                .with_hint("modules will be discovered from filesystem if not listed"),
            );
        }

        Ok(())
    }
```

</details>





### `plissken-core::config::ProjectConfig`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Project metadata

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `version_from` | `VersionSource` |  |



### `plissken-core::config::OutputConfig`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Output configuration

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `format` | `String` |  |
| `path` | `PathBuf` |  |
| `template` | `Option < String >` |  |
| `prefix` | `Option < String >` | Path prefix for nav entries when rendering into a subfolder of an existing doc site.
E.g., `prefix = "api"` makes nav entries like `api/rust/mycrate.md` instead of `rust/mycrate.md`. |



### `plissken-core::config::RustConfig`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Rust source configuration

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `crates` | `Vec < PathBuf >` |  |
| `entry_point` | `Option < String >` |  |



### `plissken-core::config::PythonConfig`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`

Python source configuration

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `package` | `String` | The Python package name |
| `source` | `Option < PathBuf >` | Source directory containing Python files (defaults to package name) |
| `auto_discover` | `bool` | Automatically discover Python modules by walking the filesystem |
| `modules` | `HashMap < String , ModuleSourceType >` | Explicit module mappings (overrides auto-discovered modules) |



### `plissken-core::config::LinksConfig`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`, `Default`

Linking configuration

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `dependencies` | `DependencySource` |  |
| `docs_rs_base` | `String` |  |



### `plissken-core::config::QualityConfig`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`, `Deserialize`, `Default`

Quality/linting configuration

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `require_docstrings` | `bool` |  |
| `min_coverage` | `Option < f64 >` |  |
| `fail_on_broken_links` | `bool` |  |



## Enums

### `plissken-core::config::ConfigError` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Configuration validation error

#### Variants

- **`NoLanguageConfigured`**
- **`VersionSourceNotFound`**
- **`RustCrateNotFound`**
- **`PythonSourceNotFound`**
- **`GitRepoNotFound`**



### `plissken-core::config::VersionSource` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Where to get version information

#### Variants

- **`Git`**
- **`Cargo`**
- **`Pyproject`**



### `plissken-core::config::ModuleSourceType` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Source type for a Python module

#### Variants

- **`Pyo3`**
- **`Python`**



### `plissken-core::config::DependencySource` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Where to get dependency versions

#### Variants

- **`CargoLock`**
- **`CargoToml`**
- **`None`**



## Functions

### `plissken-core::config::default_version_from`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn default_version_from () -> VersionSource
```

<details>
<summary>Source</summary>

```rust
fn default_version_from() -> VersionSource {
    VersionSource::Git
}
```

</details>



### `plissken-core::config::default_format`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn default_format () -> String
```

<details>
<summary>Source</summary>

```rust
fn default_format() -> String {
    DEFAULT_OUTPUT_FORMAT.to_string()
}
```

</details>



### `plissken-core::config::default_output_path`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn default_output_path () -> PathBuf
```

<details>
<summary>Source</summary>

```rust
fn default_output_path() -> PathBuf {
    PathBuf::from(DEFAULT_OUTPUT_PATH)
}
```

</details>



### `plissken-core::config::default_dependencies`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn default_dependencies () -> DependencySource
```

<details>
<summary>Source</summary>

```rust
fn default_dependencies() -> DependencySource {
    DependencySource::CargoLock
}
```

</details>



### `plissken-core::config::default_docs_rs`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn default_docs_rs () -> String
```

<details>
<summary>Source</summary>

```rust
fn default_docs_rs() -> String {
    DEFAULT_DOCS_RS_URL.to_string()
}
```

</details>



