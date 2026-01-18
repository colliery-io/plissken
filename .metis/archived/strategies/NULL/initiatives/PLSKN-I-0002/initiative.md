---
id: documentation-renderer
level: initiative
title: "Documentation Renderer"
short_code: "PLSKN-I-0002"
created_at: 2026-01-14T14:24:15.699478+00:00
updated_at: 2026-01-14T20:56:21.549275+00:00
parent: PLSKN-V-0001
blocked_by: []
archived: true

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: M
strategy_id: NULL
initiative_id: documentation-renderer
---

# Documentation Renderer Initiative

## Context

With PLSKN-I-0001 complete, we have a working parser that produces a DocModel containing:
- Rust modules with structs, functions, traits, impls, and PyO3 metadata
- Python modules with classes, functions, and parsed docstrings
- Cross-references linking Python items to their Rust implementations

Now we need to **render** this model into actual documentation. The vision emphasizes that "auto-generated docs are ugly because they treat documentation as a data dump, not communication." This initiative focuses on producing documentation that people actually want to read.

## Goals & Non-Goals

**Goals:**
- Render DocModel to Markdown with embedded HTML/CSS where needed
- Smart signature formatting (line breaks, type highlighting)
- Parameter tables instead of walls of text
- Semantic badges (async, unsafe, deprecated, visibility levels)
- Collapsible source code blocks
- Cross-reference links between Python and Rust items
- Template system (Tera) for customization
- Built-in templates for MkDocs Material and mdBook

**Non-Goals:**
- Full SSG (we output Markdown, not a complete site)
- JavaScript interactivity beyond what SSGs provide
- PDF/ePub output (Markdown first)
- Incremental/watch mode (future initiative)



## Detailed Design

### Output Structure

```
docs/api/
├── index.md                    # Package overview
├── python/
│   ├── module.md              # Python module docs
│   └── module.submodule.md
└── rust/
    ├── crate.md               # Rust crate docs  
    └── crate.module.md
```

**No external CSS files** - styling is inlined using SSG's native CSS variables.

### Rendering Pipeline

```
DocModel + ThemeAdapter → Tera Templates → Markdown + Inline HTML → Output Files
```

### Key Innovation: Theme Adapters

Instead of shipping CSS that might conflict with SSG themes, we **inline styles that reference the SSG's own CSS variables**. This gives us:

- **Zero CSS files** to configure or maintain
- **Automatic dark mode** - SSG updates variables, our output follows
- **No style conflicts** - we use their variables, not fight them
- **Easy extensibility** - new SSG = new adapter implementation

#### ThemeAdapter Trait

```rust
trait ThemeAdapter {
    fn code_bg(&self) -> &str;
    fn code_fg(&self) -> &str;
    fn primary(&self) -> &str;
    fn accent(&self) -> &str;
    fn muted(&self) -> &str;
    fn border(&self) -> &str;
}
```

#### SSG Implementations

**MkDocs Material:**
```rust
impl ThemeAdapter for MkDocsMaterial {
    fn code_bg(&self) -> &str { "var(--md-code-bg-color)" }
    fn code_fg(&self) -> &str { "var(--md-code-fg-color)" }
    fn primary(&self) -> &str { "var(--md-primary-fg-color)" }
    fn accent(&self) -> &str  { "var(--md-accent-fg-color)" }
    fn muted(&self) -> &str   { "var(--md-default-fg-color--light)" }
    fn border(&self) -> &str  { "var(--md-default-fg-color--lightest)" }
}
```

**mdBook:**
```rust
impl ThemeAdapter for MdBook {
    fn code_bg(&self) -> &str { "var(--code-bg)" }
    fn code_fg(&self) -> &str { "var(--inline-code-color)" }
    fn primary(&self) -> &str { "var(--links)" }
    fn accent(&self) -> &str  { "var(--links)" }
    fn muted(&self) -> &str   { "var(--fg)" }
    fn border(&self) -> &str  { "var(--quote-border)" }
}
```

**Minimal (fallback):**
```rust
impl ThemeAdapter for Minimal {
    fn code_bg(&self) -> &str { "#f5f5f5" }
    fn code_fg(&self) -> &str { "#333" }
    fn primary(&self) -> &str { "#1976d2" }
    fn accent(&self) -> &str  { "#448aff" }
    fn muted(&self) -> &str   { "#757575" }
    fn border(&self) -> &str  { "#e0e0e0" }
}
```

### Templates Use Semantic Names

```html
<span class="plissken-badge" style="
  background: {{ theme.accent }};
  color: white;
  padding: 0.2em 0.5em;
  border-radius: 4px;
  font-size: 0.85em;
">async</span>
```

Output for MkDocs Material:
```html
<span class="plissken-badge" style="background: var(--md-accent-fg-color); color: white; padding: 0.2em 0.5em; border-radius: 4px; font-size: 0.85em;">async</span>
```

The `class="plissken-badge"` is optional but gives users a hook to override if desired.

### Key Components

**1. Template Engine (Tera)**
- Base templates for module, class, function pages
- Partial templates for signatures, parameter tables, badges
- Theme adapter injected into template context

**2. Signature Formatter**
- Line breaks for long signatures
- Type highlighting using theme colors
- Generic parameter formatting

**3. Badge System**
- `async`, `unsafe`, `deprecated` markers
- Visibility badges: `pub`, `pub(crate)`, private
- Source type: 🐍 Python, 🦀 Rust, 🐍↔🦀 Binding

**4. Source Code Blocks**
- Collapsible `<details>` for implementation
- Native markdown code fences (SSG handles highlighting)
- "View source" links when git info available

**5. Cross-Reference Links**
- Python item → Rust implementation section
- Rust item → Python exposure
- Internal links between documented items

### Example Output (MkDocs Material)

```markdown
## `DataFrame.filter`

<span class="plissken-badge" style="background: var(--md-accent-fg-color); color: white; padding: 0.2em 0.5em; border-radius: 4px; font-size: 0.85em;">🐍↔🦀 binding</span>

```python
def filter(self, predicate: Expr) -> DataFrame
```

Filters rows based on a boolean expression.

### Parameters

| Name | Type | Description |
|------|------|-------------|
| predicate | `Expr` | Boolean filter expression |

### Returns

`DataFrame` — Filtered dataframe

<details>
<summary>🦀 Rust Implementation</summary>

Implements: [`polars::LazyFrame::filter`](rust/polars.lazyframe.md#filter)

```rust
pub fn filter(&self, predicate: PyExpr) -> PyResult<Self>
```

</details>
```



## Alternatives Considered

**Bundled CSS file**: Ship `plissken.css` that users include in their SSG config.
- Rejected: Creates potential conflicts with SSG themes, requires user configuration, dark mode requires duplicating variables

**CSS with fallback cascade**: `var(--md-code-bg-color, var(--code-bg, #f5f5f5))`
- Rejected: Verbose, harder to maintain, still requires external CSS file

**SSG plugins**: Native MkDocs/mdBook plugins that run during build.
- Rejected for MVP: Higher complexity, separate codebases per SSG. Theme adapters get us 90% there with simpler architecture. Could revisit later.

**Direct HTML generation**: Skip Markdown, generate HTML directly.
- Rejected: Markdown integrates with existing SSGs that users already use

**Handlebars instead of Tera**: More widespread in Rust ecosystem.
- Rejected: Tera is more powerful (inheritance, macros), Jinja2-like syntax familiar to Python users

## Implementation Plan

### Tasks (to be decomposed)

1. **Theme Adapter System** - Trait + impls for MkDocs Material, mdBook, Minimal
2. **Tera Template Engine** - Set up Tera, inject theme adapter, create base templates
3. **Badge & Inline Styling** - Badge HTML generation with theme-aware inline styles
4. **Module Page Renderer** - Generate per-module Markdown files for Python + Rust
5. **Parameter Tables** - Render parsed docstrings as formatted tables
6. **Cross-Reference Links** - Generate internal links between Python ↔ Rust items
7. **CLI Render Command** - `plissken render` to output docs to configured path

### MVP Scope

For MVP, focus on:
- Theme adapters: MkDocs Material + Minimal fallback
- Python module pages with classes and functions
- Parameter tables from parsed docstrings
- Basic badges (async, visibility, source type)
- Collapsible Rust implementation blocks via `<details>`
- Cross-reference links between Python and Rust items

### Success Criteria

- Run `plissken render` on hybrid_binary fixture
- Output renders correctly in MkDocs Material (badges styled, dark mode works)
- Output is readable in plain Markdown (fallback works)
- Cross-references link Python classes to their Rust implementations