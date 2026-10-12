//! Cross-reference builder for linking Python and Rust items
//!
//! This module builds cross-references between Python items and their Rust
//! implementations by matching PyO3 metadata.

use crate::config::{Config, ModuleSourceType};
use crate::model::*;
use std::collections::{HashMap, HashSet};

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
                    RustItem::Impl(impl_block) if impl_block.pymethods => {
                        // Index methods from #[pymethods] blocks
                        for method in &impl_block.methods {
                            let py_name = method
                                .pyfunction
                                .as_ref()
                                .and_then(|pf| pf.name.clone())
                                .unwrap_or_else(|| method.name.clone());

                            self.pymethod_map
                                .insert((impl_block.target.clone(), py_name), method.name.clone());
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
/// As [`synthesize_python_api`], with the given crate as the entry point.
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
    synthesize_python_api(rust_modules, python_package, Some(rust_entry_point))
}

/// Synthesize the Python API of the PyO3 bindings of Rust modules.
///
/// A `#[pymodule]` function names a Python module: `fn cloaca(m)` is the
/// module `cloaca`, or `{python_package}.{name}` when the package has
/// another name. Its classes and functions are those that it adds, and
/// those that its helpers add (`register(m)`); a submodule that it adds
/// (`m.add_submodule(&sub)`) is a module below it.
///
/// A binding that no `#[pymodule]` adds goes to the Python module of its
/// Rust module, the crate name replaced by the package
/// (`rustscale::handlers` -> `pysnake.handlers`). With an entry point,
/// only that crate is replaced; without one, every crate.
///
/// Returns a list of (PythonModule, Vec<CrossRef>) pairs.
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

/// The Rust path of an item of a module.
fn rust_item_path(module: &RustModule, name: &str) -> String {
    format!("{}::{}", module.path, name)
}

/// The Python module of a Rust module: the crate name replaced by the
/// package (only the entry point crate, if given).
fn python_module_path(rust_path: &str, python_package: &str, entry_point: Option<&str>) -> String {
    let dotted = rust_path.replace("::", ".");
    let (krate, rest) = match dotted.split_once('.') {
        Some((k, r)) => (k, Some(r)),
        None => (dotted.as_str(), None),
    };
    let replace = match entry_point {
        Some(entry) => krate == entry || krate.replace('-', "_") == entry.replace('-', "_"),
        None => true,
    };
    if !replace {
        // Not the entry point crate: use as-is
        return dotted;
    }
    match rest {
        Some(rest) => format!("{}.{}", python_package, rest),
        None => python_package.to_string(),
    }
}

/// The Python module of a `#[pymodule]` named `name`.
fn pymodule_path(python_package: &str, name: &str) -> String {
    let package_name = python_package.rsplit('.').next().unwrap_or(python_package);
    if python_package.is_empty() || package_name == name {
        if python_package.is_empty() {
            name.to_string()
        } else {
            python_package.to_string()
        }
    } else {
        format!("{}.{}", python_package, name)
    }
}

/// The classes and functions of a synthesized Python module.
#[derive(Default)]
struct SynthModule<'a> {
    doc_comment: Option<String>,
    classes: Vec<(&'a RustModule, &'a RustStruct)>,
    functions: Vec<(&'a RustModule, &'a RustFunction)>,
}

impl SynthModule<'_> {
    /// The Python module, and the cross-references of its items.
    fn build(
        &self,
        py_module_path: &str,
        rust_modules: &[RustModule],
        source: &SourceSpan,
    ) -> (PythonModule, Vec<CrossRef>) {
        let mut items = Vec::new();
        let mut cross_refs = Vec::new();
        for (module, s) in &self.classes {
            let pyclass = s.pyclass.as_ref();
            let py_name = pyclass
                .and_then(|p| p.name.clone())
                .unwrap_or_else(|| s.name.clone());
            let rust_path = rust_item_path(module, &s.name);

            // Methods of the #[pymethods] blocks of the struct
            let methods: Vec<PythonFunction> = rust_modules
                .iter()
                .filter(|m| m.path == module.path)
                .flat_map(|m| m.items.iter())
                .filter_map(|item| match item {
                    RustItem::Impl(i)
                        if i.pymethods
                            && i.target.split('<').next().unwrap_or(&i.target).trim() == s.name =>
                    {
                        Some(i)
                    }
                    _ => None,
                })
                .flat_map(|i| {
                    i.methods
                        .iter()
                        .map(|m| synthesize_python_method(m, &i.target, &module.path))
                })
                .collect();

            items.push(PythonItem::Class(PythonClass {
                name: py_name.clone(),
                docstring: s.doc_comment.clone(),
                parsed_doc: None,
                bases: Vec::new(),
                methods,
                attributes: Vec::new(),
                decorators: Vec::new(),
                rust_impl: Some(RustItemRef::new(&rust_path, &s.name)),
                source: s.source.clone(),
            }));
            cross_refs.push(CrossRef::binding(
                format!("{}.{}", py_module_path, py_name),
                &rust_path,
            ));
        }
        for (module, f) in &self.functions {
            let py_name = f
                .pyfunction
                .as_ref()
                .and_then(|p| p.name.clone())
                .unwrap_or_else(|| f.name.clone());
            let rust_path = rust_item_path(module, &f.name);
            items.push(PythonItem::Function(synthesize_python_function(
                f, &rust_path,
            )));
            cross_refs.push(CrossRef::binding(
                format!("{}.{}", py_module_path, py_name),
                &rust_path,
            ));
        }
        (
            PythonModule {
                path: py_module_path.to_string(),
                docstring: self.doc_comment.clone(),
                parsed_doc: None,
                items,
                source_type: SourceType::PyO3Binding,
                source: source.clone(),
            },
            cross_refs,
        )
    }
}

/// The Python modules that the `#[pymodule]` functions build. Each item that
/// they place is added to `placed` (its Rust path).
fn pymodule_api(
    rust_modules: &[RustModule],
    python_package: &str,
    placed: &mut HashSet<String>,
) -> Vec<(PythonModule, Vec<CrossRef>)> {
    let crate_of = |path: &str| path.split("::").next().unwrap_or(path).to_string();

    // The registrars of each crate, by function name
    let mut registrars: HashMap<(String, String), (&RustModule, &PyModuleRegistrar)> =
        HashMap::new();
    for module in rust_modules {
        for r in &module.py_registrars {
            registrars
                .entry((crate_of(&module.path), r.function.clone()))
                .or_insert((module, r));
        }
    }

    // A #[pymodule] that another one wraps is its submodule, not a root
    let nested: HashSet<(String, String)> = rust_modules
        .iter()
        .flat_map(|m| {
            m.py_registrars.iter().flat_map(move |r| {
                r.adds
                    .iter()
                    .filter(|a| matches!(a.kind, PyModuleAddKind::Pymodule | PyModuleAddKind::Call))
                    .map(move |a| (crate_of(&m.path), a.path.clone()))
            })
        })
        .collect();

    let mut modules: Vec<(String, SynthModule, &SourceSpan)> = Vec::new();
    for module in rust_modules {
        for r in module.py_registrars.iter() {
            let Some(name) = &r.pymodule else {
                continue;
            };
            if nested.contains(&(crate_of(&module.path), r.function.clone())) {
                continue;
            }
            let root = pymodule_path(python_package, name);
            let mut expansion = Expansion {
                rust_modules,
                registrars: &registrars,
                krate: crate_of(&module.path),
                modules: Vec::new(),
                visited: HashSet::new(),
                placed,
            };
            expansion.module(&root, r.doc_comment.clone());
            expansion.expand(r, &root, 0);
            for (path, synth) in expansion.modules {
                modules.push((path, synth, &module.source));
            }
        }
    }

    modules
        .into_iter()
        .filter(|(_, synth, _)| {
            !synth.classes.is_empty() || !synth.functions.is_empty() || synth.doc_comment.is_some()
        })
        .map(|(path, synth, source)| synth.build(&path, rust_modules, source))
        .collect()
}

/// The walk from a `#[pymodule]` function through its helpers.
struct Expansion<'a, 'p> {
    rust_modules: &'a [RustModule],
    registrars: &'a HashMap<(String, String), (&'a RustModule, &'a PyModuleRegistrar)>,
    krate: String,
    modules: Vec<(String, SynthModule<'a>)>,
    visited: HashSet<(String, String)>,
    placed: &'p mut HashSet<String>,
}

impl<'a> Expansion<'a, '_> {
    /// The synthesized module of a Python path.
    fn module(&mut self, path: &str, doc: Option<String>) -> &mut SynthModule<'a> {
        let pos = match self.modules.iter().position(|(p, _)| p == path) {
            Some(pos) => pos,
            None => {
                self.modules
                    .push((path.to_string(), SynthModule::default()));
                self.modules.len() - 1
            }
        };
        let synth = &mut self.modules[pos].1;
        if synth.doc_comment.is_none() {
            synth.doc_comment = doc;
        }
        synth
    }

    /// Add what a registrar adds to the module `base`.
    fn expand(&mut self, r: &'a PyModuleRegistrar, base: &str, depth: usize) {
        if depth > 16 || !self.visited.insert((r.function.clone(), base.to_string())) {
            return;
        }
        // Python paths of the submodules that the function makes
        let mut paths: HashMap<&str, String> = HashMap::new();
        paths.insert("", base.to_string());
        for add in r
            .adds
            .iter()
            .filter(|a| a.kind == PyModuleAddKind::Submodule)
        {
            let parent = paths
                .get(add.module.as_str())
                .cloned()
                .unwrap_or_else(|| base.to_string());
            paths.insert(&add.path, format!("{}.{}", parent, add.path));
        }
        for add in &r.adds {
            let target = paths
                .get(add.module.as_str())
                .cloned()
                .unwrap_or_else(|| format!("{}.{}", base, add.module));
            match add.kind {
                PyModuleAddKind::Class => {
                    if let Some((m, s)) = self.find_class(&add.path)
                        && self.placed.insert(rust_item_path(m, &s.name))
                    {
                        self.module(&target, None).classes.push((m, s));
                    }
                }
                PyModuleAddKind::Function => {
                    if let Some((m, f)) = self.find_function(&add.path)
                        && self.placed.insert(rust_item_path(m, &f.name))
                    {
                        self.module(&target, None).functions.push((m, f));
                    }
                }
                PyModuleAddKind::Submodule => {
                    let path = paths.get(add.path.as_str()).cloned().unwrap_or(target);
                    self.module(&path, None);
                }
                PyModuleAddKind::Call => {
                    if let Some((_, callee)) =
                        self.registrars.get(&(self.krate.clone(), add.path.clone()))
                    {
                        self.expand(callee, &target, depth + 1);
                    }
                }
                PyModuleAddKind::Pymodule => {
                    if let Some((_, child)) =
                        self.registrars.get(&(self.krate.clone(), add.path.clone()))
                    {
                        let name = child.pymodule.clone().unwrap_or_else(|| add.path.clone());
                        let path = format!("{}.{}", target, name);
                        self.module(&path, child.doc_comment.clone());
                        self.expand(child, &path, depth + 1);
                    }
                }
            }
        }
    }

    /// The items of the crate named by the last segment of `path`; with
    /// more than one, those whose Rust path ends with `path`.
    fn find<T>(
        &self,
        path: &str,
        pick: impl Fn(&'a RustItem) -> Option<(&'a T, &'a str)>,
    ) -> Option<(&'a RustModule, &'a T)> {
        let wanted = path
            .trim_start_matches("crate::")
            .trim_start_matches("self::")
            .trim_start_matches("super::");
        let name = wanted.rsplit("::").next().unwrap_or(wanted);
        let crate_prefix = format!("{}::", self.krate);
        let candidates: Vec<(&'a RustModule, &'a T)> = self
            .rust_modules
            .iter()
            .filter(|m| m.path == self.krate || m.path.starts_with(&crate_prefix))
            .flat_map(|m| {
                m.items
                    .iter()
                    .filter_map(&pick)
                    .filter(|(_, n)| *n == name)
                    .map(move |(t, _)| (m, t))
            })
            .collect();
        match candidates.as_slice() {
            [] => None,
            [one] => Some(*one),
            many => {
                // The path from the crate root, else the one that ends with it
                let exact = format!("{}::{}", self.krate, wanted);
                let suffix = format!("::{}", wanted);
                many.iter()
                    .find(|(m, _)| format!("{}::{}", m.path, name) == exact)
                    .or_else(|| {
                        many.iter()
                            .find(|(m, _)| format!("{}::{}", m.path, name).ends_with(&suffix))
                    })
                    .or(many.first())
                    .copied()
            }
        }
    }

    fn find_class(&self, path: &str) -> Option<(&'a RustModule, &'a RustStruct)> {
        self.find(path, |item| match item {
            RustItem::Struct(s) if s.pyclass.is_some() => Some((s, s.name.as_str())),
            _ => None,
        })
    }

    fn find_function(&self, path: &str) -> Option<(&'a RustModule, &'a RustFunction)> {
        self.find(path, |item| match item {
            RustItem::Function(f) if f.pyfunction.is_some() => Some((f, f.name.as_str())),
            _ => None,
        })
    }
}

/// Convert a Rust type to Python type hint (best effort)
fn rust_type_to_python(rust_type: &str) -> String {
    match rust_type {
        "i8" | "i16" | "i32" | "i64" | "isize" | "u8" | "u16" | "u32" | "u64" | "usize" => {
            "int".to_string()
        }
        "f32" | "f64" => "float".to_string(),
        "bool" => "bool".to_string(),
        "String" | "&str" | "&String" => "str".to_string(),
        "()" => "None".to_string(),
        "Self" => "Self".to_string(),
        s if s.starts_with("Vec<") => {
            let inner = &s[4..s.len() - 1];
            format!("List[{}]", rust_type_to_python(inner))
        }
        s if s.starts_with("Option<") => {
            let inner = &s[7..s.len() - 1];
            format!("Optional[{}]", rust_type_to_python(inner))
        }
        s if s.starts_with("HashMap<") || s.starts_with("BTreeMap<") => {
            // Simplified - would need proper parsing for complex types
            "Dict[str, Any]".to_string()
        }
        s if s.starts_with("PyResult<") => {
            let inner = &s[9..s.len() - 1];
            rust_type_to_python(inner)
        }
        s if s.starts_with("&") => rust_type_to_python(&s[1..]),
        s if s.starts_with("&mut ") => rust_type_to_python(&s[5..]),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::*;

    fn parse_rust(path: &str, code: &str) -> RustModule {
        let mut m = crate::parser::RustParser::new()
            .parse_str(code, std::path::Path::new("lib.rs"))
            .unwrap();
        m.path = path.to_string();
        m
    }

    #[test]
    fn test_python_api_from_pymodule() {
        // Item 23 of PLSKN-T-0057: cloacina's bindings were named after the
        // crate (`cloacina-python.*`), not the Python module `cloaca`
        let root = parse_rust(
            "cloacina-python",
            r#"
/// The cloaca module.
#[pymodule]
fn cloaca(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_authoring(m)?;
    m.add_class::<bindings::runner::PyDefaultRunner>()?;
    let events = PyModule::new(m.py(), "events")?;
    events.add_function(wrap_pyfunction!(trigger::emit, &events)?)?;
    m.add_submodule(&events)?;
    Ok(())
}
pub(crate) fn register_authoring(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<trigger::TriggerDecorator>()?;
    Ok(())
}
"#,
        );
        let runner = parse_rust(
            "cloacina-python::bindings::runner",
            "#[pyclass(name = \"DefaultRunner\")]\npub struct PyDefaultRunner;\n#[pymethods]\nimpl PyDefaultRunner { fn run(&self) {} }",
        );
        let trigger = parse_rust(
            "cloacina-python::trigger",
            "#[pyclass]\npub struct TriggerDecorator;\n#[pyfunction]\npub fn emit() {}",
        );
        // Same name, not registered: no Python page
        let other = parse_rust(
            "cloacina-python::bindings::trigger",
            "#[pyclass]\npub struct TriggerDecorator;",
        );
        let api = synthesize_python_api(&[root, runner, trigger, other], "cloaca", None);
        let paths: Vec<&str> = api.iter().map(|(m, _)| m.path.as_str()).collect();
        assert_eq!(paths, vec!["cloaca", "cloaca.events"]);

        let (cloaca, refs) = &api[0];
        assert_eq!(cloaca.docstring.as_deref(), Some("The cloaca module."));
        let names: Vec<String> = cloaca
            .items
            .iter()
            .map(|i| match i {
                PythonItem::Class(c) => format!("class {} {}", c.name, c.methods.len()),
                PythonItem::Function(f) => format!("fn {}", f.name),
                PythonItem::Variable(v) => v.name.clone(),
            })
            .collect();
        assert_eq!(
            names,
            vec!["class TriggerDecorator 0", "class DefaultRunner 1"]
        );
        assert!(
            refs.iter()
                .any(|x| x.python_path == "cloaca.TriggerDecorator"
                    && x.rust_path == "cloacina-python::trigger::TriggerDecorator")
        );
        assert_eq!(api[1].0.items.len(), 1);
    }

    #[test]
    fn test_python_api_without_pymodule() {
        // No #[pymodule]: each crate is the package (no entry point given)
        let m = parse_rust(
            "cloacina-python::bindings",
            "#[pyclass]\npub struct Context;",
        );
        let api = synthesize_python_api(std::slice::from_ref(&m), "cloaca", None);
        assert_eq!(api[0].0.path, "cloaca.bindings");
        // With an entry point, another crate keeps its name
        let api = synthesize_python_api(&[m], "cloaca", Some("cloacina"));
        assert_eq!(api[0].0.path, "cloacina-python.bindings");
    }

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
                include_private: false,
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
                include_private: false,
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
            if let PythonItem::Class(c) = i
                && c.name == "Task"
            {
                return Some(c);
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
            if let PythonItem::Function(f) = i
                && f.name == "create"
            {
                return Some(f);
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
    fn test_rust_type_to_python() {
        assert_eq!(rust_type_to_python("i32"), "int");
        assert_eq!(rust_type_to_python("f64"), "float");
        assert_eq!(rust_type_to_python("bool"), "bool");
        assert_eq!(rust_type_to_python("String"), "str");
        assert_eq!(rust_type_to_python("&str"), "str");
        assert_eq!(rust_type_to_python("Vec<String>"), "List[str]");
        assert_eq!(rust_type_to_python("Option<i32>"), "Optional[int]");
        assert_eq!(rust_type_to_python("PyResult<bool>"), "bool");
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
