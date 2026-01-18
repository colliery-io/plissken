# hybrid_binary::decorators <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #ff5722; color: white;">Rust</span>

Python decorators for task registration.

Provides a decorator-based API for registering tasks:

```python
from hybrid_binary import task, Runner

runner = Runner()

@task(runner, name="build", description="Build the project")
def build():
    print("Building...")
```

## Structs

### `struct TaskDecorator` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span> <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #9c27b0; color: white;">Binding</span>

Decorator state that captures registration info.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `runner` | `Py < PyRunner >` |  |
| `name` | `String` |  |
| `description` | `Option < String >` |  |
| `depends` | `Vec < String >` |  |

#### Methods <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #9c27b0; color: white;">Binding</span>

##### `__call__` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span>

```rust
fn __call__ (& self , py : Python < '_ > , func : PyObject) -> PyResult < PyObject >
```

Called when the decorator is applied to a function.





## Functions

### `task` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span> <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #9c27b0; color: white;">Binding</span>

```rust
fn task (runner : Py < PyRunner > , kwargs : Option < & Bound < '_ , PyDict > >) -> PyResult < TaskDecorator >
```

Create a task decorator.

This is the main entry point for the decorator API.

Args:
    runner: The Runner instance to register with.
    name: Unique name for this task.
    description: Human-readable description.
    depends: List of task names this depends on.

Returns:
    A decorator that registers the function as a task.

Example:
    ```python
    @task(runner, name="test", depends=["build"])
    def run_tests():
        subprocess.run(["pytest"])
    ```



### `register` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span>

```rust
fn register (m : & Bound < '_ , PyModule >) -> PyResult < () >
```

Register decorator functions with the module.



