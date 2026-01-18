//! Tera-based documentation renderer
//!
//! This module provides the `Renderer` struct which combines Tera templating
//! with theme adapters to generate styled documentation output.

use super::templates::TemplateLoader;
use super::theme::{ThemeAdapter, get_theme_adapter};
use std::path::Path;
use tera::{Context, Tera};

/// Documentation renderer using Tera templates with SSG-native theming.
///
/// The `Renderer` holds a Tera instance with pre-loaded templates and a
/// theme adapter for CSS variable injection. Templates access theme values
/// through the `theme` context variable.
///
/// # Template Customization
///
/// Templates can be customized by placing override files in `.plissken/templates/`
/// within your project root. User templates take precedence over bundled defaults.
///
/// # Example
///
/// ```rust
/// use plissken_core::render::Renderer;
///
/// // Without user overrides
/// let renderer = Renderer::new(Some("mkdocs-material"), None).unwrap();
/// let output = renderer.badge_async().unwrap();
/// assert!(output.contains("var(--md-"));
///
/// // With user overrides from project root
/// use std::path::Path;
/// let renderer = Renderer::new(Some("mkdocs-material"), Some(Path::new("."))).unwrap();
/// ```
pub struct Renderer {
    tera: Tera,
    theme: Box<dyn ThemeAdapter>,
    template_loader: TemplateLoader,
}

impl Renderer {
    /// Create a new renderer with the specified template theme.
    ///
    /// # Arguments
    ///
    /// * `template` - Optional template name (e.g., "mkdocs-material", "mdbook")
    /// * `project_root` - Optional project root for user template overrides.
    ///   If provided, templates in `.plissken/templates/` will override defaults.
    ///
    /// # Errors
    ///
    /// Returns `PlisskenError::Template` if template initialization fails.
    pub fn new(template: Option<&str>, project_root: Option<&Path>) -> crate::error::Result<Self> {
        let template_loader = TemplateLoader::new(project_root);
        let mut tera = Tera::default();

        // Load templates from the template loader (supports user overrides)
        tera.add_raw_templates(vec![
            (
                "partials/badge.html",
                template_loader.get("partials/badge.html")?,
            ),
            (
                "partials/code_block.html",
                template_loader.get("partials/code_block.html")?,
            ),
            (
                "partials/signature.html",
                template_loader.get("partials/signature.html")?,
            ),
            ("module.html", template_loader.get("module.html")?),
        ])?;

        let theme = get_theme_adapter(template);

        Ok(Self {
            tera,
            theme,
            template_loader,
        })
    }

    /// Check if a user override exists for the given template.
    pub fn has_user_override(&self, template_name: &str) -> bool {
        self.template_loader.has_user_override(template_name)
    }

    /// Get the template loader for direct access to templates.
    pub fn template_loader(&self) -> &TemplateLoader {
        &self.template_loader
    }

    /// Get the theme adapter name.
    pub fn theme_name(&self) -> &str {
        self.theme.name()
    }

    /// Create a base Tera context with theme values injected.
    ///
    /// This context includes the `theme` object with all CSS variable mappings,
    /// ready for use in templates.
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

    /// Render a badge with the given text, color type, and semantic type.
    ///
    /// # Arguments
    ///
    /// * `text` - The badge text (e.g., "async", "deprecated")
    /// * `color_type` - Color type: "blue", "green", "yellow", "red", "purple", "orange", "gray"
    /// * `badge_type` - Semantic type for CSS class (e.g., "async", "visibility", "source")
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

    // =========================================================================
    // Semantic Badge Methods
    // =========================================================================

    /// Render an "async" badge (blue).
    pub fn badge_async(&self) -> Result<String, tera::Error> {
        self.render_badge("async", "blue", "async")
    }

    /// Render an "unsafe" badge (red).
    pub fn badge_unsafe(&self) -> Result<String, tera::Error> {
        self.render_badge("unsafe", "red", "unsafe")
    }

    /// Render a "deprecated" badge (yellow).
    pub fn badge_deprecated(&self) -> Result<String, tera::Error> {
        self.render_badge("deprecated", "yellow", "deprecated")
    }

    /// Render a visibility badge.
    ///
    /// # Arguments
    ///
    /// * `visibility` - One of: "pub", "pub(crate)", "private"
    pub fn badge_visibility(&self, visibility: &str) -> Result<String, tera::Error> {
        let color = match visibility {
            "pub" => "green",
            "pub(crate)" => "orange",
            _ => "gray", // private
        };
        self.render_badge(visibility, color, "visibility")
    }

    /// Render a source type badge with emoji.
    ///
    /// # Arguments
    ///
    /// * `source_type` - One of: "python", "rust", "binding"
    pub fn badge_source(&self, source_type: &str) -> Result<String, tera::Error> {
        let (text, color) = match source_type.to_lowercase().as_str() {
            "python" => ("Python", "blue"),
            "rust" => ("Rust", "orange"),
            "binding" | "pyo3" => ("Binding", "purple"),
            _ => (source_type, "gray"),
        };
        self.render_badge(text, color, "source")
    }

    /// Render a code block with optional language and caption.
    ///
    /// # Arguments
    ///
    /// * `code` - The code content
    /// * `language` - Optional language for syntax highlighting
    /// * `caption` - Optional caption/title for the code block
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

    /// Render a function/method signature.
    ///
    /// # Arguments
    ///
    /// * `name` - Function name
    /// * `params` - Parameter list as a string
    /// * `return_type` - Optional return type
    /// * `is_async` - Whether the function is async
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

    /// Render a module documentation page.
    ///
    /// # Arguments
    ///
    /// * `module_name` - Name of the module
    /// * `description` - Module description/docstring
    /// * `functions` - List of function documentation sections
    /// * `classes` - List of class documentation sections
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

    /// Render an arbitrary template with the given context.
    ///
    /// The theme values are automatically injected into the context.
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
}

/// Theme context for Tera templates.
///
/// This struct is serialized into the Tera context as the `theme` variable,
/// allowing templates to access theme values like `{{ theme.code_bg }}`.
#[derive(serde::Serialize)]
struct ThemeContext {
    // Core colors
    code_bg: String,
    code_fg: String,
    primary: String,
    accent: String,
    muted: String,
    border: String,
    name: String,
    // Semantic colors
    success: String,
    warning: String,
    error: String,
    info: String,
    // Badge colors
    badge_async: String,
    badge_unsafe: String,
    badge_deprecated: String,
    badge_binding: String,
    badge_pub: String,
    badge_pub_crate: String,
    badge_rust: String,
    badge_python: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = Renderer::new(Some("mkdocs-material"), None).unwrap();
        assert_eq!(renderer.theme_name(), "mkdocs-material");

        let renderer2 = Renderer::new(Some("mdbook"), None).unwrap();
        assert_eq!(renderer2.theme_name(), "mdbook");

        let renderer3 = Renderer::new(None, None).unwrap();
        assert_eq!(renderer3.theme_name(), "minimal");
    }

    #[test]
    fn test_base_context_has_theme() {
        let renderer = Renderer::new(Some("mkdocs-material"), None).unwrap();
        let ctx = renderer.base_context();

        // Context should have theme values
        let json = ctx.into_json();
        let theme = json.get("theme").expect("theme should exist");

        assert_eq!(theme.get("code_bg").unwrap(), "var(--md-code-bg-color)");
        assert_eq!(theme.get("name").unwrap(), "mkdocs-material");
    }

    #[test]
    fn test_render_badge_mkdocs() {
        let renderer = Renderer::new(Some("mkdocs-material"), None).unwrap();
        let badge = renderer.render_badge("async", "blue", "async").unwrap();

        assert!(badge.contains("async"));
        assert!(badge.contains("var(--md-primary-fg-color)"));
        assert!(badge.contains("display: inline-block"));
        assert!(badge.contains("plissken-badge"));
        assert!(badge.contains("plissken-badge-async"));
    }

    #[test]
    fn test_render_badge_colors() {
        let renderer = Renderer::new(None, None).unwrap();

        let blue = renderer.render_badge("info", "blue", "info").unwrap();
        assert!(blue.contains("#1976d2")); // minimal theme primary

        let green = renderer
            .render_badge("success", "green", "success")
            .unwrap();
        assert!(green.contains("#4caf50"));

        let yellow = renderer
            .render_badge("warning", "yellow", "warning")
            .unwrap();
        assert!(yellow.contains("#ff9800"));

        let red = renderer.render_badge("error", "red", "error").unwrap();
        assert!(red.contains("#f44336"));

        let purple = renderer
            .render_badge("binding", "purple", "source")
            .unwrap();
        assert!(purple.contains("#9c27b0"));

        let orange = renderer
            .render_badge("pub(crate)", "orange", "visibility")
            .unwrap();
        assert!(orange.contains("#ff5722"));

        let gray = renderer.render_badge("note", "gray", "note").unwrap();
        assert!(gray.contains("#757575")); // minimal theme muted
    }

    #[test]
    fn test_badge_async() {
        let renderer = Renderer::new(Some("mkdocs-material"), None).unwrap();
        let badge = renderer.badge_async().unwrap();

        assert!(badge.contains("async"));
        assert!(badge.contains("plissken-badge-async"));
        assert!(badge.contains("var(--md-primary-fg-color)")); // blue uses theme.primary
    }

    #[test]
    fn test_badge_unsafe() {
        let renderer = Renderer::new(None, None).unwrap();
        let badge = renderer.badge_unsafe().unwrap();

        assert!(badge.contains("unsafe"));
        assert!(badge.contains("plissken-badge-unsafe"));
        assert!(badge.contains("#f44336")); // red
    }

    #[test]
    fn test_badge_deprecated() {
        let renderer = Renderer::new(None, None).unwrap();
        let badge = renderer.badge_deprecated().unwrap();

        assert!(badge.contains("deprecated"));
        assert!(badge.contains("plissken-badge-deprecated"));
        assert!(badge.contains("#ff9800")); // yellow
    }

    #[test]
    fn test_badge_visibility() {
        let renderer = Renderer::new(None, None).unwrap();

        let pub_badge = renderer.badge_visibility("pub").unwrap();
        assert!(pub_badge.contains("pub"));
        assert!(pub_badge.contains("plissken-badge-visibility"));
        assert!(pub_badge.contains("#4caf50")); // green

        let pub_crate = renderer.badge_visibility("pub(crate)").unwrap();
        assert!(pub_crate.contains("pub(crate)"));
        assert!(pub_crate.contains("#ff5722")); // orange

        let private = renderer.badge_visibility("private").unwrap();
        assert!(private.contains("private"));
        assert!(private.contains("#757575")); // gray (muted)
    }

    #[test]
    fn test_badge_source() {
        let renderer = Renderer::new(None, None).unwrap();

        let python = renderer.badge_source("python").unwrap();
        assert!(python.contains("Python"));
        assert!(python.contains("plissken-badge-source"));
        assert!(python.contains("#1976d2")); // blue

        let rust = renderer.badge_source("rust").unwrap();
        assert!(rust.contains("Rust"));
        assert!(rust.contains("#ff5722")); // orange

        let binding = renderer.badge_source("binding").unwrap();
        assert!(binding.contains("Binding"));
        assert!(binding.contains("#9c27b0")); // purple

        // pyo3 is an alias for binding
        let pyo3 = renderer.badge_source("pyo3").unwrap();
        assert!(pyo3.contains("Binding"));
    }

    #[test]
    fn test_badge_has_override_class() {
        let renderer = Renderer::new(None, None).unwrap();

        // All badges should have the plissken-badge class for CSS override
        let badge = renderer.badge_async().unwrap();
        assert!(badge.contains(r#"class="plissken-badge plissken-badge-async""#));

        let badge = renderer.badge_source("rust").unwrap();
        assert!(badge.contains(r#"class="plissken-badge plissken-badge-source""#));
    }

    #[test]
    fn test_render_code_block() {
        let renderer = Renderer::new(Some("mdbook"), None).unwrap();
        let code = renderer
            .render_code_block("fn main() {}", Some("rust"), Some("Example"))
            .unwrap();

        assert!(code.contains("fn main() {}"));
        assert!(code.contains("language-rust"));
        assert!(code.contains("Example"));
        assert!(code.contains("var(--code-bg)"));
    }

    #[test]
    fn test_render_code_block_no_caption() {
        let renderer = Renderer::new(None, None).unwrap();
        let code = renderer
            .render_code_block("print(x)", Some("python"), None)
            .unwrap();

        assert!(code.contains("print(x)"));
        assert!(code.contains("language-python"));
        // Should not have caption div
        assert!(!code.contains("margin-bottom: 0.25em"));
    }

    #[test]
    fn test_render_signature() {
        let renderer = Renderer::new(Some("mkdocs-material"), None).unwrap();
        let sig = renderer
            .render_signature(
                "process_data",
                "data: str, timeout: int = 30",
                Some("Result"),
                false,
            )
            .unwrap();

        assert!(sig.contains("process_data"));
        assert!(sig.contains("data: str, timeout: int = 30"));
        assert!(sig.contains("Result"));
        assert!(sig.contains("var(--md-primary-fg-color)"));
        assert!(!sig.contains("async"));
    }

    #[test]
    fn test_render_signature_async() {
        let renderer = Renderer::new(Some("mkdocs-material"), None).unwrap();
        let sig = renderer
            .render_signature("fetch_data", "url: str", Some("bytes"), true)
            .unwrap();

        assert!(sig.contains("async"));
        assert!(sig.contains("var(--md-accent-fg-color)"));
    }

    #[test]
    fn test_render_module() {
        let renderer = Renderer::new(None, None).unwrap();
        let module = renderer
            .render_module(
                "my_module",
                "A test module for demonstration.",
                &["func1 docs".to_string(), "func2 docs".to_string()],
                &["Class1 docs".to_string()],
            )
            .unwrap();

        assert!(module.contains("# my_module"));
        assert!(module.contains("A test module for demonstration."));
        assert!(module.contains("## Functions"));
        assert!(module.contains("func1 docs"));
        assert!(module.contains("func2 docs"));
        assert!(module.contains("## Classes"));
        assert!(module.contains("Class1 docs"));
    }

    #[test]
    fn test_render_module_no_functions() {
        let renderer = Renderer::new(None, None).unwrap();
        let module = renderer
            .render_module(
                "constants",
                "Module containing only classes.",
                &[],
                &["MyClass docs".to_string()],
            )
            .unwrap();

        assert!(!module.contains("## Functions"));
        assert!(module.contains("## Classes"));
    }

    #[test]
    fn test_theme_values_in_templates() {
        // Test MkDocs Material CSS variables
        let mkdocs = Renderer::new(Some("mkdocs-material"), None).unwrap();
        let code = mkdocs.render_code_block("x = 1", None, None).unwrap();
        assert!(code.contains("var(--md-code-bg-color)"));
        assert!(code.contains("var(--md-code-fg-color)"));
        assert!(code.contains("var(--md-default-fg-color--lightest)")); // border

        // Test mdBook CSS variables
        let mdbook = Renderer::new(Some("mdbook"), None).unwrap();
        let code = mdbook.render_code_block("x = 1", None, None).unwrap();
        assert!(code.contains("var(--code-bg)"));
        assert!(code.contains("var(--inline-code-color)"));
        assert!(code.contains("var(--quote-border)")); // border

        // Test Minimal hex colors
        let minimal = Renderer::new(None, None).unwrap();
        let code = minimal.render_code_block("x = 1", None, None).unwrap();
        assert!(code.contains("#f5f5f5")); // code_bg
        assert!(code.contains("#333333")); // code_fg
        assert!(code.contains("#e0e0e0")); // border
    }
}
