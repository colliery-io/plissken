//! Which items are documented
//!
//! The parsers keep all items. Before rendering, these functions leave out
//! what the documentation of a public API does not show: test code, and
//! private items unless the configuration asks for them. PyO3 bindings are
//! part of the Python API, so they stay whatever their Rust visibility.

use crate::model::{PythonClass, PythonItem, PythonModule, RustItem, RustModule, Visibility};
use std::collections::HashSet;

/// The Rust modules to document.
///
/// Leaves out the modules declared `#[cfg(test)] mod name;` and their
/// submodules. Unless `include_private`, leaves out the items, fields and
/// methods that are not `pub` (bindings stay), and `pub(crate)` re-exports.
/// Then leaves out the modules with nothing to show: no doc comment, no
/// items, no re-exports.
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

fn is_pub(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public)
}

/// Leave out the items of a module that are not `pub`.
fn keep_public_rust(module: &mut RustModule) {
    let mut kept_types: HashSet<String> = HashSet::new();
    module.items.retain_mut(|item| match item {
        RustItem::Struct(s) => {
            let keep = is_pub(&s.visibility) || s.pyclass.is_some();
            if keep {
                if s.pyclass.is_none() {
                    s.fields.retain(|f| is_pub(&f.visibility));
                }
                kept_types.insert(s.name.clone());
            }
            keep
        }
        RustItem::Enum(e) => {
            let keep = is_pub(&e.visibility);
            if keep {
                kept_types.insert(e.name.clone());
            }
            keep
        }
        RustItem::Function(f) => is_pub(&f.visibility) || f.pyfunction.is_some(),
        RustItem::Trait(t) => is_pub(&t.visibility),
        RustItem::Const(c) => is_pub(&c.visibility),
        RustItem::TypeAlias(t) => is_pub(&t.visibility),
        RustItem::Impl(_) => true,
    });
    module.items.retain_mut(|item| match item {
        RustItem::Impl(i) => {
            let target = i.target.split('<').next().unwrap_or(&i.target).trim();
            if !kept_types.contains(target) {
                return false;
            }
            // Trait methods have the visibility of the trait
            if i.trait_.is_none() && !i.pymethods {
                i.methods
                    .retain(|f| is_pub(&f.visibility) || f.pyfunction.is_some());
            }
            !i.methods.is_empty()
        }
        _ => true,
    });
    module.reexports.retain(|r| is_pub(&r.visibility));
}

/// Whether a Rust module page would show anything.
fn has_rust_content(module: &RustModule) -> bool {
    module
        .doc_comment
        .as_deref()
        .is_some_and(|d| !d.trim().is_empty())
        || !module.reexports.is_empty()
        || module.items.iter().any(|i| !matches!(i, RustItem::Impl(_)))
}

/// Whether a Python name is private: it starts with `_` and is not a
/// `__dunder__` name.
pub fn is_private_python_name(name: &str) -> bool {
    name.starts_with('_') && !(name.starts_with("__") && name.ends_with("__") && name.len() > 4)
}

/// The Python modules to document.
///
/// Unless `include_private`, leaves out the classes, functions, variables,
/// methods and attributes whose names are private (`_helper`). Then leaves
/// out the modules with nothing to show.
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

fn keep_public_python_class(class: &mut PythonClass) {
    class.methods.retain(|f| !is_private_python_name(&f.name));
    class
        .attributes
        .retain(|a| !is_private_python_name(&a.name));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::RustParser;
    use std::path::Path;

    fn parse(path: &str, code: &str) -> RustModule {
        let mut m = RustParser::new()
            .parse_str(code, Path::new("test.rs"))
            .unwrap();
        m.path = path.to_string();
        m
    }

    #[test]
    fn test_cfg_test_modules_are_left_out() {
        // Items 1 and 16 of PLSKN-T-0057
        let root = parse(
            "my_crate",
            "//! The crate.\n#[cfg(test)]\npub mod test_fixtures;\n#[cfg(test)]\nmod tests;\npub mod real;\n",
        );
        assert_eq!(root.test_modules, vec!["test_fixtures", "tests"]);
        let fixtures = parse("my_crate::test_fixtures", "pub fn fixture() {}");
        let nested = parse("my_crate::tests::deep", "pub fn helper() {}");
        let real = parse("my_crate::real", "pub fn run() {}");
        let kept = documented_rust_modules(vec![root, fixtures, nested, real], false);
        let paths: Vec<&str> = kept.iter().map(|m| m.path.as_str()).collect();
        assert_eq!(paths, vec!["my_crate", "my_crate::real"]);
    }

    #[test]
    fn test_test_items_are_not_parsed() {
        let m = parse(
            "my_crate",
            "pub fn run() {}\n#[test]\nfn it_works() {}\n#[cfg(test)]\nfn helper() {}\n",
        );
        assert_eq!(m.items.len(), 1);
    }

    #[test]
    fn test_private_items_are_left_out() {
        // Item 7
        let m = parse(
            "my_crate",
            r#"
pub struct Public { pub a: u8, b: u8 }
struct Private;
impl Public { pub fn open(&self) {} fn helper(&self) {} }
impl Private { pub fn hidden(&self) {} }
pub(crate) fn internal() {}
pub fn api() {}
#[pyclass]
struct PyThing;
#[pymethods]
impl PyThing { fn method(&self) {} }
#[pyfunction]
fn py_func() {}
pub const LIMIT: usize = 3;
const SECRET: u8 = 1;
"#,
        );
        let kept = documented_rust_modules(vec![m.clone()], false);
        let names: Vec<String> = kept[0]
            .items
            .iter()
            .map(|i| match i {
                RustItem::Struct(s) => format!("struct {} {}", s.name, s.fields.len()),
                RustItem::Function(f) => format!("fn {}", f.name),
                RustItem::Impl(i) => format!(
                    "impl {} {}",
                    i.target,
                    i.methods
                        .iter()
                        .map(|f| f.name.as_str())
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                RustItem::Const(c) => format!("const {}", c.name),
                _ => "other".to_string(),
            })
            .collect();
        assert_eq!(
            names,
            vec![
                "struct Public 1",
                "impl Public open",
                "fn api",
                "struct PyThing 0",
                "impl PyThing method",
                "fn py_func",
                "const LIMIT",
            ]
        );
        // With the setting, all stay
        let all = documented_rust_modules(vec![m], true);
        assert_eq!(all[0].items.len(), 11);
    }

    #[test]
    fn test_empty_modules_are_left_out() {
        let empty = parse("my_crate::internal", "fn helper() {}");
        let reexports = parse("my_crate::context", "pub use other::Context;");
        let kept = documented_rust_modules(vec![empty, reexports], false);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].path, "my_crate::context");
    }

    #[test]
    fn test_reexports_are_parsed() {
        // Item 24
        let m = parse(
            "my_crate::context",
            "pub use crate::inner::{Context, helpers::{self, Helper as H}, prelude::*};\nuse std::fmt;\npub(crate) use x::Y;\n",
        );
        let found: Vec<(String, String)> = m
            .reexports
            .iter()
            .map(|r| (r.name.clone(), r.path.clone()))
            .collect();
        assert_eq!(
            found,
            vec![
                ("Context".to_string(), "crate::inner::Context".to_string()),
                ("helpers".to_string(), "crate::inner::helpers".to_string()),
                ("H".to_string(), "crate::inner::helpers::Helper".to_string()),
                ("*".to_string(), "crate::inner::prelude".to_string()),
                ("Y".to_string(), "x::Y".to_string()),
            ]
        );
        let kept = documented_rust_modules(vec![m], false);
        assert_eq!(kept[0].reexports.len(), 4);
    }

    #[test]
    fn test_private_python_names() {
        // Item 20
        assert!(is_private_python_name("_expect"));
        assert!(is_private_python_name("_DEFAULT_TIMEOUT"));
        assert!(is_private_python_name("__mangled"));
        assert!(!is_private_python_name("__init__"));
        assert!(!is_private_python_name("__call__"));
        assert!(!is_private_python_name("client"));

        let mut m = PythonModule::test("brokkr.client");
        m.items = vec![
            PythonItem::Variable(crate::model::PythonVariable::test("_DEFAULT_URL")),
            PythonItem::Variable(crate::model::PythonVariable::test("VERSION")),
            PythonItem::Function(crate::model::PythonFunction::test("_expect")),
            PythonItem::Class(
                crate::model::PythonClass::test("Client")
                    .with_method(crate::model::PythonFunction::test("__init__"))
                    .with_method(crate::model::PythonFunction::test("_request")),
            ),
        ];
        let kept = documented_python_modules(vec![m], false);
        assert_eq!(kept[0].items.len(), 2);
        let PythonItem::Class(c) = &kept[0].items[1] else {
            panic!("Expected class");
        };
        assert_eq!(c.methods.len(), 1);
        assert_eq!(c.methods[0].name, "__init__");
    }
}
