---
id: namespace-based-documentation
level: initiative
title: "Namespace-Based Documentation Structure"
short_code: "PLSKN-I-0004"
created_at: 2026-01-16T02:34:49.581573+00:00
updated_at: 2026-01-16T02:38:00.276398+00:00
parent: PLSKN-V-0001
blocked_by: []
archived: true

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: M
strategy_id: NULL
initiative_id: namespace-based-documentation
---

# Namespace-Based Documentation Structure Initiative

## Context

Current plissken output generates documentation organized by language (`python/`, `rust/`) but doesn't properly reflect namespace hierarchies or provide navigable structure that mirrors how developers think about their code.

Documentation should mirror the code structure and use full namespace paths for navigation, similar to how SciPy and other mature projects organize their API reference.

## Goals & Non-Goals

**Goals:**
- Documentation directory structure mirrors code structure
- Navigation shows full namespace paths (`package.submodule`, `crate::submodule`)
- Expandable nav entries showing symbols (classes, functions, etc.)
- PyO3 bindings appear in BOTH Python and Rust sections
- Module index pages show tables of all symbols
- Plissken auto-generates navigation config (no manual nav required)

**Non-Goals:**
- Manual nav configuration by users
- Separate documentation sites for Python/Rust

## Detailed Design

### Documentation Structure

Mirrors code structure with `python/` and `rust/` language separators:

```
docs/
  python/
    {package}/                    # Package directory
      index.md                    # "{package}" module docs
      submodule.md                # "{package}.submodule" (single-file module)
      subpackage/
        index.md                  # "{package}.subpackage" module docs
        nested.md                 # "{package}.subpackage.nested" module docs
  rust/
    {crate}/                      # Crate directory
      index.md                    # "{crate}" crate docs
      submodule.md                # "{crate}::submodule" (single-file module)
      nested/
        index.md                  # "{crate}::nested" module docs
```

### Navigation Structure

Auto-generated nav organized by language, showing full namespace paths:

```
Project Name
─────────────

Python
  ▸ package
      SomeClass
      some_function
  ▸ package.submodule
      AnotherClass
  ▸ package.subpackage
      NestedClass

Rust
  ▸ crate
      Runner
      Task
  ▸ crate::submodule
      PyRunner
  ▸ crate::nested
      InternalStruct
```

Each namespace entry is expandable to show symbols within that module.

### PyO3 Bindings

Bindings appear in both sections:
- **Python section**: Documented as Python API (Python-style signatures, Python types)
- **Rust section**: Documented as Rust implementation (Rust signatures, source code)

Cross-links connect the two views.

### Module Index Pages

When clicking a module namespace, the page shows:
- Module docstring/description
- Tables of symbols organized by type (Classes, Functions, Constants, etc.)
- Each symbol links to its detailed documentation

## Test Fixture

Create a minimal test project with:
- Python package named `pysnake` (different from crate name)
- Rust crate named `rustscale` (different from package name)
- One of each symbol type (class, function, enum, constant)
- Nested structures (single-file modules, subpackages/submodules)

## Alternatives Considered

1. **Additional MkDocs plugin** - Rejected; plissken should generate nav directly
2. **Flat namespace list** - Rejected; expandable symbols provide better UX
3. **Single merged view** - Rejected; Python and Rust audiences need different perspectives

## Implementation Plan

1. Create minimal test fixture (`pysnake` / `rustscale`)
2. Update directory structure generation to mirror code
3. Implement nav generation with full namespace paths
4. Add expandable symbol entries to nav
5. Update index page rendering with symbol tables
6. Verify with MkDocs Material rendering