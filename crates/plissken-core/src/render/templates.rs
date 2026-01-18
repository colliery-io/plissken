//! Template loading with user override support
//!
//! This module provides template loading functionality that supports:
//! - Bundled default templates (embedded at compile time)
//! - User overrides from `.plissken/templates/` directory
//!
//! User templates take precedence over bundled defaults on a per-file basis.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Bundled templates embedded at compile time.
mod bundled {
    pub const BADGE: &str = include_str!("../../templates/partials/badge.html");
    pub const CODE_BLOCK: &str = include_str!("../../templates/partials/code_block.html");
    pub const SIGNATURE: &str = include_str!("../../templates/partials/signature.html");
    pub const MODULE: &str = include_str!("../../templates/module.html");
}

/// Template loader with user override support.
///
/// The loader first checks for user-provided templates in the configured
/// override directory, then falls back to bundled defaults.
///
/// # Example
///
/// ```rust
/// use plissken_core::render::TemplateLoader;
/// use std::path::Path;
///
/// // Create loader without user overrides (uses bundled only)
/// let loader = TemplateLoader::new(None);
///
/// // Create loader with project root for user overrides
/// let loader = TemplateLoader::new(Some(Path::new("/path/to/project")));
///
/// // Get a template
/// let badge_template = loader.get("partials/badge.html").unwrap();
/// ```
pub struct TemplateLoader {
    /// Bundled templates (name -> content)
    bundled: HashMap<&'static str, &'static str>,
    /// Optional user override directory
    user_dir: Option<PathBuf>,
}

impl TemplateLoader {
    /// Create a new template loader.
    ///
    /// # Arguments
    ///
    /// * `project_root` - Optional path to project root. If provided, user
    ///   templates will be loaded from `{project_root}/.plissken/templates/`.
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

    /// Get a template by name.
    ///
    /// First checks for a user override, then falls back to the bundled default.
    ///
    /// # Arguments
    ///
    /// * `name` - Template name (e.g., "partials/badge.html", "module.html")
    ///
    /// # Returns
    ///
    /// The template content as a string, or an error if not found.
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

    /// Check if a user override exists for the given template.
    pub fn has_user_override(&self, name: &str) -> bool {
        if let Some(ref dir) = self.user_dir {
            dir.join(name).exists()
        } else {
            false
        }
    }

    /// List all available template names.
    pub fn template_names(&self) -> Vec<&'static str> {
        self.bundled.keys().copied().collect()
    }

    /// Get the user override directory, if configured and exists.
    pub fn user_override_dir(&self) -> Option<&Path> {
        self.user_dir.as_deref()
    }

    fn load_bundled() -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();
        map.insert("partials/badge.html", bundled::BADGE);
        map.insert("partials/code_block.html", bundled::CODE_BLOCK);
        map.insert("partials/signature.html", bundled::SIGNATURE);
        map.insert("module.html", bundled::MODULE);
        map
    }
}

impl Default for TemplateLoader {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_load_bundled_templates() {
        let loader = TemplateLoader::new(None);

        // Should have all bundled templates
        assert!(loader.get("partials/badge.html").is_ok());
        assert!(loader.get("partials/code_block.html").is_ok());
        assert!(loader.get("partials/signature.html").is_ok());
        assert!(loader.get("module.html").is_ok());
    }

    #[test]
    fn test_bundled_template_content() {
        let loader = TemplateLoader::new(None);

        let badge = loader.get("partials/badge.html").unwrap();
        assert!(badge.contains("plissken-badge"));
        assert!(badge.contains("{{ badge_type }}"));

        let module = loader.get("module.html").unwrap();
        assert!(module.contains("{{ module_name }}"));
        assert!(module.contains("## Functions"));
    }

    #[test]
    fn test_template_not_found() {
        let loader = TemplateLoader::new(None);

        let result = loader.get("nonexistent.html");
        assert!(result.is_err());
    }

    #[test]
    fn test_user_override() {
        let temp_dir = TempDir::new().unwrap();
        let project_root = temp_dir.path();

        // Create user override directory
        let templates_dir = project_root.join(".plissken").join("templates").join("partials");
        fs::create_dir_all(&templates_dir).unwrap();

        // Create custom badge template
        let custom_badge = "<span class=\"custom-badge\">{{ text }}</span>";
        fs::write(templates_dir.join("badge.html"), custom_badge).unwrap();

        let loader = TemplateLoader::new(Some(project_root));

        // Should load user override
        let badge = loader.get("partials/badge.html").unwrap();
        assert_eq!(badge, custom_badge);
        assert!(loader.has_user_override("partials/badge.html"));

        // Other templates should still use bundled
        let module = loader.get("module.html").unwrap();
        assert!(module.contains("{{ module_name }}"));
        assert!(!loader.has_user_override("module.html"));
    }

    #[test]
    fn test_no_user_dir() {
        let temp_dir = TempDir::new().unwrap();
        let project_root = temp_dir.path();

        // Don't create .plissken/templates
        let loader = TemplateLoader::new(Some(project_root));

        // Should use bundled templates
        assert!(loader.get("partials/badge.html").is_ok());
        assert!(loader.user_override_dir().is_none());
    }

    #[test]
    fn test_template_names() {
        let loader = TemplateLoader::new(None);

        let names = loader.template_names();
        assert!(names.contains(&"partials/badge.html"));
        assert!(names.contains(&"partials/code_block.html"));
        assert!(names.contains(&"partials/signature.html"));
        assert!(names.contains(&"module.html"));
    }
}
