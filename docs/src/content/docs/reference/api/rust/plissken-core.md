---
title: "plissken-core"
description: "plissken-core: Documentation extraction for Rust-Python hybrid projects"
editUrl: false
sidebar:
  order: 2
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

plissken-core: Documentation extraction for Rust-Python hybrid projects

This crate provides the core functionality for parsing Rust and Python
source code and extracting documentation into a unified model.

**API Structure**

The public API is organized into tiers:

- **Core API** (root): Essential types for typical usage
- **Detail API** (`detail`): Advanced types for fine-grained control
- **Modules** (`config`, `parser`, `render`, etc.): Full module access

**Quick Start**

```rust
use plissken_core::{Config, RustParser, PythonParser, DocModel};

// Load configuration
let config = Config::from_file("plissken.toml")?;

// Parse source files
let mut rust_parser = RustParser::new();
let rust_module = rust_parser.parse_file("src/lib.rs")?;

// Build documentation model
let model = DocModel::test("my-project")
    .with_rust_module(rust_module);
```

## Re-exports

- [`Config`](../../rust/plissken-core/config/#struct.Config)
- [`ConfigError`](../../rust/plissken-core/config/#enum.ConfigError)
- [`ConfigWarning`](../../rust/plissken-core/config/#struct.ConfigWarning)
- [`ValidationResult`](../../rust/plissken-core/config/#struct.ValidationResult)
- [`CARGO_MANIFEST`](../../rust/plissken-core/config/#const.CARGO_MANIFEST)
- [`DEFAULT_CRATES`](../../rust/plissken-core/config/#const.DEFAULT_CRATES)
- [`DEFAULT_DOCS_RS_URL`](../../rust/plissken-core/config/#const.DEFAULT_DOCS_RS_URL)
- [`DEFAULT_OUTPUT_FORMAT`](../../rust/plissken-core/config/#const.DEFAULT_OUTPUT_FORMAT)
- [`DEFAULT_OUTPUT_PATH`](../../rust/plissken-core/config/#const.DEFAULT_OUTPUT_PATH)
- [`DEFAULT_STARLIGHT_OUTPUT_PATH`](../../rust/plissken-core/config/#const.DEFAULT_STARLIGHT_OUTPUT_PATH)
- [`DEFAULT_TEMPLATE`](../../rust/plissken-core/config/#const.DEFAULT_TEMPLATE)
- [`PLISSKEN_CONFIG`](../../rust/plissken-core/config/#const.PLISSKEN_CONFIG)
- [`PYPROJECT_MANIFEST`](../../rust/plissken-core/config/#const.PYPROJECT_MANIFEST)
- [`TEMPLATE_MDBOOK`](../../rust/plissken-core/config/#const.TEMPLATE_MDBOOK)
- [`TEMPLATE_MKDOCS_MATERIAL`](../../rust/plissken-core/config/#const.TEMPLATE_MKDOCS_MATERIAL)
- [`TEMPLATE_STARLIGHT`](../../rust/plissken-core/config/#const.TEMPLATE_STARLIGHT)
- [`VERSION_SOURCE_CARGO`](../../rust/plissken-core/config/#const.VERSION_SOURCE_CARGO)
- [`VERSION_SOURCE_PYPROJECT`](../../rust/plissken-core/config/#const.VERSION_SOURCE_PYPROJECT)
- [`PlisskenError`](../../rust/plissken-core/error/#enum.PlisskenError)
- [`Result`](../../rust/plissken-core/error/#type.Result)
- [`CrossRef`](../../rust/plissken-core/model/#struct.CrossRef)
- [`CrossRefKind`](../../rust/plissken-core/model/#enum.CrossRefKind)
- [`DocModel`](../../rust/plissken-core/model/#struct.DocModel)
- [`ProjectMetadata`](../../rust/plissken-core/model/#struct.ProjectMetadata)
- [`SourceType`](../../rust/plissken-core/model/#enum.SourceType)
- [`PythonModule`](../../rust/plissken-core/model/#struct.PythonModule)
- [`RustModule`](../../rust/plissken-core/model/#struct.RustModule)
- [`ParsedModule`](../../rust/plissken-core/parser/traits/#enum.Module) (`parser::Module`)
- [`Parser`](../../rust/plissken-core/parser/traits/#trait.Parser)
- [`ParserLanguage`](../../rust/plissken-core/parser/traits/#enum.ParserLanguage)
- [`create_parser`](../../rust/plissken-core/parser/traits/#fn.create_parser)
- [`parser_for_extension`](../../rust/plissken-core/parser/traits/#fn.parser_for_extension)
- [`PythonParser`](../../rust/plissken-core/parser/python/#struct.PythonParser)
- [`RustParser`](../../rust/plissken-core/parser/rust/#struct.RustParser)
- [`ModuleRenderer`](../../rust/plissken-core/render/module_renderer/#struct.ModuleRenderer)
- [`RenderedPage`](../../rust/plissken-core/render/module_renderer/#struct.RenderedPage)
- [`Renderer`](../../rust/plissken-core/render/renderer/#struct.Renderer)
- [`ThemeAdapter`](../../rust/plissken-core/render/theme/#trait.ThemeAdapter)
- [`get_theme_adapter`](../../rust/plissken-core/render/theme/#fn.get_theme_adapter)
- [`ParsedDocstring`](../../rust/plissken-core/model/#struct.ParsedDocstring)
- [`PythonClass`](../../rust/plissken-core/model/#struct.PythonClass)
- [`PythonFunction`](../../rust/plissken-core/model/#struct.PythonFunction)
- [`PythonItem`](../../rust/plissken-core/model/#enum.PythonItem)
- [`PythonParam`](../../rust/plissken-core/model/#struct.PythonParam)
- [`PythonVariable`](../../rust/plissken-core/model/#struct.PythonVariable)
- [`RustEnum`](../../rust/plissken-core/model/#struct.RustEnum)
- [`RustField`](../../rust/plissken-core/model/#struct.RustField)
- [`RustFunction`](../../rust/plissken-core/model/#struct.RustFunction)
- [`RustImpl`](../../rust/plissken-core/model/#struct.RustImpl)
- [`RustItem`](../../rust/plissken-core/model/#enum.RustItem)
- [`RustItemRef`](../../rust/plissken-core/model/#struct.RustItemRef)
- [`RustParam`](../../rust/plissken-core/model/#struct.RustParam)
- [`RustStruct`](../../rust/plissken-core/model/#struct.RustStruct)
- [`RustTrait`](../../rust/plissken-core/model/#struct.RustTrait)
- [`SourceLocation`](../../rust/plissken-core/model/#struct.SourceLocation)
- [`SourceSpan`](../../rust/plissken-core/model/#struct.SourceSpan)
- [`Visibility`](../../rust/plissken-core/model/#enum.Visibility)
- [`DiscoveredModule`](../../rust/plissken-core/discover/#struct.DiscoveredModule)
- [`discover_python_modules`](../../rust/plissken-core/discover/#fn.discover_python_modules)
- [`merge_modules`](../../rust/plissken-core/discover/#fn.merge_modules)
- [`CargoManifest`](../../rust/plissken-core/manifest/#struct.CargoManifest)
- [`InferredConfig`](../../rust/plissken-core/manifest/#struct.InferredConfig)
- [`PyProjectManifest`](../../rust/plissken-core/manifest/#struct.PyProjectManifest)
- [`build_cross_refs`](../../rust/plissken-core/crossref/#fn.build_cross_refs)
- [`synthesize_python_api`](../../rust/plissken-core/crossref/#fn.synthesize_python_api)
- [`synthesize_python_from_rust`](../../rust/plissken-core/crossref/#fn.synthesize_python_from_rust)
- [`synthesize_python_modules_from_rust`](../../rust/plissken-core/crossref/#fn.synthesize_python_modules_from_rust)
- [`parse_docstring`](../../rust/plissken-core/docstring/#fn.parse_docstring)
- [`parse_rust_doc`](../../rust/plissken-core/docstring/#fn.parse_rust_doc)
- [`CrossRefLink`](../../rust/plissken-core/render/crossref_renderer/#struct.CrossRefLink)
- [`Language`](../../rust/plissken-core/render/crossref_renderer/#enum.Language)
- [`crossref_link`](../../rust/plissken-core/render/crossref_renderer/#fn.crossref_link)
- [`link_to_python`](../../rust/plissken-core/render/crossref_renderer/#fn.link_to_python)
- [`link_to_rust`](../../rust/plissken-core/render/crossref_renderer/#fn.link_to_rust)
- [`render_docstring`](../../rust/plissken-core/render/docstring_renderer/#fn.render_docstring)
- [`render_examples`](../../rust/plissken-core/render/docstring_renderer/#fn.render_examples)
- [`render_params_table`](../../rust/plissken-core/render/docstring_renderer/#fn.render_params_table)
- [`render_python_exposure_details`](../../rust/plissken-core/render/crossref_renderer/#fn.render_python_exposure_details)
- [`render_raises_table`](../../rust/plissken-core/render/docstring_renderer/#fn.render_raises_table)
- [`render_returns`](../../rust/plissken-core/render/docstring_renderer/#fn.render_returns)
- [`render_rust_docstring`](../../rust/plissken-core/render/docstring_renderer/#fn.render_rust_docstring)
- [`render_rust_impl_details`](../../rust/plissken-core/render/crossref_renderer/#fn.render_rust_impl_details)
