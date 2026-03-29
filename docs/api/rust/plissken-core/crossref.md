# plissken-core::crossref <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Cross-reference builder for linking Python and Rust items

This module builds cross-references between Python items and their Rust
implementations by matching PyO3 metadata.

## Structs

### `plissken-core::crossref::CrossRefBuilder`<'a>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Builds cross-references between Python and Rust items

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `config` | `& 'a Config` |  |
| `pyclass_map` | `HashMap < String , (String , String) >` | Map of Python name → Rust item (struct name, module path) |
| `pyfunction_map` | `HashMap < String , (String , String) >` | Map of Python name → Rust function (fn name, module path) |
| `pymethod_map` | `HashMap < (String , String) , String >` | Map of (Rust struct name, method name) → method info |



## Functions

### `plissken-core::crossref::build_cross_refs`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn build_cross_refs (config : & Config , rust_modules : & [RustModule] , python_modules : Vec < PythonModule > ,) -> (Vec < PythonModule > , Vec < CrossRef >)
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



### `plissken-core::crossref::synthesize_python_from_rust`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn synthesize_python_from_rust (rust_modules : & [RustModule] , module_name : & str ,) -> (PythonModule , Vec < CrossRef >)
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



### `plissken-core::crossref::synthesize_python_function`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn synthesize_python_function (rust_fn : & RustFunction , rust_path : & str) -> PythonFunction
```

Synthesize a Python function from a Rust function

<details>
<summary>Source</summary>

```rust
fn synthesize_python_function(rust_fn: &RustFunction, rust_path: &str) -> PythonFunction {
    let py_name = rust_fn
        .pyfunction
        .as_ref()
        .and_then(|pf| pf.name.clone())
        .unwrap_or_else(|| rust_fn.name.clone());

    // Convert Rust params to Python params
    let params: Vec<PythonParam> = rust_fn
        .signature
        .params
        .iter()
        .filter(|p| p.name != "self" && p.name != "&self" && p.name != "py")
        .map(|p| PythonParam {
            name: p.name.clone(),
            ty: Some(rust_type_to_python(&p.ty)),
            default: p.default.clone(),
        })
        .collect();

    // Build signature string
    let sig_str = if let Some(ref pyfunc) = rust_fn.pyfunction {
        if let Some(ref sig) = pyfunc.signature {
            format!("def {}{}:", py_name, sig)
        } else {
            format!("def {}(...):", py_name)
        }
    } else {
        format!("def {}(...):", py_name)
    };

    PythonFunction {
        name: py_name,
        docstring: rust_fn.doc_comment.clone(),
        signature_str: sig_str,
        signature: PythonFunctionSig {
            params,
            return_type: rust_fn
                .signature
                .return_type
                .as_ref()
                .map(|t| rust_type_to_python(t)),
        },
        decorators: Vec::new(),
        is_async: rust_fn.is_async,
        is_staticmethod: false,
        is_classmethod: false,
        is_property: false,
        parsed_doc: None,
        rust_impl: Some(RustItemRef::new(rust_path, &rust_fn.name)),
        source: rust_fn.source.clone(),
    }
}
```

</details>



### `plissken-core::crossref::synthesize_python_method`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn synthesize_python_method (rust_fn : & RustFunction , rust_struct : & str , rust_module : & str ,) -> PythonFunction
```

Synthesize a Python method from a Rust method in pymethods block

<details>
<summary>Source</summary>

```rust
fn synthesize_python_method(
    rust_fn: &RustFunction,
    rust_struct: &str,
    rust_module: &str,
) -> PythonFunction {
    let rust_path = format!("{}::{}::{}", rust_module, rust_struct, rust_fn.name);
    synthesize_python_function(rust_fn, &rust_path)
}
```

</details>



### `plissken-core::crossref::synthesize_python_modules_from_rust`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn synthesize_python_modules_from_rust (rust_modules : & [RustModule] , python_package : & str , rust_entry_point : & str ,) -> Vec < (PythonModule , Vec < CrossRef >) >
```

Synthesize Python modules from Rust modules with PyO3 bindings.

Unlike `synthesize_python_from_rust` which flattens into one module,
this preserves the module structure - each Rust module with bindings
becomes a separate Python module under the configured Python package namespace.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `rust_modules` | `-` | The Rust modules to synthesize from |
| `python_package` | `-` | The Python package name (e.g., "pysnake") |
| `rust_entry_point` | `-` | The Rust crate entry point (e.g., "rustscale") |


<details>
<summary>Source</summary>

```rust
pub fn synthesize_python_modules_from_rust(
    rust_modules: &[RustModule],
    python_package: &str,
    rust_entry_point: &str,
) -> Vec<(PythonModule, Vec<CrossRef>)> {
    let mut result = Vec::new();

    for module in rust_modules {
        // Check if this module has any bindings
        let has_bindings = module.items.iter().any(|item| match item {
            RustItem::Struct(s) => s.pyclass.is_some(),
            RustItem::Function(f) => f.pyfunction.is_some(),
            RustItem::Impl(i) => i.pymethods,
            _ => false,
        });

        if !has_bindings {
            continue;
        }

        let mut items = Vec::new();
        let mut cross_refs = Vec::new();

        // Convert Rust path to Python path, remapping the crate name to the Python package
        // e.g., rustscale::handlers -> pysnake.handlers
        let rust_path_dotted = module.path.replace("::", ".");
        let py_module_path = if rust_path_dotted == rust_entry_point {
            // Root module: rustscale -> pysnake
            python_package.to_string()
        } else if rust_path_dotted.starts_with(&format!("{}.", rust_entry_point)) {
            // Submodule: rustscale.handlers -> pysnake.handlers
            format!(
                "{}{}",
                python_package,
                &rust_path_dotted[rust_entry_point.len()..]
            )
        } else {
            // Fallback: use as-is (shouldn't happen with proper config)
            rust_path_dotted
        };

        // Collect methods from pymethods impl blocks
        let mut pyclass_methods: HashMap<String, Vec<PythonFunction>> = HashMap::new();
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

        // Create Python classes and functions
        for item in &module.items {
            match item {
                RustItem::Struct(s) => {
                    if let Some(ref pyclass) = s.pyclass {
                        let py_name = pyclass.name.clone().unwrap_or_else(|| s.name.clone());
                        let rust_path = format!("{}::{}", module.path, s.name);

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
                            format!("{}.{}", py_module_path, py_name),
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
                            format!("{}.{}", py_module_path, py_name),
                            &rust_path,
                        ));

                        items.push(PythonItem::Function(func));
                    }
                }
                _ => {}
            }
        }

        if !items.is_empty() {
            let python_module = PythonModule {
                path: py_module_path,
                docstring: module.doc_comment.clone(),
                parsed_doc: None,
                items,
                source_type: SourceType::PyO3Binding,
                source: module.source.clone(),
            };

            result.push((python_module, cross_refs));
        }
    }

    result
}
```

</details>



### `plissken-core::crossref::rust_type_to_python`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn rust_type_to_python (rust_type : & str) -> String
```

Convert a Rust type to Python type hint (best effort)

Handles PyO3 types, generic wrappers, and normalizes whitespace in type strings
(e.g., `PyResult < String >` -> unwrapped to Python equivalent).

<details>
<summary>Source</summary>

```rust
fn rust_type_to_python(rust_type: &str) -> String {
    // Normalize whitespace: "PyResult < String >" -> "PyResult<String>"
    let normalized: String = rust_type
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    rust_type_to_python_normalized(&normalized)
}
```

</details>



### `plissken-core::crossref::rust_type_to_python_normalized`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn rust_type_to_python_normalized (s : & str) -> String
```

Internal conversion after whitespace normalization

<details>
<summary>Source</summary>

```rust
fn rust_type_to_python_normalized(s: &str) -> String {
    // Handle tuple types first: (T1, T2, ...) -> Tuple[T1, T2, ...]
    if s.starts_with('(') && s.ends_with(')') {
        let inner = &s[1..s.len() - 1];
        if inner.is_empty() {
            return "None".to_string(); // () -> None
        }
        let elements = split_tuple_elements(inner);
        let converted: Vec<String> = elements
            .iter()
            .map(|e| rust_type_to_python_normalized(e.trim()))
            .collect();
        return format!("Tuple[{}]", converted.join(", "));
    }

    // Handle slice types: [T] -> List[T], with special case for [u8] -> bytes
    if s.starts_with('[') && s.ends_with(']') {
        let inner = &s[1..s.len() - 1];
        if inner == "u8" {
            return "bytes".to_string();
        }
        return format!("List[{}]", rust_type_to_python_normalized(inner));
    }

    // Strip path qualifiers (e.g., "pyo3::types::PyString" -> "PyString")
    let base_type = s.rsplit("::").next().unwrap_or(s);

    // Handle direct PyO3 type mappings first
    match base_type {
        // PyO3 primitive types
        "PyString" => return "str".to_string(),
        "PyList" => return "list".to_string(),
        "PyDict" => return "dict".to_string(),
        "PyTuple" => return "tuple".to_string(),
        "PySet" => return "set".to_string(),
        "PyFrozenSet" => return "frozenset".to_string(),
        "PyBytes" => return "bytes".to_string(),
        "PyByteArray" => return "bytearray".to_string(),
        "PyInt" | "PyLong" => return "int".to_string(),
        "PyFloat" => return "float".to_string(),
        "PyBool" => return "bool".to_string(),
        "PyNone" => return "None".to_string(),
        "PyModule" => return "ModuleType".to_string(),
        "PyType" => return "type".to_string(),
        "PyObject" | "PyAny" => return "Any".to_string(),
        _ => {}
    }

    // Handle the full string for generics and wrappers
    match s {
        // Rust primitives
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize"
        | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => "int".to_string(),
        "f32" | "f64" => "float".to_string(),
        "bool" => "bool".to_string(),
        "String" | "str" | "&str" | "&String" => "str".to_string(),
        "()" => "None".to_string(),
        "Self" => "Self".to_string(),
        "char" => "str".to_string(),

        // PyO3 direct types (without path)
        "PyObject" | "PyAny" => "Any".to_string(),
        "PyString" => "str".to_string(),
        "PyList" => "list".to_string(),
        "PyDict" => "dict".to_string(),
        "PyTuple" => "tuple".to_string(),
        "PySet" => "set".to_string(),
        "PyBytes" => "bytes".to_string(),
        "PyBool" => "bool".to_string(),
        "PyInt" | "PyLong" => "int".to_string(),
        "PyFloat" => "float".to_string(),
        "PyNone" => "None".to_string(),

        // Generic wrappers - extract inner type
        _ if s.starts_with("Vec<") && s.ends_with(">") => {
            let inner = &s[4..s.len() - 1];
            format!("List[{}]", rust_type_to_python_normalized(inner))
        }
        _ if s.starts_with("Option<") && s.ends_with(">") => {
            let inner = &s[7..s.len() - 1];
            format!("Optional[{}]", rust_type_to_python_normalized(inner))
        }
        _ if (s.starts_with("HashMap<") || s.starts_with("BTreeMap<")) && s.ends_with(">") => {
            // Try to extract key and value types
            let start = s.find('<').unwrap() + 1;
            let inner = &s[start..s.len() - 1];
            if let Some((key, val)) = split_generic_pair(inner) {
                format!("Dict[{}, {}]", rust_type_to_python_normalized(key), rust_type_to_python_normalized(val))
            } else {
                "Dict[str, Any]".to_string()
            }
        }
        _ if (s.starts_with("HashSet<") || s.starts_with("BTreeSet<")) && s.ends_with(">") => {
            let start = s.find('<').unwrap() + 1;
            let inner = &s[start..s.len() - 1];
            format!("Set[{}]", rust_type_to_python_normalized(inner))
        }

        // PyO3 wrappers - unwrap to inner type
        _ if s.starts_with("PyResult<") && s.ends_with(">") => {
            let inner = &s[9..s.len() - 1];
            rust_type_to_python_normalized(inner)
        }
        _ if s.starts_with("Py<") && s.ends_with(">") => {
            let inner = &s[3..s.len() - 1];
            rust_type_to_python_normalized(inner)
        }
        _ if s.starts_with("Bound<") && s.ends_with(">") => {
            // Bound<'_, PyDict> -> extract the type after the lifetime
            let inner = &s[6..s.len() - 1];
            // Skip lifetime parameter: "'_," or "'py," etc.
            if let Some(comma_pos) = inner.find(',') {
                let type_part = inner[comma_pos + 1..].trim_start_matches(|c: char| c.is_whitespace());
                rust_type_to_python_normalized(type_part)
            } else {
                rust_type_to_python_normalized(inner)
            }
        }
        _ if s.starts_with("Result<") && s.ends_with(">") => {
            // Result<T, E> -> T (assuming success type is what matters for Python)
            let inner = &s[7..s.len() - 1];
            if let Some((ok_type, _err_type)) = split_generic_pair(inner) {
                rust_type_to_python_normalized(ok_type)
            } else {
                rust_type_to_python_normalized(inner)
            }
        }

        // Reference stripping
        _ if s.starts_with("&mut") => {
            rust_type_to_python_normalized(s[4..].trim_start())
        }
        _ if s.starts_with("&") => {
            rust_type_to_python_normalized(&s[1..])
        }

        // Python<'_> is the GIL token, not a real type - return empty or skip
        _ if s.starts_with("Python<") => "".to_string(),

        // Path-qualified types - try stripping the path
        _ if s.contains("::") => {
            let last_segment = s.rsplit("::").next().unwrap_or(s);
            // Recurse to handle the base type
            let converted = rust_type_to_python_normalized(last_segment);
            // If it converted to something different, use that; otherwise keep original
            if converted != last_segment {
                converted
            } else {
                last_segment.to_string()
            }
        }

        // Default: return as-is
        other => other.to_string(),
    }
}
```

</details>



### `plissken-core::crossref::split_generic_pair`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn split_generic_pair (s : & str) -> Option < (& str , & str) >
```

Split a generic pair like "String,PyObject" into ("String", "PyObject") Handles nested generics by counting angle brackets

<details>
<summary>Source</summary>

```rust
fn split_generic_pair(s: &str) -> Option<(&str, &str)> {
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        match c {
            '<' | '(' => depth += 1,
            '>' | ')' => depth -= 1,
            ',' if depth == 0 => {
                return Some((&s[..i], s[i + 1..].trim_start()));
            }
            _ => {}
        }
    }
    None
}
```

</details>



### `plissken-core::crossref::split_tuple_elements`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn split_tuple_elements (s : & str) -> Vec < & str >
```

Split tuple elements like "i32,String,String" into ["i32", "String", "String"] Handles nested generics and tuples by counting brackets

<details>
<summary>Source</summary>

```rust
fn split_tuple_elements(s: &str) -> Vec<&str> {
    let mut elements = Vec::new();
    let mut depth = 0;
    let mut start = 0;

    for (i, c) in s.char_indices() {
        match c {
            '<' | '(' | '[' => depth += 1,
            '>' | ')' | ']' => depth -= 1,
            ',' if depth == 0 => {
                elements.push(s[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }

    // Don't forget the last element
    let last = s[start..].trim();
    if !last.is_empty() {
        elements.push(last);
    }

    elements
}
```

</details>



