# parser <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #ff5722; color: white;">Rust</span>


Parsing infrastructure for Rust and Python source code

This module provides parsers for extracting documentation from source code.
The [`Parser`] trait provides a language-agnostic interface, while
[`RustParser`] and [`PythonParser`] provide concrete implementations.

