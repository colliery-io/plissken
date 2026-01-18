//! Theme adapters for SSG-native CSS variable integration
//!
//! This module provides adapters that map semantic color names to the native
//! CSS variables of various static site generators. By using inline styles
//! that reference these variables, generated documentation automatically
//! inherits the SSG's theme (including dark mode support) without shipping
//! any external CSS.
//!
//! # Supported SSGs
//!
//! - **MkDocs Material**: Uses `var(--md-*)` CSS variables
//! - **mdBook**: Uses `var(--*)` CSS variables
//! - **Minimal**: Hardcoded fallback colors for unstyled contexts
//!
//! # Example
//!
//! ```rust
//! use plissken_core::render::{get_theme_adapter, ThemeAdapter};
//!
//! let adapter = get_theme_adapter(Some("mkdocs-material"));
//! let style = format!("background: {}; color: {}", adapter.code_bg(), adapter.code_fg());
//! // Results in: "background: var(--md-code-bg-color); color: var(--md-code-fg-color)"
//! ```

/// Trait for mapping semantic color names to SSG-specific CSS variables.
///
/// Implementations of this trait provide the actual CSS variable references
/// (or hardcoded colors) for each semantic color role. Templates use these
/// methods to generate inline styles that integrate with the target SSG's
/// theming system.
pub trait ThemeAdapter: Send + Sync {
    // =========================================================================
    // Core Colors
    // =========================================================================

    /// Background color for code blocks and inline code
    fn code_bg(&self) -> &str;

    /// Foreground (text) color for code
    fn code_fg(&self) -> &str;

    /// Primary/brand color for headings, important elements
    fn primary(&self) -> &str;

    /// Accent color for links, interactive elements
    fn accent(&self) -> &str;

    /// Muted/secondary text color
    fn muted(&self) -> &str;

    /// Border color for tables, separators
    fn border(&self) -> &str;

    /// Name of this theme adapter (for debugging/logging)
    fn name(&self) -> &str;

    // =========================================================================
    // Semantic Colors (status indicators)
    // =========================================================================

    /// Success color (typically green) - for positive states
    fn success(&self) -> &str {
        "#4caf50"
    }

    /// Warning color (typically yellow/orange) - for caution states
    fn warning(&self) -> &str {
        "#ff9800"
    }

    /// Error color (typically red) - for error/danger states
    fn error(&self) -> &str {
        "#f44336"
    }

    /// Info color (typically blue) - for informational states
    fn info(&self) -> &str {
        "#2196f3"
    }

    // =========================================================================
    // Badge Colors (specific badge styling)
    // =========================================================================

    /// Color for async badges
    fn badge_async(&self) -> &str {
        self.primary()
    }

    /// Color for unsafe badges
    fn badge_unsafe(&self) -> &str {
        self.error()
    }

    /// Color for deprecated badges
    fn badge_deprecated(&self) -> &str {
        self.warning()
    }

    /// Color for PyO3/binding badges
    fn badge_binding(&self) -> &str {
        "#9c27b0" // purple
    }

    /// Color for visibility pub badges
    fn badge_pub(&self) -> &str {
        self.success()
    }

    /// Color for visibility pub(crate) badges
    fn badge_pub_crate(&self) -> &str {
        "#ff5722" // deep orange
    }

    /// Color for Rust source badges
    fn badge_rust(&self) -> &str {
        "#ff5722" // deep orange (Rust's brand color is orange-ish)
    }

    /// Color for Python source badges
    fn badge_python(&self) -> &str {
        "#306998" // Python blue
    }
}

/// MkDocs Material theme adapter
///
/// Uses MkDocs Material's `--md-*` CSS custom properties for seamless
/// integration with Material for MkDocs themes.
///
/// Reference: <https://squidfunk.github.io/mkdocs-material/setup/changing-the-colors/>
#[derive(Debug, Clone, Copy, Default)]
pub struct MkDocsMaterial;

impl ThemeAdapter for MkDocsMaterial {
    fn code_bg(&self) -> &str {
        "var(--md-code-bg-color)"
    }

    fn code_fg(&self) -> &str {
        "var(--md-code-fg-color)"
    }

    fn primary(&self) -> &str {
        "var(--md-primary-fg-color)"
    }

    fn accent(&self) -> &str {
        "var(--md-accent-fg-color)"
    }

    fn muted(&self) -> &str {
        "var(--md-default-fg-color--light)"
    }

    fn border(&self) -> &str {
        "var(--md-default-fg-color--lightest)"
    }

    fn name(&self) -> &str {
        "mkdocs-material"
    }

    // MkDocs Material uses CSS variables that adapt to dark mode automatically
    fn badge_async(&self) -> &str {
        "var(--md-primary-fg-color)"
    }

    fn badge_binding(&self) -> &str {
        "var(--md-accent-fg-color)"
    }
}

/// mdBook theme adapter
///
/// Uses mdBook's CSS custom properties for integration with mdBook themes.
/// mdBook uses simpler variable names without a prefix.
///
/// Reference: <https://rust-lang.github.io/mdBook/format/theme/index.html>
#[derive(Debug, Clone, Copy, Default)]
pub struct MdBook;

impl ThemeAdapter for MdBook {
    fn code_bg(&self) -> &str {
        "var(--code-bg)"
    }

    fn code_fg(&self) -> &str {
        "var(--inline-code-color)"
    }

    fn primary(&self) -> &str {
        "var(--links)"
    }

    fn accent(&self) -> &str {
        "var(--links)"
    }

    fn muted(&self) -> &str {
        "var(--fg)"
    }

    fn border(&self) -> &str {
        "var(--quote-border)"
    }

    fn name(&self) -> &str {
        "mdbook"
    }
}

/// Minimal theme adapter with hardcoded fallback colors
///
/// Provides reasonable default colors for contexts where CSS variables
/// are not available (plain markdown viewers, unstyled HTML, etc.).
/// Uses a light theme with accessible contrast ratios.
#[derive(Debug, Clone, Copy, Default)]
pub struct Minimal;

impl ThemeAdapter for Minimal {
    fn code_bg(&self) -> &str {
        "#f5f5f5"
    }

    fn code_fg(&self) -> &str {
        "#333333"
    }

    fn primary(&self) -> &str {
        "#1976d2"
    }

    fn accent(&self) -> &str {
        "#ff4081"
    }

    fn muted(&self) -> &str {
        "#757575"
    }

    fn border(&self) -> &str {
        "#e0e0e0"
    }

    fn name(&self) -> &str {
        "minimal"
    }
}

/// Get a theme adapter based on the template name from config.
///
/// Returns the appropriate adapter for known SSG templates, or the
/// `Minimal` adapter as a fallback for unknown templates.
///
/// # Arguments
///
/// * `template` - Optional template name from `output.template` in config
///
/// # Returns
///
/// A boxed trait object implementing `ThemeAdapter`
///
/// # Examples
///
/// ```rust
/// use plissken_core::render::get_theme_adapter;
///
/// // Known templates
/// let mkdocs = get_theme_adapter(Some("mkdocs-material"));
/// assert_eq!(mkdocs.name(), "mkdocs-material");
///
/// let mdbook = get_theme_adapter(Some("mdbook"));
/// assert_eq!(mdbook.name(), "mdbook");
///
/// // Unknown or missing template falls back to minimal
/// let minimal = get_theme_adapter(None);
/// assert_eq!(minimal.name(), "minimal");
/// ```
pub fn get_theme_adapter(template: Option<&str>) -> Box<dyn ThemeAdapter> {
    match template {
        Some(t) => match t.to_lowercase().as_str() {
            "mkdocs-material" | "mkdocs_material" | "material" => Box::new(MkDocsMaterial),
            "mdbook" | "md-book" | "md_book" => Box::new(MdBook),
            _ => Box::new(Minimal),
        },
        None => Box::new(Minimal),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mkdocs_material_adapter() {
        let adapter = MkDocsMaterial;

        assert_eq!(adapter.code_bg(), "var(--md-code-bg-color)");
        assert_eq!(adapter.code_fg(), "var(--md-code-fg-color)");
        assert_eq!(adapter.primary(), "var(--md-primary-fg-color)");
        assert_eq!(adapter.accent(), "var(--md-accent-fg-color)");
        assert_eq!(adapter.muted(), "var(--md-default-fg-color--light)");
        assert_eq!(adapter.border(), "var(--md-default-fg-color--lightest)");
        assert_eq!(adapter.name(), "mkdocs-material");
    }

    #[test]
    fn test_mdbook_adapter() {
        let adapter = MdBook;

        assert_eq!(adapter.code_bg(), "var(--code-bg)");
        assert_eq!(adapter.code_fg(), "var(--inline-code-color)");
        assert_eq!(adapter.primary(), "var(--links)");
        assert_eq!(adapter.accent(), "var(--links)");
        assert_eq!(adapter.muted(), "var(--fg)");
        assert_eq!(adapter.border(), "var(--quote-border)");
        assert_eq!(adapter.name(), "mdbook");
    }

    #[test]
    fn test_minimal_adapter() {
        let adapter = Minimal;

        // Minimal uses hardcoded hex colors
        assert_eq!(adapter.code_bg(), "#f5f5f5");
        assert_eq!(adapter.code_fg(), "#333333");
        assert_eq!(adapter.primary(), "#1976d2");
        assert_eq!(adapter.accent(), "#ff4081");
        assert_eq!(adapter.muted(), "#757575");
        assert_eq!(adapter.border(), "#e0e0e0");
        assert_eq!(adapter.name(), "minimal");
    }

    #[test]
    fn test_get_theme_adapter_mkdocs_material() {
        // Various ways to specify mkdocs-material
        let adapter1 = get_theme_adapter(Some("mkdocs-material"));
        assert_eq!(adapter1.name(), "mkdocs-material");

        let adapter2 = get_theme_adapter(Some("mkdocs_material"));
        assert_eq!(adapter2.name(), "mkdocs-material");

        let adapter3 = get_theme_adapter(Some("material"));
        assert_eq!(adapter3.name(), "mkdocs-material");

        let adapter4 = get_theme_adapter(Some("MkDocs-Material"));
        assert_eq!(adapter4.name(), "mkdocs-material");
    }

    #[test]
    fn test_get_theme_adapter_mdbook() {
        // Various ways to specify mdbook
        let adapter1 = get_theme_adapter(Some("mdbook"));
        assert_eq!(adapter1.name(), "mdbook");

        let adapter2 = get_theme_adapter(Some("md-book"));
        assert_eq!(adapter2.name(), "mdbook");

        let adapter3 = get_theme_adapter(Some("md_book"));
        assert_eq!(adapter3.name(), "mdbook");

        let adapter4 = get_theme_adapter(Some("MDBOOK"));
        assert_eq!(adapter4.name(), "mdbook");
    }

    #[test]
    fn test_get_theme_adapter_fallback() {
        // Unknown template falls back to minimal
        let adapter1 = get_theme_adapter(Some("unknown-ssg"));
        assert_eq!(adapter1.name(), "minimal");

        let adapter2 = get_theme_adapter(Some("docusaurus"));
        assert_eq!(adapter2.name(), "minimal");

        // None also falls back to minimal
        let adapter3 = get_theme_adapter(None);
        assert_eq!(adapter3.name(), "minimal");
    }

    #[test]
    fn test_adapters_return_valid_css() {
        // Verify MkDocs Material returns valid CSS variable syntax
        let mkdocs = MkDocsMaterial;
        assert!(mkdocs.code_bg().starts_with("var(--md-"));
        assert!(mkdocs.code_bg().ends_with(")"));

        // Verify mdBook returns valid CSS variable syntax
        let mdbook = MdBook;
        assert!(mdbook.code_bg().starts_with("var(--"));
        assert!(mdbook.code_bg().ends_with(")"));

        // Verify Minimal returns valid hex colors
        let minimal = Minimal;
        assert!(minimal.code_bg().starts_with("#"));
        assert_eq!(minimal.code_bg().len(), 7); // #RRGGBB format
    }

    #[test]
    fn test_semantic_colors() {
        let adapter = Minimal;

        // Default semantic colors
        assert_eq!(adapter.success(), "#4caf50");
        assert_eq!(adapter.warning(), "#ff9800");
        assert_eq!(adapter.error(), "#f44336");
        assert_eq!(adapter.info(), "#2196f3");
    }

    #[test]
    fn test_badge_colors() {
        let adapter = Minimal;

        // Badge colors should return valid hex or CSS variable
        assert!(adapter.badge_async().starts_with("#") || adapter.badge_async().starts_with("var("));
        assert!(adapter.badge_unsafe().starts_with("#") || adapter.badge_unsafe().starts_with("var("));
        assert!(adapter.badge_deprecated().starts_with("#") || adapter.badge_deprecated().starts_with("var("));
        assert!(adapter.badge_binding().starts_with("#"));
        assert!(adapter.badge_pub().starts_with("#") || adapter.badge_pub().starts_with("var("));
        assert!(adapter.badge_pub_crate().starts_with("#"));
        assert!(adapter.badge_rust().starts_with("#"));
        assert!(adapter.badge_python().starts_with("#"));
    }

    #[test]
    fn test_mkdocs_material_badge_colors() {
        let adapter = MkDocsMaterial;

        // MkDocs Material should use CSS variables for badges
        assert_eq!(adapter.badge_async(), "var(--md-primary-fg-color)");
        assert_eq!(adapter.badge_binding(), "var(--md-accent-fg-color)");

        // Other badges use defaults (hardcoded colors that still work in dark mode)
        assert!(adapter.badge_unsafe().starts_with("#") || adapter.badge_unsafe().starts_with("var("));
    }

    #[test]
    fn test_badge_colors_default_to_semantic() {
        let adapter = Minimal;

        // badge_async defaults to primary
        assert_eq!(adapter.badge_async(), adapter.primary());

        // badge_unsafe defaults to error
        assert_eq!(adapter.badge_unsafe(), adapter.error());

        // badge_deprecated defaults to warning
        assert_eq!(adapter.badge_deprecated(), adapter.warning());

        // badge_pub defaults to success
        assert_eq!(adapter.badge_pub(), adapter.success());
    }
}
