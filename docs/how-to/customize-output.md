# Customization

Customize plissken's output with template overrides and theme configuration.

## Template Overrides

Create custom templates in `.plissken/templates/`:

```
.plissken/
└── templates/
    ├── partials/
    │   ├── badge.html       # Badge appearance
    │   ├── signature.html   # Function signatures
    │   └── code_block.html  # Code blocks
    └── module.html          # Module page layout
```

Templates use [Tera](https://keats.github.io/tera/) syntax.

### Badge Template

Override `partials/badge.html`:

```html
<span class="plissken-badge plissken-badge-{{ badge_type }}"
      style="background: {{ color }}; color: white;
             padding: 0.2em 0.5em; border-radius: 3px;
             font-size: 0.8em; font-weight: bold;">
    {{ text }}
</span>
```

Available variables:

| Variable | Description |
|----------|-------------|
| `text` | Badge text |
| `badge_type` | Type: async, unsafe, deprecated, etc. |
| `color_type` | Color hint: blue, green, red, etc. |
| `theme.*` | Theme colors |

### Signature Template

Override `partials/signature.html`:

```html
<div class="plissken-signature" style="font-family: monospace;">
    {% if is_async %}<span style="color: {{ theme.accent }}">async </span>{% endif %}
    <span style="color: {{ theme.primary }}; font-weight: bold;">{{ name }}</span>
    (<span>{{ params }}</span>)
    {% if return_type %} -> <span>{{ return_type }}</span>{% endif %}
</div>
```

## Theme Colors

Themes provide semantic colors via CSS variables:

### MkDocs Material

Uses Material Design CSS variables that adapt to dark mode:

| Color | CSS Variable |
|-------|-------------|
| Primary | `var(--md-primary-fg-color)` |
| Accent | `var(--md-accent-fg-color)` |
| Code BG | `var(--md-code-bg-color)` |
| Code FG | `var(--md-code-fg-color)` |
| Muted | `var(--md-default-fg-color--light)` |
| Border | `var(--md-default-fg-color--lightest)` |

### mdBook

Uses mdBook's theme variables:

| Color | CSS Variable |
|-------|-------------|
| Primary | `var(--links)` |
| Code BG | `var(--code-bg)` |
| Code FG | `var(--inline-code-color)` |

### Badge Colors

| Badge | Default Color |
|-------|--------------|
| async | Primary (blue) |
| unsafe | Error (red) |
| deprecated | Warning (yellow) |
| binding | Purple |
| pub | Success (green) |
| pub(crate) | Orange |

## CSS Customization

Add custom CSS in MkDocs:

```yaml
# mkdocs.yml
extra_css:
  - stylesheets/plissken.css
```

```css
/* docs/stylesheets/plissken.css */

/* Override badge styles */
.plissken-badge {
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

/* Custom async badge */
.plissken-badge-async {
    background: linear-gradient(45deg, #4CAF50, #8BC34A) !important;
}

/* Signature styling */
.plissken-signature {
    background: var(--md-code-bg-color);
    padding: 1em;
    border-radius: 4px;
    overflow-x: auto;
}
```

## Navigation Customization

The generated `_nav.yml` can be included in your mkdocs.yml:

```yaml
nav:
  - Home: index.md
  - Guide:
    - Overview: guide/overview.md
  - API Reference:
    - api/index.md
    # Manually curate important items
    - Core:
      - DataProcessor: api/python/mypackage/DataProcessor.md
      - process(): api/python/mypackage/process.md
    # Or include everything
    - Full API: api/_nav.yml
```

## Output Path Customization

Change where docs are generated:

```toml
[output]
path = "docs/reference"  # Custom path
```

For mdBook:

```toml
[output]
path = "src"  # mdBook convention
template = "mdbook"
```
