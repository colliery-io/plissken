---
title: "API reference"
description: "The API reference, written by plissken from the source."
editUrl: false
sidebar:
  order: 0
  hidden: true
---

## Rust

- [`plissken`](rust/plissken/): plissken CLI - Documentation for the Rust-Python bridge
- [`plissken-core`](rust/plissken-core/): plissken-core: Documentation extraction for Rust-Python hybrid projects
- [`plissken-core::config`](rust/plissken-core/config/): Configuration for plissken projects
- [`plissken-core::crossref`](rust/plissken-core/crossref/): Cross-reference builder for linking Python and Rust items
- [`plissken-core::discover`](rust/plissken-core/discover/): Python module auto-discovery
- [`plissken-core::docstring`](rust/plissken-core/docstring/): Docstring parser for Google, NumPy, and Rust doc comment styles
- [`plissken-core::error`](rust/plissken-core/error/): Unified error types for plissken-core.
- [`plissken-core::filter`](rust/plissken-core/filter/): Which items are documented
- [`plissken-core::manifest`](rust/plissken-core/manifest/): Manifest file parsing for Cargo.toml and pyproject.toml
- [`plissken-core::model`](rust/plissken-core/model/): Unified documentation model for Rust and Python items
- [`plissken-core::parser`](rust/plissken-core/parser/): Parsing infrastructure for Rust and Python source code
- [`plissken-core::parser::pymodule`](rust/plissken-core/parser/pymodule/): What PyO3 module functions register
- [`plissken-core::parser::python`](rust/plissken-core/parser/python/): Python source code parser using tree-sitter
- [`plissken-core::parser::rust`](rust/plissken-core/parser/rust/): Rust source code parser using syn
- [`plissken-core::parser::traits`](rust/plissken-core/parser/traits/): Parser trait and related types for language-agnostic parsing.
- [`plissken-core::render`](rust/plissken-core/render/): Rendering module for documentation output
- [`plissken-core::render::crossref_renderer`](rust/plissken-core/render/crossref_renderer/): Cross-reference link generation for Python-Rust documentation
- [`plissken-core::render::doc_links`](rust/plissken-core/render/doc_links/): Intra-doc links of Rust doc comments
- [`plissken-core::render::docstring_renderer`](rust/plissken-core/render/docstring_renderer/): Docstring rendering for parsed documentation
- [`plissken-core::render::module`](rust/plissken-core/render/module/): Module documentation components
- [`plissken-core::render::module::crossref`](rust/plissken-core/render/module/crossref/): Cross-reference link generation for bidirectional Python/Rust documentation.
- [`plissken-core::render::module::layout`](rust/plissken-core/render/module/layout/): Page layout and path computation for documentation output.
- [`plissken-core::render::module_renderer`](rust/plissken-core/render/module_renderer/): Module page rendering for Python and Rust documentation
- [`plissken-core::render::renderer`](rust/plissken-core/render/renderer/): Tera-based documentation renderer
- [`plissken-core::render::ssg`](rust/plissken-core/render/ssg/): Static Site Generator (SSG) adapters
- [`plissken-core::render::ssg::mdbook`](rust/plissken-core/render/ssg/mdbook/): mdBook adapter implementation
- [`plissken-core::render::ssg::mkdocs`](rust/plissken-core/render/ssg/mkdocs/): MkDocs adapter implementation
- [`plissken-core::render::ssg::starlight`](rust/plissken-core/render/ssg/starlight/): Astro Starlight adapter implementation
- [`plissken-core::render::ssg::traits`](rust/plissken-core/render/ssg/traits/): SSG adapter trait definition
- [`plissken-core::render::templates`](rust/plissken-core/render/templates/): Template loading with user override support
- [`plissken-core::render::theme`](rust/plissken-core/render/theme/): Theme adapters for SSG-native CSS variable integration
