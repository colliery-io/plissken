# plissken-core::render::module::crossref <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Cross-reference link generation for bidirectional Python/Rust documentation.

This module handles the computation of cross-reference links between
Python and Rust documentation pages, enabling seamless navigation
between binding implementations and their Python APIs.

## Structs

### `plissken-core::render::module::crossref::CrossRefLinker`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `Default`

Generates cross-reference links between Python and Rust documentation.

`CrossRefLinker` encapsulates the logic for finding and formatting
links between Python classes/functions and their Rust implementations,
and vice versa.

**Examples:**

```rust
use plissken_core::render::module::CrossRefLinker;
use plissken_core::model::{CrossRef, CrossRefKind};

let cross_refs = vec![
    CrossRef {
        python_path: "mypackage.MyClass".to_string(),
        rust_path: "mycrate::MyStruct".to_string(),
        relationship: CrossRefKind::Binding,
    },
];

let linker = CrossRefLinker::new(cross_refs);

// Generate link from Rust struct page to Python class
let link = linker.python_link_for_rust_struct("mycrate", "MyStruct");
assert!(link.is_some());
```

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `cross_refs` | `Vec < CrossRef >` |  |

#### Methods

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn new (cross_refs : Vec < CrossRef >) -> Self
```

Create a new CrossRefLinker with the given cross-references.

<details>
<summary>Source</summary>

```rust
    pub fn new(cross_refs: Vec<CrossRef>) -> Self {
        Self { cross_refs }
    }
```

</details>



##### `empty` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn empty () -> Self
```

Create an empty CrossRefLinker (no cross-references).

<details>
<summary>Source</summary>

```rust
    pub fn empty() -> Self {
        Self::default()
    }
```

</details>



##### `has_refs` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn has_refs (& self) -> bool
```

Check if any cross-references are available.

<details>
<summary>Source</summary>

```rust
    pub fn has_refs(&self) -> bool {
        !self.cross_refs.is_empty()
    }
```

</details>



##### `python_link_for_rust_method` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn python_link_for_rust_method (& self , rust_path : & str , method_name : & str , parent_struct : Option < & str > ,) -> Option < String >
```

Find cross-ref for a Rust method and generate link to Python counterpart (inline format).

For methods (parent_struct is Some), links to the method anchor within the Python module page.
For standalone functions (parent_struct is None), links to the function anchor.

<details>
<summary>Source</summary>

```rust
    pub fn python_link_for_rust_method(
        &self,
        rust_path: &str,
        method_name: &str,
        parent_struct: Option<&str>,
    ) -> Option<String> {
        match parent_struct {
            Some(struct_name) => {
                // Look up the parent struct's cross-ref
                let full_rust_path = format!("{}::{}", rust_path, struct_name);
                for xref in &self.cross_refs {
                    if xref.rust_path == full_rust_path
                        || xref.rust_path.ends_with(&format!("::{}", struct_name))
                    {
                        // Get Python module and class name
                        let (python_module, _python_class) =
                            if let Some(pos) = xref.python_path.rfind('.') {
                                (&xref.python_path[..pos], &xref.python_path[pos + 1..])
                            } else {
                                (xref.python_path.as_str(), xref.python_path.as_str())
                            };

                        // Compute path to Python module page
                        let prefix = compute_rust_relative_prefix(rust_path);
                        let python_page = compute_python_page_path(python_module);
                        let anchor = method_name.to_lowercase();
                        let python_method_path = format!("{}.{}", xref.python_path, method_name);
                        return Some(format!(
                            "> **Python API**: [{}]({}{}#{})\n\n",
                            python_method_path, prefix, python_page, anchor
                        ));
                    }
                }
                None
            }
            None => {
                // Fall back to function linking for standalone functions
                self.python_link_for_rust_function(rust_path, method_name)
            }
        }
    }
```

</details>



##### `python_link_for_rust_function` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn python_link_for_rust_function (& self , rust_path : & str , func_name : & str ,) -> Option < String >
```

Find cross-ref for a Rust function and generate link to Python function (inline format).

With inline rendering, links go to the module page with an anchor for the function.
From: `rust/{crate}.md` -> To: `{module}.md#{funcname}`

<details>
<summary>Source</summary>

```rust
    pub fn python_link_for_rust_function(
        &self,
        rust_path: &str,
        func_name: &str,
    ) -> Option<String> {
        let full_rust_path = format!("{}::{}", rust_path, func_name);
        for xref in &self.cross_refs {
            if xref.rust_path == full_rust_path
                || xref.rust_path.ends_with(&format!("::{}", func_name))
            {
                // Get Python module path
                let python_module = if let Some(pos) = xref.python_path.rfind('.') {
                    &xref.python_path[..pos]
                } else {
                    xref.python_path.as_str()
                };

                // Compute path to Python module page
                let prefix = compute_rust_relative_prefix(rust_path);
                let python_page = compute_python_page_path(python_module);
                let anchor = func_name.to_lowercase();

                return Some(format!(
                    "> **Python API**: [{}]({}{}#{})\n\n",
                    xref.python_path, prefix, python_page, anchor
                ));
            }
        }
        None
    }
```

</details>



##### `python_link_for_rust_struct` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn python_link_for_rust_struct (& self , rust_path : & str , struct_name : & str ,) -> Option < String >
```

Find cross-ref for a Rust struct and generate link to Python class (inline format).

With inline rendering, links go to the module page with an anchor for the class.
From: `rust/{crate}.md` -> To: `{module}.md#class-{classname}`
Note: The anchor uses the Python class name (e.g., `Task`), not the Rust
struct name (e.g., `PyTask`), because the rendered markdown heading shows
the Python class name.

<details>
<summary>Source</summary>

```rust
    pub fn python_link_for_rust_struct(
        &self,
        rust_path: &str,
        struct_name: &str,
    ) -> Option<String> {
        let full_rust_path = format!("{}::{}", rust_path, struct_name);
        for xref in &self.cross_refs {
            if xref.rust_path == full_rust_path
                || xref.rust_path.ends_with(&format!("::{}", struct_name))
            {
                // Get Python module and class name
                let (python_module, python_class) = if let Some(pos) = xref.python_path.rfind('.') {
                    (&xref.python_path[..pos], &xref.python_path[pos + 1..])
                } else {
                    (xref.python_path.as_str(), xref.python_path.as_str())
                };

                // Compute path to Python module page
                let prefix = compute_rust_relative_prefix(rust_path);
                let python_page = compute_python_page_path(python_module);
                // Use Python class name for anchor (matches the rendered heading)
                let anchor = python_class.to_lowercase();
                return Some(format!(
                    "> **Python API**: [{}]({}{}#class-{})\n\n",
                    xref.python_path, prefix, python_page, anchor
                ));
            }
        }
        None
    }
```

</details>



##### `rust_link_for_python_function` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn rust_link_for_python_function (& self , python_path : & str , func_name : & str ,) -> Option < String >
```

Find cross-ref for a Python function and generate link to Rust function (inline format).

With inline rendering, links go to the Rust module page with an anchor for the function.
From: `{module}.md` -> To: `rust/{crate}.md#fn-{funcname}`
Note: Rust function headings use `### \`fn funcname\`` format, generating `#fn-funcname` anchors.

<details>
<summary>Source</summary>

```rust
    pub fn rust_link_for_python_function(
        &self,
        python_path: &str,
        func_name: &str,
    ) -> Option<String> {
        let full_python_path = format!("{}.{}", python_path, func_name);
        for xref in &self.cross_refs {
            if xref.python_path == full_python_path
                || xref.python_path.ends_with(&format!(".{}", func_name))
            {
                // Get the Rust module and function name
                let (rust_module, rust_func) = if let Some(pos) = xref.rust_path.rfind("::") {
                    (&xref.rust_path[..pos], &xref.rust_path[pos + 2..])
                } else {
                    (xref.rust_path.as_str(), xref.rust_path.as_str())
                };

                // Compute path from Python module page to Rust module page
                let prefix = compute_python_relative_prefix(python_path);
                let rust_page = compute_rust_page_path(rust_module);
                // Rust function anchor includes "fn-" prefix (from heading `### \`fn funcname\``)
                let anchor = format!("fn-{}", rust_func.to_lowercase());

                return Some(format!(
                    "> **Rust Implementation**: [{}]({}{}#{})\n\n",
                    xref.rust_path, prefix, rust_page, anchor
                ));
            }
        }
        None
    }
```

</details>



##### `rust_link_for_python_class` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn rust_link_for_python_class (& self , python_path : & str , class_name : & str ,) -> Option < String >
```

Find cross-ref for a Python class and generate link to Rust struct (inline format).

With inline rendering, links go to the Rust module page with an anchor for the struct.
From: `{module}.md` -> To: `rust/{crate}.md#class-{classname}`
Note: The anchor uses the Python class name (e.g., `Task`), not the Rust
struct name (e.g., `PyTask`), because the Rust doc heading shows the
Python binding name.

<details>
<summary>Source</summary>

```rust
    pub fn rust_link_for_python_class(
        &self,
        python_path: &str,
        class_name: &str,
    ) -> Option<String> {
        let full_python_path = format!("{}.{}", python_path, class_name);
        for xref in &self.cross_refs {
            if xref.python_path == full_python_path
                || xref.python_path.ends_with(&format!(".{}", class_name))
            {
                // Get the Rust module
                let rust_module = if let Some(pos) = xref.rust_path.rfind("::") {
                    &xref.rust_path[..pos]
                } else {
                    xref.rust_path.as_str()
                };

                // Compute path from Python module page to Rust module page
                let prefix = compute_python_relative_prefix(python_path);
                let rust_page = compute_rust_page_path(rust_module);
                // Use Python class name for anchor (Rust docs show binding name in heading)
                let anchor = class_name.to_lowercase();

                return Some(format!(
                    "> **Rust Implementation**: [{}]({}{}#class-{})\n\n",
                    xref.rust_path, prefix, rust_page, anchor
                ));
            }
        }
        None
    }
```

</details>



##### `rust_link_for_python_method` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn rust_link_for_python_method (& self , python_path : & str , method_name : & str , parent_class : Option < & str > ,) -> Option < String >
```

Find cross-ref for a Python method and generate link to Rust implementation (inline format).

With inline rendering, links go to the Rust module page with an anchor for the method.
From: `{module}.md` -> To: `rust/{crate}.md#{method}`
For methods (parent_class is Some), looks up the parent class's cross-ref
and generates a link with the method as an anchor.
For standalone functions (parent_class is None), falls back to function linking.

<details>
<summary>Source</summary>

```rust
    pub fn rust_link_for_python_method(
        &self,
        python_path: &str,
        method_name: &str,
        parent_class: Option<&str>,
    ) -> Option<String> {
        match parent_class {
            Some(class_name) => {
                // Look up the parent class's cross-ref
                let full_python_path = format!("{}.{}", python_path, class_name);
                for xref in &self.cross_refs {
                    if xref.python_path == full_python_path
                        || xref.python_path.ends_with(&format!(".{}", class_name))
                    {
                        // Get the Rust module
                        let rust_module = if let Some(pos) = xref.rust_path.rfind("::") {
                            &xref.rust_path[..pos]
                        } else {
                            xref.rust_path.as_str()
                        };

                        // Compute path from Python module page to Rust module page
                        let prefix = compute_python_relative_prefix(python_path);
                        let rust_page = compute_rust_page_path(rust_module);
                        let anchor = method_name.to_lowercase();
                        let rust_method_path = format!("{}::{}", xref.rust_path, method_name);

                        return Some(format!(
                            "> **Rust Implementation**: [{}]({}{}#{})\n\n",
                            rust_method_path, prefix, rust_page, anchor
                        ));
                    }
                }
                None
            }
            None => {
                // Fall back to function-level linking for standalone functions
                self.rust_link_for_python_function(python_path, method_name)
            }
        }
    }
```

</details>





## Functions

### `plissken-core::render::module::crossref::compute_python_page_path`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn compute_python_page_path (module_path : & str) -> String
```

Compute the relative file path for a Python module page.

Single segment: `mymodule` -> `mymodule.md`
Nested: `mypackage.sub.module` -> `mypackage/sub/module.md`

<details>
<summary>Source</summary>

```rust
fn compute_python_page_path(module_path: &str) -> String {
    let parts: Vec<&str> = module_path.split('.').collect();
    if parts.len() == 1 {
        format!("{}.md", parts[0])
    } else {
        let last = parts.last().unwrap();
        let parent = parts[..parts.len() - 1].join("/");
        format!("{}/{}.md", parent, last)
    }
}
```

</details>



### `plissken-core::render::module::crossref::compute_rust_page_path`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn compute_rust_page_path (module_path : & str) -> String
```

Compute the relative file path for a Rust module page.

Single segment: `mycrate` -> `rust/mycrate.md`
Nested: `mycrate::sub::module` -> `rust/mycrate/sub/module.md`

<details>
<summary>Source</summary>

```rust
fn compute_rust_page_path(module_path: &str) -> String {
    format!("rust/{}.md", module_path.replace("::", "/"))
}
```

</details>



### `plissken-core::render::module::crossref::compute_rust_relative_prefix`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn compute_rust_relative_prefix (rust_path : & str) -> String
```

Compute the `../` prefix to navigate from a Rust module to the root.

The depth is 1 (for `rust/` directory) plus the number of `::` separators.

<details>
<summary>Source</summary>

```rust
fn compute_rust_relative_prefix(rust_path: &str) -> String {
    let depth = 1 + rust_path.matches("::").count();
    "../".repeat(depth)
}
```

</details>



### `plissken-core::render::module::crossref::compute_python_relative_prefix`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn compute_python_relative_prefix (python_path : & str) -> String
```

Compute the `../` prefix to navigate from a Python module to the root.

Single segment modules (e.g., `mypackage`) are at root level (depth 0).
Nested modules need to go up for each parent directory.

<details>
<summary>Source</summary>

```rust
fn compute_python_relative_prefix(python_path: &str) -> String {
    let parts: Vec<&str> = python_path.split('.').collect();
    let depth = if parts.len() == 1 { 0 } else { parts.len() - 1 };
    "../".repeat(depth)
}
```

</details>



