---
title: "plissken-core::parser"
description: "Parsing infrastructure for Rust and Python source code"
editUrl: false
sidebar:
  order: 11
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Parsing infrastructure for Rust and Python source code

This module provides parsers for extracting documentation from source code.
The [`Parser`](../../../rust/plissken-core/parser/traits/#trait.Parser) trait provides a language-agnostic interface, while
[`RustParser`](../../../rust/plissken-core/parser/rust/#struct.RustParser) and [`PythonParser`](../../../rust/plissken-core/parser/python/#struct.PythonParser) provide concrete implementations.

## Re-exports

- [`PythonParser`](../../../rust/plissken-core/parser/python/#struct.PythonParser)
- [`RustParser`](../../../rust/plissken-core/parser/rust/#struct.RustParser)
- [`Module`](../../../rust/plissken-core/parser/traits/#enum.Module)
- [`Parser`](../../../rust/plissken-core/parser/traits/#trait.Parser)
- [`ParserLanguage`](../../../rust/plissken-core/parser/traits/#enum.ParserLanguage)
- [`create_parser`](../../../rust/plissken-core/parser/traits/#fn.create_parser)
- [`parser_for_extension`](../../../rust/plissken-core/parser/traits/#fn.parser_for_extension)
