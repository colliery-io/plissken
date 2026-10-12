---
title: "plissken-core::parser"
description: "Parsing infrastructure for Rust and Python source code"
sidebar:
  order: 10
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


Parsing infrastructure for Rust and Python source code

This module provides parsers for extracting documentation from source code.
The [`Parser`] trait provides a language-agnostic interface, while
[`RustParser`] and [`PythonParser`] provide concrete implementations.

