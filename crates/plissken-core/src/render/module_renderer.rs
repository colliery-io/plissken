//! Module page rendering for Python and Rust documentation
//!
//! This module provides rendering functionality for converting `PythonModule`
//! and `RustModule` structures into Markdown documentation files.

use crate::docstring::{parse_docstring, parse_rust_doc};
use crate::model::{
    CrossRef, ParamDoc, PythonClass, PythonFunction, PythonItem, PythonModule, PythonParam,
    PythonVariable, RustEnum, RustFunction, RustImpl, RustItem, RustModule, RustStruct, SourceType,
    Visibility,
};
use crate::render::docstring_renderer::render_docstring;
use crate::render::module::{CrossRefLinker, PageLayout};
use crate::render::ssg::{python_nav_entries, rust_nav_entries};
use std::collections::HashMap;
use std::path::PathBuf;

use super::Renderer;

/// Rendered output for a documentation file
#[derive(Debug, Clone)]
pub struct RenderedPage {
    /// Relative path for output (e.g., "my_module.md" or "rust/my_crate.md")
    pub path: PathBuf,
    /// The rendered Markdown content
    pub content: String,
}

/// Builder for constructing module documentation pages.
///
/// This provides a common structure for both Python and Rust module pages,
/// reducing code duplication between `render_python_module_inline` and
/// `render_rust_module_inline`.
struct ModulePageBuilder {
    content: String,
}

impl ModulePageBuilder {
    /// Create a new page builder
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    /// Add the module header with a badge
    fn add_header(&mut self, module_name: &str, badge: &str) {
        self.content
            .push_str(&format!("# {} {}\n\n", module_name, badge));
    }

    /// Add a parsed docstring section
    fn add_docstring(&mut self, docstring: &crate::model::ParsedDocstring) {
        self.content.push_str(&render_docstring(docstring));
        self.content.push_str("\n\n");
    }

    /// Add a section header (h2)
    fn add_section(&mut self, title: &str) {
        self.content.push_str(&format!("## {}\n\n", title));
    }

    /// Add rendered item content with spacing
    fn add_item(&mut self, item_content: &str) {
        self.content.push_str(item_content);
        self.content.push_str("\n\n");
    }

    /// Add a variables/constants table
    fn add_variables_table<T, F>(&mut self, title: &str, items: &[T], row_renderer: F)
    where
        F: Fn(&T) -> (String, String, String), // (name, type, desc)
    {
        if items.is_empty() {
            return;
        }
        self.content.push_str(&format!("## {}\n\n", title));
        self.content.push_str("| Name | Type | Description |\n");
        self.content.push_str("|------|------|-------------|\n");
        for item in items {
            let (name, ty, desc) = row_renderer(item);
            self.content
                .push_str(&format!("| `{}` | `{}` | {} |\n", name, ty, desc));
        }
        self.content.push('\n');
    }

    /// Build the final content
    fn build(self) -> String {
        self.content
    }
}

/// Module page renderer that converts DocModel modules into Markdown files.
pub struct ModuleRenderer<'a> {
    renderer: &'a Renderer,
    linker: CrossRefLinker,
}

// Allow dead code for page-per-item rendering methods (reserved for future use)
#[allow(dead_code)]
impl<'a> ModuleRenderer<'a> {
    /// Create a new module renderer
    pub fn new(renderer: &'a Renderer) -> Self {
        Self {
            renderer,
            linker: CrossRefLinker::empty(),
        }
    }

    /// Create a module renderer with cross-references for bi-directional linking
    pub fn with_cross_refs(renderer: &'a Renderer, cross_refs: Vec<CrossRef>) -> Self {
        Self {
            renderer,
            linker: CrossRefLinker::new(cross_refs),
        }
    }

    // =========================================================================
    // Python Module Rendering
    // =========================================================================

    /// Render a Python module to a single Markdown file with all content inline.
    ///
    /// This creates one file per module with classes, methods, and functions
    /// all rendered inline rather than as separate pages.
    pub fn render_python_module(
        &self,
        module: &PythonModule,
    ) -> Result<Vec<RenderedPage>, tera::Error> {
        // Separate classes and functions
        let mut classes = Vec::new();
        let mut functions = Vec::new();
        let mut variables = Vec::new();

        for item in &module.items {
            match item {
                PythonItem::Class(c) => classes.push(c),
                PythonItem::Function(f) => functions.push(f),
                PythonItem::Variable(v) => variables.push(v),
            }
        }

        // Render everything into a single page
        let page = self.render_python_module_inline(module, &classes, &functions, &variables)?;

        Ok(vec![page])
    }

    /// Render a Python module with all content inline in a single page.
    fn render_python_module_inline(
        &self,
        module: &PythonModule,
        classes: &[&PythonClass],
        functions: &[&PythonFunction],
        variables: &[&PythonVariable],
    ) -> Result<RenderedPage, tera::Error> {
        let mut builder = ModulePageBuilder::new();
        let layout = PageLayout::new();

        // Module header with source badge
        let source_badge = self.source_badge(&module.source_type)?;
        let module_name = module.path.split('.').next_back().unwrap_or(&module.path);
        builder.add_header(module_name, &source_badge);

        // Module docstring
        if let Some(ref docstring) = module.docstring {
            builder.add_docstring(&parse_docstring(docstring));
        }

        // Module-level variables (constants)
        builder.add_variables_table("Variables", variables, |var| {
            (
                var.name.clone(),
                var.ty.clone().unwrap_or_else(|| "-".to_string()),
                var.docstring.clone().unwrap_or_default(),
            )
        });

        // Classes section
        if !classes.is_empty() {
            builder.add_section("Classes");
            for class in classes {
                builder.add_item(&self.render_python_class_inline(class, &module.path)?);
            }
        }

        // Functions section
        if !functions.is_empty() {
            builder.add_section("Functions");
            for func in functions {
                builder.add_item(&self.render_python_function_inline(func, &module.path)?);
            }
        }

        let path = layout.python_module_page(&module.path);
        Ok(RenderedPage {
            path,
            content: builder.build(),
        })
    }

    /// Render a Python class inline (for single-page module format).
    fn render_python_class_inline(
        &self,
        class: &PythonClass,
        module_path: &str,
    ) -> Result<String, tera::Error> {
        let mut content = String::new();
        let is_binding = class.rust_impl.is_some();

        // Class header (h3 since Classes is h2)
        content.push_str(&format!("### `class {}`\n\n", class.name));

        // Base classes
        if !class.bases.is_empty() {
            content.push_str(&format!(
                "**Inherits from:** {}\n\n",
                class.bases.join(", ")
            ));
        }

        // For bindings, add link to Rust implementation
        if is_binding
            && let Some(link) = self
                .linker
                .rust_link_for_python_class(module_path, &class.name)
        {
            content.push_str(&link);
        }

        // Docstring - parse and render properly
        if let Some(ref docstring) = class.docstring {
            let parsed = parse_docstring(docstring);
            content.push_str(&render_docstring(&parsed));
            content.push_str("\n\n");
        }

        // Check if this is an Enum class
        let is_enum = class.bases.iter().any(|b| {
            b == "Enum" || b.ends_with(".Enum") || b == "IntEnum" || b.ends_with(".IntEnum")
        });

        // For enum classes, render variants as a list (unified format with Rust)
        if is_enum && !class.attributes.is_empty() {
            content.push_str("#### Variants\n\n");
            for attr in &class.attributes {
                content.push_str(&format!("- **`{}`**", attr.name));
                // Add value if present
                if let Some(ref value) = attr.value {
                    content.push_str(&format!(" = `{}`", value));
                }
                // Add description if present
                if let Some(ref doc) = attr.docstring {
                    content.push_str(&format!(" - {}", doc));
                }
                content.push('\n');
            }
            content.push('\n');
        } else if !class.attributes.is_empty() {
            // For regular classes, render attributes
            content.push_str("#### Attributes\n\n");
            content.push_str("| Name | Type | Description |\n");
            content.push_str("|------|------|-------------|\n");
            for attr in &class.attributes {
                let ty = attr.ty.as_deref().unwrap_or("-");
                let desc = attr.docstring.as_deref().unwrap_or("");
                content.push_str(&format!("| `{}` | `{}` | {} |\n", attr.name, ty, desc));
            }
            content.push('\n');
        }

        // Class methods (h4 since class is h3)
        if !class.methods.is_empty() {
            content.push_str("#### Methods\n\n");
            for method in &class.methods {
                let parent_class = if is_binding {
                    Some(class.name.as_str())
                } else {
                    None
                };
                content.push_str(&self.render_python_function_with_context(
                    method,
                    5,
                    module_path,
                    parent_class,
                )?);
                content.push_str("\n\n");
            }
        }

        Ok(content)
    }

    /// Render a Python function inline (for single-page module format).
    fn render_python_function_inline(
        &self,
        func: &PythonFunction,
        module_path: &str,
    ) -> Result<String, tera::Error> {
        let mut content = String::new();
        let is_binding = func.rust_impl.is_some();

        // Function header (h3 since Functions is h2)
        content.push_str(&format!("### `{}`", func.name));

        // Additional badges
        if func.is_async {
            content.push(' ');
            content.push_str(&self.renderer.badge_async()?);
        }
        content.push_str("\n\n");

        // Signature
        content.push_str(&self.renderer.render_signature(
            &func.name,
            &self.format_python_params(&func.signature.params),
            func.signature.return_type.as_deref(),
            func.is_async,
        )?);
        content.push_str("\n\n");

        // For bindings, add link to Rust implementation
        if is_binding
            && let Some(link) = self
                .linker
                .rust_link_for_python_function(module_path, &func.name)
        {
            content.push_str(&link);
        }

        // Docstring - parse and render with merged signature params for types
        if let Some(ref docstring) = func.docstring {
            content.push_str(&Self::render_docstring_with_merged_params(
                &func.signature.params,
                docstring,
                is_binding,
            ));
            content.push_str("\n\n");
        }

        // Source code (collapsible)
        if !func.source.source.is_empty() {
            content.push_str("<details>\n<summary>Source</summary>\n\n");
            content.push_str("```python\n");
            content.push_str(&func.source.source);
            content.push_str("\n```\n\n</details>\n\n");
        }

        Ok(content)
    }

    /// Render a Python module index page with class cards
    fn render_python_module_index(
        &self,
        module: &PythonModule,
        classes: &[&PythonClass],
        functions: &[&PythonFunction],
        variables: &[&PythonVariable],
        module_dir: &str,
    ) -> Result<RenderedPage, tera::Error> {
        let mut content = String::new();

        // Module header with source badge
        let source_badge = self.source_badge(&module.source_type)?;
        content.push_str(&format!("# {} {}\n\n", module.path, source_badge));

        // Module docstring - parse and render properly
        if let Some(ref docstring) = module.docstring {
            let parsed = parse_docstring(docstring);
            content.push_str(&render_docstring(&parsed));
            content.push_str("\n\n");
        }

        // Module-level variables (constants)
        if !variables.is_empty() {
            content.push_str("## Variables\n\n");
            content.push_str("| Name | Type | Description |\n");
            content.push_str("|------|------|-------------|\n");
            for var in variables {
                let ty = var.ty.as_deref().unwrap_or("-");
                let desc = var.docstring.as_deref().unwrap_or("");
                content.push_str(&format!("| `{}` | `{}` | {} |\n", var.name, ty, desc));
            }
            content.push('\n');
        }

        // Classes section with cards linking to individual pages
        if !classes.is_empty() {
            content.push_str("## Classes\n\n");
            for class in classes {
                let is_binding = class.rust_impl.is_some();
                let badge = if is_binding {
                    format!("{} ", self.renderer.badge_source("binding")?)
                } else {
                    String::new()
                };

                // Get first line of docstring as summary
                let summary = class
                    .docstring
                    .as_ref()
                    .map(|d| d.lines().next().unwrap_or("").to_string())
                    .unwrap_or_default();

                content.push_str(&format!(
                    "### {}[`{}`]({}.md)\n\n{}\n\n",
                    badge, class.name, class.name, summary
                ));
            }
        }

        // Functions section with links to individual function pages
        if !functions.is_empty() {
            content.push_str("## Functions\n\n");

            // List functions briefly with links to their individual pages
            for func in functions {
                let is_binding = func.rust_impl.is_some();
                let badge = if is_binding {
                    format!("{} ", self.renderer.badge_source("binding")?)
                } else {
                    String::new()
                };

                let summary = func
                    .docstring
                    .as_ref()
                    .map(|d| d.lines().next().unwrap_or("").to_string())
                    .unwrap_or_default();

                content.push_str(&format!("- {}[`{}`]({}.md)", badge, func.name, func.name));
                if !summary.is_empty() {
                    content.push_str(&format!(" - {}", summary));
                }
                content.push('\n');
            }
            content.push('\n');
        }

        let path = PathBuf::from(format!("{}/index.md", module_dir));
        Ok(RenderedPage { path, content })
    }

    /// Render a single Python class as its own page
    fn render_python_class_page(
        &self,
        class: &PythonClass,
        module_path: &str,
        module_dir: &str,
    ) -> Result<RenderedPage, tera::Error> {
        let mut content = String::new();
        let is_binding = class.rust_impl.is_some();

        // Class header with badges (h1 since it's the page title)
        let badge = if is_binding {
            format!("{} ", self.renderer.badge_source("binding")?)
        } else {
            String::new()
        };
        content.push_str(&format!("# {}`class {}`\n\n", badge, class.name));

        // Breadcrumb back to module
        content.push_str(&format!("*Module: [{}](index.md)*\n\n", module_path));

        // Base classes
        if !class.bases.is_empty() {
            content.push_str(&format!(
                "**Inherits from:** {}\n\n",
                class.bases.join(", ")
            ));
        }

        // For bindings, add link to Rust implementation
        if is_binding
            && let Some(link) = self
                .linker
                .rust_link_for_python_class(module_path, &class.name)
        {
            content.push_str(&link);
        }

        // Docstring - parse and render properly
        if let Some(ref docstring) = class.docstring {
            let parsed = parse_docstring(docstring);
            content.push_str(&render_docstring(&parsed));
            content.push_str("\n\n");
        }

        // Check if this is an Enum class
        let is_enum = class.bases.iter().any(|b| {
            b == "Enum" || b.ends_with(".Enum") || b == "IntEnum" || b.ends_with(".IntEnum")
        });

        // For enum classes, render members as a table
        if is_enum && !class.attributes.is_empty() {
            content.push_str("### Members\n\n");
            content.push_str("| Name | Value |\n");
            content.push_str("|------|-------|\n");
            for attr in &class.attributes {
                let value = attr.value.as_deref().unwrap_or("-");
                content.push_str(&format!("| `{}` | `{}` |\n", attr.name, value));
            }
            content.push('\n');
        } else if !class.attributes.is_empty() {
            // For regular classes, render attributes
            content.push_str("### Attributes\n\n");
            content.push_str("| Name | Type | Description |\n");
            content.push_str("|------|------|-------------|\n");
            for attr in &class.attributes {
                let ty = attr.ty.as_deref().unwrap_or("-");
                let desc = attr.docstring.as_deref().unwrap_or("");
                content.push_str(&format!("| `{}` | `{}` | {} |\n", attr.name, ty, desc));
            }
            content.push('\n');
        }

        // Class methods
        if !class.methods.is_empty() {
            content.push_str("### Methods\n\n");
            for method in &class.methods {
                // Pass parent class name for method-level cross-links
                let parent_class = if is_binding {
                    Some(class.name.as_str())
                } else {
                    None
                };
                content.push_str(&self.render_python_function_for_class_page(
                    method,
                    module_path,
                    &class.name,
                    parent_class,
                )?);
                content.push_str("\n\n");
            }
        }

        let path = PathBuf::from(format!("{}/{}.md", module_dir, class.name));
        Ok(RenderedPage { path, content })
    }

    /// Render Python module-level functions as their own page
    /// Render a single Python function to its own page
    fn render_python_function_page(
        &self,
        func: &PythonFunction,
        module_path: &str,
        module_dir: &str,
    ) -> Result<RenderedPage, tera::Error> {
        let mut content = String::new();
        let is_binding = func.rust_impl.is_some();

        // Function header with badges (h1 since it's the page title)
        let badge = if is_binding {
            format!("{} ", self.renderer.badge_source("binding")?)
        } else {
            String::new()
        };
        content.push_str(&format!("# {}`{}`", badge, func.name));

        // Additional badges
        if func.is_async {
            content.push(' ');
            content.push_str(&self.renderer.badge_async()?);
        }
        content.push_str("\n\n");

        // Breadcrumb back to module
        content.push_str(&format!("*Module: [{}](index.md)*\n\n", module_path));

        // For bindings, add link to Rust implementation
        if is_binding
            && let Some(link) = self
                .linker
                .rust_link_for_python_function(module_path, &func.name)
        {
            content.push_str(&link);
        }

        // Signature
        content.push_str(&self.renderer.render_signature(
            &func.name,
            &self.format_python_params(&func.signature.params),
            func.signature.return_type.as_deref(),
            func.is_async,
        )?);
        content.push_str("\n\n");

        // Docstring - parse and render with merged signature params for types
        if let Some(ref docstring) = func.docstring {
            content.push_str(&Self::render_docstring_with_merged_params(
                &func.signature.params,
                docstring,
                is_binding,
            ));
            content.push_str("\n\n");
        }

        // Source code (collapsible)
        if !func.source.source.is_empty() {
            content.push_str("<details>\n<summary>Source</summary>\n\n");
            content.push_str("```python\n");
            content.push_str(&func.source.source);
            content.push_str("\n```\n\n</details>\n\n");
        }

        let path = PathBuf::from(format!("{}/{}.md", module_dir, func.name));
        Ok(RenderedPage { path, content })
    }

    /// Render a Python function/method for a class page (uses h4 for methods)
    fn render_python_function_for_class_page(
        &self,
        func: &PythonFunction,
        module_path: &str,
        _class_name: &str,
        parent_class: Option<&str>,
    ) -> Result<String, tera::Error> {
        self.render_python_function_with_context(func, 4, module_path, parent_class)
    }

    /// Render a Python function/method with class context
    ///
    /// `parent_class` is used for method-level cross-links - when set, we look up
    /// the parent class's cross-ref and link to the method within it
    fn render_python_function_with_context(
        &self,
        func: &PythonFunction,
        heading_level: usize,
        module_path: &str,
        parent_class: Option<&str>,
    ) -> Result<String, tera::Error> {
        let mut content = String::new();
        let is_binding = func.rust_impl.is_some();

        // Function heading with badges
        let heading_prefix = "#".repeat(heading_level);
        content.push_str(&format!("{} `{}`", heading_prefix, func.name));

        // Badges
        if func.is_async {
            content.push(' ');
            content.push_str(&self.renderer.badge_async()?);
        }
        if func.is_property {
            content.push(' ');
            content.push_str(&self.renderer.render_badge("property", "gray", "property")?);
        }
        if func.is_staticmethod {
            content.push(' ');
            content.push_str(&self.renderer.render_badge(
                "staticmethod",
                "gray",
                "staticmethod",
            )?);
        }
        if func.is_classmethod {
            content.push(' ');
            content.push_str(
                &self
                    .renderer
                    .render_badge("classmethod", "gray", "classmethod")?,
            );
        }
        // Note: Binding badge intentionally omitted at method level
        // The binding nature is evident from the Rust cross-ref link
        content.push_str("\n\n");

        // Signature
        content.push_str(&self.renderer.render_signature(
            &func.name,
            &self.format_python_params(&func.signature.params),
            func.signature.return_type.as_deref(),
            func.is_async,
        )?);
        content.push_str("\n\n");

        // For bindings, add link to Rust implementation (method-level if parent_class is set)
        if is_binding
            && let Some(link) =
                self.linker
                    .rust_link_for_python_method(module_path, &func.name, parent_class)
        {
            content.push_str(&link);
        }

        // Docstring - parse and render with merged signature params for types
        if let Some(ref docstring) = func.docstring {
            content.push_str(&Self::render_docstring_with_merged_params(
                &func.signature.params,
                docstring,
                is_binding,
            ));
            content.push_str("\n\n");
        }

        // Source code (collapsible) - only for pure Python functions
        if !func.source.source.is_empty() {
            content.push_str("<details>\n<summary>Source</summary>\n\n");
            content.push_str("```python\n");
            content.push_str(&func.source.source);
            content.push_str("\n```\n\n</details>\n\n");
        }

        Ok(content)
    }

    /// Format Python parameters for display
    fn format_python_params(&self, params: &[PythonParam]) -> String {
        params
            .iter()
            .map(|p| {
                let mut s = p.name.clone();
                if let Some(ref ty) = p.ty {
                    s.push_str(": ");
                    s.push_str(ty);
                }
                if let Some(ref default) = p.default {
                    s.push_str(" = ");
                    s.push_str(default);
                }
                s
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Merge signature params with docstring params to get types + descriptions
    ///
    /// Signature params have types from Python annotations, docstring params have descriptions.
    /// This merges them to create ParamDoc entries with both.
    fn merge_params_with_docstring(
        sig_params: &[PythonParam],
        docstring_params: &[ParamDoc],
    ) -> Vec<ParamDoc> {
        sig_params
            .iter()
            .map(|sig_param| {
                // Find matching docstring param by name
                let doc_param = docstring_params.iter().find(|dp| dp.name == sig_param.name);

                ParamDoc {
                    name: sig_param.name.clone(),
                    // Prefer signature type, fall back to docstring type
                    ty: sig_param
                        .ty
                        .clone()
                        .or_else(|| doc_param.and_then(|dp| dp.ty.clone())),
                    // Use docstring description if available
                    description: doc_param
                        .map(|dp| dp.description.clone())
                        .unwrap_or_default(),
                }
            })
            .collect()
    }

    /// Render docstring with merged parameters from signature
    ///
    /// Detects whether the docstring is Rust-style (from a binding) or Python-style
    /// and uses the appropriate parser.
    fn render_docstring_with_merged_params(
        sig_params: &[PythonParam],
        docstring: &str,
        is_binding: bool,
    ) -> String {
        // Use appropriate parser based on docstring style
        // Rust-style uses `# Arguments`, Python uses `Args:` or NumPy style
        let mut parsed = if is_binding || Self::is_rust_style_docstring(docstring) {
            parse_rust_doc(docstring)
        } else {
            parse_docstring(docstring)
        };

        // Merge signature params into docstring params
        parsed.params = Self::merge_params_with_docstring(sig_params, &parsed.params);

        render_docstring(&parsed)
    }

    /// Detect if a docstring uses Rust-style markdown headers
    fn is_rust_style_docstring(docstring: &str) -> bool {
        // Rust doc comments use markdown headers like `# Arguments`, `# Returns`
        docstring.contains("# Arguments")
            || docstring.contains("# Parameters")
            || docstring.contains("# Returns")
            || docstring.contains("# Errors")
            || docstring.contains("# Panics")
            || docstring.contains("# Examples")
            || docstring.contains("# Safety")
    }

    /// Get source type badge
    fn source_badge(&self, source_type: &SourceType) -> Result<String, tera::Error> {
        match source_type {
            SourceType::Python => self.renderer.badge_source("python"),
            SourceType::PyO3Binding => self.renderer.badge_source("binding"),
            SourceType::Rust => self.renderer.badge_source("rust"),
        }
    }

    // =========================================================================
    // Rust Module Rendering
    // =========================================================================

    /// Render a Rust module to a single Markdown file with all content inline.
    ///
    /// This creates one file per module with structs, enums, and functions
    /// all rendered inline rather than as separate pages.
    pub fn render_rust_module(
        &self,
        module: &RustModule,
    ) -> Result<Vec<RenderedPage>, tera::Error> {
        // Categorize items
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        let mut functions = Vec::new();
        let mut impls: HashMap<String, Vec<&RustImpl>> = HashMap::new();

        for item in &module.items {
            match item {
                RustItem::Struct(s) => structs.push(s),
                RustItem::Enum(e) => enums.push(e),
                RustItem::Function(f) => functions.push(f),
                RustItem::Impl(i) => {
                    impls.entry(i.target.clone()).or_default().push(i);
                }
                _ => {} // Skip traits, consts, type aliases for MVP
            }
        }

        // Render everything into a single page
        let page = self.render_rust_module_inline(module, &structs, &enums, &functions, &impls)?;

        Ok(vec![page])
    }

    /// Render a Rust module with all content inline in a single page.
    fn render_rust_module_inline(
        &self,
        module: &RustModule,
        structs: &[&RustStruct],
        enums: &[&RustEnum],
        functions: &[&RustFunction],
        impls: &HashMap<String, Vec<&RustImpl>>,
    ) -> Result<RenderedPage, tera::Error> {
        let mut builder = ModulePageBuilder::new();
        let layout = PageLayout::new();

        // Module header with source badge
        let rust_badge = self.renderer.badge_source("rust")?;
        let module_name = module.path.split("::").last().unwrap_or(&module.path);
        builder.add_header(module_name, &rust_badge);

        // Module doc comment
        if let Some(ref doc) = module.doc_comment {
            builder.add_docstring(&parse_rust_doc(doc));
        }

        // Structs section
        if !structs.is_empty() {
            builder.add_section("Structs");
            for s in structs {
                let struct_impls = impls.get(&s.name).map(|v| v.as_slice()).unwrap_or(&[]);
                builder.add_item(&self.render_rust_struct_inline(s, struct_impls, &module.path)?);
            }
        }

        // Enums section
        if !enums.is_empty() {
            builder.add_section("Enums");
            for e in enums {
                builder.add_item(&self.render_rust_enum_inline(e, &module.path)?);
            }
        }

        // Functions section
        if !functions.is_empty() {
            builder.add_section("Functions");
            for func in functions {
                builder.add_item(&self.render_rust_function_inline(func, &module.path)?);
            }
        }

        let path = layout.rust_module_page(&module.path);
        Ok(RenderedPage {
            path,
            content: builder.build(),
        })
    }

    /// Render a Rust struct inline (for single-page module format).
    fn render_rust_struct_inline(
        &self,
        s: &RustStruct,
        impls: &[&RustImpl],
        module_path: &str,
    ) -> Result<String, tera::Error> {
        let mut content = String::new();
        let is_pyclass = s.pyclass.is_some();

        // For pyclass, show as "class" (Python-style), otherwise "struct"
        let type_name = if is_pyclass { "class" } else { "struct" };

        // Get Python name if different from Rust name
        let display_name = s
            .pyclass
            .as_ref()
            .and_then(|pc| pc.name.as_ref())
            .unwrap_or(&s.name);

        // Struct/class header (h3 since Structs is h2)
        // Badge on new line after heading to not affect anchor generation
        content.push_str(&format!("### `{} {}`", type_name, display_name));
        if !is_pyclass && let Some(ref generics) = s.generics {
            content.push_str(generics);
        }
        content.push_str("\n\n");

        // Badge on its own line (won't affect heading anchor)
        if is_pyclass {
            content.push_str(&self.renderer.badge_source("binding")?);
            content.push_str("\n\n");
        } else {
            content.push_str(&self.visibility_badge(&s.visibility)?);
            content.push_str("\n\n");
        }

        // For pyclass, add link to Python API
        if is_pyclass
            && let Some(link) = self
                .linker
                .python_link_for_rust_struct(module_path, &s.name)
        {
            content.push_str(&link);
        }

        // Derives - only show for pure Rust
        if !is_pyclass && !s.derives.is_empty() {
            content.push_str(&format!("**Derives:** `{}`\n\n", s.derives.join("`, `")));
        }

        // Doc comment - parse and render properly
        if let Some(ref doc) = s.doc_comment {
            let parsed = parse_rust_doc(doc);
            content.push_str(&render_docstring(&parsed));
            content.push_str("\n\n");
        }

        // Fields - for pyclass, show Python-style types
        if !s.fields.is_empty() {
            content.push_str("#### Fields\n\n");
            content.push_str("| Name | Type | Description |\n");
            content.push_str("|------|------|-------------|\n");
            for field in &s.fields {
                let doc = field.doc_comment.as_deref().unwrap_or("");
                content.push_str(&format!(
                    "| `{}` | `{}` | {} |\n",
                    field.name, field.ty, doc
                ));
            }
            content.push('\n');
        }

        // Methods from impl blocks - track which are from pymethods blocks
        let mut methods: Vec<(&RustFunction, bool)> = Vec::new(); // (method, is_pymethod)

        for impl_block in impls {
            if impl_block.trait_.is_none() {
                // Inherent impl - mark methods based on whether block has pymethods
                for method in &impl_block.methods {
                    methods.push((method, impl_block.pymethods));
                }
            }
        }

        if !methods.is_empty() {
            content.push_str("#### Methods\n\n");

            for (method, is_pymethod) in methods {
                // Pass parent struct name for method-level cross-links (use h5 for methods)
                let parent_struct = if is_pymethod {
                    Some(s.name.as_str())
                } else {
                    None
                };
                content.push_str(&self.render_rust_function_with_context(
                    method,
                    5,
                    is_pymethod,
                    module_path,
                    parent_struct,
                )?);
                content.push_str("\n\n");
            }
        }

        Ok(content)
    }

    /// Render a Rust enum inline (for single-page module format).
    fn render_rust_enum_inline(
        &self,
        e: &RustEnum,
        _module_path: &str,
    ) -> Result<String, tera::Error> {
        let mut content = String::new();

        // Enum header (h3 since Enums is h2)
        content.push_str(&format!("### `enum {}`", e.name));
        if let Some(ref generics) = e.generics {
            content.push_str(generics);
        }
        content.push(' ');
        content.push_str(&self.visibility_badge(&e.visibility)?);
        content.push_str("\n\n");

        // Doc comment - parse and render properly
        if let Some(ref doc) = e.doc_comment {
            let parsed = parse_rust_doc(doc);
            content.push_str(&render_docstring(&parsed));
            content.push_str("\n\n");
        }

        // Variants
        if !e.variants.is_empty() {
            content.push_str("#### Variants\n\n");
            for variant in &e.variants {
                content.push_str(&format!("- **`{}`**", variant.name));
                if let Some(ref doc) = variant.doc_comment {
                    content.push_str(&format!(" - {}", doc));
                }
                content.push('\n');
            }
            content.push('\n');
        }

        Ok(content)
    }

    /// Render a Rust function inline (for single-page module format).
    fn render_rust_function_inline(
        &self,
        func: &RustFunction,
        module_path: &str,
    ) -> Result<String, tera::Error> {
        let mut content = String::new();
        let is_binding = func.pyfunction.is_some();

        // Function header with badge on separate line for proper anchor generation
        content.push_str(&format!("### `fn {}`\n\n", func.name));
        if is_binding {
            content.push_str(&self.renderer.badge_source("binding")?);
        } else {
            content.push_str(&self.visibility_badge(&func.visibility)?);
        }
        content.push_str("\n\n");

        // For bindings, add link to Python API
        if is_binding
            && let Some(link) = self
                .linker
                .python_link_for_rust_function(module_path, &func.name)
        {
            content.push_str(&link);
        }

        // Signature
        content.push_str("```rust\n");
        content.push_str(&func.signature_str);
        content.push_str("\n```\n\n");

        // Docstring - parse and render properly
        if let Some(ref doc) = func.doc_comment {
            let parsed = parse_rust_doc(doc);
            content.push_str(&render_docstring(&parsed));
            content.push_str("\n\n");
        }

        // Source code (collapsible)
        if !func.source.source.is_empty() {
            content.push_str("<details>\n<summary>Source</summary>\n\n");
            content.push_str("```rust\n");
            content.push_str(&func.source.source);
            content.push_str("\n```\n\n</details>\n\n");
        }

        Ok(content)
    }

    /// Render a Rust module index page with struct/enum cards
    fn render_rust_module_index(
        &self,
        module: &RustModule,
        structs: &[&RustStruct],
        enums: &[&RustEnum],
        functions: &[&RustFunction],
        module_dir: &str,
    ) -> Result<RenderedPage, tera::Error> {
        let mut content = String::new();

        // Module header with source badge (badge on left)
        let rust_badge = self.renderer.badge_source("rust")?;
        content.push_str(&format!("# {} {}\n\n", rust_badge, module.path));

        // Module doc comment - parse and render properly
        if let Some(ref doc) = module.doc_comment {
            let parsed = parse_rust_doc(doc);
            content.push_str(&render_docstring(&parsed));
            content.push_str("\n\n");
        }

        // Structs section with cards linking to individual pages
        if !structs.is_empty() {
            content.push_str("## Structs\n\n");
            for s in structs {
                let is_pyclass = s.pyclass.is_some();
                let type_name = if is_pyclass { "class" } else { "struct" };
                let display_name = s
                    .pyclass
                    .as_ref()
                    .and_then(|pc| pc.name.as_ref())
                    .unwrap_or(&s.name);

                let badge = if is_pyclass {
                    format!("{} ", self.renderer.badge_source("binding")?)
                } else {
                    format!("{} ", self.visibility_badge(&s.visibility)?)
                };

                // Get first line of doc as summary
                let summary = s
                    .doc_comment
                    .as_ref()
                    .map(|d| d.lines().next().unwrap_or("").to_string())
                    .unwrap_or_default();

                content.push_str(&format!(
                    "### {}[`{} {}`]({}.md)\n\n{}\n\n",
                    badge, type_name, display_name, s.name, summary
                ));
            }
        }

        // Enums section with cards
        if !enums.is_empty() {
            content.push_str("## Enums\n\n");
            for e in enums {
                let badge = self.visibility_badge(&e.visibility)?;
                let summary = e
                    .doc_comment
                    .as_ref()
                    .map(|d| d.lines().next().unwrap_or("").to_string())
                    .unwrap_or_default();

                content.push_str(&format!(
                    "### {} [`enum {}`]({}.md)\n\n{}\n\n",
                    badge, e.name, e.name, summary
                ));
            }
        }

        // Functions section with links to individual function pages
        if !functions.is_empty() {
            content.push_str("## Functions\n\n");

            // List functions briefly with links to their individual pages
            for func in functions {
                let is_binding = func.pyfunction.is_some();
                let badge = if is_binding {
                    format!("{} ", self.renderer.badge_source("binding")?)
                } else {
                    format!("{} ", self.visibility_badge(&func.visibility)?)
                };

                let summary = func
                    .doc_comment
                    .as_ref()
                    .map(|d| d.lines().next().unwrap_or("").to_string())
                    .unwrap_or_default();

                content.push_str(&format!("- {}[`{}`]({}.md)", badge, func.name, func.name));
                if !summary.is_empty() {
                    content.push_str(&format!(" - {}", summary));
                }
                content.push('\n');
            }
            content.push('\n');
        }

        let path = PathBuf::from(format!("{}/index.md", module_dir));
        Ok(RenderedPage { path, content })
    }

    /// Render a single Rust struct as its own page
    fn render_rust_struct_page(
        &self,
        s: &RustStruct,
        impls: &[&RustImpl],
        module_path: &str,
        module_dir: &str,
    ) -> Result<RenderedPage, tera::Error> {
        let mut content = String::new();
        let is_pyclass = s.pyclass.is_some();

        // For pyclass, show as "class" (Python-style), otherwise "struct"
        let type_name = if is_pyclass { "class" } else { "struct" };

        // Get Python name if different from Rust name
        let display_name = s
            .pyclass
            .as_ref()
            .and_then(|pc| pc.name.as_ref())
            .unwrap_or(&s.name);

        // Struct/class header (h1 since it's the page title) - badge on left
        let badge = if is_pyclass {
            format!("{} ", self.renderer.badge_source("binding")?)
        } else {
            format!("{} ", self.visibility_badge(&s.visibility)?)
        };
        content.push_str(&format!("# {}`{} {}`", badge, type_name, display_name));
        if !is_pyclass && let Some(ref generics) = s.generics {
            content.push_str(generics);
        }
        content.push_str("\n\n");

        // Breadcrumb back to module
        content.push_str(&format!("*Module: [{}](index.md)*\n\n", module_path));

        // For pyclass, add link to Python API
        if is_pyclass
            && let Some(link) = self
                .linker
                .python_link_for_rust_struct(module_path, &s.name)
        {
            content.push_str(&link);
        }

        // Derives - only show for pure Rust
        if !is_pyclass && !s.derives.is_empty() {
            content.push_str(&format!("**Derives:** `{}`\n\n", s.derives.join("`, `")));
        }

        // Doc comment - parse and render properly
        if let Some(ref doc) = s.doc_comment {
            let parsed = parse_rust_doc(doc);
            content.push_str(&render_docstring(&parsed));
            content.push_str("\n\n");
        }

        // Fields - for pyclass, show Python-style types
        if !s.fields.is_empty() {
            content.push_str("## Fields\n\n");
            content.push_str("| Name | Type | Description |\n");
            content.push_str("|------|------|-------------|\n");
            for field in &s.fields {
                let doc = field.doc_comment.as_deref().unwrap_or("");
                content.push_str(&format!(
                    "| `{}` | `{}` | {} |\n",
                    field.name, field.ty, doc
                ));
            }
            content.push('\n');
        }

        // Methods from impl blocks - track which are from pymethods blocks
        let mut methods: Vec<(&RustFunction, bool)> = Vec::new(); // (method, is_pymethod)

        for impl_block in impls {
            if impl_block.trait_.is_none() {
                // Inherent impl - mark methods based on whether block has pymethods
                for method in &impl_block.methods {
                    methods.push((method, impl_block.pymethods));
                }
            }
        }

        if !methods.is_empty() {
            content.push_str("### Methods\n\n");

            for (method, is_pymethod) in methods {
                // Pass parent struct name for method-level cross-links (use h4 for methods on struct page)
                let parent_struct = if is_pymethod {
                    Some(s.name.as_str())
                } else {
                    None
                };
                content.push_str(&self.render_rust_function_with_context(
                    method,
                    4,
                    is_pymethod,
                    module_path,
                    parent_struct,
                )?);
                content.push_str("\n\n");
            }
        }

        let path = PathBuf::from(format!("{}/{}.md", module_dir, s.name));
        Ok(RenderedPage { path, content })
    }

    /// Render a single Rust enum as its own page
    fn render_rust_enum_page(
        &self,
        e: &RustEnum,
        module_path: &str,
        module_dir: &str,
    ) -> Result<RenderedPage, tera::Error> {
        let mut content = String::new();

        // Enum header (h1 since it's the page title)
        content.push_str(&format!("# `enum {}`", e.name));
        if let Some(ref generics) = e.generics {
            content.push_str(generics);
        }
        content.push(' ');
        content.push_str(&self.visibility_badge(&e.visibility)?);
        content.push_str("\n\n");

        // Breadcrumb back to module
        content.push_str(&format!("*Module: [{}](index.md)*\n\n", module_path));

        // Doc comment - parse and render properly
        if let Some(ref doc) = e.doc_comment {
            let parsed = parse_rust_doc(doc);
            content.push_str(&render_docstring(&parsed));
            content.push_str("\n\n");
        }

        // Variants
        if !e.variants.is_empty() {
            content.push_str("## Variants\n\n");
            for variant in &e.variants {
                content.push_str(&format!("- **`{}`**", variant.name));
                if let Some(ref doc) = variant.doc_comment {
                    content.push_str(&format!(" - {}", doc));
                }
                content.push('\n');
            }
            content.push('\n');
        }

        let path = PathBuf::from(format!("{}/{}.md", module_dir, e.name));
        Ok(RenderedPage { path, content })
    }

    /// Render a single Rust function to its own page
    fn render_rust_function_page(
        &self,
        func: &RustFunction,
        module_path: &str,
        module_dir: &str,
    ) -> Result<RenderedPage, tera::Error> {
        let mut content = String::new();
        let is_binding = func.pyfunction.is_some();

        // Function header with badges (h1 since it's the page title)
        let badge = if is_binding {
            format!("{} ", self.renderer.badge_source("binding")?)
        } else {
            format!("{} ", self.visibility_badge(&func.visibility)?)
        };
        content.push_str(&format!("# {}`fn {}`\n\n", badge, func.name));

        // Breadcrumb back to module
        content.push_str(&format!("*Module: [{}](index.md)*\n\n", module_path));

        // For bindings, add link to Python API
        if is_binding
            && let Some(link) = self
                .linker
                .python_link_for_rust_function(module_path, &func.name)
        {
            content.push_str(&link);
        }

        // Signature
        content.push_str("```rust\n");
        content.push_str(&func.signature_str);
        content.push_str("\n```\n\n");

        // Docstring - parse and render properly
        if let Some(ref doc) = func.doc_comment {
            let parsed = parse_rust_doc(doc);
            content.push_str(&render_docstring(&parsed));
            content.push_str("\n\n");
        }

        // Source code (collapsible)
        if !func.source.source.is_empty() {
            content.push_str("<details>\n<summary>Source</summary>\n\n");
            content.push_str("```rust\n");
            content.push_str(&func.source.source);
            content.push_str("\n```\n\n</details>\n\n");
        }

        let path = PathBuf::from(format!("{}/{}.md", module_dir, func.name));
        Ok(RenderedPage { path, content })
    }

    /// Render a Rust function with pymethod context
    ///
    /// `parent_struct` is used for method-level cross-links - when set, we look up
    /// the parent struct's cross-ref and link to the method within it
    fn render_rust_function_with_context(
        &self,
        f: &RustFunction,
        heading_level: usize,
        is_pymethod: bool,
        module_path: &str,
        parent_struct: Option<&str>,
    ) -> Result<String, tera::Error> {
        let mut content = String::new();
        // A function is a binding if it has #[pyfunction] or is in a #[pymethods] block
        let is_binding = f.pyfunction.is_some() || is_pymethod;

        // Function heading with badges
        let heading_prefix = "#".repeat(heading_level);
        content.push_str(&format!("{} `{}`", heading_prefix, f.name));

        // For bindings, no badge at method level (evident from cross-ref link)
        // For pure Rust methods, show visibility
        if !is_binding {
            content.push(' ');
            content.push_str(&self.visibility_badge(&f.visibility)?);
        }
        if f.is_async {
            content.push(' ');
            content.push_str(&self.renderer.badge_async()?);
        }
        if f.is_unsafe {
            content.push(' ');
            content.push_str(&self.renderer.badge_unsafe()?);
        }
        if f.is_const {
            content.push(' ');
            content.push_str(&self.renderer.render_badge("const", "blue", "const")?);
        }
        content.push_str("\n\n");

        // Signature - always show Rust-style for Rust docs
        content.push_str("```rust\n");
        content.push_str(&f.signature_str);
        content.push_str("\n```\n\n");

        // For bindings, add link to Python API (method-level if parent_struct is set)
        if is_binding
            && let Some(link) =
                self.linker
                    .python_link_for_rust_method(module_path, &f.name, parent_struct)
        {
            content.push_str(&link);
        }

        // Doc comment - parse and render with proper tables
        // Use parse_rust_doc for Rust doc comments (uses # Headers for sections)
        if let Some(ref doc) = f.doc_comment {
            let parsed = parse_rust_doc(doc);
            content.push_str(&render_docstring(&parsed));
            content.push_str("\n\n");
        }

        // Show source code in collapsible for all methods (bindings and pure Rust)
        if !f.source.source.is_empty() {
            content.push_str("<details>\n<summary>Source</summary>\n\n");
            content.push_str("```rust\n");
            content.push_str(&f.source.source);
            content.push_str("\n```\n\n</details>\n\n");
        }

        Ok(content)
    }

    /// Get visibility badge
    fn visibility_badge(&self, vis: &Visibility) -> Result<String, tera::Error> {
        match vis {
            Visibility::Public => self.renderer.badge_visibility("pub"),
            Visibility::PubCrate => self.renderer.badge_visibility("pub(crate)"),
            Visibility::PubSuper => self.renderer.badge_visibility("pub(super)"),
            Visibility::Private => self.renderer.badge_visibility("private"),
        }
    }

    // =========================================================================
    // Batch Rendering
    // =========================================================================

    /// Render all Python modules from a list
    pub fn render_python_modules(
        &self,
        modules: &[PythonModule],
    ) -> Result<Vec<RenderedPage>, tera::Error> {
        let mut all_pages = Vec::new();
        for module in modules {
            let pages = self.render_python_module(module)?;
            all_pages.extend(pages);
        }
        Ok(all_pages)
    }

    /// Render all Rust modules from a list
    pub fn render_rust_modules(
        &self,
        modules: &[RustModule],
    ) -> Result<Vec<RenderedPage>, tera::Error> {
        let mut all_pages = Vec::new();
        for module in modules {
            let pages = self.render_rust_module(module)?;
            all_pages.extend(pages);
        }
        Ok(all_pages)
    }

    // =========================================================================
    // SSG Adapter Integration
    // =========================================================================

    /// Generate navigation using an SSG adapter.
    ///
    /// This method provides a unified interface for generating navigation
    /// regardless of the target SSG. Use [`super::ssg::get_ssg_adapter`] to
    /// get the appropriate adapter for your template.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use plissken_core::render::ssg::get_ssg_adapter;
    ///
    /// let adapter = get_ssg_adapter(Some("mkdocs-material"));
    /// let nav = module_renderer.generate_nav(&adapter, &python_modules, &rust_modules);
    /// ```
    pub fn generate_nav(
        &self,
        adapter: &dyn super::ssg::SSGAdapter,
        python_modules: &[PythonModule],
        rust_modules: &[RustModule],
    ) -> String {
        adapter.generate_nav(python_modules, rust_modules)
    }

    /// Generate SSG config file using an adapter.
    ///
    /// Returns `None` for SSGs that don't need generated config (e.g., MkDocs).
    pub fn generate_config(
        &self,
        adapter: &dyn super::ssg::SSGAdapter,
        title: &str,
        authors: &[String],
    ) -> Option<String> {
        adapter.generate_config(title, authors)
    }

    /// Generate custom CSS using an adapter.
    ///
    /// Returns `None` for SSGs that don't need custom CSS.
    pub fn generate_custom_css(&self, adapter: &dyn super::ssg::SSGAdapter) -> Option<String> {
        adapter.generate_custom_css()
    }

    // =========================================================================
    // MkDocs Output Generation (legacy methods - use SSG adapter for new code)
    // =========================================================================

    /// Generate navigation YAML for mkdocs.yml
    ///
    /// Creates a nav structure with inline format (one file per module):
    /// ```yaml
    /// nav:
    ///   - Python:
    ///     - pysnake: pysnake.md
    ///   - Rust:
    ///     - rustscale: rust/rustscale.md
    ///     - rustscale::config: rust/rustscale/config.md
    /// ```
    pub fn generate_nav_yaml(
        &self,
        python_modules: &[PythonModule],
        rust_modules: &[RustModule],
    ) -> String {
        let mut yaml = String::new();

        // Add recommended toc setting
        yaml.push_str("# Recommended: add to markdown_extensions in mkdocs.yml\n");
        yaml.push_str("# to hide method-level entries from the table of contents:\n");
        yaml.push_str("#\n");
        yaml.push_str("# markdown_extensions:\n");
        yaml.push_str("#   - toc:\n");
        yaml.push_str("#       toc_depth: 2\n");
        yaml.push('\n');

        yaml.push_str("nav:\n");

        // Python section
        if !python_modules.is_empty() {
            yaml.push_str("  - Python:\n");
            for entry in python_nav_entries(python_modules) {
                yaml.push_str(&format!(
                    "    - {}: {}\n",
                    entry.path,
                    entry.file_path.display()
                ));
            }
        }

        // Rust section
        if !rust_modules.is_empty() {
            yaml.push_str("  - Rust:\n");
            for entry in rust_nav_entries(rust_modules) {
                yaml.push_str(&format!(
                    "    - {}: {}\n",
                    entry.path,
                    entry.file_path.display()
                ));
            }
        }

        yaml
    }

    // =========================================================================
    // mdBook Output Generation
    // =========================================================================

    /// Generate SUMMARY.md for mdBook navigation
    ///
    /// Creates a hierarchical navigation with inline format (one file per module):
    /// ```markdown
    /// # Summary
    ///
    /// # Python
    ///
    /// - [pysnake](pysnake.md)
    ///   - [pysnake.handlers](pysnake/handlers.md)
    ///
    /// # Rust
    ///
    /// - [rustscale](rust/rustscale.md)
    ///   - [rustscale::config](rust/rustscale/config.md)
    /// ```
    pub fn generate_mdbook_summary(
        &self,
        python_modules: &[PythonModule],
        rust_modules: &[RustModule],
    ) -> String {
        let mut summary = String::new();
        summary.push_str("# Summary\n\n");

        // Python section
        if !python_modules.is_empty() {
            summary.push_str("# Python\n\n");
            for entry in python_nav_entries(python_modules) {
                let indent = "  ".repeat(entry.depth);
                summary.push_str(&format!(
                    "{}- [{}]({})\n",
                    indent,
                    entry.path,
                    entry.file_path.display()
                ));
            }
        }

        // Rust section
        if !rust_modules.is_empty() {
            summary.push_str("\n# Rust\n\n");
            for entry in rust_nav_entries(rust_modules) {
                let indent = "  ".repeat(entry.depth);
                summary.push_str(&format!(
                    "{}- [{}]({})\n",
                    indent,
                    entry.path,
                    entry.file_path.display()
                ));
            }
        }

        summary
    }

    /// Generate book.toml configuration for mdBook
    ///
    /// Includes fold configuration for collapsible sidebar sections.
    pub fn generate_mdbook_config(&self, title: &str, authors: &[String]) -> String {
        let authors_toml = if authors.is_empty() {
            String::from("[]")
        } else {
            format!(
                "[{}]",
                authors
                    .iter()
                    .map(|a| format!("\"{}\"", a))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };

        format!(
            r#"[book]
title = "{}"
authors = {}
language = "en"
src = "src"

[build]
build-dir = "book"

[output.html]
default-theme = "rust"
preferred-dark-theme = "coal"
additional-css = ["theme/custom.css"]

[output.html.fold]
enable = true
level = 1
"#,
            title, authors_toml
        )
    }

    /// Generate custom CSS for mdBook
    ///
    /// Hides chapter numbering for reference documentation sections (python/, rust/)
    /// while preserving numbering for other documentation sections.
    pub fn generate_mdbook_css(&self) -> String {
        r#"/* Hide chapter numbering for reference documentation sections only.
   Scoped to python/ and rust/ paths to avoid affecting other documentation.
   Uses *= (contains) since mdbook prepends path_to_root to hrefs. */
.chapter-item a[href*="python/"] strong[aria-hidden="true"],
.chapter-item a[href*="rust/"] strong[aria-hidden="true"] {
    display: none !important;
}
"#
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;

    fn test_renderer() -> Renderer {
        Renderer::new(None, None).unwrap()
    }

    /// Helper to find a page by path suffix
    fn find_page<'a>(pages: &'a [RenderedPage], suffix: &str) -> Option<&'a RenderedPage> {
        pages
            .iter()
            .find(|p| p.path.to_string_lossy().ends_with(suffix))
    }

    #[test]
    fn test_render_simple_python_module() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = PythonModule::test("my_module")
            .with_docstring("A simple test module.")
            .with_item(PythonItem::Function(
                PythonFunction::test("greet")
                    .with_docstring("Say hello")
                    .with_param(PythonParam::test("name").with_type("str"))
                    .with_return_type("str"),
            ));

        let pages = module_renderer.render_python_module(&module).unwrap();

        // Inline format: single page per module
        assert_eq!(pages.len(), 1, "Should produce a single page");
        let page = &pages[0];
        assert!(page.path.to_string_lossy().ends_with("my_module.md"));
        assert!(page.content.contains("# my_module"));
        assert!(page.content.contains("A simple test module."));
        assert!(page.content.contains("## Functions"));
        // Function is rendered inline in the same file
        assert!(page.content.contains("greet"));
    }

    #[test]
    fn test_render_python_module_with_class() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = PythonModule::test("myapp.models").with_item(PythonItem::Class(
            PythonClass::test("User")
                .with_docstring("A user model")
                .with_base("BaseModel")
                .with_method(
                    PythonFunction::test("get_name")
                        .with_return_type("str")
                        .with_docstring("Get user name"),
                ),
        ));

        let pages = module_renderer.render_python_module(&module).unwrap();

        // Inline format: single page per module
        assert_eq!(pages.len(), 1, "Should produce a single page");
        let page = &pages[0];
        assert!(page.path.to_string_lossy().ends_with("models.md"));
        assert!(page.content.contains("## Classes"));
        // Class rendered inline (h3 since Classes is h2)
        assert!(page.content.contains("### `class User`"));
        assert!(page.content.contains("**Inherits from:** BaseModel"));
        assert!(page.content.contains("#### Methods"));
        assert!(page.content.contains("`get_name`"));
    }

    #[test]
    fn test_render_python_module_pyo3_binding() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = PythonModule::test("binding_module")
            .pyo3_binding()
            .with_item(PythonItem::Class(
                PythonClass::test("NativeClass")
                    .with_rust_impl(RustItemRef::new("crate::native", "NativeClass")),
            ));

        let pages = module_renderer.render_python_module(&module).unwrap();

        // Inline format: single page with binding badge in module header
        assert_eq!(pages.len(), 1, "Should produce a single page");
        let page = &pages[0];
        assert!(page.content.contains("Binding"));
        assert!(page.content.contains("class NativeClass"));
    }

    #[test]
    fn test_render_python_module_with_cross_refs() {
        let renderer = test_renderer();

        // Create cross-refs for the binding
        let cross_refs = vec![CrossRef {
            python_path: "binding_module.NativeClass".to_string(),
            rust_path: "crate::native::NativeClass".to_string(),
            relationship: crate::model::CrossRefKind::Binding,
        }];

        let module_renderer = ModuleRenderer::with_cross_refs(&renderer, cross_refs);

        let module = PythonModule::test("binding_module")
            .pyo3_binding()
            .with_item(PythonItem::Class(
                PythonClass::test("NativeClass")
                    .with_rust_impl(RustItemRef::new("crate::native", "NativeClass")),
            ));

        let pages = module_renderer.render_python_module(&module).unwrap();

        // Inline format: single page with binding badge and cross-ref link
        assert_eq!(pages.len(), 1, "Should produce a single page");
        let page = &pages[0];
        assert!(page.content.contains("Binding"));
        // Should have Rust implementation link as blockquote
        assert!(page.content.contains("**Rust Implementation**"));
        assert!(page.content.contains("crate::native::NativeClass"));
    }

    #[test]
    fn test_render_simple_rust_module() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = RustModule::test("crate::utils")
            .with_doc("Utility functions")
            .with_item(RustItem::Function(
                RustFunction::test("process")
                    .with_doc("Process some data")
                    .with_signature("pub fn process(data: &[u8]) -> Vec<u8>"),
            ));

        let pages = module_renderer.render_rust_module(&module).unwrap();

        // Inline format: single page per module
        assert_eq!(pages.len(), 1, "Should produce a single page");
        let page = &pages[0];
        assert!(page.content.contains("utils")); // module name in header
        assert!(page.content.contains("Rust")); // source badge
        assert!(page.content.contains("## Functions"));
        // Function is rendered inline
        assert!(page.content.contains("process"));
        assert!(page.content.contains("```rust"));
    }

    #[test]
    fn test_render_rust_module_with_struct() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = RustModule::test("crate::models")
            .with_item(RustItem::Struct(
                RustStruct::test("Config")
                    .with_doc("Application configuration")
                    .with_generics("<T>")
                    .with_field(RustField::test("value", "T").with_doc("The stored value"))
                    .with_derive("Debug")
                    .with_derive("Clone"),
            ))
            .with_item(RustItem::Impl(RustImpl {
                generics: Some("<T>".to_string()),
                target: "Config".to_string(),
                trait_: None,
                where_clause: None,
                methods: vec![
                    RustFunction::test("new").with_signature("pub fn new(value: T) -> Self"),
                ],
                pymethods: false,
                source: SourceSpan::test("test.rs", 1, 10),
            }));

        let pages = module_renderer.render_rust_module(&module).unwrap();

        // Inline format: single page per module
        assert_eq!(pages.len(), 1, "Should produce a single page");
        let page = &pages[0];
        assert!(page.content.contains("## Structs"));
        // Struct rendered inline (h3 since Structs is h2)
        assert!(page.content.contains("### `struct Config`"));
        assert!(page.content.contains("<T>")); // generics
        assert!(page.content.contains("**Derives:** `Debug`, `Clone`"));
        assert!(page.content.contains("#### Fields"));
        assert!(
            page.content
                .contains("| `value` | `T` | The stored value |")
        );
        assert!(page.content.contains("#### Methods"));
        assert!(page.content.contains("`new`"));
    }

    #[test]
    fn test_render_rust_struct_with_pyclass() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = RustModule::test("crate::bindings")
            .with_item(RustItem::Struct(
                RustStruct::test("PyData")
                    .with_pyclass(PyClassMeta::new().with_name("Data"))
                    .with_doc("Python-exposed data structure"),
            ))
            .with_item(RustItem::Impl(RustImpl {
                generics: None,
                target: "PyData".to_string(),
                trait_: None,
                where_clause: None,
                methods: vec![
                    RustFunction::test("value").with_signature("pub fn value(&self) -> i32"),
                ],
                pymethods: true,
                source: SourceSpan::test("test.rs", 1, 10),
            }));

        let pages = module_renderer.render_rust_module(&module).unwrap();

        // Inline format: single page per module
        assert_eq!(pages.len(), 1, "Should produce a single page");
        let page = &pages[0];
        // PyClass struct should display as "class" with Python name
        assert!(
            page.content.contains("`class Data`"),
            "Should render pyclass as class with Python name, got: {}",
            page.content
        );
        assert!(page.content.contains("Binding"));
        // Methods section should also have binding badge
        assert!(page.content.contains("#### Methods"));
    }

    #[test]
    fn test_render_rust_function_badges() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = RustModule::test("crate::async_utils").with_item(RustItem::Function(
            RustFunction::test("dangerous_async")
                .async_()
                .unsafe_()
                .with_signature("pub async unsafe fn dangerous_async()"),
        ));

        let pages = module_renderer.render_rust_module(&module).unwrap();

        // Inline format: single page with function inline
        assert_eq!(pages.len(), 1, "Should produce a single page");
        let page = &pages[0];
        assert!(page.content.contains("async")); // badge
        assert!(page.content.contains("unsafe")); // badge
        assert!(page.content.contains("pub")); // visibility badge
    }

    #[test]
    fn test_render_rust_enum() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = RustModule::test("crate::types").with_item(RustItem::Enum(RustEnum {
            name: "Status".to_string(),
            visibility: Visibility::Public,
            doc_comment: Some("Request status".to_string()),
            parsed_doc: None,
            generics: None,
            variants: vec![
                RustVariant {
                    name: "Pending".to_string(),
                    doc_comment: Some("Waiting to start".to_string()),
                    fields: vec![],
                },
                RustVariant {
                    name: "Complete".to_string(),
                    doc_comment: Some("Finished successfully".to_string()),
                    fields: vec![],
                },
            ],
            source: SourceSpan::test("test.rs", 1, 10),
        }));

        let pages = module_renderer.render_rust_module(&module).unwrap();

        // Inline format: single page per module
        assert_eq!(pages.len(), 1, "Should produce a single page");
        let page = &pages[0];
        assert!(page.content.contains("## Enums"));
        // Enum rendered inline
        assert!(page.content.contains("### `enum Status`"));
        assert!(page.content.contains("#### Variants"));
        assert!(page.content.contains("**`Pending`** - Waiting to start"));
        assert!(
            page.content
                .contains("**`Complete`** - Finished successfully")
        );
    }

    #[test]
    fn test_batch_render_modules() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        // Empty modules still produce pages
        let python_modules = vec![PythonModule::test("mod1"), PythonModule::test("mod2")];
        let rust_modules = vec![RustModule::test("crate::a"), RustModule::test("crate::b")];

        let py_pages = module_renderer
            .render_python_modules(&python_modules)
            .unwrap();
        let rs_pages = module_renderer.render_rust_modules(&rust_modules).unwrap();

        // Inline format: each module produces a single page
        assert_eq!(py_pages.len(), 2);
        assert_eq!(rs_pages.len(), 2);

        // Check paths are module_name.md format (inline)
        assert!(find_page(&py_pages, "mod1.md").is_some());
        assert!(find_page(&py_pages, "mod2.md").is_some());
        // For Rust, crate root = crate_name.md, submodule = crate/submod.md
        assert!(find_page(&rs_pages, "crate/a.md").is_some());
        assert!(find_page(&rs_pages, "crate/b.md").is_some());
    }

    #[test]
    fn test_python_async_function() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = PythonModule::test("async_mod").with_item(PythonItem::Function(
            PythonFunction::test("fetch")
                .async_()
                .with_param(PythonParam::test("url").with_type("str"))
                .with_return_type("bytes"),
        ));

        let pages = module_renderer.render_python_module(&module).unwrap();

        // Inline format: single page with async badge inline
        assert_eq!(pages.len(), 1, "Should produce a single page");
        let page = &pages[0];
        assert!(page.content.contains("async"));
    }

    #[test]
    fn test_python_class_methods_types() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = PythonModule::test("methods_mod").with_item(PythonItem::Class(
            PythonClass::test("MyClass")
                .with_method(PythonFunction::test("regular_method"))
                .with_method(PythonFunction::test("static_method").staticmethod())
                .with_method(PythonFunction::test("class_method").classmethod())
                .with_method(PythonFunction::test("prop").property()),
        ));

        let pages = module_renderer.render_python_module(&module).unwrap();

        // Inline format: single page with all method types inline
        assert_eq!(pages.len(), 1, "Should produce a single page");
        let page = &pages[0];
        assert!(page.content.contains("staticmethod"));
        assert!(page.content.contains("classmethod"));
        assert!(page.content.contains("property"));
    }
}

/// Snapshot tests for rendered output using insta.
///
/// These tests capture the exact rendered Markdown output to detect
/// regressions in formatting, badges, headings, and other output details.
///
/// To update snapshots after intentional changes:
/// ```bash
/// cargo insta review
/// ```
/// or run tests with `INSTA_UPDATE=always cargo test`
#[cfg(test)]
mod snapshot_tests {
    use super::*;
    use crate::model::*;
    use insta::assert_snapshot;

    fn test_renderer() -> Renderer {
        Renderer::new(None, None).unwrap()
    }

    // =========================================================================
    // Python Module Snapshots
    // =========================================================================

    #[test]
    fn snapshot_python_module_simple() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = PythonModule::test("mymodule")
            .with_docstring("A simple Python module.\n\nThis module provides basic utilities.");

        let pages = module_renderer.render_python_module(&module).unwrap();
        assert_snapshot!("python_module_simple", &pages[0].content);
    }

    #[test]
    fn snapshot_python_module_with_function() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = PythonModule::test("utils")
            .with_docstring("Utility functions")
            .with_item(PythonItem::Function(
                PythonFunction::test("process")
                    .with_docstring("Process the input data.\n\nArgs:\n    data: The input data to process\n\nReturns:\n    The processed result")
                    .with_param(PythonParam::test("data").with_type("bytes"))
                    .with_return_type("str"),
            ));

        let pages = module_renderer.render_python_module(&module).unwrap();
        assert_snapshot!("python_module_with_function", &pages[0].content);
    }

    #[test]
    fn snapshot_python_class_with_methods() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = PythonModule::test("models").with_item(PythonItem::Class(
            PythonClass::test("User")
                .with_docstring("A user model.\n\nRepresents a system user with authentication.")
                .with_base("BaseModel")
                .with_attribute(
                    PythonVariable::test("id")
                        .with_type("int")
                        .with_docstring("User ID"),
                )
                .with_attribute(
                    PythonVariable::test("name")
                        .with_type("str")
                        .with_docstring("User name"),
                )
                .with_method(
                    PythonFunction::test("__init__")
                        .with_param(PythonParam::test("self"))
                        .with_param(PythonParam::test("name").with_type("str"))
                        .with_param(PythonParam::test("id").with_type("int").with_default("0")),
                )
                .with_method(
                    PythonFunction::test("get_display_name")
                        .with_docstring("Get formatted display name")
                        .with_param(PythonParam::test("self"))
                        .with_return_type("str"),
                )
                .with_method(
                    PythonFunction::test("from_dict")
                        .classmethod()
                        .with_docstring("Create user from dictionary")
                        .with_param(PythonParam::test("cls"))
                        .with_param(PythonParam::test("data").with_type("dict"))
                        .with_return_type("User"),
                ),
        ));

        let pages = module_renderer.render_python_module(&module).unwrap();
        assert_snapshot!("python_class_with_methods", &pages[0].content);
    }

    #[test]
    fn snapshot_python_async_function() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = PythonModule::test("async_utils")
            .with_item(PythonItem::Function(
                PythonFunction::test("fetch")
                    .async_()
                    .with_docstring("Fetch data from URL.\n\nArgs:\n    url: The URL to fetch\n    timeout: Request timeout in seconds\n\nReturns:\n    Response bytes")
                    .with_param(PythonParam::test("url").with_type("str"))
                    .with_param(PythonParam::test("timeout").with_type("float").with_default("30.0"))
                    .with_return_type("bytes"),
            ));

        let pages = module_renderer.render_python_module(&module).unwrap();
        assert_snapshot!("python_async_function", &pages[0].content);
    }

    #[test]
    fn snapshot_python_pyo3_binding() {
        let renderer = test_renderer();

        let cross_refs = vec![CrossRef {
            python_path: "native.DataProcessor".to_string(),
            rust_path: "crate::data::DataProcessor".to_string(),
            relationship: CrossRefKind::Binding,
        }];

        let module_renderer = ModuleRenderer::with_cross_refs(&renderer, cross_refs);

        let module = PythonModule::test("native")
            .pyo3_binding()
            .with_docstring("Native bindings for data processing")
            .with_item(PythonItem::Class(
                PythonClass::test("DataProcessor")
                    .with_docstring(
                        "High-performance data processor.\n\nWraps native Rust implementation.",
                    )
                    .with_rust_impl(RustItemRef::new("crate::data", "DataProcessor"))
                    .with_method(
                        PythonFunction::test("process")
                            .with_docstring("Process data efficiently")
                            .with_param(PythonParam::test("self"))
                            .with_param(PythonParam::test("data").with_type("bytes"))
                            .with_return_type("bytes")
                            .with_rust_impl(RustItemRef::new(
                                "crate::data::DataProcessor",
                                "process",
                            )),
                    ),
            ));

        let pages = module_renderer.render_python_module(&module).unwrap();
        assert_snapshot!("python_pyo3_binding", &pages[0].content);
    }

    #[test]
    fn snapshot_python_enum_class() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = PythonModule::test("enums").with_item(PythonItem::Class(
            PythonClass::test("Status")
                .with_docstring("Request status enumeration")
                .with_base("Enum")
                .with_attribute(PythonVariable::test("PENDING").with_value("\"pending\""))
                .with_attribute(PythonVariable::test("RUNNING").with_value("\"running\""))
                .with_attribute(PythonVariable::test("COMPLETED").with_value("\"completed\""))
                .with_attribute(PythonVariable::test("FAILED").with_value("\"failed\"")),
        ));

        let pages = module_renderer.render_python_module(&module).unwrap();
        assert_snapshot!("python_enum_class", &pages[0].content);
    }

    #[test]
    fn snapshot_python_module_variables() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = PythonModule::test("constants")
            .with_docstring("Module-level constants")
            .with_item(PythonItem::Variable(
                PythonVariable::test("VERSION")
                    .with_type("str")
                    .with_value("\"1.0.0\"")
                    .with_docstring("Package version"),
            ))
            .with_item(PythonItem::Variable(
                PythonVariable::test("MAX_RETRIES")
                    .with_type("int")
                    .with_value("3")
                    .with_docstring("Maximum retry attempts"),
            ));

        let pages = module_renderer.render_python_module(&module).unwrap();
        assert_snapshot!("python_module_variables", &pages[0].content);
    }

    // =========================================================================
    // Rust Module Snapshots
    // =========================================================================

    #[test]
    fn snapshot_rust_module_simple() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = RustModule::test("crate::utils")
            .with_doc("Utility functions for common operations.\n\nThis module provides helpers for data manipulation.");

        let pages = module_renderer.render_rust_module(&module).unwrap();
        assert_snapshot!("rust_module_simple", &pages[0].content);
    }

    #[test]
    fn snapshot_rust_struct_with_fields() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = RustModule::test("crate::config")
            .with_item(RustItem::Struct(
                RustStruct::test("Config")
                    .with_doc("Application configuration.\n\nStores all runtime settings.")
                    .with_generics("<T: Default>")
                    .with_field(RustField::test("name", "String").with_doc("Configuration name"))
                    .with_field(RustField::test("value", "T").with_doc("Configuration value"))
                    .with_field(
                        RustField::test("enabled", "bool").with_doc("Whether config is active"),
                    )
                    .with_derive("Debug")
                    .with_derive("Clone")
                    .with_derive("Serialize"),
            ))
            .with_item(RustItem::Impl(RustImpl {
                generics: Some("<T: Default>".to_string()),
                target: "Config".to_string(),
                trait_: None,
                where_clause: None,
                methods: vec![
                    RustFunction::test("new")
                        .with_doc("Create a new Config with default value")
                        .with_signature("pub fn new(name: impl Into<String>) -> Self"),
                    RustFunction::test("with_value")
                        .with_doc("Set the configuration value")
                        .with_signature("pub fn with_value(mut self, value: T) -> Self"),
                ],
                pymethods: false,
                source: SourceSpan::test("test.rs", 1, 20),
            }));

        let pages = module_renderer.render_rust_module(&module).unwrap();
        assert_snapshot!("rust_struct_with_fields", &pages[0].content);
    }

    #[test]
    fn snapshot_rust_enum_with_variants() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = RustModule::test("crate::types").with_item(RustItem::Enum(RustEnum {
            name: "Result".to_string(),
            visibility: Visibility::Public,
            doc_comment: Some(
                "Operation result type.\n\nRepresents success or failure of an operation."
                    .to_string(),
            ),
            parsed_doc: None,
            generics: Some("<T, E>".to_string()),
            variants: vec![
                RustVariant {
                    name: "Ok".to_string(),
                    doc_comment: Some("Operation succeeded with value".to_string()),
                    fields: vec![RustField::test("0", "T")],
                },
                RustVariant {
                    name: "Err".to_string(),
                    doc_comment: Some("Operation failed with error".to_string()),
                    fields: vec![RustField::test("0", "E")],
                },
            ],
            source: SourceSpan::test("test.rs", 1, 15),
        }));

        let pages = module_renderer.render_rust_module(&module).unwrap();
        assert_snapshot!("rust_enum_with_variants", &pages[0].content);
    }

    #[test]
    fn snapshot_rust_function_with_generics() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = RustModule::test("crate::convert")
            .with_item(RustItem::Function(
                RustFunction::test("transform")
                    .with_doc("Transform input to output.\n\n# Arguments\n\n* `input` - The input value\n* `mapper` - Transformation function\n\n# Returns\n\nThe transformed value")
                    .with_generics("<T, U>")
                    .with_signature("pub fn transform<T, U>(input: T, mapper: impl Fn(T) -> U) -> U"),
            ));

        let pages = module_renderer.render_rust_module(&module).unwrap();
        assert_snapshot!("rust_function_with_generics", &pages[0].content);
    }

    #[test]
    fn snapshot_rust_async_unsafe_function() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = RustModule::test("crate::low_level")
            .with_item(RustItem::Function(
                RustFunction::test("dangerous_read")
                    .async_()
                    .unsafe_()
                    .with_doc("Read from raw pointer asynchronously.\n\n# Safety\n\nPointer must be valid and properly aligned.")
                    .with_signature("pub async unsafe fn dangerous_read(ptr: *const u8, len: usize) -> Vec<u8>"),
            ));

        let pages = module_renderer.render_rust_module(&module).unwrap();
        assert_snapshot!("rust_async_unsafe_function", &pages[0].content);
    }

    #[test]
    fn snapshot_rust_pyclass_struct() {
        let renderer = test_renderer();

        let cross_refs = vec![CrossRef {
            python_path: "native.Buffer".to_string(),
            rust_path: "crate::buffer::RustBuffer".to_string(),
            relationship: CrossRefKind::Binding,
        }];

        let module_renderer = ModuleRenderer::with_cross_refs(&renderer, cross_refs);

        let module = RustModule::test("crate::buffer")
            .with_item(RustItem::Struct(
                RustStruct::test("RustBuffer")
                    .with_doc(
                        "High-performance buffer for Python.\n\nExposed to Python as `Buffer`.",
                    )
                    .with_pyclass(PyClassMeta::new().with_name("Buffer").with_module("native"))
                    .with_field(
                        RustField::test("data", "Vec<u8>").with_doc("Internal data storage"),
                    ),
            ))
            .with_item(RustItem::Impl(RustImpl {
                generics: None,
                target: "RustBuffer".to_string(),
                trait_: None,
                where_clause: None,
                methods: vec![
                    RustFunction::test("new")
                        .with_doc("Create a new empty buffer")
                        .with_signature("#[new]\npub fn new() -> Self"),
                    RustFunction::test("len")
                        .with_doc("Get buffer length")
                        .with_signature("pub fn len(&self) -> usize"),
                ],
                pymethods: true,
                source: SourceSpan::test("test.rs", 1, 20),
            }));

        let pages = module_renderer.render_rust_module(&module).unwrap();
        assert_snapshot!("rust_pyclass_struct", &pages[0].content);
    }

    #[test]
    fn snapshot_rust_pyfunction() {
        let renderer = test_renderer();

        let cross_refs = vec![CrossRef {
            python_path: "native.compute".to_string(),
            rust_path: "crate::compute::compute".to_string(),
            relationship: CrossRefKind::Binding,
        }];

        let module_renderer = ModuleRenderer::with_cross_refs(&renderer, cross_refs);

        let module = RustModule::test("crate::compute").with_item(RustItem::Function(
            RustFunction::test("compute")
                .with_doc("Compute result from input.\n\nExposed to Python as `compute()`.")
                .with_pyfunction(PyFunctionMeta::new().with_name("compute"))
                .with_signature("#[pyfunction]\npub fn compute(value: i64) -> i64"),
        ));

        let pages = module_renderer.render_rust_module(&module).unwrap();
        assert_snapshot!("rust_pyfunction", &pages[0].content);
    }

    // =========================================================================
    // Complex Module Snapshots
    // =========================================================================

    #[test]
    fn snapshot_complex_python_module() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = PythonModule::test("mypackage.api")
            .with_docstring("API module for external interactions.\n\nProvides client classes and utility functions.")
            .with_item(PythonItem::Variable(
                PythonVariable::test("API_VERSION").with_type("str").with_value("\"v2\"").with_docstring("Current API version"),
            ))
            .with_item(PythonItem::Class(
                PythonClass::test("Client")
                    .with_docstring("HTTP client for API requests.\n\nHandles authentication and rate limiting.")
                    .with_attribute(PythonVariable::test("base_url").with_type("str"))
                    .with_attribute(PythonVariable::test("timeout").with_type("float"))
                    .with_method(
                        PythonFunction::test("__init__")
                            .with_param(PythonParam::test("self"))
                            .with_param(PythonParam::test("base_url").with_type("str"))
                            .with_param(PythonParam::test("timeout").with_type("float").with_default("30.0")),
                    )
                    .with_method(
                        PythonFunction::test("get")
                            .async_()
                            .with_docstring("Perform GET request")
                            .with_param(PythonParam::test("self"))
                            .with_param(PythonParam::test("path").with_type("str"))
                            .with_return_type("Response"),
                    )
                    .with_method(
                        PythonFunction::test("post")
                            .async_()
                            .with_docstring("Perform POST request")
                            .with_param(PythonParam::test("self"))
                            .with_param(PythonParam::test("path").with_type("str"))
                            .with_param(PythonParam::test("data").with_type("dict"))
                            .with_return_type("Response"),
                    ),
            ))
            .with_item(PythonItem::Function(
                PythonFunction::test("create_client")
                    .with_docstring("Factory function to create a configured client")
                    .with_param(PythonParam::test("config").with_type("Config"))
                    .with_return_type("Client"),
            ));

        let pages = module_renderer.render_python_module(&module).unwrap();
        assert_snapshot!("complex_python_module", &pages[0].content);
    }

    #[test]
    fn snapshot_complex_rust_module() {
        let renderer = test_renderer();
        let module_renderer = ModuleRenderer::new(&renderer);

        let module = RustModule::test("crate::engine")
            .with_doc("Core processing engine.\n\nProvides the main computation pipeline.")
            .with_item(RustItem::Struct(
                RustStruct::test("Engine")
                    .with_doc("Main processing engine")
                    .with_generics("<T: Process>")
                    .with_field(RustField::test("state", "State").with_doc("Current engine state"))
                    .with_field(RustField::test("processor", "T").with_doc("Item processor"))
                    .with_derive("Debug"),
            ))
            .with_item(RustItem::Impl(RustImpl {
                generics: Some("<T: Process>".to_string()),
                target: "Engine".to_string(),
                trait_: None,
                where_clause: None,
                methods: vec![
                    RustFunction::test("new")
                        .with_doc("Create new engine")
                        .with_signature("pub fn new(processor: T) -> Self"),
                    RustFunction::test("run")
                        .async_()
                        .with_doc("Run the engine")
                        .with_signature("pub async fn run(&mut self) -> Result<(), Error>"),
                ],
                pymethods: false,
                source: SourceSpan::test("test.rs", 1, 30),
            }))
            .with_item(RustItem::Enum(RustEnum {
                name: "State".to_string(),
                visibility: Visibility::Public,
                doc_comment: Some("Engine state".to_string()),
                parsed_doc: None,
                generics: None,
                variants: vec![
                    RustVariant {
                        name: "Idle".to_string(),
                        doc_comment: Some("Engine is idle".to_string()),
                        fields: vec![],
                    },
                    RustVariant {
                        name: "Running".to_string(),
                        doc_comment: Some("Engine is running".to_string()),
                        fields: vec![],
                    },
                    RustVariant {
                        name: "Error".to_string(),
                        doc_comment: Some("Engine encountered an error".to_string()),
                        fields: vec![RustField::test("message", "String")],
                    },
                ],
                source: SourceSpan::test("test.rs", 31, 45),
            }))
            .with_item(RustItem::Function(
                RustFunction::test("default_engine")
                    .with_doc("Create engine with default processor")
                    .with_signature("pub fn default_engine() -> Engine<DefaultProcessor>"),
            ));

        let pages = module_renderer.render_rust_module(&module).unwrap();
        assert_snapshot!("complex_rust_module", &pages[0].content);
    }
}
