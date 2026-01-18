---
id: add-external-template-file-support
level: task
title: "Add external template file support with user overrides"
short_code: "PLSKN-T-0032"
created_at: 2026-01-16T18:01:47.398867+00:00
updated_at: 2026-01-17T01:47:40.643828+00:00
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

# Add external template file support with user overrides

## Parent Initiative

[[PLSKN-I-0009]] Rendering System Refactor

## Objective

Move templates from hardcoded Rust strings to external files that users can override. This enables customization of badge appearance, module layout, and navigation structure without recompiling plissken.

## Current Problem

Templates are currently embedded as string literals in Rust code:
```rust
const BADGE_TEMPLATE: &str = r#"<span style="background-color:...">#
```

This means:
- Users cannot customize output appearance
- Changing templates requires rebuilding
- Template strings are hard to read/maintain in code
- No syntax highlighting or template tooling

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

- [x] Extract all templates to separate `.html`/`.jinja2` files
- [x] Bundle templates as default with `include_str!` or embed
- [x] Load user templates from `.plissken/templates/` if present
- [x] User templates override bundled defaults (per-file granularity)
- [ ] Document available template variables and extension points (deferred - templates/README.md)
- [x] Maintain backwards compatibility (no config change needed)

## Template Directory Structure

### Bundled (in crate)
```
crates/plissken-core/
  templates/
    badge.html           # Type/async/unsafe badges
    module.html          # Module page layout
    function.html        # Function documentation block
    class.html           # Class documentation block
    nav_mkdocs.jinja2    # MkDocs navigation YAML
    nav_mdbook.jinja2    # mdBook SUMMARY.md
```

### User Override
```
.plissken/
  templates/
    badge.html           # Overrides bundled badge.html
    custom_footer.html   # New template (for future use)
```

## Implementation Notes

### Files to Create/Modify

1. **`crates/plissken-core/templates/`** (new directory) - Extract templates:
   - Move badge template from `module_renderer.rs`
   - Create module/function/class templates from render logic

2. **`crates/plissken-core/src/render/templates.rs`** (new) - Template loader:
   ```rust
   pub struct TemplateLoader {
       bundled: HashMap<String, &'static str>,
       user_dir: Option<PathBuf>,
   }
   
   impl TemplateLoader {
       pub fn new(project_root: &Path) -> Self {
           let user_dir = project_root.join(".plissken/templates");
           Self {
               bundled: Self::load_bundled(),
               user_dir: user_dir.exists().then_some(user_dir),
           }
       }
       
       pub fn get(&self, name: &str) -> Result<String> {
           // Check user override first
           if let Some(dir) = &self.user_dir {
               let user_path = dir.join(name);
               if user_path.exists() {
                   return Ok(fs::read_to_string(user_path)?);
               }
           }
           // Fall back to bundled
           self.bundled.get(name)
               .map(|s| s.to_string())
               .ok_or_else(|| PlisskenError::TemplateNotFound(name.into()))
       }
       
       fn load_bundled() -> HashMap<String, &'static str> {
           let mut map = HashMap::new();
           map.insert("badge.html".into(), include_str!("../templates/badge.html"));
           map.insert("module.html".into(), include_str!("../templates/module.html"));
           // ...
           map
       }
   }
   ```

3. **`crates/plissken-core/src/render/module_renderer.rs`** - Use loader

### Template Variables Documentation

Create `templates/README.md` documenting available variables:
```markdown
# Template Variables

## badge.html
- `{{ badge_type }}` - "async", "unsafe", "deprecated", etc.
- `{{ color }}` - Badge background color
- `{{ text }}` - Badge text content

## module.html
- `{{ module }}` - Full module object
- `{{ functions }}` - List of functions
- `{{ classes }}` - List of classes
- `{{ theme }}` - Current theme object
```

## Status Updates

### 2026-01-17: Implementation Complete

**Files created:**
- `crates/plissken-core/templates/partials/badge.html` - Badge template
- `crates/plissken-core/templates/partials/code_block.html` - Code block template
- `crates/plissken-core/templates/partials/signature.html` - Function signature template
- `crates/plissken-core/templates/module.html` - Module page template
- `crates/plissken-core/src/render/templates.rs` - TemplateLoader struct

**Files modified:**
- `crates/plissken-core/src/render/mod.rs` - Added templates module and export
- `crates/plissken-core/src/render/renderer.rs` - Updated to use TemplateLoader, removed hardcoded templates
- `crates/plissken-core/src/render/module_renderer.rs` - Updated Renderer::new signature in tests
- `crates/plissken-cli/src/main.rs` - Updated Renderer::new to pass project_root

**Implementation details:**
- TemplateLoader loads bundled templates via `include_str!` at compile time
- User overrides checked in `.plissken/templates/` directory
- Per-file override granularity (user badge.html doesn't affect other templates)
- Renderer::new() now accepts optional `project_root: Option<&Path>` parameter
- Fully backwards compatible - existing code works with `Renderer::new(theme, None)`
- All 183 tests passing (6 new tests for TemplateLoader)