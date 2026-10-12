---
title: "plissken-core::filter"
description: "Which items are documented"
editUrl: false
sidebar:
  order: 8
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Which items are documented

The parsers keep all items. Before rendering, these functions leave out
what the documentation of a public API does not show: test code, and
private items unless the configuration asks for them. PyO3 bindings are
part of the Python API, so they stay whatever their Rust visibility.

## Functions

### <span id="fn.documented_rust_modules"></span><code style="font-size: 0.85em">fn documented_​rust_​modules</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn documented_rust_modules(
    modules: Vec<RustModule>,
    include_private: bool,
) -> Vec<RustModule>
```

The Rust modules to document.

Leaves out the modules declared `#[cfg(test)] mod name;` and their
submodules. Unless `include_private`, leaves out the items, fields and
methods that are not `pub` (bindings stay), and `pub(crate)` re-exports.
Then leaves out the modules with nothing to show: no doc comment, no
items, no re-exports.

<details>
<summary>Source</summary>

```rust
pub fn documented_rust_modules(modules: Vec<RustModule>, include_private: bool) -> Vec<RustModule> {
    let test_paths: Vec<String> = modules
        .iter()
        .flat_map(|m| {
            m.test_modules
                .iter()
                .map(move |t| format!("{}::{}", m.path, t))
        })
        .collect();
    let is_test = |path: &str| {
        test_paths
            .iter()
            .any(|t| path == t || path.starts_with(&format!("{}::", t)))
    };

    modules
        .into_iter()
        .filter(|m| !is_test(&m.path))
        .map(|mut m| {
            if !include_private {
                keep_public_rust(&mut m);
            }
            m
        })
        .filter(has_rust_content)
        .collect()
}
```

</details>

### <span id="fn.is_private_python_name"></span><code style="font-size: 0.85em">fn is_​private_​python_​name</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn is_private_python_name(name: &str) -> bool
```

Whether a Python name is private: it starts with `_` and is not a `__dunder__` name.

<details>
<summary>Source</summary>

```rust
pub fn is_private_python_name(name: &str) -> bool {
    name.starts_with('_') && !(name.starts_with("__") && name.ends_with("__") && name.len() > 4)
}
```

</details>

### <span id="fn.documented_python_modules"></span><code style="font-size: 0.85em">fn documented_​python_​modules</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn documented_python_modules(
    modules: Vec<PythonModule>,
    include_private: bool,
) -> Vec<PythonModule>
```

The Python modules to document.

Unless `include_private`, leaves out the classes, functions, variables,
methods and attributes whose names are private (`_helper`). Then leaves
out the modules with nothing to show.

<details>
<summary>Source</summary>

```rust
pub fn documented_python_modules(
    modules: Vec<PythonModule>,
    include_private: bool,
) -> Vec<PythonModule> {
    modules
        .into_iter()
        .map(|mut m| {
            if !include_private {
                m.items.retain_mut(|item| match item {
                    PythonItem::Class(c) => {
                        keep_public_python_class(c);
                        !is_private_python_name(&c.name)
                    }
                    PythonItem::Function(f) => !is_private_python_name(&f.name),
                    PythonItem::Variable(v) => !is_private_python_name(&v.name),
                });
            }
            m
        })
        .filter(|m| {
            m.docstring.as_deref().is_some_and(|d| !d.trim().is_empty()) || !m.items.is_empty()
        })
        .collect()
}
```

</details>

### <span id="fn.documented_cross_refs"></span><code style="font-size: 0.85em">fn documented_​cross_​refs</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn documented_cross_refs(
    cross_refs: Vec<CrossRef>,
    python_modules: &[PythonModule],
    rust_modules: &[RustModule],
) -> Vec<CrossRef>
```

The cross-references whose Python item and Rust item are both documented, so that no link points to an item that was left out.

<details>
<summary>Source</summary>

```rust
pub fn documented_cross_refs(
    cross_refs: Vec<CrossRef>,
    python_modules: &[PythonModule],
    rust_modules: &[RustModule],
) -> Vec<CrossRef> {
    let mut python: HashSet<String> = HashSet::new();
    for m in python_modules {
        for item in &m.items {
            match item {
                PythonItem::Class(c) => {
                    python.insert(format!("{}.{}", m.path, c.name));
                    for f in &c.methods {
                        python.insert(format!("{}.{}.{}", m.path, c.name, f.name));
                    }
                }
                PythonItem::Function(f) => {
                    python.insert(format!("{}.{}", m.path, f.name));
                }
                PythonItem::Variable(v) => {
                    python.insert(format!("{}.{}", m.path, v.name));
                }
            }
        }
    }
    let mut rust: HashSet<String> = HashSet::new();
    for m in rust_modules {
        for item in &m.items {
            match item {
                RustItem::Struct(s) => {
                    rust.insert(format!("{}::{}", m.path, s.name));
                }
                RustItem::Function(f) => {
                    rust.insert(format!("{}::{}", m.path, f.name));
                }
                RustItem::Impl(i) => {
                    let target = i.target.split('<').next().unwrap_or(&i.target).trim();
                    for f in &i.methods {
                        rust.insert(format!("{}::{}::{}", m.path, target, f.name));
                    }
                }
                _ => {}
            }
        }
    }
    cross_refs
        .into_iter()
        .filter(|x| python.contains(&x.python_path) && rust.contains(&x.rust_path))
        .collect()
}
```

</details>
