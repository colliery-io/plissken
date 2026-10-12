---
title: "plissken-core::manifest"
description: "Manifest file parsing for Cargo.toml and pyproject.toml"
sidebar:
  order: 8
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


Manifest file parsing for Cargo.toml and pyproject.toml

This module provides functionality to parse project manifest files and
extract metadata that can be used to infer default configuration values.

## Structs

### <span id="struct.CargoManifest"></span>`struct CargoManifest`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


**Derives:** `Debug`, `Clone`

Parsed Cargo.toml manifest

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `Option < String >` | Package name from [package].name |
| `version` | `Option < String >` | Package version from [package].version |
| `is_workspace` | `bool` | Whether this is a workspace root (has [workspace] section) |
| `workspace_members` | `Vec < String >` | Workspace members if this is a workspace root |

#### Methods

##### <span id="CargoManifest.parse"></span>`parse`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn parse (path : & Path) -> Result < Self , ManifestError >
```

Parse a Cargo.toml file

<details>
<summary>Source</summary>

```rust
    pub fn parse(path: &Path) -> Result<Self, ManifestError> {
        let content = std::fs::read_to_string(path)?;
        Self::parse_str(&content)
    }
```

</details>



##### <span id="CargoManifest.parse_str"></span>`parse_str`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn parse_str (content : & str) -> Result < Self , ManifestError >
```

Parse Cargo.toml content from a string

<details>
<summary>Source</summary>

```rust
    pub fn parse_str(content: &str) -> Result<Self, ManifestError> {
        let toml: CargoToml = toml::from_str(content)?;

        let (name, version) = if let Some(pkg) = toml.package {
            (pkg.name, pkg.version)
        } else {
            (None, None)
        };

        let (is_workspace, workspace_members) = if let Some(ws) = toml.workspace {
            (true, ws.members.unwrap_or_default())
        } else {
            (false, Vec::new())
        };

        Ok(CargoManifest {
            name,
            version,
            is_workspace,
            workspace_members,
        })
    }
```

</details>





### <span id="struct.PyProjectManifest"></span>`struct PyProjectManifest`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


**Derives:** `Debug`, `Clone`

Parsed pyproject.toml manifest

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `Option < String >` | Project name from [project].name |
| `version` | `Option < String >` | Project version from [project].version |
| `package_dir` | `Option < PathBuf >` | Package directory from [tool.setuptools.package-dir] or similar |

#### Methods

##### <span id="PyProjectManifest.parse"></span>`parse`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn parse (path : & Path) -> Result < Self , ManifestError >
```

Parse a pyproject.toml file

<details>
<summary>Source</summary>

```rust
    pub fn parse(path: &Path) -> Result<Self, ManifestError> {
        let content = std::fs::read_to_string(path)?;
        Self::parse_str(&content)
    }
```

</details>



##### <span id="PyProjectManifest.parse_str"></span>`parse_str`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn parse_str (content : & str) -> Result < Self , ManifestError >
```

Parse pyproject.toml content from a string

<details>
<summary>Source</summary>

```rust
    pub fn parse_str(content: &str) -> Result<Self, ManifestError> {
        let toml: PyProjectToml = toml::from_str(content)?;

        let (name, version) = if let Some(proj) = toml.project {
            (proj.name, proj.version)
        } else {
            (None, None)
        };

        // Try to find package directory from various tool configurations
        let package_dir = if let Some(tool) = toml.tool {
            // First check maturin python-source
            if let Some(maturin) = tool.maturin {
                maturin.python_source.map(PathBuf::from)
            // Then check setuptools package-dir
            } else if let Some(setuptools) = tool.setuptools {
                if let Some(pkg_dir) = setuptools.package_dir {
                    // Get the root package directory (empty string key or "")
                    pkg_dir.get("").map(PathBuf::from)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        Ok(PyProjectManifest {
            name,
            version,
            package_dir,
        })
    }
```

</details>





### <span id="struct.CargoToml"></span>`struct CargoToml`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


**Derives:** `Deserialize`

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `package` | `Option < CargoPackage >` |  |
| `workspace` | `Option < CargoWorkspace >` |  |



### <span id="struct.CargoPackage"></span>`struct CargoPackage`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


**Derives:** `Deserialize`

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `Option < String >` |  |
| `version` | `Option < String >` |  |



### <span id="struct.CargoWorkspace"></span>`struct CargoWorkspace`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


**Derives:** `Deserialize`

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `members` | `Option < Vec < String > >` |  |



### <span id="struct.PyProjectToml"></span>`struct PyProjectToml`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


**Derives:** `Deserialize`

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `project` | `Option < PyProject >` |  |
| `tool` | `Option < PyProjectTool >` |  |



### <span id="struct.PyProject"></span>`struct PyProject`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


**Derives:** `Deserialize`

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `Option < String >` |  |
| `version` | `Option < String >` |  |



### <span id="struct.PyProjectTool"></span>`struct PyProjectTool`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


**Derives:** `Deserialize`

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `setuptools` | `Option < SetuptoolsConfig >` |  |
| `maturin` | `Option < MaturinConfig >` |  |



### <span id="struct.SetuptoolsConfig"></span>`struct SetuptoolsConfig`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


**Derives:** `Deserialize`

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `package_dir` | `Option < std :: collections :: HashMap < String , String > >` |  |



### <span id="struct.MaturinConfig"></span>`struct MaturinConfig`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


**Derives:** `Deserialize`

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `python_source` | `Option < String >` |  |
| `module_name` | `Option < String >` |  |



### <span id="struct.InferredConfig"></span>`struct InferredConfig`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


**Derives:** `Debug`, `Default`

Inferred configuration values from manifest files

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `project_name` | `Option < String >` | Project name (from Cargo.toml or pyproject.toml) |
| `rust_crates` | `Option < Vec < PathBuf > >` | Rust crate paths (from workspace members or current directory) |
| `rust_entry_point` | `Option < String >` | Rust entry point (package name from Cargo.toml) |
| `python_package` | `Option < String >` | Python package name (from pyproject.toml) |
| `python_source` | `Option < PathBuf >` | Python source directory (from tool.maturin.python-source) |

#### Methods

##### <span id="InferredConfig.from_directory"></span>`from_directory`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn from_directory (project_root : & Path) -> Self
```

Infer configuration from manifest files in the given directory

<details>
<summary>Source</summary>

```rust
    pub fn from_directory(project_root: &Path) -> Self {
        let mut inferred = InferredConfig::default();

        // Try to parse Cargo.toml
        let cargo_path = project_root.join("Cargo.toml");
        if cargo_path.exists()
            && let Ok(cargo) = CargoManifest::parse(&cargo_path)
        {
            // Project name from package name
            if let Some(name) = &cargo.name {
                inferred.project_name = Some(name.clone());
                inferred.rust_entry_point = Some(name.clone());
            }

            // Rust crates from workspace members or current directory
            if cargo.is_workspace && !cargo.workspace_members.is_empty() {
                inferred.rust_crates =
                    Some(cargo.workspace_members.iter().map(PathBuf::from).collect());
            } else if cargo.name.is_some() {
                // Single crate project
                inferred.rust_crates = Some(vec![PathBuf::from(".")]);
            }
        }

        // Try to parse pyproject.toml
        let pyproject_path = project_root.join("pyproject.toml");
        if pyproject_path.exists()
            && let Ok(pyproject) = PyProjectManifest::parse(&pyproject_path)
        {
            // Project name from pyproject takes precedence
            if let Some(name) = &pyproject.name {
                inferred.project_name = Some(name.clone());
                // Python package name (convert dashes to underscores)
                inferred.python_package = Some(name.replace('-', "_"));
            }

            // Python source directory
            if let Some(pkg_dir) = pyproject.package_dir {
                inferred.python_source = Some(pkg_dir);
            }
        }

        inferred
    }
```

</details>





## Enums

### <span id="enum.ManifestError"></span>`enum ManifestError`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


Errors that can occur when parsing manifest files

#### Variants

- **`IoError`**
- **`TomlError`**



