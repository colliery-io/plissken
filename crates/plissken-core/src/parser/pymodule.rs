//! What PyO3 module functions register
//!
//! A `#[pymodule]` function names a Python module and adds its classes and
//! functions:
//!
//! ```text
//! #[pymodule]
//! fn cloaca(m: &Bound<'_, PyModule>) -> PyResult<()> {
//!     m.add_class::<bindings::runner::PyDefaultRunner>()?;
//!     m.add_function(wrap_pyfunction!(loader::py_var, m)?)?;
//!     register_authoring(m)?;          // a helper adds more
//!     let sub = PyModule::new(m.py(), "events")?;
//!     sub.add_class::<Event>()?;
//!     m.add_submodule(&sub)?;          // the submodule cloaca.events
//!     Ok(())
//! }
//! ```
//!
//! This module reads such a function, and any helper that gets a module as
//! a parameter, into a [`PyModuleRegistrar`]. The synthesis of the Python
//! API follows the calls from each `#[pymodule]` function.

use super::rust::{extract_doc_comments, type_str};
use crate::model::{PyModuleAdd, PyModuleAddKind, PyModuleRegistrar};
use quote::ToTokens;
use std::collections::HashMap;
use syn::visit::Visit;
use syn::{Attribute, Expr, FnArg, ItemFn, Meta, Pat};

/// The registrar of a function, if it is a `#[pymodule]` function or has a
/// `PyModule` parameter.
pub(crate) fn registrar(f: &ItemFn) -> Option<PyModuleRegistrar> {
    let pymodule = pymodule_name(&f.attrs, &f.sig.ident.to_string());

    // Module parameters: `m: &Bound<'_, PyModule>`, `parent: &PyModule`
    let mut vars: HashMap<String, String> = HashMap::new();
    for arg in &f.sig.inputs {
        if let FnArg::Typed(t) = arg
            && let Pat::Ident(ident) = &*t.pat
            && t.ty.to_token_stream().to_string().contains("PyModule")
        {
            vars.insert(ident.ident.to_string(), String::new());
        }
    }
    if vars.is_empty() {
        return None;
    }

    let mut visitor = Registrations {
        vars,
        wrapped: HashMap::new(),
        adds: Vec::new(),
    };
    visitor.visit_block(&f.block);
    if visitor.adds.is_empty() && pymodule.is_none() {
        return None;
    }
    Some(PyModuleRegistrar {
        function: f.sig.ident.to_string(),
        pymodule,
        doc_comment: extract_doc_comments(&f.attrs),
        adds: visitor.adds,
    })
}

/// The Python name of a `#[pymodule]` function: `#[pyo3(name = "x")]` or
/// `#[pymodule(name = "x")]`, else the function name.
fn pymodule_name(attrs: &[Attribute], fn_name: &str) -> Option<String> {
    let mut is_pymodule = false;
    let mut name = None;
    for attr in attrs {
        let last = attr.path().segments.last().map(|s| s.ident.to_string());
        match last.as_deref() {
            Some("pymodule") => {
                is_pymodule = true;
                if let Meta::List(list) = &attr.meta {
                    name = name.or_else(|| name_arg(&list.tokens.to_string()));
                }
            }
            Some("pyo3") => {
                if let Meta::List(list) = &attr.meta {
                    name = name.or_else(|| name_arg(&list.tokens.to_string()));
                }
            }
            _ => {}
        }
    }
    is_pymodule.then(|| name.unwrap_or_else(|| fn_name.to_string()))
}

/// The value of `name = "x"` in attribute tokens.
fn name_arg(tokens: &str) -> Option<String> {
    tokens.split(',').find_map(|part| {
        let (key, value) = part.split_once('=')?;
        (key.trim() == "name").then(|| value.trim().trim_matches('"').to_string())
    })
}

struct Registrations {
    /// Module variables: name -> Python name of the submodule (empty for a
    /// module parameter)
    vars: HashMap<String, String>,
    /// Variables that hold a wrapped `#[pymodule]`: name -> function name
    wrapped: HashMap<String, String>,
    adds: Vec<PyModuleAdd>,
}

/// The variable name of an expression such as `m`, `&m`, `(m)`,
/// `m.as_ref()`, `m.bind(py)`.
fn var_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Path(p) => p.path.get_ident().map(|i| i.to_string()),
        Expr::Reference(r) => var_name(&r.expr),
        Expr::Paren(p) => var_name(&p.expr),
        Expr::Try(t) => var_name(&t.expr),
        Expr::MethodCall(m) => var_name(&m.receiver),
        _ => None,
    }
}

/// The function name in `wrap_pymodule!(path)`, also when it is called:
/// `wrap_pymodule!(path)(py)`.
fn wrapped_pymodule(expr: &Expr) -> Option<String> {
    let expr = match unwrap_expr(expr) {
        Expr::Call(call) => &*call.func,
        Expr::MethodCall(m) => return wrapped_pymodule(&m.receiver),
        other => other,
    };
    let Expr::Macro(mac) = expr else {
        return None;
    };
    if mac.mac.path.segments.last()?.ident != "wrap_pymodule" {
        return None;
    }
    let tokens = mac.mac.tokens.to_string().replace(' ', "");
    tokens.rsplit("::").next().map(str::to_string)
}

/// The expression inside `?`, `.unwrap()`, `.expect(..)` and references.
fn unwrap_expr(expr: &Expr) -> &Expr {
    match expr {
        Expr::Try(t) => unwrap_expr(&t.expr),
        Expr::Reference(r) => unwrap_expr(&r.expr),
        Expr::Paren(p) => unwrap_expr(&p.expr),
        Expr::MethodCall(m) if matches!(m.method.to_string().as_str(), "unwrap" | "expect") => {
            unwrap_expr(&m.receiver)
        }
        _ => expr,
    }
}

/// The Python name of `PyModule::new(py, "name")` (or `new_bound`).
fn new_submodule_name(expr: &Expr) -> Option<String> {
    let Expr::Call(call) = unwrap_expr(expr) else {
        return None;
    };
    let Expr::Path(func) = &*call.func else {
        return None;
    };
    let segments: Vec<String> = func
        .path
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect();
    let [.., ty, method] = segments.as_slice() else {
        return None;
    };
    if ty != "PyModule" || !method.starts_with("new") {
        return None;
    }
    call.args.iter().find_map(|arg| match arg {
        Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(s),
            ..
        }) => Some(s.value()),
        _ => None,
    })
}

/// The function path in `wrap_pyfunction!(path, m)`.
fn wrapped_function(expr: &Expr) -> Option<String> {
    let Expr::Macro(mac) = unwrap_expr(expr) else {
        return None;
    };
    let name = mac.mac.path.segments.last()?.ident.to_string();
    if name != "wrap_pyfunction" {
        return None;
    }
    let tokens = mac.mac.tokens.to_string();
    let path = tokens.split(',').next()?.replace(' ', "");
    (!path.is_empty()).then_some(path)
}

impl<'ast> Visit<'ast> for Registrations {
    fn visit_local(&mut self, local: &'ast syn::Local) {
        if let Pat::Ident(ident) = &local.pat
            && let Some(init) = &local.init
        {
            if let Some(name) = new_submodule_name(&init.expr) {
                self.vars.insert(ident.ident.to_string(), name);
            } else if let Some(function) = wrapped_pymodule(&init.expr) {
                self.wrapped.insert(ident.ident.to_string(), function);
            }
        }
        syn::visit::visit_local(self, local);
    }

    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        if let Some(module) = var_name(&call.receiver).and_then(|v| self.vars.get(&v).cloned()) {
            let method = call.method.to_string();
            let add = |kind, path| PyModuleAdd {
                module: module.clone(),
                kind,
                path,
            };
            match method.as_str() {
                "add_class" => {
                    if let Some(args) = &call.turbofish
                        && let Some(syn::GenericArgument::Type(ty)) = args.args.first()
                    {
                        self.adds.push(add(PyModuleAddKind::Class, type_str(ty)));
                    }
                }
                "add_function" | "add_wrapped" | "add_submodule" => {
                    let arg = call.args.first();
                    let var = arg.and_then(var_name);
                    if let Some(path) = arg.and_then(wrapped_function) {
                        self.adds.push(add(PyModuleAddKind::Function, path));
                    } else if let Some(function) = arg
                        .and_then(wrapped_pymodule)
                        .or_else(|| var.as_ref().and_then(|v| self.wrapped.get(v).cloned()))
                    {
                        self.adds.push(add(PyModuleAddKind::Pymodule, function));
                    } else if let Some(child) = var
                        .and_then(|v| self.vars.get(&v).cloned())
                        .filter(|c| !c.is_empty())
                    {
                        self.adds.push(add(PyModuleAddKind::Submodule, child));
                    }
                }
                _ => {}
            }
        }
        syn::visit::visit_expr_method_call(self, call);
    }

    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        // A helper that gets a module: `register(m)`, `utils::register(py, m)`
        if let Expr::Path(func) = &*call.func
            && let Some(last) = func.path.segments.last()
        {
            // `m.py()` is the Python token, not the module
            let module = call
                .args
                .iter()
                .filter(|a| !matches!(a, Expr::MethodCall(mc) if mc.method == "py"))
                .find_map(|a| var_name(a).and_then(|v| self.vars.get(&v).cloned()));
            if let Some(module) = module
                && last.ident != "new"
                && last.ident != "new_bound"
            {
                self.adds.push(PyModuleAdd {
                    module,
                    kind: PyModuleAddKind::Call,
                    path: last.ident.to_string(),
                });
            }
        }
        syn::visit::visit_expr_call(self, call);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registrar_of(code: &str) -> Option<PyModuleRegistrar> {
        let f: ItemFn = syn::parse_str(code).unwrap();
        registrar(&f)
    }

    #[test]
    fn test_pymodule_registrations() {
        let r = registrar_of(
            r#"
/// The cloaca module.
#[pymodule]
fn cloaca(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_authoring(m)?;
    let integrations = PyModule::new(m.py(), "integrations")?;
    integrations::register(m.py(), &integrations)?;
    m.add_class::<bindings::runner::PyDefaultRunner>()?;
    m.add_function(wrap_pyfunction!(bindings::runner::_shutdown_all_runners, m)?)?;
    let sub = PyModule::new(m.py(), "events")?;
    sub.add_class::<Event>()?;
    sub.add_function(pyo3::wrap_pyfunction!(emit, &sub)?)?;
    m.add_submodule(&sub)?;
    Ok(())
}"#,
        )
        .unwrap();
        assert_eq!(r.function, "cloaca");
        assert_eq!(r.pymodule.as_deref(), Some("cloaca"));
        assert_eq!(r.doc_comment.as_deref(), Some("The cloaca module."));
        let adds: Vec<(&str, PyModuleAddKind, &str)> = r
            .adds
            .iter()
            .map(|a| (a.module.as_str(), a.kind, a.path.as_str()))
            .collect();
        assert_eq!(
            adds,
            vec![
                ("", PyModuleAddKind::Call, "register_authoring"),
                ("integrations", PyModuleAddKind::Call, "register"),
                (
                    "",
                    PyModuleAddKind::Class,
                    "bindings::runner::PyDefaultRunner"
                ),
                (
                    "",
                    PyModuleAddKind::Function,
                    "bindings::runner::_shutdown_all_runners"
                ),
                ("events", PyModuleAddKind::Class, "Event"),
                ("events", PyModuleAddKind::Function, "emit"),
                ("", PyModuleAddKind::Submodule, "events"),
            ]
        );
    }

    #[test]
    fn test_wrapped_pymodules() {
        let r = registrar_of(
            r#"
#[pymodule]
pub fn integrations(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(docker::docker_integration))?;
    let git_module = wrap_pymodule!(git::git_integration)(py);
    m.add_submodule(git_module.bind(py))?;
    Ok(())
}"#,
        )
        .unwrap();
        let adds: Vec<(PyModuleAddKind, &str)> =
            r.adds.iter().map(|a| (a.kind, a.path.as_str())).collect();
        assert_eq!(
            adds,
            vec![
                (PyModuleAddKind::Pymodule, "docker_integration"),
                (PyModuleAddKind::Pymodule, "git_integration"),
            ]
        );
    }

    #[test]
    fn test_helper_and_name() {
        let r = registrar_of(
            "pub fn register(parent: &Bound<'_, PyModule>) -> PyResult<()> { let m = PyModule::new(parent.py(), \"config\")?; m.add_class::<PyRunnerConfig>()?; parent.add_submodule(&m)?; Ok(()) }",
        )
        .unwrap();
        assert_eq!(r.pymodule, None);
        assert_eq!(r.adds.len(), 2);
        assert_eq!(r.adds[0].module, "config");

        let r = registrar_of(
            "#[pymodule]\n#[pyo3(name = \"_native\")]\nfn native(m: &Bound<'_, PyModule>) -> PyResult<()> { Ok(()) }",
        )
        .unwrap();
        assert_eq!(r.pymodule.as_deref(), Some("_native"));

        assert!(registrar_of("fn plain(x: u8) {}").is_none());
    }
}
