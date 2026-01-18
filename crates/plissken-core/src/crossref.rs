//! Cross-reference builder for linking Python and Rust items
//!
//! This module builds cross-references between Python items and their Rust
//! implementations by matching PyO3 metadata.

use crate::config::{Config, ModuleSourceType};
use crate::model::*;
use std::collections::HashMap;

/// Builds cross-references between Python and Rust items
pub struct CrossRefBuilder<'a> {
    config: &'a Config,
    /// Map of Python name → Rust item (struct name, module path)
    pyclass_map: HashMap<String, (String, String)>,
    /// Map of Python name → Rust function (fn name, module path)
    pyfunction_map: HashMap<String, (String, String)>,
    /// Map of (Rust struct name, method name) → method info
    pymethod_map: HashMap<(String, String), String>,
}

impl<'a> CrossRefBuilder<'a> {
    /// Create a new cross-reference builder
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            pyclass_map: HashMap::new(),
            pyfunction_map: HashMap::new(),
            pymethod_map: HashMap::new(),
        }
    }

    /// Build cross-references and return updated modules and refs
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

    /// Index all PyO3 items from Rust modules
    fn index_rust_modules(&mut self, modules: &[RustModule]) {
        for module in modules {
            for item in &module.items {
                match item {
                    RustItem::Struct(s) => {
                        if let Some(ref pyclass) = s.pyclass {
                            // Get the Python-visible name
                            let py_name = pyclass.name.clone().unwrap_or_else(|| s.name.clone());
                            self.pyclass_map
                                .insert(py_name, (s.name.clone(), module.path.clone()));
                        }
                    }
                    RustItem::Enum(e) => {
                        // PyO3 enums can also be exposed - check for pyclass-like attribute
                        // For now, we'd need to add pyclass support to enums in the parser
                        let _ = e; // TODO: Support pyclass on enums
                    }
                    RustItem::Function(f) => {
                        if let Some(ref pyfunc) = f.pyfunction {
                            let py_name = pyfunc.name.clone().unwrap_or_else(|| f.name.clone());
                            self.pyfunction_map
                                .insert(py_name, (f.name.clone(), module.path.clone()));
                        }
                    }
                    RustItem::Impl(impl_block) => {
                        if impl_block.pymethods {
                            // Index methods from #[pymethods] blocks
                            for method in &impl_block.methods {
                                let py_name = method
                                    .pyfunction
                                    .as_ref()
                                    .and_then(|pf| pf.name.clone())
                                    .unwrap_or_else(|| method.name.clone());

                                self.pymethod_map.insert(
                                    (impl_block.target.clone(), py_name),
                                    method.name.clone(),
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// Check if a Python module is a PyO3 binding
    fn is_pyo3_module(&self, module_path: &str) -> bool {
        if let Some(ref python_config) = self.config.python
            && let Some(source_type) = python_config.modules.get(module_path)
        {
            return matches!(source_type, ModuleSourceType::Pyo3);
        }
        false
    }

    /// Process a Python module, updating items with Rust references
    fn process_python_module(
        &self,
        mut module: PythonModule,
        cross_refs: &mut Vec<CrossRef>,
    ) -> PythonModule {
        // Check if this module is marked as PyO3 bindings
        if !self.is_pyo3_module(&module.path) {
            return module;
        }

        // Update source type
        module.source_type = SourceType::PyO3Binding;

        // Process each item
        let items = std::mem::take(&mut module.items);
        module.items = items
            .into_iter()
            .map(|item| self.process_python_item(item, &module.path, cross_refs))
            .collect();

        module
    }

    /// Process a single Python item
    fn process_python_item(
        &self,
        item: PythonItem,
        module_path: &str,
        cross_refs: &mut Vec<CrossRef>,
    ) -> PythonItem {
        match item {
            PythonItem::Class(mut class) => {
                if let Some((rust_name, rust_module)) = self.pyclass_map.get(&class.name) {
                    // Found matching Rust class
                    let rust_path = format!("{}::{}", rust_module, rust_name);
                    class.rust_impl = Some(RustItemRef::new(&rust_path, rust_name));

                    cross_refs.push(CrossRef::binding(
                        format!("{}.{}", module_path, class.name),
                        &rust_path,
                    ));

                    // Also process methods
                    class.methods = class
                        .methods
                        .into_iter()
                        .map(|method| {
                            self.process_python_method(
                                method,
                                rust_name,
                                rust_module,
                                module_path,
                                &class.name,
                                cross_refs,
                            )
                        })
                        .collect();
                }
                PythonItem::Class(class)
            }
            PythonItem::Function(mut func) => {
                if let Some((rust_name, rust_module)) = self.pyfunction_map.get(&func.name) {
                    let rust_path = format!("{}::{}", rust_module, rust_name);
                    func.rust_impl = Some(RustItemRef::new(&rust_path, rust_name));

                    cross_refs.push(CrossRef::binding(
                        format!("{}.{}", module_path, func.name),
                        &rust_path,
                    ));
                }
                PythonItem::Function(func)
            }
            PythonItem::Variable(var) => {
                // Variables typically don't have Rust implementations
                PythonItem::Variable(var)
            }
        }
    }

    /// Process a Python method, linking it to Rust
    fn process_python_method(
        &self,
        mut method: PythonFunction,
        rust_struct_name: &str,
        rust_module: &str,
        python_module: &str,
        python_class: &str,
        cross_refs: &mut Vec<CrossRef>,
    ) -> PythonFunction {
        // Look up method in pymethod_map
        let key = (rust_struct_name.to_string(), method.name.clone());
        if self.pymethod_map.contains_key(&key) {
            let rust_path = format!("{}::{}::{}", rust_module, rust_struct_name, method.name);
            method.rust_impl = Some(RustItemRef::new(&rust_path, &method.name));

            cross_refs.push(CrossRef::binding(
                format!("{}.{}.{}", python_module, python_class, method.name),
                &rust_path,
            ));
        }
        method
    }
}

/// Build cross-references for a doc model
pub fn build_cross_refs(
    config: &Config,
    rust_modules: &[RustModule],
    python_modules: Vec<PythonModule>,
) -> (Vec<PythonModule>, Vec<CrossRef>) {
    let builder = CrossRefBuilder::new(config);
    builder.build(rust_modules, python_modules)
}

/// Synthesize Python modules from Rust PyO3 bindings
///
/// This creates Python module representations from Rust code that uses PyO3.
/// It extracts #[pymodule], #[pyclass], #[pyfunction], and #[pymethods] to
/// build a Python-side view of the API.
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

/// Synthesize a Python function from a Rust function
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

/// Synthesize a Python method from a Rust method in pymethods block
fn synthesize_python_method(
    rust_fn: &RustFunction,
    rust_struct: &str,
    rust_module: &str,
) -> PythonFunction {
    let rust_path = format!("{}::{}::{}", rust_module, rust_struct, rust_fn.name);
    synthesize_python_function(rust_fn, &rust_path)
}

/// Synthesize Python modules from Rust modules with PyO3 bindings.
///
/// Unlike `synthesize_python_from_rust` which flattens into one module,
/// this preserves the module structure - each Rust module with bindings
/// becomes a separate Python module under the configured Python package namespace.
///
/// # Arguments
/// * `rust_modules` - The Rust modules to synthesize from
/// * `python_package` - The Python package name (e.g., "pysnake")
/// * `rust_entry_point` - The Rust crate entry point (e.g., "rustscale")
///
/// Returns a list of (PythonModule, Vec<CrossRef>) pairs.
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

/// Convert a Rust type to Python type hint (best effort)
///
/// Handles PyO3 types, generic wrappers, and normalizes whitespace in type strings
/// (e.g., `PyResult < String >` -> unwrapped to Python equivalent).
fn rust_type_to_python(rust_type: &str) -> String {
    // Normalize whitespace: "PyResult < String >" -> "PyResult<String>"
    let normalized: String = rust_type
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    rust_type_to_python_normalized(&normalized)
}

/// Internal conversion after whitespace normalization
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

/// Split a generic pair like "String,PyObject" into ("String", "PyObject")
/// Handles nested generics by counting angle brackets
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

/// Split tuple elements like "i32,String,String" into ["i32", "String", "String"]
/// Handles nested generics and tuples by counting brackets
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::*;

    fn test_config() -> Config {
        Config {
            project: ProjectConfig {
                name: "test".to_string(),
                version_from: VersionSource::Git,
            },
            output: OutputConfig {
                format: "markdown".to_string(),
                path: "docs".into(),
                template: None,
            },
            rust: Some(RustConfig {
                crates: vec![".".into()],
                entry_point: Some("test".to_string()),
            }),
            python: Some(PythonConfig {
                package: "test".to_string(),
                source: Some("python".into()),
                auto_discover: false,
                modules: {
                    let mut m = HashMap::new();
                    m.insert("test".to_string(), ModuleSourceType::Pyo3);
                    m.insert("test.helpers".to_string(), ModuleSourceType::Python);
                    m
                },
            }),
            links: LinksConfig::default(),
            quality: QualityConfig::default(),
        }
    }

    #[test]
    fn test_pyclass_matching() {
        let config = test_config();

        // Rust module with PyTask -> Task mapping
        let rust_module = RustModule::test("test").with_item(RustItem::Struct(
            RustStruct::test("PyTask").with_pyclass(PyClassMeta::new().with_name("Task")),
        ));

        // Python module with Task class
        let python_module =
            PythonModule::test("test").with_item(PythonItem::Class(PythonClass::test("Task")));

        let (updated_modules, cross_refs) =
            build_cross_refs(&config, &[rust_module], vec![python_module]);

        // Check that rust_impl was populated
        let module = &updated_modules[0];
        assert_eq!(module.source_type, SourceType::PyO3Binding);

        if let PythonItem::Class(class) = &module.items[0] {
            assert!(class.rust_impl.is_some());
            let rust_ref = class.rust_impl.as_ref().unwrap();
            assert_eq!(rust_ref.name, "PyTask");
            assert_eq!(rust_ref.path, "test::PyTask");
        } else {
            panic!("Expected class");
        }

        // Check cross-refs
        assert_eq!(cross_refs.len(), 1);
        assert_eq!(cross_refs[0].python_path, "test.Task");
        assert_eq!(cross_refs[0].rust_path, "test::PyTask");
    }

    #[test]
    fn test_pyfunction_matching() {
        let config = test_config();

        let rust_module = RustModule::test("test").with_item(RustItem::Function(
            RustFunction::test("py_process")
                .with_pyfunction(PyFunctionMeta::new().with_name("process")),
        ));

        let python_module = PythonModule::test("test")
            .with_item(PythonItem::Function(PythonFunction::test("process")));

        let (updated_modules, cross_refs) =
            build_cross_refs(&config, &[rust_module], vec![python_module]);

        if let PythonItem::Function(func) = &updated_modules[0].items[0] {
            assert!(func.rust_impl.is_some());
            assert_eq!(func.rust_impl.as_ref().unwrap().name, "py_process");
        } else {
            panic!("Expected function");
        }

        assert_eq!(cross_refs.len(), 1);
        assert_eq!(cross_refs[0].python_path, "test.process");
    }

    #[test]
    fn test_pymethods_matching() {
        let config = test_config();

        // Rust module with pymethods impl block
        let rust_module = RustModule::test("test")
            .with_item(RustItem::Struct(
                RustStruct::test("PyRunner").with_pyclass(PyClassMeta::new().with_name("Runner")),
            ))
            .with_item(RustItem::Impl(RustImpl {
                generics: None,
                target: "PyRunner".to_string(),
                trait_: None,
                where_clause: None,
                methods: vec![RustFunction::test("new"), RustFunction::test("run")],
                pymethods: true,
                source: SourceSpan::test("test.rs", 1, 10),
            }));

        let python_module = PythonModule::test("test").with_item(PythonItem::Class(
            PythonClass::test("Runner")
                .with_method(PythonFunction::test("new"))
                .with_method(PythonFunction::test("run")),
        ));

        let (updated_modules, cross_refs) =
            build_cross_refs(&config, &[rust_module], vec![python_module]);

        if let PythonItem::Class(class) = &updated_modules[0].items[0] {
            // Class should have rust_impl
            assert!(class.rust_impl.is_some());

            // Methods should have rust_impl
            assert_eq!(class.methods.len(), 2);
            assert!(class.methods[0].rust_impl.is_some());
            assert!(class.methods[1].rust_impl.is_some());
        } else {
            panic!("Expected class");
        }

        // Should have 3 cross-refs: class + 2 methods
        assert_eq!(cross_refs.len(), 3);
    }

    #[test]
    fn test_pure_python_unchanged() {
        let config = test_config();

        // Python-only module should not be modified
        let python_module = PythonModule::test("test.helpers")
            .with_item(PythonItem::Class(PythonClass::test("Helper")));

        let (updated_modules, cross_refs) = build_cross_refs(&config, &[], vec![python_module]);

        let module = &updated_modules[0];
        assert_eq!(module.source_type, SourceType::Python);

        if let PythonItem::Class(class) = &module.items[0] {
            assert!(class.rust_impl.is_none());
        }

        assert!(cross_refs.is_empty());
    }

    #[test]
    fn test_name_fallback_to_rust_name() {
        let config = test_config();

        // When pyclass has no explicit name, use Rust struct name
        let rust_module = RustModule::test("test").with_item(RustItem::Struct(
            RustStruct::test("DirectClass").with_pyclass(PyClassMeta::new()),
        ));

        let python_module = PythonModule::test("test")
            .with_item(PythonItem::Class(PythonClass::test("DirectClass")));

        let (updated_modules, cross_refs) =
            build_cross_refs(&config, &[rust_module], vec![python_module]);

        if let PythonItem::Class(class) = &updated_modules[0].items[0] {
            assert!(class.rust_impl.is_some());
        }
        assert_eq!(cross_refs.len(), 1);
    }

    #[test]
    fn test_synthesize_python_from_rust() {
        // Create Rust module with PyO3 bindings
        let rust_module = RustModule::test("mylib")
            .with_doc("My library with Python bindings")
            .with_item(RustItem::Struct(
                RustStruct::test("PyTask")
                    .with_doc("A task")
                    .with_pyclass(PyClassMeta::new().with_name("Task")),
            ))
            .with_item(RustItem::Impl(RustImpl {
                generics: None,
                target: "PyTask".to_string(),
                trait_: None,
                where_clause: None,
                methods: vec![
                    RustFunction::test("new")
                        .with_doc("Create a new task")
                        .with_param(RustParam::test("name", "&str")),
                    RustFunction::test("run")
                        .with_doc("Run the task")
                        .with_return_type("bool"),
                ],
                pymethods: true,
                source: SourceSpan::test("lib.rs", 10, 30),
            }))
            .with_item(RustItem::Function(
                RustFunction::test("create_task")
                    .with_doc("Create a task from config")
                    .with_pyfunction(PyFunctionMeta::new().with_name("create")),
            ));

        let (python_module, cross_refs) = synthesize_python_from_rust(&[rust_module], "mylib");

        // Check module
        assert_eq!(python_module.path, "mylib");
        assert_eq!(python_module.source_type, SourceType::PyO3Binding);
        assert!(python_module.docstring.is_some());

        // Should have 2 items: Task class and create function
        assert_eq!(python_module.items.len(), 2);

        // Check the class
        let task_class = python_module.items.iter().find_map(|i| {
            if let PythonItem::Class(c) = i {
                if c.name == "Task" {
                    return Some(c);
                }
            }
            None
        });
        assert!(task_class.is_some());
        let task = task_class.unwrap();
        assert_eq!(task.docstring.as_deref(), Some("A task"));
        assert!(task.rust_impl.is_some());
        assert_eq!(task.methods.len(), 2);

        // Check the function
        let create_fn = python_module.items.iter().find_map(|i| {
            if let PythonItem::Function(f) = i {
                if f.name == "create" {
                    return Some(f);
                }
            }
            None
        });
        assert!(create_fn.is_some());

        // Check cross-refs
        assert!(cross_refs.len() >= 2);
        assert!(cross_refs.iter().any(|r| r.python_path == "mylib.Task"));
        assert!(cross_refs.iter().any(|r| r.python_path == "mylib.create"));
    }

    #[test]
    fn test_rust_type_to_python_primitives() {
        // Integer types
        assert_eq!(rust_type_to_python("i8"), "int");
        assert_eq!(rust_type_to_python("i16"), "int");
        assert_eq!(rust_type_to_python("i32"), "int");
        assert_eq!(rust_type_to_python("i64"), "int");
        assert_eq!(rust_type_to_python("u8"), "int");
        assert_eq!(rust_type_to_python("u32"), "int");
        assert_eq!(rust_type_to_python("usize"), "int");

        // Float types
        assert_eq!(rust_type_to_python("f32"), "float");
        assert_eq!(rust_type_to_python("f64"), "float");

        // Other primitives
        assert_eq!(rust_type_to_python("bool"), "bool");
        assert_eq!(rust_type_to_python("char"), "str");
        assert_eq!(rust_type_to_python("()"), "None");

        // String types
        assert_eq!(rust_type_to_python("String"), "str");
        assert_eq!(rust_type_to_python("&str"), "str");
        assert_eq!(rust_type_to_python("str"), "str");
    }

    #[test]
    fn test_rust_type_to_python_generics() {
        // Vec
        assert_eq!(rust_type_to_python("Vec<String>"), "List[str]");
        assert_eq!(rust_type_to_python("Vec<i32>"), "List[int]");

        // Option
        assert_eq!(rust_type_to_python("Option<i32>"), "Optional[int]");
        assert_eq!(rust_type_to_python("Option<String>"), "Optional[str]");

        // HashMap/BTreeMap
        assert_eq!(rust_type_to_python("HashMap<String, i32>"), "Dict[str, int]");
        assert_eq!(rust_type_to_python("BTreeMap<String, bool>"), "Dict[str, bool]");

        // HashSet/BTreeSet
        assert_eq!(rust_type_to_python("HashSet<String>"), "Set[str]");
        assert_eq!(rust_type_to_python("BTreeSet<i32>"), "Set[int]");

        // Result
        assert_eq!(rust_type_to_python("Result<String, Error>"), "str");
    }

    #[test]
    fn test_rust_type_to_python_pyo3_types() {
        // PyO3 primitive types
        assert_eq!(rust_type_to_python("PyString"), "str");
        assert_eq!(rust_type_to_python("PyList"), "list");
        assert_eq!(rust_type_to_python("PyDict"), "dict");
        assert_eq!(rust_type_to_python("PyTuple"), "tuple");
        assert_eq!(rust_type_to_python("PySet"), "set");
        assert_eq!(rust_type_to_python("PyBytes"), "bytes");
        assert_eq!(rust_type_to_python("PyBool"), "bool");
        assert_eq!(rust_type_to_python("PyInt"), "int");
        assert_eq!(rust_type_to_python("PyFloat"), "float");
        assert_eq!(rust_type_to_python("PyNone"), "None");

        // PyObject/PyAny
        assert_eq!(rust_type_to_python("PyObject"), "Any");
        assert_eq!(rust_type_to_python("PyAny"), "Any");

        // Path-qualified PyO3 types
        assert_eq!(rust_type_to_python("pyo3::types::PyString"), "str");
        assert_eq!(rust_type_to_python("pyo3::types::PyDict"), "dict");
    }

    #[test]
    fn test_rust_type_to_python_pyo3_wrappers() {
        // PyResult
        assert_eq!(rust_type_to_python("PyResult<bool>"), "bool");
        assert_eq!(rust_type_to_python("PyResult<String>"), "str");
        assert_eq!(rust_type_to_python("PyResult<()>"), "None");
        assert_eq!(rust_type_to_python("PyResult<Vec<String>>"), "List[str]");

        // Py<T>
        assert_eq!(rust_type_to_python("Py<PyDict>"), "dict");
        assert_eq!(rust_type_to_python("Py<PyAny>"), "Any");

        // Bound<'_, T>
        assert_eq!(rust_type_to_python("Bound<'_, PyDict>"), "dict");
        assert_eq!(rust_type_to_python("Bound<'py, PyList>"), "list");
        assert_eq!(rust_type_to_python("Bound<'_, PyModule>"), "ModuleType");
    }

    #[test]
    fn test_rust_type_to_python_spaced_types() {
        // Types with spaces (as they come from Rust parser)
        assert_eq!(rust_type_to_python("PyResult < bool >"), "bool");
        assert_eq!(rust_type_to_python("PyResult < () >"), "None");
        assert_eq!(rust_type_to_python("Option < usize >"), "Optional[int]");
        assert_eq!(rust_type_to_python("Vec < String >"), "List[str]");
        assert_eq!(rust_type_to_python("HashMap < String , PyObject >"), "Dict[str, Any]");
        assert_eq!(rust_type_to_python("Bound < '_ , PyDict >"), "dict");
    }

    #[test]
    fn test_rust_type_to_python_references() {
        // References should be stripped
        assert_eq!(rust_type_to_python("&str"), "str");
        assert_eq!(rust_type_to_python("&String"), "str");
        assert_eq!(rust_type_to_python("&mut String"), "str");
        assert_eq!(rust_type_to_python("&PyDict"), "dict");
    }

    #[test]
    fn test_rust_type_to_python_tuples() {
        // Tuple types
        assert_eq!(rust_type_to_python("(i32, String, String)"), "Tuple[int, str, str]");
        assert_eq!(rust_type_to_python("(bool, i32)"), "Tuple[bool, int]");
        assert_eq!(rust_type_to_python("(String,)"), "Tuple[str]");
        // Tuple in PyResult
        assert_eq!(rust_type_to_python("PyResult<(i32, String, String)>"), "Tuple[int, str, str]");
        // Spaced tuple
        assert_eq!(rust_type_to_python("PyResult < (i32 , String , String) >"), "Tuple[int, str, str]");
    }

    #[test]
    fn test_rust_type_to_python_nested_references() {
        // Nested references in generics (the reported bug)
        assert_eq!(rust_type_to_python("Option<&str>"), "Optional[str]");
        assert_eq!(rust_type_to_python("Option < & str >"), "Optional[str]");
        assert_eq!(rust_type_to_python("Option<&Bound<'_, PyDict>>"), "Optional[dict]");
        assert_eq!(rust_type_to_python("Option < & Bound < '_ , pyo3 :: types :: PyDict > >"), "Optional[dict]");
    }

    #[test]
    fn test_rust_type_to_python_slices() {
        // Slice types
        assert_eq!(rust_type_to_python("&[u8]"), "bytes");
        assert_eq!(rust_type_to_python("&[String]"), "List[str]");
        assert_eq!(rust_type_to_python("&[i32]"), "List[int]");
    }

    #[test]
    fn test_rust_type_to_python_complex_nested() {
        // Complex nested types
        assert_eq!(rust_type_to_python("Vec<Vec<String>>"), "List[List[str]]");
        assert_eq!(rust_type_to_python("HashMap<String, Vec<i32>>"), "Dict[str, List[int]]");
        assert_eq!(rust_type_to_python("Option<Vec<String>>"), "Optional[List[str]]");
        assert_eq!(rust_type_to_python("PyResult<Vec<HashMap<String, PyObject>>>"), "List[Dict[str, Any]]");
        assert_eq!(rust_type_to_python("Vec<(String, PyObject)>"), "List[Tuple[str, Any]]");
    }

    #[test]
    fn test_synthesize_hybrid_binary_fixture() {
        use crate::parser::RustParser;
        use crate::test_fixtures::hybrid_binary;

        let fixture_path = hybrid_binary::rust_lib();

        let parser = RustParser::new();
        let rust_module = parser.parse_file(&fixture_path).unwrap();

        let (python_module, cross_refs) =
            synthesize_python_from_rust(&[rust_module], "hybrid_binary");

        // Should have PyO3 binding type
        assert_eq!(python_module.source_type, SourceType::PyO3Binding);

        // Should synthesize classes for PyTask, PyRunner, PyRunResult
        let class_names: Vec<_> = python_module
            .items
            .iter()
            .filter_map(|i| {
                if let PythonItem::Class(c) = i {
                    Some(c.name.clone())
                } else {
                    None
                }
            })
            .collect();

        assert!(
            class_names.contains(&"Task".to_string()),
            "Should have Task class"
        );
        assert!(
            class_names.contains(&"Runner".to_string()),
            "Should have Runner class"
        );
        assert!(
            class_names.contains(&"RunResult".to_string()),
            "Should have RunResult class"
        );

        // Check cross-refs exist
        assert!(!cross_refs.is_empty());
        assert!(
            cross_refs
                .iter()
                .any(|r| r.python_path == "hybrid_binary.Task")
        );
        // The rust path includes the file path as module, so check contains
        assert!(
            cross_refs.iter().any(|r| r.rust_path.contains("PyTask")),
            "Expected rust_path containing PyTask"
        );
    }

    #[test]
    fn test_synthesize_separate_bindings_fixture() {
        use crate::parser::RustParser;
        use crate::test_fixtures::separate_bindings;

        let fixture_path = separate_bindings::bindings_lib();

        let parser = RustParser::new();
        let rust_module = parser.parse_file(&fixture_path).unwrap();

        let (python_module, cross_refs) =
            synthesize_python_from_rust(&[rust_module], "separate_bindings");

        // Should synthesize Pipeline, PipelineResult, DataBatch
        let class_names: Vec<_> = python_module
            .items
            .iter()
            .filter_map(|i| {
                if let PythonItem::Class(c) = i {
                    Some(c.name.clone())
                } else {
                    None
                }
            })
            .collect();

        assert!(
            class_names.contains(&"Pipeline".to_string()),
            "Should have Pipeline class"
        );
        assert!(
            class_names.contains(&"DataBatch".to_string()),
            "Should have DataBatch class"
        );
        assert!(
            class_names.contains(&"PipelineResult".to_string()),
            "Should have PipelineResult class"
        );

        // Verify cross-refs
        assert!(
            cross_refs
                .iter()
                .any(|r| r.python_path == "separate_bindings.Pipeline")
        );
        assert!(
            cross_refs
                .iter()
                .any(|r| r.rust_path.contains("PyPipeline"))
        );
    }
}
