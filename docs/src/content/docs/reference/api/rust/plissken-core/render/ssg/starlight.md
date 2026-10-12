---
title: "plissken-core::render::ssg::starlight"
description: "Astro Starlight adapter implementation"
sidebar:
  order: 25
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


Astro Starlight adapter implementation

Pages go into a folder of the Starlight content collection
(`src/content/docs/...`). Starlight builds a route for each page from its
file path, so this module also holds the rules that turn a page path into
a route, and a route into a relative link that works under any `base`.

## Structs

### <span id="struct.StarlightAdapter"></span>`struct StarlightAdapter`

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

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `route_base` | `String` |  |

#### Methods

##### <span id="StarlightAdapter.new"></span>`new`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn new (route_base : & str) -> Self
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



##### <span id="StarlightAdapter.for_output_path"></span>`for_output_path`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn for_output_path (output_path : & Path) -> Option < Self >
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



##### <span id="StarlightAdapter.route_base"></span>`route_base`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn route_base (& self) -> & str
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



##### <span id="StarlightAdapter.slug"></span>`slug`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn slug (& self , file_path : & Path) -> String
```

The Starlight slug of a page (its content id).

<details>
<summary>Source</summary>

```rust
    fn slug(&self, file_path: &Path) -> String {
        let route = page_route(&file_path.to_string_lossy().replace('\\', "/"));
        match (self.route_base.is_empty(), route.is_empty()) {
            (true, _) => route,
            (false, true) => self.route_base.clone(),
            (false, false) => format!("{}/{}", self.route_base, route),
        }
    }
```

</details>



##### <span id="StarlightAdapter.group"></span>`group`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn group (& self , label : & str , entries : Vec < NavEntry > , separator : & str) -> String
```

The sidebar group of one language, modules nested by path.

<details>
<summary>Source</summary>

```rust
    fn group(&self, label: &str, entries: Vec<NavEntry>, separator: &str) -> String {
        let mut root = Node::default();
        for entry in entries {
            let mut node = &mut root;
            for part in entry.path.split(separator) {
                node = node.child(part);
            }
            node.slug = Some(self.slug(&entry.file_path));
        }
        let items: Vec<String> = root.children.iter().map(|c| c.to_item(4)).collect();
        group_item(label, false, &items, 2)
    }
```

</details>





### <span id="struct.Node"></span>`struct Node`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


**Derives:** `Default`

A module in the sidebar tree.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `slug` | `Option < String >` |  |
| `children` | `Vec < Node >` |  |

#### Methods

##### <span id="Node.child"></span>`child`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn child (& mut self , name : & str) -> & mut Node
```

<details>
<summary>Source</summary>

```rust
    fn child(&mut self, name: &str) -> &mut Node {
        let pos = match self.children.iter().position(|c| c.name == name) {
            Some(pos) => pos,
            None => {
                self.children.push(Node {
                    name: name.to_string(),
                    ..Default::default()
                });
                self.children.len() - 1
            }
        };
        &mut self.children[pos]
    }
```

</details>



##### <span id="Node.to_item"></span>`to_item`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn to_item (& self , indent : usize) -> String
```

The sidebar item of this module: a link, or a collapsed group of its own page ("Overview") and its submodules.

<details>
<summary>Source</summary>

```rust
    fn to_item(&self, indent: usize) -> String {
        match (&self.slug, self.children.is_empty()) {
            (Some(slug), true) => link_item(&self.name, slug),
            (slug, _) => {
                let mut items = Vec::new();
                if let Some(slug) = slug {
                    items.push(link_item("Overview", slug));
                }
                items.extend(self.children.iter().map(|c| c.to_item(indent + 2)));
                group_item(&self.name, true, &items, indent)
            }
        }
    }
```

</details>





## Functions

### <span id="fn.github_slug"></span>`fn github_slug`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn github_slug (value : & str) -> String
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



### <span id="fn.page_route"></span>`fn page_route`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn page_route (file_path : & str) -> String
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



### <span id="fn.starlight_file_path"></span>`fn starlight_file_path`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn starlight_file_path (path : & Path) -> PathBuf
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



### <span id="fn.route_href"></span>`fn route_href`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn route_href (from_file : & str , to_file : & str , anchor : Option < & str >) -> String
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



### <span id="fn.content_route_base"></span>`fn content_route_base`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn content_route_base (output_path : & Path) -> Option < String >
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



### <span id="fn.yaml_string"></span>`fn yaml_string`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn yaml_string (value : & str) -> String
```

Quote a string for YAML front matter (a JSON string is valid YAML).

<details>
<summary>Source</summary>

```rust
fn yaml_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}
```

</details>



### <span id="fn.front_matter"></span>`fn front_matter`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn front_matter (title : & str , description : & str , order : usize) -> String
```

Front matter of a Starlight page.

`sidebar.hidden` keeps the page out of an `autogenerate` sidebar group:
the generated sidebar module lists the pages instead.

<details>
<summary>Source</summary>

```rust
pub fn front_matter(title: &str, description: &str, order: usize) -> String {
    let description = description.split_whitespace().collect::<Vec<_>>().join(" ");
    format!(
        "---\ntitle: {}\ndescription: {}\nsidebar:\n  order: {}\n  hidden: true\n---\n\n",
        yaml_string(title),
        yaml_string(&description),
        order
    )
}
```

</details>



### <span id="fn.anchor_span"></span>`fn anchor_span`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn anchor_span (id : & str) -> String
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



### <span id="fn.normalize_rustdoc_fences"></span>`fn normalize_rustdoc_fences`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn normalize_rustdoc_fences (content : & str) -> String
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



### <span id="fn.js_string"></span>`fn js_string`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn js_string (value : & str) -> String
```

A JavaScript string literal.

<details>
<summary>Source</summary>

```rust
fn js_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}
```

</details>



### <span id="fn.link_item"></span>`fn link_item`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn link_item (label : & str , slug : & str) -> String
```

A sidebar link to a page of the content collection.

<details>
<summary>Source</summary>

```rust
fn link_item(label: &str, slug: &str) -> String {
    format!(
        "{{ label: {}, slug: {} }}",
        js_string(label),
        js_string(slug)
    )
}
```

</details>



### <span id="fn.group_item"></span>`fn group_item`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn group_item (label : & str , collapsed : bool , items : & [String] , indent : usize) -> String
```

A sidebar group, as a JavaScript object literal.

<details>
<summary>Source</summary>

```rust
fn group_item(label: &str, collapsed: bool, items: &[String], indent: usize) -> String {
    let pad = "  ".repeat(indent);
    let mut out = format!("{{\n{pad}  label: {},\n", js_string(label));
    if collapsed {
        out.push_str(&format!("{pad}  collapsed: true,\n"));
    }
    out.push_str(&format!("{pad}  items: [\n"));
    for item in items {
        out.push_str(&format!("{pad}    {},\n", item));
    }
    out.push_str(&format!("{pad}  ],\n{pad}}}"));
    out
}
```

</details>



