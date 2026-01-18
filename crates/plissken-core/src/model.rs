//! Unified documentation model for Rust and Python items

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Source type indicator for Python modules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SourceType {
    /// Pure Python source code
    Python,
    /// Rust code exposed via PyO3
    PyO3Binding,
    /// Pure Rust (no Python exposure)
    Rust,
}

/// Visibility level for Rust items
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Visibility {
    Public,
    PubCrate,
    PubSuper,
    Private,
}

/// A reference to a location in source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line_start: usize,
    pub line_end: usize,
}

/// Source code span with the actual text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceSpan {
    pub location: SourceLocation,
    /// The actual source code text
    pub source: String,
}

/// A Rust module with its items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustModule {
    pub path: String,
    pub doc_comment: Option<String>,
    pub parsed_doc: Option<ParsedDocstring>,
    pub items: Vec<RustItem>,
    pub source: SourceSpan,
}

/// A Rust item (struct, enum, function, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum RustItem {
    Struct(RustStruct),
    Enum(RustEnum),
    Function(RustFunction),
    Trait(RustTrait),
    Impl(RustImpl),
    Const(RustConst),
    TypeAlias(RustTypeAlias),
}

/// A Rust struct definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustStruct {
    pub name: String,
    pub visibility: Visibility,
    pub doc_comment: Option<String>,
    pub parsed_doc: Option<ParsedDocstring>,
    /// Generic parameters as string, e.g. "<T: Clone, const N: usize>"
    pub generics: Option<String>,
    pub fields: Vec<RustField>,
    pub derives: Vec<String>,
    pub pyclass: Option<PyClassMeta>,
    pub source: SourceSpan,
}

/// PyO3 #[pyclass] metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PyClassMeta {
    pub name: Option<String>,
    pub module: Option<String>,
}

/// A Rust field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustField {
    pub name: String,
    pub ty: String,
    pub visibility: Visibility,
    pub doc_comment: Option<String>,
}

/// A Rust enum definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustEnum {
    pub name: String,
    pub visibility: Visibility,
    pub doc_comment: Option<String>,
    pub parsed_doc: Option<ParsedDocstring>,
    /// Generic parameters as string
    pub generics: Option<String>,
    pub variants: Vec<RustVariant>,
    pub source: SourceSpan,
}

/// A Rust enum variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustVariant {
    pub name: String,
    pub doc_comment: Option<String>,
    pub fields: Vec<RustField>,
}

/// A Rust function definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustFunction {
    pub name: String,
    pub visibility: Visibility,
    pub doc_comment: Option<String>,
    pub parsed_doc: Option<ParsedDocstring>,
    /// Generic parameters as string, e.g. "<'a, T: Clone>"
    pub generics: Option<String>,
    /// Full signature as string for display
    pub signature_str: String,
    /// Parsed signature for structured access
    pub signature: RustFunctionSig,
    pub is_async: bool,
    pub is_unsafe: bool,
    pub is_const: bool,
    pub pyfunction: Option<PyFunctionMeta>,
    pub source: SourceSpan,
}

/// Rust function signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustFunctionSig {
    pub params: Vec<RustParam>,
    pub return_type: Option<String>,
}

/// A Rust function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustParam {
    pub name: String,
    pub ty: String,
    pub default: Option<String>,
}

/// PyO3 #[pyfunction] metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PyFunctionMeta {
    pub name: Option<String>,
    pub signature: Option<String>,
}

/// A Rust trait definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustTrait {
    pub name: String,
    pub visibility: Visibility,
    pub doc_comment: Option<String>,
    pub parsed_doc: Option<ParsedDocstring>,
    /// Generic parameters as string
    pub generics: Option<String>,
    /// Supertraits as string, e.g. ": Clone + Send"
    pub bounds: Option<String>,
    pub associated_types: Vec<RustAssociatedType>,
    pub methods: Vec<RustFunction>,
    pub source: SourceSpan,
}

/// A Rust associated type in a trait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustAssociatedType {
    pub name: String,
    pub doc_comment: Option<String>,
    /// Generic parameters (for GATs)
    pub generics: Option<String>,
    /// Bounds on the associated type
    pub bounds: Option<String>,
}

/// A Rust impl block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustImpl {
    /// Generic parameters on the impl block
    pub generics: Option<String>,
    /// The type being implemented for
    pub target: String,
    /// Trait being implemented (if any)
    pub trait_: Option<String>,
    /// Where clause constraints
    pub where_clause: Option<String>,
    pub methods: Vec<RustFunction>,
    /// Whether this is a #[pymethods] block
    pub pymethods: bool,
    pub source: SourceSpan,
}

/// A Rust const item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustConst {
    pub name: String,
    pub visibility: Visibility,
    pub doc_comment: Option<String>,
    pub ty: String,
    pub value: Option<String>,
    pub source: SourceSpan,
}

/// A Rust type alias
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustTypeAlias {
    pub name: String,
    pub visibility: Visibility,
    pub doc_comment: Option<String>,
    pub generics: Option<String>,
    /// The aliased type as string
    pub ty: String,
    pub source: SourceSpan,
}

// ============================================================================
// Python Model
// ============================================================================

/// A Python module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonModule {
    pub path: String,
    pub docstring: Option<String>,
    pub parsed_doc: Option<ParsedDocstring>,
    pub items: Vec<PythonItem>,
    pub source_type: SourceType,
    pub source: SourceSpan,
}

/// A Python item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum PythonItem {
    Class(PythonClass),
    Function(PythonFunction),
    Variable(PythonVariable),
}

/// A Python class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonClass {
    pub name: String,
    pub docstring: Option<String>,
    pub parsed_doc: Option<ParsedDocstring>,
    pub bases: Vec<String>,
    pub methods: Vec<PythonFunction>,
    pub attributes: Vec<PythonVariable>,
    pub decorators: Vec<String>,
    pub rust_impl: Option<RustItemRef>,
    pub source: SourceSpan,
}

/// A Python function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonFunction {
    pub name: String,
    pub docstring: Option<String>,
    /// Full signature as string for display
    pub signature_str: String,
    /// Parsed signature for structured access
    pub signature: PythonFunctionSig,
    pub decorators: Vec<String>,
    pub is_async: bool,
    pub is_staticmethod: bool,
    pub is_classmethod: bool,
    pub is_property: bool,
    pub parsed_doc: Option<ParsedDocstring>,
    pub rust_impl: Option<RustItemRef>,
    pub source: SourceSpan,
}

/// Python function signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonFunctionSig {
    pub params: Vec<PythonParam>,
    pub return_type: Option<String>,
}

/// A Python function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonParam {
    pub name: String,
    pub ty: Option<String>,
    pub default: Option<String>,
}

/// A Python variable/attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonVariable {
    pub name: String,
    pub ty: Option<String>,
    pub value: Option<String>,
    pub docstring: Option<String>,
}

/// Parsed docstring (Google/NumPy style)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedDocstring {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub params: Vec<ParamDoc>,
    pub returns: Option<ReturnDoc>,
    pub raises: Vec<RaisesDoc>,
    pub examples: Vec<String>,
}

/// Documented parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamDoc {
    pub name: String,
    pub ty: Option<String>,
    pub description: String,
}

/// Documented return value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnDoc {
    pub ty: Option<String>,
    pub description: String,
}

/// Documented exception
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaisesDoc {
    pub ty: String,
    pub description: String,
}

// ============================================================================
// Cross-Reference
// ============================================================================

/// Reference to a Rust item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustItemRef {
    pub path: String,
    pub name: String,
}

/// Cross-reference between Python and Rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossRef {
    pub python_path: String,
    pub rust_path: String,
    pub relationship: CrossRefKind,
}

/// Kind of cross-reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossRefKind {
    /// Direct PyO3 binding
    Binding,
    /// Python wraps Rust
    Wraps,
    /// Python delegates to Rust
    Delegates,
}

// ============================================================================
// Top-Level Model
// ============================================================================

/// The complete documentation model for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocModel {
    pub metadata: ProjectMetadata,
    pub rust_modules: Vec<RustModule>,
    pub python_modules: Vec<PythonModule>,
    pub cross_refs: Vec<CrossRef>,
}

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// Project name
    pub name: String,
    /// Project version (from Cargo.toml or pyproject.toml)
    pub version: Option<String>,
    /// Project description
    pub description: Option<String>,
    /// Git ref (branch or tag) this was generated from
    pub git_ref: Option<String>,
    /// Git commit hash
    pub git_commit: Option<String>,
    /// When the documentation was generated
    pub generated_at: String,
}

// ============================================================================
// Builder APIs for Testing
// ============================================================================

impl SourceLocation {
    /// Create a test source location
    pub fn test(file: impl Into<PathBuf>, line_start: usize, line_end: usize) -> Self {
        Self {
            file: file.into(),
            line_start,
            line_end,
        }
    }
}

impl SourceSpan {
    /// Create a test source span with empty source
    pub fn test(file: impl Into<PathBuf>, line_start: usize, line_end: usize) -> Self {
        Self {
            location: SourceLocation::test(file, line_start, line_end),
            source: String::new(),
        }
    }

    /// Create a source span with actual source code
    pub fn new(
        file: impl Into<PathBuf>,
        line_start: usize,
        line_end: usize,
        source: impl Into<String>,
    ) -> Self {
        Self {
            location: SourceLocation::test(file, line_start, line_end),
            source: source.into(),
        }
    }
}

impl RustModule {
    /// Create a test Rust module
    pub fn test(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            doc_comment: None,
            parsed_doc: None,
            items: Vec::new(),
            source: SourceSpan::test("test.rs", 1, 1),
        }
    }

    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc_comment = Some(doc.into());
        self
    }

    pub fn with_item(mut self, item: RustItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn with_source(mut self, source: SourceSpan) -> Self {
        self.source = source;
        self
    }
}

impl RustStruct {
    /// Create a test struct
    pub fn test(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visibility: Visibility::Public,
            doc_comment: None,
            parsed_doc: None,
            generics: None,
            fields: Vec::new(),
            derives: Vec::new(),
            pyclass: None,
            source: SourceSpan::test("test.rs", 1, 1),
        }
    }

    pub fn with_visibility(mut self, vis: Visibility) -> Self {
        self.visibility = vis;
        self
    }

    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc_comment = Some(doc.into());
        self
    }

    pub fn with_generics(mut self, generics: impl Into<String>) -> Self {
        self.generics = Some(generics.into());
        self
    }

    pub fn with_field(mut self, field: RustField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn with_derive(mut self, derive: impl Into<String>) -> Self {
        self.derives.push(derive.into());
        self
    }

    pub fn with_pyclass(mut self, meta: PyClassMeta) -> Self {
        self.pyclass = Some(meta);
        self
    }

    pub fn with_source(mut self, source: SourceSpan) -> Self {
        self.source = source;
        self
    }
}

impl RustField {
    /// Create a test field
    pub fn test(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
            visibility: Visibility::Public,
            doc_comment: None,
        }
    }

    pub fn private(mut self) -> Self {
        self.visibility = Visibility::Private;
        self
    }

    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc_comment = Some(doc.into());
        self
    }
}

impl RustFunction {
    /// Create a test function
    pub fn test(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            name: name.clone(),
            visibility: Visibility::Public,
            doc_comment: None,
            parsed_doc: None,
            generics: None,
            signature_str: format!("fn {}()", name),
            signature: RustFunctionSig {
                params: Vec::new(),
                return_type: None,
            },
            is_async: bool::default(),
            is_unsafe: bool::default(),
            is_const: bool::default(),
            pyfunction: None,
            source: SourceSpan::test("test.rs", 1, 1),
        }
    }

    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc_comment = Some(doc.into());
        self
    }

    pub fn with_generics(mut self, generics: impl Into<String>) -> Self {
        self.generics = Some(generics.into());
        self
    }

    pub fn with_signature(mut self, sig: impl Into<String>) -> Self {
        self.signature_str = sig.into();
        self
    }

    pub fn with_param(mut self, param: RustParam) -> Self {
        self.signature.params.push(param);
        self
    }

    pub fn with_return_type(mut self, ty: impl Into<String>) -> Self {
        self.signature.return_type = Some(ty.into());
        self
    }

    pub fn async_(mut self) -> Self {
        self.is_async = true;
        self
    }

    pub fn unsafe_(mut self) -> Self {
        self.is_unsafe = true;
        self
    }

    pub fn const_(mut self) -> Self {
        self.is_const = true;
        self
    }

    pub fn with_pyfunction(mut self, meta: PyFunctionMeta) -> Self {
        self.pyfunction = Some(meta);
        self
    }

    pub fn with_source(mut self, source: SourceSpan) -> Self {
        self.source = source;
        self
    }
}

impl RustParam {
    /// Create a test parameter
    pub fn test(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
            default: None,
        }
    }

    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }
}

impl PyClassMeta {
    pub fn new() -> Self {
        Self {
            name: None,
            module: None,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_module(mut self, module: impl Into<String>) -> Self {
        self.module = Some(module.into());
        self
    }
}

impl Default for PyClassMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl PyFunctionMeta {
    pub fn new() -> Self {
        Self {
            name: None,
            signature: None,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_signature(mut self, sig: impl Into<String>) -> Self {
        self.signature = Some(sig.into());
        self
    }
}

impl Default for PyFunctionMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl PythonModule {
    /// Create a test Python module
    pub fn test(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            docstring: None,
            parsed_doc: None,
            items: Vec::new(),
            source_type: SourceType::Python,
            source: SourceSpan::test("test.py", 1, 1),
        }
    }

    pub fn with_docstring(mut self, doc: impl Into<String>) -> Self {
        self.docstring = Some(doc.into());
        self
    }

    pub fn with_item(mut self, item: PythonItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn pyo3_binding(mut self) -> Self {
        self.source_type = SourceType::PyO3Binding;
        self
    }

    pub fn with_source(mut self, source: SourceSpan) -> Self {
        self.source = source;
        self
    }
}

impl PythonClass {
    /// Create a test Python class
    pub fn test(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            docstring: None,
            parsed_doc: None,
            bases: Vec::new(),
            methods: Vec::new(),
            attributes: Vec::new(),
            decorators: Vec::new(),
            rust_impl: None,
            source: SourceSpan::test("test.py", 1, 1),
        }
    }

    pub fn with_docstring(mut self, doc: impl Into<String>) -> Self {
        self.docstring = Some(doc.into());
        self
    }

    pub fn with_base(mut self, base: impl Into<String>) -> Self {
        self.bases.push(base.into());
        self
    }

    pub fn with_method(mut self, method: PythonFunction) -> Self {
        self.methods.push(method);
        self
    }

    pub fn with_attribute(mut self, attr: PythonVariable) -> Self {
        self.attributes.push(attr);
        self
    }

    pub fn with_decorator(mut self, decorator: impl Into<String>) -> Self {
        self.decorators.push(decorator.into());
        self
    }

    pub fn with_rust_impl(mut self, rust_ref: RustItemRef) -> Self {
        self.rust_impl = Some(rust_ref);
        self
    }

    pub fn with_source(mut self, source: SourceSpan) -> Self {
        self.source = source;
        self
    }
}

impl PythonFunction {
    /// Create a test Python function
    pub fn test(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            name: name.clone(),
            docstring: None,
            signature_str: format!("def {}()", name),
            signature: PythonFunctionSig {
                params: Vec::new(),
                return_type: None,
            },
            decorators: Vec::new(),
            is_async: bool::default(),
            is_staticmethod: bool::default(),
            is_classmethod: bool::default(),
            is_property: bool::default(),
            parsed_doc: None,
            rust_impl: None,
            source: SourceSpan::test("test.py", 1, 1),
        }
    }

    pub fn with_docstring(mut self, doc: impl Into<String>) -> Self {
        self.docstring = Some(doc.into());
        self
    }

    pub fn with_signature(mut self, sig: impl Into<String>) -> Self {
        self.signature_str = sig.into();
        self
    }

    pub fn with_param(mut self, param: PythonParam) -> Self {
        self.signature.params.push(param);
        self
    }

    pub fn with_return_type(mut self, ty: impl Into<String>) -> Self {
        self.signature.return_type = Some(ty.into());
        self
    }

    pub fn with_decorator(mut self, decorator: impl Into<String>) -> Self {
        self.decorators.push(decorator.into());
        self
    }

    pub fn async_(mut self) -> Self {
        self.is_async = true;
        self
    }

    pub fn staticmethod(mut self) -> Self {
        self.is_staticmethod = true;
        self
    }

    pub fn classmethod(mut self) -> Self {
        self.is_classmethod = true;
        self
    }

    pub fn property(mut self) -> Self {
        self.is_property = true;
        self
    }

    pub fn with_rust_impl(mut self, rust_ref: RustItemRef) -> Self {
        self.rust_impl = Some(rust_ref);
        self
    }

    pub fn with_source(mut self, source: SourceSpan) -> Self {
        self.source = source;
        self
    }
}

impl PythonParam {
    /// Create a test parameter
    pub fn test(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: None,
            default: None,
        }
    }

    pub fn with_type(mut self, ty: impl Into<String>) -> Self {
        self.ty = Some(ty.into());
        self
    }

    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }
}

impl PythonVariable {
    /// Create a test variable
    pub fn test(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: None,
            value: None,
            docstring: None,
        }
    }

    pub fn with_type(mut self, ty: impl Into<String>) -> Self {
        self.ty = Some(ty.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_docstring(mut self, doc: impl Into<String>) -> Self {
        self.docstring = Some(doc.into());
        self
    }
}

impl RustItemRef {
    pub fn new(path: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            name: name.into(),
        }
    }
}

impl CrossRef {
    pub fn binding(python_path: impl Into<String>, rust_path: impl Into<String>) -> Self {
        Self {
            python_path: python_path.into(),
            rust_path: rust_path.into(),
            relationship: CrossRefKind::Binding,
        }
    }

    pub fn wraps(python_path: impl Into<String>, rust_path: impl Into<String>) -> Self {
        Self {
            python_path: python_path.into(),
            rust_path: rust_path.into(),
            relationship: CrossRefKind::Wraps,
        }
    }

    pub fn delegates(python_path: impl Into<String>, rust_path: impl Into<String>) -> Self {
        Self {
            python_path: python_path.into(),
            rust_path: rust_path.into(),
            relationship: CrossRefKind::Delegates,
        }
    }
}

impl DocModel {
    /// Create an empty doc model for testing
    pub fn test(name: impl Into<String>) -> Self {
        Self {
            metadata: ProjectMetadata::test(name),
            rust_modules: Vec::new(),
            python_modules: Vec::new(),
            cross_refs: Vec::new(),
        }
    }

    pub fn with_rust_module(mut self, module: RustModule) -> Self {
        self.rust_modules.push(module);
        self
    }

    pub fn with_python_module(mut self, module: PythonModule) -> Self {
        self.python_modules.push(module);
        self
    }

    pub fn with_cross_ref(mut self, cross_ref: CrossRef) -> Self {
        self.cross_refs.push(cross_ref);
        self
    }
}

impl ProjectMetadata {
    /// Create test metadata
    pub fn test(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: None,
            description: None,
            git_ref: None,
            git_commit: None,
            generated_at: "2024-01-01T00:00:00Z".to_string(),
        }
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn with_git_ref(mut self, git_ref: impl Into<String>) -> Self {
        self.git_ref = Some(git_ref.into());
        self
    }

    pub fn with_git_commit(mut self, commit: impl Into<String>) -> Self {
        self.git_commit = Some(commit.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_serialization_roundtrip() {
        let model = DocModel::test("test-project")
            .with_rust_module(
                RustModule::test("crate::module")
                    .with_doc("Module documentation")
                    .with_item(RustItem::Struct(
                        RustStruct::test("MyStruct")
                            .with_doc("A test struct")
                            .with_generics("<T: Clone>")
                            .with_field(RustField::test("data", "Vec<T>"))
                            .with_derive("Debug")
                            .with_derive("Clone"),
                    ))
                    .with_item(RustItem::Function(
                        RustFunction::test("process")
                            .with_doc("Process the data")
                            .with_generics("<T>")
                            .with_signature("fn process<T>(data: &[T]) -> Result<(), Error>")
                            .with_param(RustParam::test("data", "&[T]"))
                            .with_return_type("Result<(), Error>"),
                    )),
            )
            .with_python_module(
                PythonModule::test("mymodule")
                    .with_docstring("Python module docs")
                    .with_item(PythonItem::Class(
                        PythonClass::test("MyClass")
                            .with_docstring("A Python class")
                            .with_method(
                                PythonFunction::test("__init__")
                                    .with_param(PythonParam::test("self"))
                                    .with_param(
                                        PythonParam::test("value")
                                            .with_type("int")
                                            .with_default("0"),
                                    ),
                            ),
                    )),
            )
            .with_cross_ref(CrossRef::binding(
                "mymodule.MyClass",
                "crate::module::MyStruct",
            ));

        // Serialize to JSON
        let json = serde_json::to_string_pretty(&model).expect("serialize failed");

        // Deserialize back
        let roundtrip: DocModel = serde_json::from_str(&json).expect("deserialize failed");

        // Verify key fields survived
        assert_eq!(roundtrip.metadata.name, "test-project");
        assert_eq!(roundtrip.rust_modules.len(), 1);
        assert_eq!(roundtrip.python_modules.len(), 1);
        assert_eq!(roundtrip.cross_refs.len(), 1);
    }

    #[test]
    fn test_rust_struct_builder() {
        let s = RustStruct::test("Buffer")
            .with_visibility(Visibility::Public)
            .with_doc("A fixed-size buffer")
            .with_generics("<const N: usize>")
            .with_field(RustField::test("data", "[u8; N]").private())
            .with_field(RustField::test("len", "usize").private())
            .with_derive("Debug")
            .with_pyclass(PyClassMeta::new().with_name("Buffer"));

        assert_eq!(s.name, "Buffer");
        assert_eq!(s.visibility, Visibility::Public);
        assert!(s.doc_comment.is_some());
        assert_eq!(s.generics, Some("<const N: usize>".to_string()));
        assert_eq!(s.fields.len(), 2);
        assert_eq!(s.derives, vec!["Debug"]);
        assert!(s.pyclass.is_some());
    }

    #[test]
    fn test_python_function_builder() {
        let f = PythonFunction::test("fetch")
            .with_docstring("Fetch data from URL")
            .with_signature("def fetch(url: str, *, timeout: float = 30.0) -> bytes")
            .with_param(PythonParam::test("url").with_type("str"))
            .with_param(
                PythonParam::test("timeout")
                    .with_type("float")
                    .with_default("30.0"),
            )
            .with_return_type("bytes")
            .async_();

        assert_eq!(f.name, "fetch");
        assert!(f.is_async);
        assert_eq!(f.signature.params.len(), 2);
        assert_eq!(f.signature.return_type, Some("bytes".to_string()));
    }

    #[test]
    fn test_cross_ref_constructors() {
        let binding = CrossRef::binding("pkg.Class", "crate::Class");
        assert!(matches!(binding.relationship, CrossRefKind::Binding));

        let wraps = CrossRef::wraps("pkg.Wrapper", "crate::Inner");
        assert!(matches!(wraps.relationship, CrossRefKind::Wraps));

        let delegates = CrossRef::delegates("pkg.Client", "crate::http::Client");
        assert!(matches!(delegates.relationship, CrossRefKind::Delegates));
    }
}
