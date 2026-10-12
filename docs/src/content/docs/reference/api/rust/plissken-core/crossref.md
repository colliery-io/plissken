---
title: "plissken-core::crossref"
description: "Cross-reference builder for linking Python and Rust items"
editUrl: false
sidebar:
  order: 4
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Cross-reference builder for linking Python and Rust items

This module builds cross-references between Python items and their Rust
implementations by matching PyO3 metadata.

## Structs

### <span id="struct.CrossRefBuilder"></span><code style="font-size: 0.85em">struct CrossRefBuilder&lt;&#39;a&gt;</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Builds cross-references between Python and Rust items

#### Methods

##### <span id="CrossRefBuilder.new"></span><code>new</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn new(config: &'a Config) -> Self
```

Create a new cross-reference builder

<details>
<summary>Source</summary>

```rust
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            pyclass_map: HashMap::new(),
            pyfunction_map: HashMap::new(),
            pymethod_map: HashMap::new(),
        }
    }
```

</details>

##### <span id="CrossRefBuilder.build"></span><code>build</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn build(
    mut self,
    rust_modules: &[RustModule],
    python_modules: Vec<PythonModule>,
) -> (Vec<PythonModule>, Vec<CrossRef>)
```

Build cross-references and return updated modules and refs

<details>
<summary>Source</summary>

```rust
    pub fn build(
        mut self,
        rust_modules: &[RustModule],
        python_modules: Vec<PythonModule>,
    ) -> (Vec<PythonModule>, Vec<CrossRef>) {
        // Step 1: Index all PyO3 items from Rust
        self.index_rust_modules(rust_modules);

        // Step 2: Match Python items to Rust and collect cross-refs
        let mut cross_refs = Vec::new();
        let updated_modules = python_modules
            .into_iter()
            .map(|module| self.process_python_module(module, &mut cross_refs))
            .collect();

        (updated_modules, cross_refs)
    }
```

</details>

## Functions

### <span id="fn.build_cross_refs"></span><code style="font-size: 0.85em">fn build_​cross_​refs</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn build_cross_refs(
    config: &Config,
    rust_modules: &[RustModule],
    python_modules: Vec<PythonModule>,
) -> (Vec<PythonModule>, Vec<CrossRef>)
```

Build cross-references for a doc model

<details>
<summary>Source</summary>

```rust
pub fn build_cross_refs(
    config: &Config,
    rust_modules: &[RustModule],
    python_modules: Vec<PythonModule>,
) -> (Vec<PythonModule>, Vec<CrossRef>) {
    let builder = CrossRefBuilder::new(config);
    builder.build(rust_modules, python_modules)
}
```

</details>

### <span id="fn.synthesize_python_from_rust"></span><code style="font-size: 0.85em">fn synthesize_​python_​from_​rust</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn synthesize_python_from_rust(
    rust_modules: &[RustModule],
    module_name: &str,
) -> (PythonModule, Vec<CrossRef>)
```

Synthesize Python modules from Rust PyO3 bindings

This creates Python module representations from Rust code that uses PyO3.
It extracts #[pymodule], #[pyclass], #[pyfunction], and #[pymethods] to
build a Python-side view of the API.

<details>
<summary>Source</summary>

```rust
pub fn synthesize_python_from_rust(
    rust_modules: &[RustModule],
    module_name: &str,
) -> (PythonModule, Vec<CrossRef>) {
    let mut items = Vec::new();
    let mut cross_refs = Vec::new();

    // Collect all pyclass structs and their methods
    let mut pyclass_methods: HashMap<String, Vec<PythonFunction>> = HashMap::new();

    for module in rust_modules {
        // First pass: collect methods from pymethods impl blocks
        for item in &module.items {
            if let RustItem::Impl(impl_block) = item
                && impl_block.pymethods
            {
                let methods: Vec<_> = impl_block
                    .methods
                    .iter()
                    .map(|m| synthesize_python_method(m, &impl_block.target, &module.path))
                    .collect();
                pyclass_methods
                    .entry(impl_block.target.clone())
                    .or_default()
                    .extend(methods);
            }
        }

        // Second pass: create Python classes/functions
        for item in &module.items {
            match item {
                RustItem::Struct(s) => {
                    if let Some(ref pyclass) = s.pyclass {
                        let py_name = pyclass.name.clone().unwrap_or_else(|| s.name.clone());
                        let rust_path = format!("{}::{}", module.path, s.name);

                        // Get methods for this class
                        let methods = pyclass_methods.remove(&s.name).unwrap_or_default();

                        let class = PythonClass {
                            name: py_name.clone(),
                            docstring: s.doc_comment.clone(),
                            parsed_doc: None,
                            bases: Vec::new(),
                            methods,
                            attributes: Vec::new(),
                            decorators: Vec::new(),
                            rust_impl: Some(RustItemRef::new(&rust_path, &s.name)),
                            source: s.source.clone(),
                        };

                        cross_refs.push(CrossRef::binding(
                            format!("{}.{}", module_name, py_name),
                            &rust_path,
                        ));

                        items.push(PythonItem::Class(class));
                    }
                }
                RustItem::Function(f) => {
                    if let Some(ref pyfunc) = f.pyfunction {
                        let py_name = pyfunc.name.clone().unwrap_or_else(|| f.name.clone());
                        let rust_path = format!("{}::{}", module.path, f.name);

                        let func = synthesize_python_function(f, &rust_path);

                        cross_refs.push(CrossRef::binding(
                            format!("{}.{}", module_name, py_name),
                            &rust_path,
                        ));

                        items.push(PythonItem::Function(func));
                    }
                }
                _ => {}
            }
        }
    }

    let python_module = PythonModule {
        path: module_name.to_string(),
        docstring: rust_modules.first().and_then(|m| m.doc_comment.clone()),
        parsed_doc: None,
        items,
        source_type: SourceType::PyO3Binding,
        source: rust_modules
            .first()
            .map(|m| m.source.clone())
            .unwrap_or_else(|| SourceSpan::test("", 0, 0)),
    };

    (python_module, cross_refs)
}
```

</details>

### <span id="fn.synthesize_python_modules_from_rust"></span><code style="font-size: 0.85em">fn synthesize_​python_​modules_​from_​rust</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn synthesize_python_modules_from_rust(
    rust_modules: &[RustModule],
    python_package: &str,
    rust_entry_point: &str,
) -> Vec<(PythonModule, Vec<CrossRef>)>
```

Synthesize Python modules from Rust modules with PyO3 bindings.

As [`synthesize_python_api`](../../../rust/plissken-core/crossref/#fn.synthesize_python_api), with the given crate as the entry point.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `rust_modules` | `&[RustModule]` | The Rust modules to synthesize from |
| `python_package` | `&str` | The Python package name (e.g., "pysnake") |
| `rust_entry_point` | `&str` | The Rust crate entry point (e.g., "rustscale") |

<details>
<summary>Source</summary>

```rust
pub fn synthesize_python_modules_from_rust(
    rust_modules: &[RustModule],
    python_package: &str,
    rust_entry_point: &str,
) -> Vec<(PythonModule, Vec<CrossRef>)> {
    synthesize_python_api(rust_modules, python_package, Some(rust_entry_point))
}
```

</details>

### <span id="fn.synthesize_python_api"></span><code style="font-size: 0.85em">fn synthesize_​python_​api</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn synthesize_python_api(
    rust_modules: &[RustModule],
    python_package: &str,
    rust_entry_point: Option<&str>,
) -> Vec<(PythonModule, Vec<CrossRef>)>
```

Synthesize the Python API of the PyO3 bindings of Rust modules.

A `#[pymodule]` function names a Python module: `fn cloaca(m)` is the
module `cloaca`, or `{python_package}.{name}` when the package has
another name. Its classes and functions are those that it adds, and
those that its helpers add (`register(m)`); a submodule that it adds
(`m.add_submodule(&sub)`) is a module below it.

A binding that no `#[pymodule]` adds goes to the Python module of its
Rust module, the crate name replaced by the package
(`rustscale::handlers` -> `pysnake.handlers`). With an entry point,
only that crate is replaced; without one, every crate.

Returns a list of (PythonModule, Vec<CrossRef>) pairs.

<details>
<summary>Source</summary>

```rust
pub fn synthesize_python_api(
    rust_modules: &[RustModule],
    python_package: &str,
    rust_entry_point: Option<&str>,
) -> Vec<(PythonModule, Vec<CrossRef>)> {
    let mut placed: HashSet<String> = HashSet::new();
    let mut result = pymodule_api(rust_modules, python_package, &mut placed);

    // In a crate with a #[pymodule], a binding that it does not add cannot
    // be imported: it has no Python page
    let crate_of = |path: &str| path.split("::").next().unwrap_or(path).to_string();
    let pymodule_crates: HashSet<String> = rust_modules
        .iter()
        .filter(|m| m.py_registrars.iter().any(|r| r.pymodule.is_some()))
        .map(|m| crate_of(&m.path))
        .collect();

    for module in rust_modules {
        if pymodule_crates.contains(&crate_of(&module.path)) {
            continue;
        }
        let py_module_path = python_module_path(&module.path, python_package, rust_entry_point);
        let mut synth = SynthModule::default();
        for item in &module.items {
            match item {
                RustItem::Struct(s)
                    if s.pyclass.is_some()
                        && !placed.contains(&rust_item_path(module, &s.name)) =>
                {
                    synth.classes.push((module, s));
                }
                RustItem::Function(f)
                    if f.pyfunction.is_some()
                        && !placed.contains(&rust_item_path(module, &f.name)) =>
                {
                    synth.functions.push((module, f));
                }
                _ => {}
            }
        }
        if synth.classes.is_empty() && synth.functions.is_empty() {
            continue;
        }
        synth.doc_comment = module.doc_comment.clone();
        let (python_module, cross_refs) =
            synth.build(&py_module_path, rust_modules, &module.source);
        match result
            .iter_mut()
            .find(|(m, _)| m.path == python_module.path)
        {
            Some((existing, refs)) => {
                existing.items.extend(python_module.items);
                refs.extend(cross_refs);
            }
            None => result.push((python_module, cross_refs)),
        }
    }

    result
}
```

</details>
