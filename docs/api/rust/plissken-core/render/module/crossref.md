# crossref <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Cross-reference link generation for bidirectional Python/Rust documentation.

This module handles the computation of cross-reference links between
Python and Rust documentation pages, enabling seamless navigation
between binding implementations and their Python APIs.

## Structs

### `struct CrossRefLinker`

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
                        let rust_depth = 1 + rust_path.matches("::").count();
                        let prefix = "../".repeat(rust_depth);

                        // Python module page path
                        let module_parts: Vec<&str> = python_module.split('.').collect();
                        let python_page = if module_parts.len() == 1 {
                            format!("{}.md", module_parts[0])
                        } else {
                            let last = module_parts.last().unwrap();
                            let parent = module_parts[..module_parts.len()-1].join("/");
                            format!("{}/{}.md", parent, last)
                        };

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
                let rust_depth = 1 + rust_path.matches("::").count();
                let prefix = "../".repeat(rust_depth);

                // Python module page path
                let module_parts: Vec<&str> = python_module.split('.').collect();
                let python_page = if module_parts.len() == 1 {
                    format!("{}.md", module_parts[0])
                } else {
                    let last = module_parts.last().unwrap();
                    let parent = module_parts[..module_parts.len()-1].join("/");
                    format!("{}/{}.md", parent, last)
                };

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
                let (python_module, python_class) =
                    if let Some(pos) = xref.python_path.rfind('.') {
                        (&xref.python_path[..pos], &xref.python_path[pos + 1..])
                    } else {
                        (xref.python_path.as_str(), xref.python_path.as_str())
                    };

                // Compute path to Python module page
                // From rust/{path}.md to {module}.md (or nested path)
                let rust_depth = 1 + rust_path.matches("::").count(); // rust/ + submodules
                let prefix = "../".repeat(rust_depth);

                // Python module page path
                let module_parts: Vec<&str> = python_module.split('.').collect();
                let python_page = if module_parts.len() == 1 {
                    format!("{}.md", module_parts[0])
                } else {
                    let last = module_parts.last().unwrap();
                    let parent = module_parts[..module_parts.len()-1].join("/");
                    format!("{}/{}.md", parent, last)
                };

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
                let python_parts: Vec<&str> = python_path.split('.').collect();
                let python_depth = if python_parts.len() == 1 { 0 } else { python_parts.len() - 1 };
                let prefix = "../".repeat(python_depth);

                // Rust module page path
                let rust_page = format!("rust/{}.md", rust_module.replace("::", "/"));
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
                // Python pages are at: {module}.md or {parent}/{module}.md
                // Rust pages are at: rust/{crate}.md or rust/{crate}/{submod}.md
                let python_parts: Vec<&str> = python_path.split('.').collect();
                let python_depth = if python_parts.len() == 1 { 0 } else { python_parts.len() - 1 };
                let prefix = "../".repeat(python_depth);

                // Rust module page path
                let rust_page = format!("rust/{}.md", rust_module.replace("::", "/"));
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
                        let python_parts: Vec<&str> = python_path.split('.').collect();
                        let python_depth = if python_parts.len() == 1 { 0 } else { python_parts.len() - 1 };
                        let prefix = "../".repeat(python_depth);

                        // Rust module page path
                        let rust_page = format!("rust/{}.md", rust_module.replace("::", "/"));
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





