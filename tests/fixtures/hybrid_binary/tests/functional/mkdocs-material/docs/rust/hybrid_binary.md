# hybrid_binary <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #ff5722; color: white;">Rust</span>

A task runner library with Python bindings.

This module provides task execution and scheduling capabilities
exposed to Python via PyO3.

## Structs

### `struct PyTask` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span> <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #9c27b0; color: white;">Binding</span>

**Derives:** `Clone`

A task that can be executed by the runner.

Tasks have a name, optional description, and can be marked
as async or having dependencies.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `inner` | `internal :: Task` |  |

#### Methods <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #9c27b0; color: white;">Binding</span>

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span> <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #9c27b0; color: white;">Binding</span>

```rust
fn new (name : & str , description : Option < & str >) -> Self
```

Create a new task.

Args:
    name: The unique identifier for this task.
    description: Optional human-readable description.

Returns:
    A new Task instance.



##### `name` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span>

```rust
fn name (& self) -> & str
```

Get the task name.



##### `description` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span>

```rust
fn description (& self) -> Option < & str >
```

Get the task description.



##### `depends_on` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span>

```rust
fn depends_on (& mut self , task_name : & str)
```

Add a dependency on another task.

Args:
    task_name: Name of the task this depends on.



##### `set_async` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span>

```rust
fn set_async (& mut self , is_async : bool)
```

Mark this task as async.





### `struct PyRunner` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span> <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #9c27b0; color: white;">Binding</span>

The main task runner that executes tasks.

Handles dependency resolution, parallel execution, and
error reporting.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `executor` | `TaskExecutor` |  |

#### Methods <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #9c27b0; color: white;">Binding</span>

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span> <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #9c27b0; color: white;">Binding</span>

```rust
fn new (max_parallel : Option < usize >) -> PyResult < Self >
```

Create a new task runner.

Args:
    max_parallel: Maximum number of concurrent tasks (default: 4).



##### `register` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span>

```rust
fn register (& mut self , task : & PyTask , handler : PyObject) -> PyResult < () >
```

Register a task with the runner.

Args:
    task: The task to register.
    handler: Python callable to execute for this task.

Raises:
    ValueError: If a task with this name already exists.



##### `run` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span> <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #9c27b0; color: white;">Binding</span>

```rust
fn run (& self , task_name : & str , dry_run : bool) -> PyResult < PyRunResult >
```

Run a task and all its dependencies.

Args:
    task_name: Name of the task to run.
    dry_run: If True, only print what would be executed.

Returns:
    RunResult with execution details.

Raises:
    RuntimeError: If task execution fails.



##### `list_tasks` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span>

```rust
fn list_tasks (& self) -> Vec < String >
```

List all registered tasks.





### `struct PyRunResult` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span> <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #9c27b0; color: white;">Binding</span>

Result of a task run.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `inner` | `internal :: RunResult` |  |

#### Methods <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #9c27b0; color: white;">Binding</span>

##### `success` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span>

```rust
fn success (& self) -> bool
```

Whether all tasks succeeded.



##### `tasks_run` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span>

```rust
fn tasks_run (& self) -> usize
```

Number of tasks executed.



##### `duration_secs` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span>

```rust
fn duration_secs (& self) -> f64
```

Total execution time in seconds.



##### `failed` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span>

```rust
fn failed (& self) -> Vec < String >
```

List of failed task names, if any.





## Functions

### `hybrid_binary` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">private</span>

```rust
fn hybrid_binary (m : & Bound < '_ , PyModule >) -> PyResult < () >
```

Register the module with Python.



