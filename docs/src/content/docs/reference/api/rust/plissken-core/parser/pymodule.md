---
title: "plissken-core::parser::pymodule"
description: "What PyO3 module functions register"
editUrl: false
sidebar:
  order: 12
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

What PyO3 module functions register

A `#[pymodule]` function names a Python module and adds its classes and
functions:

```text
#[pymodule]
fn cloaca(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<bindings::runner::PyDefaultRunner>()?;
    m.add_function(wrap_pyfunction!(loader::py_var, m)?)?;
    register_authoring(m)?;          // a helper adds more
    let sub = PyModule::new(m.py(), "events")?;
    sub.add_class::<Event>()?;
    m.add_submodule(&sub)?;          // the submodule cloaca.events
    Ok(())
}
```

This module reads such a function, and any helper that gets a module as
a parameter, into a [`PyModuleRegistrar`](../../../../rust/plissken-core/model/#struct.PyModuleRegistrar). The synthesis of the Python
API follows the calls from each `#[pymodule]` function.
