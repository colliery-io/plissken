# How To: Customize Templates

## Override Directory

Create a `.plissken/templates/` directory in your project root:

```
.plissken/
└── templates/
    ├── partials/
    │   ├── badge.html
    │   ├── signature.html
    │   └── code_block.html
    └── module.html
```

plissken checks this directory first when loading templates. Any file you
place here overrides the bundled default with the same name. Files you
don't override continue to use the built-in templates.

## Available Templates

| Template | Purpose |
|----------|---------|
| `partials/badge.html` | Inline badge rendering (async, unsafe, visibility, etc.) |
| `partials/signature.html` | Function/method signature display |
| `partials/code_block.html` | Fenced code block rendering |
| `module.html` | Full module page layout |

## Template Syntax

Templates use [Tera](https://keats.github.io/tera/) syntax, which is
similar to Jinja2/Django templates. Every template receives a `theme`
object and template-specific variables.

For the complete list of variables available in each template, see the
[Template Variables Reference](../reference/template-variables.md).

## Example: Custom Badge

Create `.plissken/templates/partials/badge.html`:

```html
{% if color_type == "blue" %}{% set color = theme.primary %}
{% elif color_type == "green" %}{% set color = theme.success %}
{% elif color_type == "red" %}{% set color = theme.error %}
{% elif color_type == "yellow" %}{% set color = theme.warning %}
{% elif color_type == "purple" %}{% set color = theme.badge_binding %}
{% elif color_type == "orange" %}{% set color = theme.badge_pub_crate %}
{% else %}{% set color = theme.muted %}{% endif %}
<span class="plissken-badge plissken-badge-{{ badge_type }}"
      style="display: inline-block;
             background: {{ color }};
             color: white;
             padding: 0.15em 0.5em;
             border-radius: 12px;
             font-size: 0.75em;
             font-weight: 600;
             font-family: system-ui, sans-serif;
             text-transform: uppercase;
             letter-spacing: 0.05em;">
    {{ text }}
</span>
```

The template receives `color_type` (a string like `"blue"`, `"green"`,
etc.) and must resolve it to a concrete color using the `theme` object.
The CSS classes `plissken-badge` and `plissken-badge-{{ badge_type }}`
allow additional styling via external CSS.

## CSS Overrides Without Template Changes

If you only want to change colors, you can use CSS instead of template
overrides. Add a stylesheet to MkDocs:

```yaml
# mkdocs.yml
extra_css:
  - stylesheets/plissken.css
```

```css
/* docs/stylesheets/plissken.css */
.plissken-badge-async {
    background: linear-gradient(45deg, #4CAF50, #8BC34A) !important;
}

.plissken-badge-unsafe {
    background: #b71c1c !important;
}

.plissken-signature {
    border-left: 3px solid var(--md-primary-fg-color);
    padding-left: 1em;
}
```
