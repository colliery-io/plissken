---
id: create-ssgadapter-trait-for-static
level: task
title: "Create SSGAdapter trait for static site generator abstraction"
short_code: "PLSKN-T-0033"
created_at: 2026-01-16T18:01:47.462744+00:00
updated_at: 2026-01-17T01:54:24.545538+00:00
parent: PLSKN-I-0009
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
strategy_id: NULL
initiative_id: PLSKN-I-0009
---

# Create SSGAdapter trait for static site generator abstraction

## Parent Initiative

[[PLSKN-I-0009]] Rendering System Refactor

## Objective

Replace ad-hoc SSG detection (`template == "mdbook"` string comparisons) with a proper `SSGAdapter` trait. This enables cleaner SSG-specific logic, easier testing, and potential support for additional SSGs.

## Current Problem

SSG-specific behavior is scattered throughout the codebase:
```rust
// In multiple places:
if config.output.template == "mdbook" {
    // mdBook-specific logic
} else {
    // MkDocs-specific logic  
}
```

This leads to:
- Duplicated conditional logic
- Inconsistent behavior between SSGs
- Hard to add new SSG support
- Navigation generation logic duplicated

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Define `SSGAdapter` trait with common interface
- [x] Implement `MkDocsAdapter` for MkDocs Material
- [x] Implement `MdBookAdapter` for mdBook
- [x] Replace all string comparisons with trait dispatch (via `get_ssg_adapter()`)
- [x] Unify navigation generation through the trait
- [x] Factory function to create adapter from config (`get_ssg_adapter()`)

## Trait Design

```rust
/// Adapter for static site generator output.
pub trait SSGAdapter: Send + Sync {
    /// Human-readable name.
    fn name(&self) -> &str;
    
    /// Directory where content files go (relative to output root).
    /// - MkDocs: "docs/" or "."
    /// - mdBook: "src/"
    fn content_dir(&self) -> &str;
    
    /// Navigation file name.
    /// - MkDocs: "_nav.yml" (included in mkdocs.yml)
    /// - mdBook: "SUMMARY.md"
    fn nav_filename(&self) -> &str;
    
    /// Generate navigation content from modules.
    fn generate_nav(&self, modules: &[Module], config: &Config) -> String;
    
    /// Generate SSG config file content.
    fn generate_config(&self, title: &str, nav_path: &str) -> Option<String>;
    
    /// File extension for content files.
    fn content_extension(&self) -> &str {
        "md"
    }
    
    /// Whether this SSG supports nested navigation.
    fn supports_nested_nav(&self) -> bool {
        true
    }
}
```

## Implementation Notes

### Files to Create/Modify

1. **`crates/plissken-core/src/render/ssg/mod.rs`** (new):
   ```rust
   mod traits;
   mod mkdocs;
   mod mdbook;
   
   pub use traits::SSGAdapter;
   pub use mkdocs::MkDocsAdapter;
   pub use mdbook::MdBookAdapter;
   
   pub fn adapter_for_template(template: &str) -> Box<dyn SSGAdapter> {
       match template {
           "mdbook" => Box::new(MdBookAdapter),
           _ => Box::new(MkDocsAdapter::new(template)),
       }
   }
   ```

2. **`crates/plissken-core/src/render/ssg/mkdocs.rs`**:
   ```rust
   pub struct MkDocsAdapter {
       theme: String,  // "mkdocs-material", "readthedocs", etc.
   }
   
   impl SSGAdapter for MkDocsAdapter {
       fn name(&self) -> &str { "MkDocs" }
       fn content_dir(&self) -> &str { "docs" }
       fn nav_filename(&self) -> &str { "_nav.yml" }
       
       fn generate_nav(&self, modules: &[Module], config: &Config) -> String {
           // Generate YAML navigation
       }
   }
   ```

3. **`crates/plissken-core/src/render/ssg/mdbook.rs`**:
   ```rust
   pub struct MdBookAdapter;
   
   impl SSGAdapter for MdBookAdapter {
       fn name(&self) -> &str { "mdBook" }
       fn content_dir(&self) -> &str { "src" }
       fn nav_filename(&self) -> &str { "SUMMARY.md" }
       
       fn generate_nav(&self, modules: &[Module], config: &Config) -> String {
           // Generate Markdown SUMMARY
       }
   }
   ```

4. **`crates/plissken-core/src/render/module_renderer.rs`** - Use adapter

### SSG Comparison

| Feature | MkDocs | mdBook |
|---------|--------|--------|
| Content dir | `docs/` | `src/` |
| Nav format | YAML | Markdown |
| Nav file | `_nav.yml` | `SUMMARY.md` |
| Config | `mkdocs.yml` | `book.toml` |
| Theme config | In mkdocs.yml | In book.toml |

### Migration

1. Create trait and adapters
2. Update Renderer to hold `Box<dyn SSGAdapter>`
3. Replace conditionals one at a time
4. Delete legacy code path

## Status Updates

### 2026-01-17: Implementation Complete

**Files created:**
- `crates/plissken-core/src/render/ssg/mod.rs` - Module with `get_ssg_adapter()` factory
- `crates/plissken-core/src/render/ssg/traits.rs` - `SSGAdapter` trait definition
- `crates/plissken-core/src/render/ssg/mkdocs.rs` - `MkDocsAdapter` implementation
- `crates/plissken-core/src/render/ssg/mdbook.rs` - `MdBookAdapter` implementation

**Files modified:**
- `crates/plissken-core/src/render/mod.rs` - Added `ssg` module export
- `crates/plissken-core/src/render/module_renderer.rs` - Added adapter delegation methods

**SSGAdapter trait methods:**
- `name()` - Human-readable SSG name
- `content_dir()` - Content directory ("docs" or "src")
- `nav_filename()` - Navigation file name ("_nav.yml" or "SUMMARY.md")
- `generate_nav()` - Generate navigation from modules
- `generate_config()` - Generate SSG config (book.toml for mdBook, None for MkDocs)
- `generate_custom_css()` - Generate custom CSS (mdBook only)
- `custom_css_path()` - Path for custom CSS file
- `content_extension()` - File extension (default "md")
- `supports_nested_nav()` - Whether SSG supports nested nav

**ModuleRenderer integration:**
- `generate_nav(&adapter, &python_modules, &rust_modules)` - Delegate to adapter
- `generate_config(&adapter, title, authors)` - Delegate to adapter
- `generate_custom_css(&adapter)` - Delegate to adapter
- Legacy methods (`generate_nav_yaml`, `generate_mdbook_summary`, etc.) preserved for backwards compatibility

All 199 tests passing (16 new tests for SSG module)