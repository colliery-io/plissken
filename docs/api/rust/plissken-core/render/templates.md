# templates <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Template loading with user override support

This module provides template loading functionality that supports:
- Bundled default templates (embedded at compile time)
- User overrides from `.plissken/templates/` directory
User templates take precedence over bundled defaults on a per-file basis.

## Structs

### `struct TemplateLoader`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Template loader with user override support.

The loader first checks for user-provided templates in the configured
override directory, then falls back to bundled defaults.

**Examples:**

```rust
use plissken_core::render::TemplateLoader;
use std::path::Path;

// Create loader without user overrides (uses bundled only)
let loader = TemplateLoader::new(None);

// Create loader with project root for user overrides
let loader = TemplateLoader::new(Some(Path::new("/path/to/project")));

// Get a template
let badge_template = loader.get("partials/badge.html").unwrap();
```

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `bundled` | `HashMap < & 'static str , & 'static str >` | Bundled templates (name -> content) |
| `user_dir` | `Option < PathBuf >` | Optional user override directory |

#### Methods

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn new (project_root : Option < & Path >) -> Self
```

Create a new template loader.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `project_root` | `-` | Optional path to project root. If provided, user templates will be loaded from `{project_root}/.plissken/templates/`. |


<details>
<summary>Source</summary>

```rust
    pub fn new(project_root: Option<&Path>) -> Self {
        let user_dir = project_root.map(|root| {
            let dir = root.join(".plissken").join("templates");
            if dir.exists() && dir.is_dir() {
                Some(dir)
            } else {
                None
            }
        }).flatten();

        Self {
            bundled: Self::load_bundled(),
            user_dir,
        }
    }
```

</details>



##### `get` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn get (& self , name : & str) -> crate :: error :: Result < String >
```

Get a template by name.

First checks for a user override, then falls back to the bundled default.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `name` | `-` | Template name (e.g., "partials/badge.html", "module.html") |


**Returns:**

The template content as a string, or an error if not found.

<details>
<summary>Source</summary>

```rust
    pub fn get(&self, name: &str) -> crate::error::Result<String> {
        // Check user override first
        if let Some(ref dir) = self.user_dir {
            let user_path = dir.join(name);
            if user_path.exists() {
                return std::fs::read_to_string(&user_path)
                    .map_err(|e| crate::error::PlisskenError::file_read(&user_path, e));
            }
        }

        // Fall back to bundled
        self.bundled
            .get(name)
            .map(|s| s.to_string())
            .ok_or_else(|| crate::error::PlisskenError::Template {
                message: format!("template not found: {}", name),
                source: tera::Error::msg(format!("template '{}' not found in bundled templates", name)),
            })
    }
```

</details>



##### `has_user_override` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn has_user_override (& self , name : & str) -> bool
```

Check if a user override exists for the given template.

<details>
<summary>Source</summary>

```rust
    pub fn has_user_override(&self, name: &str) -> bool {
        if let Some(ref dir) = self.user_dir {
            dir.join(name).exists()
        } else {
            false
        }
    }
```

</details>



##### `template_names` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn template_names (& self) -> Vec < & 'static str >
```

List all available template names.

<details>
<summary>Source</summary>

```rust
    pub fn template_names(&self) -> Vec<&'static str> {
        self.bundled.keys().copied().collect()
    }
```

</details>



##### `user_override_dir` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn user_override_dir (& self) -> Option < & Path >
```

Get the user override directory, if configured and exists.

<details>
<summary>Source</summary>

```rust
    pub fn user_override_dir(&self) -> Option<&Path> {
        self.user_dir.as_deref()
    }
```

</details>



##### `load_bundled` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn load_bundled () -> HashMap < & 'static str , & 'static str >
```

<details>
<summary>Source</summary>

```rust
    fn load_bundled() -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();
        map.insert("partials/badge.html", bundled::BADGE);
        map.insert("partials/code_block.html", bundled::CODE_BLOCK);
        map.insert("partials/signature.html", bundled::SIGNATURE);
        map.insert("module.html", bundled::MODULE);
        map
    }
```

</details>





