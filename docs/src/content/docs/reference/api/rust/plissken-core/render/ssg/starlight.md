---
title: "plissken-core::render::ssg::starlight"
description: "Astro Starlight adapter implementation"
editUrl: false
sidebar:
  order: 28
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Astro Starlight adapter implementation

Pages go into a folder of the Starlight content collection
(`src/content/docs/...`). Starlight builds a route for each page from its
file path, so this module also holds the rules that turn a page path into
a route, and a route into a relative link that works under any `base`.

## Structs

### <span id="struct.StarlightAdapter"></span><code style="font-size: 0.85em">struct StarlightAdapter</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Astro Starlight adapter.

Generates `_sidebar.mjs`: an ES module whose default export is a
Starlight sidebar group of the Python and Rust modules. The site's
`astro.config.mjs` imports it:

```js
import apiSidebar from './src/content/docs/reference/api/_sidebar.mjs';
// ...
sidebar: [ /* other groups */ apiSidebar ],
```

Each link is a `slug` entry, so Starlight adds the site `base` to it.

#### Methods

##### <span id="StarlightAdapter.new"></span><code>new</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn new(route_base: &str) -> Self
```

Create an adapter for pages under the given route of the site (for example `reference/api`).

<details>
<summary>Source</summary>

```rust
    pub fn new(route_base: &str) -> Self {
        Self {
            route_base: route_base.trim_matches('/').to_string(),
        }
    }
```

</details>

##### <span id="StarlightAdapter.for_output_path"></span><code>for_​output_​path</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn for_output_path(output_path: &Path) -> Option<Self>
```

Create an adapter for an output folder in a Starlight site.

Returns `None` if the path is not in `src/content/docs/`.

<details>
<summary>Source</summary>

```rust
    pub fn for_output_path(output_path: &Path) -> Option<Self> {
        content_route_base(output_path).map(|base| Self::new(&base))
    }
```

</details>

##### <span id="StarlightAdapter.route_base"></span><code>route_​base</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn route_base(&self) -> &str
```

The route of the output folder in the site.

<details>
<summary>Source</summary>

```rust
    pub fn route_base(&self) -> &str {
        &self.route_base
    }
```

</details>

## Functions

### <span id="fn.github_slug"></span><code style="font-size: 0.85em">fn github_​slug</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn github_slug(value: &str) -> String
```

Slug a string as Astro does (`github-slugger`).

Astro slugs each segment of a content file path with it, and Starlight
slugs heading text with it. Lowercase; keep letters, numbers, `_`, `-`
and spaces; spaces become `-`.

<details>
<summary>Source</summary>

```rust
pub fn github_slug(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == ' ')
        .map(|c| if c == ' ' { '-' } else { c })
        .collect()
}
```

</details>

### <span id="fn.page_route"></span><code style="font-size: 0.85em">fn page_​route</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn page_route(file_path: &str) -> String
```

The route of a page, relative to the output folder.

`rust/my_crate/sub.md` -> `rust/my_crate/sub`; `pkg/_core/index.md` ->
`pkg/_core`. Each segment is slugged as Astro does.

<details>
<summary>Source</summary>

```rust
pub fn page_route(file_path: &str) -> String {
    let without_ext = file_path.strip_suffix(".md").unwrap_or(file_path);
    let route = without_ext
        .split('/')
        .filter(|s| !s.is_empty())
        .map(github_slug)
        .collect::<Vec<_>>()
        .join("/");
    match route.strip_suffix("/index") {
        Some(r) => r.to_string(),
        None if route == "index" => String::new(),
        None => route,
    }
}
```

</details>

### <span id="fn.starlight_file_path"></span><code style="font-size: 0.85em">fn starlight_​file_​path</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn starlight_file_path(path: &Path) -> PathBuf
```

The file path of a page in a Starlight content folder.

Starlight skips a file whose name starts with `_`, so `pkg/_core.md` is
written as `pkg/_core/index.md`. Its route stays `pkg/_core`.

<details>
<summary>Source</summary>

```rust
pub fn starlight_file_path(path: &Path) -> PathBuf {
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    if stem.starts_with('_') {
        path.with_extension("").join("index.md")
    } else {
        path.to_path_buf()
    }
}
```

</details>

### <span id="fn.route_href"></span><code style="font-size: 0.85em">fn route_​href</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn route_href(
    from_file: &str,
    to_file: &str,
    anchor: Option<&str>,
) -> String
```

A relative link from one page to another, as a route link.

Both paths are page paths relative to the output folder (`foo/bar.md`).
Starlight serves the page `foo/bar.md` at `foo/bar/`, so the link climbs
one level per segment of the source route. The result has no `.md` and
does not start with `/`, so it works under any `base`.

<details>
<summary>Source</summary>

```rust
pub fn route_href(from_file: &str, to_file: &str, anchor: Option<&str>) -> String {
    let from = page_route(from_file);
    let depth = if from.is_empty() {
        0
    } else {
        from.split('/').count()
    };
    let to = page_route(to_file);
    let mut href = "../".repeat(depth);
    if !to.is_empty() {
        href.push_str(&to);
        href.push('/');
    }
    if href.is_empty() {
        href.push_str("./");
    }
    if let Some(anchor) = anchor {
        href.push('#');
        href.push_str(anchor);
    }
    href
}
```

</details>

### <span id="fn.content_route_base"></span><code style="font-size: 0.85em">fn content_​route_​base</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn content_route_base(output_path: &Path) -> Option<String>
```

The route of the output folder in the Starlight site.

The part of `output_path` after `src/content/docs/`, slugged:
`docs/src/content/docs/reference/api` -> `Some("reference/api")`.
`None` if the path is not in a Starlight content folder.

<details>
<summary>Source</summary>

```rust
pub fn content_route_base(output_path: &Path) -> Option<String> {
    let parts: Vec<String> = output_path
        .components()
        .filter_map(|c| match c {
            Component::Normal(s) => Some(s.to_string_lossy().to_string()),
            _ => None,
        })
        .collect();
    let start = parts
        .windows(3)
        .rposition(|w| w[0] == "src" && w[1] == "content" && w[2] == "docs")?;
    Some(
        parts[start + 3..]
            .iter()
            .map(|s| github_slug(s))
            .collect::<Vec<_>>()
            .join("/"),
    )
}
```

</details>

### <span id="fn.front_matter"></span><code style="font-size: 0.85em">fn front_​matter</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn front_matter(title: &str, description: &str, order: usize) -> String
```

Front matter of a Starlight page.

`sidebar.hidden` keeps the page out of an `autogenerate` sidebar group:
the generated sidebar module lists the pages instead. `editUrl: false`:
a generated page has no source file to edit.

<details>
<summary>Source</summary>

```rust
pub fn front_matter(title: &str, description: &str, order: usize) -> String {
    let description = description.split_whitespace().collect::<Vec<_>>().join(" ");
    format!(
        "---\ntitle: {}\ndescription: {}\neditUrl: false\nsidebar:\n  order: {}\n  hidden: true\n---\n\n",
        yaml_string(title),
        yaml_string(&description),
        order
    )
}
```

</details>

### <span id="fn.breakable_name"></span><code style="font-size: 0.85em">fn breakable_​name</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn breakable_name(text: &str) -> String
```

Heading text that a narrow column (the table of contents) can break after each `_`: a zero-width space follows it. Slugs drop the space, so anchors do not change.

<details>
<summary>Source</summary>

```rust
pub fn breakable_name(text: &str) -> String {
    text.replace('_', "_\u{200B}")
}
```

</details>

### <span id="fn.anchor_span"></span><code style="font-size: 0.85em">fn anchor_​span</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn anchor_span(id: &str) -> String
```

An HTML element that gives an item an explicit anchor id.

Put at the start of a Markdown heading. It has no text, so the slug and
the table of contents entry of the heading do not change.

<details>
<summary>Source</summary>

```rust
pub fn anchor_span(id: &str) -> String {
    format!("<span id=\"{}\"></span>", id)
}
```

</details>

### <span id="fn.normalize_rustdoc_fences"></span><code style="font-size: 0.85em">fn normalize_​rustdoc_​fences</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn normalize_rustdoc_fences(content: &str) -> String
```

Give rustdoc code fences the language `rust`.

In Rust doc comments, a fence such as ```` ```ignore ```` or
```` ```rust,no_run ```` holds Rust code. Starlight's highlighter knows no
language `ignore`, so it would show plain text.

<details>
<summary>Source</summary>

```rust
pub fn normalize_rustdoc_fences(content: &str) -> String {
    let mut out = String::with_capacity(content.len());
    for line in content.split_inclusive('\n') {
        let trimmed = line.trim_start();
        let indent = &line[..line.len() - trimmed.len()];
        let fence_len = trimmed.chars().take_while(|c| *c == '`').count();
        let info = trimmed[fence_len..].trim_end();
        let is_rustdoc = fence_len >= 3
            && !info.is_empty()
            && info
                .split(',')
                .map(str::trim)
                .all(|a| RUSTDOC_FENCE_ATTRIBUTES.contains(&a));
        if is_rustdoc && info != "rust" {
            out.push_str(indent);
            out.push_str(&trimmed[..fence_len]);
            out.push_str("rust");
            out.push_str(&line[line.trim_end().len()..]);
        } else {
            out.push_str(line);
        }
    }
    out
}
```

</details>

## Constants

| Name | Type | Value | Description |
|------|------|-------|-------------|
| <span id="const.STARLIGHT_SIDEBAR_FILE"></span>`STARLIGHT_SIDEBAR_FILE` | `&str` | `"_sidebar.mjs"` | File name of the generated sidebar module.<br>The leading `_` keeps it out of the content collection. |
| <span id="const.STARLIGHT_DEFAULT_ROUTE_BASE"></span>`STARLIGHT_DEFAULT_ROUTE_BASE` | `&str` | `"reference/api"` | The route base used when the output path is not in `src/content/docs/`. |
| <span id="const.STARLIGHT_INDEX_FILE"></span>`STARLIGHT_INDEX_FILE` | `&str` | `"index.md"` | The file of the index page of the API reference (item 15 of PLSKN-T-0057): the page at the route of the output folder. |
