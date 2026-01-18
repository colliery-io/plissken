# renderer <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Tera-based documentation renderer

This module provides the `Renderer` struct which combines Tera templating
with theme adapters to generate styled documentation output.

## Structs

### `struct Renderer`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Documentation renderer using Tera templates with SSG-native theming.

The `Renderer` holds a Tera instance with pre-loaded templates and a
theme adapter for CSS variable injection. Templates access theme values
through the `theme` context variable.

**Examples:**

```rust
use plissken_core::render::Renderer;

// Without user overrides
let renderer = Renderer::new(Some("mkdocs-material"), None).unwrap();
let output = renderer.badge_async().unwrap();
assert!(output.contains("var(--md-"));

// With user overrides from project root
use std::path::Path;
let renderer = Renderer::new(Some("mkdocs-material"), Some(Path::new("."))).unwrap();
```

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `tera` | `Tera` |  |
| `theme` | `Box < dyn ThemeAdapter >` |  |
| `template_loader` | `TemplateLoader` |  |

#### Methods

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn new (template : Option < & str > , project_root : Option < & Path >) -> crate :: error :: Result < Self >
```

Create a new renderer with the specified template theme.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `template` | `-` | Optional template name (e.g., "mkdocs-material", "mdbook") |
| `project_root` | `-` | Optional project root for user template overrides. If provided, templates in `.plissken/templates/` will override defaults. |


**Raises:**

| Exception | Description |
|-----------|-------------|
| `Error` | Returns `PlisskenError::Template` if template initialization fails. |


<details>
<summary>Source</summary>

```rust
    pub fn new(template: Option<&str>, project_root: Option<&Path>) -> crate::error::Result<Self> {
        let template_loader = TemplateLoader::new(project_root);
        let mut tera = Tera::default();

        // Load templates from the template loader (supports user overrides)
        tera.add_raw_templates(vec![
            ("partials/badge.html", template_loader.get("partials/badge.html")?),
            ("partials/code_block.html", template_loader.get("partials/code_block.html")?),
            ("partials/signature.html", template_loader.get("partials/signature.html")?),
            ("module.html", template_loader.get("module.html")?),
        ])?;

        let theme = get_theme_adapter(template);

        Ok(Self { tera, theme, template_loader })
    }
```

</details>



##### `has_user_override` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn has_user_override (& self , template_name : & str) -> bool
```

Check if a user override exists for the given template.

<details>
<summary>Source</summary>

```rust
    pub fn has_user_override(&self, template_name: &str) -> bool {
        self.template_loader.has_user_override(template_name)
    }
```

</details>



##### `template_loader` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn template_loader (& self) -> & TemplateLoader
```

Get the template loader for direct access to templates.

<details>
<summary>Source</summary>

```rust
    pub fn template_loader(&self) -> &TemplateLoader {
        &self.template_loader
    }
```

</details>



##### `theme_name` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn theme_name (& self) -> & str
```

Get the theme adapter name.

<details>
<summary>Source</summary>

```rust
    pub fn theme_name(&self) -> &str {
        self.theme.name()
    }
```

</details>



##### `base_context` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn base_context (& self) -> Context
```

Create a base Tera context with theme values injected.

This context includes the `theme` object with all CSS variable mappings,
ready for use in templates.

<details>
<summary>Source</summary>

```rust
    pub fn base_context(&self) -> Context {
        let mut ctx = Context::new();

        // Inject theme values as a nested object
        ctx.insert(
            "theme",
            &ThemeContext {
                // Core colors
                code_bg: self.theme.code_bg().to_string(),
                code_fg: self.theme.code_fg().to_string(),
                primary: self.theme.primary().to_string(),
                accent: self.theme.accent().to_string(),
                muted: self.theme.muted().to_string(),
                border: self.theme.border().to_string(),
                name: self.theme.name().to_string(),
                // Semantic colors
                success: self.theme.success().to_string(),
                warning: self.theme.warning().to_string(),
                error: self.theme.error().to_string(),
                info: self.theme.info().to_string(),
                // Badge colors
                badge_async: self.theme.badge_async().to_string(),
                badge_unsafe: self.theme.badge_unsafe().to_string(),
                badge_deprecated: self.theme.badge_deprecated().to_string(),
                badge_binding: self.theme.badge_binding().to_string(),
                badge_pub: self.theme.badge_pub().to_string(),
                badge_pub_crate: self.theme.badge_pub_crate().to_string(),
                badge_rust: self.theme.badge_rust().to_string(),
                badge_python: self.theme.badge_python().to_string(),
            },
        );

        ctx
    }
```

</details>



##### `render_badge` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn render_badge (& self , text : & str , color_type : & str , badge_type : & str ,) -> Result < String , tera :: Error >
```

Render a badge with the given text, color type, and semantic type.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `text` | `-` | The badge text (e.g., "async", "deprecated") |
| `color_type` | `-` | Color type: "blue", "green", "yellow", "red", "purple", "orange", "gray" |
| `badge_type` | `-` | Semantic type for CSS class (e.g., "async", "visibility", "source") |


<details>
<summary>Source</summary>

```rust
    pub fn render_badge(
        &self,
        text: &str,
        color_type: &str,
        badge_type: &str,
    ) -> Result<String, tera::Error> {
        let mut ctx = self.base_context();
        ctx.insert("text", text);
        ctx.insert("color_type", color_type);
        ctx.insert("badge_type", badge_type);
        self.tera.render("partials/badge.html", &ctx)
    }
```

</details>



##### `badge_async` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn badge_async (& self) -> Result < String , tera :: Error >
```

Render an "async" badge (blue).

<details>
<summary>Source</summary>

```rust
    pub fn badge_async(&self) -> Result<String, tera::Error> {
        self.render_badge("async", "blue", "async")
    }
```

</details>



##### `badge_unsafe` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn badge_unsafe (& self) -> Result < String , tera :: Error >
```

Render an "unsafe" badge (red).

<details>
<summary>Source</summary>

```rust
    pub fn badge_unsafe(&self) -> Result<String, tera::Error> {
        self.render_badge("unsafe", "red", "unsafe")
    }
```

</details>



##### `badge_deprecated` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn badge_deprecated (& self) -> Result < String , tera :: Error >
```

Render a "deprecated" badge (yellow).

<details>
<summary>Source</summary>

```rust
    pub fn badge_deprecated(&self) -> Result<String, tera::Error> {
        self.render_badge("deprecated", "yellow", "deprecated")
    }
```

</details>



##### `badge_visibility` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn badge_visibility (& self , visibility : & str) -> Result < String , tera :: Error >
```

Render a visibility badge.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `visibility` | `-` | One of: "pub", "pub(crate)", "private" |


<details>
<summary>Source</summary>

```rust
    pub fn badge_visibility(&self, visibility: &str) -> Result<String, tera::Error> {
        let color = match visibility {
            "pub" => "green",
            "pub(crate)" => "orange",
            _ => "gray", // private
        };
        self.render_badge(visibility, color, "visibility")
    }
```

</details>



##### `badge_source` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn badge_source (& self , source_type : & str) -> Result < String , tera :: Error >
```

Render a source type badge with emoji.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `source_type` | `-` | One of: "python", "rust", "binding" |


<details>
<summary>Source</summary>

```rust
    pub fn badge_source(&self, source_type: &str) -> Result<String, tera::Error> {
        let (text, color) = match source_type.to_lowercase().as_str() {
            "python" => ("Python", "blue"),
            "rust" => ("Rust", "orange"),
            "binding" | "pyo3" => ("Binding", "purple"),
            _ => (source_type, "gray"),
        };
        self.render_badge(text, color, "source")
    }
```

</details>



##### `render_code_block` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn render_code_block (& self , code : & str , language : Option < & str > , caption : Option < & str > ,) -> Result < String , tera :: Error >
```

Render a code block with optional language and caption.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `code` | `-` | The code content |
| `language` | `-` | Optional language for syntax highlighting |
| `caption` | `-` | Optional caption/title for the code block |


<details>
<summary>Source</summary>

```rust
    pub fn render_code_block(
        &self,
        code: &str,
        language: Option<&str>,
        caption: Option<&str>,
    ) -> Result<String, tera::Error> {
        let mut ctx = self.base_context();
        ctx.insert("code", code);
        ctx.insert("language", &language.unwrap_or(""));
        ctx.insert("caption", &caption.unwrap_or(""));
        self.tera.render("partials/code_block.html", &ctx)
    }
```

</details>



##### `render_signature` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn render_signature (& self , name : & str , params : & str , return_type : Option < & str > , is_async : bool ,) -> Result < String , tera :: Error >
```

Render a function/method signature.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `name` | `-` | Function name |
| `params` | `-` | Parameter list as a string |
| `return_type` | `-` | Optional return type |
| `is_async` | `-` | Whether the function is async |


<details>
<summary>Source</summary>

```rust
    pub fn render_signature(
        &self,
        name: &str,
        params: &str,
        return_type: Option<&str>,
        is_async: bool,
    ) -> Result<String, tera::Error> {
        let mut ctx = self.base_context();
        ctx.insert("name", name);
        ctx.insert("params", params);
        ctx.insert("return_type", &return_type.unwrap_or(""));
        ctx.insert("is_async", &is_async);
        self.tera.render("partials/signature.html", &ctx)
    }
```

</details>



##### `render_module` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn render_module (& self , module_name : & str , description : & str , functions : & [String] , classes : & [String] ,) -> Result < String , tera :: Error >
```

Render a module documentation page.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `module_name` | `-` | Name of the module |
| `description` | `-` | Module description/docstring |
| `functions` | `-` | List of function documentation sections |
| `classes` | `-` | List of class documentation sections |


<details>
<summary>Source</summary>

```rust
    pub fn render_module(
        &self,
        module_name: &str,
        description: &str,
        functions: &[String],
        classes: &[String],
    ) -> Result<String, tera::Error> {
        let mut ctx = self.base_context();
        ctx.insert("module_name", module_name);
        ctx.insert("description", description);
        ctx.insert("functions", functions);
        ctx.insert("classes", classes);
        self.tera.render("module.html", &ctx)
    }
```

</details>



##### `render_template` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn render_template (& self , template_name : & str , extra_context : & Context ,) -> Result < String , tera :: Error >
```

Render an arbitrary template with the given context.

The theme values are automatically injected into the context.

<details>
<summary>Source</summary>

```rust
    pub fn render_template(
        &self,
        template_name: &str,
        extra_context: &Context,
    ) -> Result<String, tera::Error> {
        let mut ctx = self.base_context();
        // Merge extra context (extra context values override base if there are conflicts)
        let json_value = extra_context.clone().into_json();
        let obj = json_value
            .as_object()
            .ok_or_else(|| tera::Error::msg("extra_context must serialize to a JSON object"))?;
        for (key, value) in obj {
            ctx.insert(key, value);
        }
        self.tera.render(template_name, &ctx)
    }
```

</details>





### `struct ThemeContext`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


**Derives:** `serde :: Serialize`

Theme context for Tera templates.

This struct is serialized into the Tera context as the `theme` variable,
allowing templates to access theme values like `{{ theme.code_bg }}`.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `code_bg` | `String` |  |
| `code_fg` | `String` |  |
| `primary` | `String` |  |
| `accent` | `String` |  |
| `muted` | `String` |  |
| `border` | `String` |  |
| `name` | `String` |  |
| `success` | `String` |  |
| `warning` | `String` |  |
| `error` | `String` |  |
| `info` | `String` |  |
| `badge_async` | `String` |  |
| `badge_unsafe` | `String` |  |
| `badge_deprecated` | `String` |  |
| `badge_binding` | `String` |  |
| `badge_pub` | `String` |  |
| `badge_pub_crate` | `String` |  |
| `badge_rust` | `String` |  |
| `badge_python` | `String` |  |



