# hybrid_binary::internal <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #ff5722; color: white;">Rust</span>

Internal implementation details (not exposed to Python).

## Structs

### `struct Task` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span>

**Derives:** `Clone`

A task definition.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `description` | `Option < String >` |  |
| `dependencies` | `Vec < String >` |  |
| `is_async` | `bool` |  |

#### Methods

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span>

```rust
fn new (name : & str , description : Option < & str >) -> Self
```



##### `add_dependency` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span>

```rust
fn add_dependency (& mut self , name : & str)
```





### `struct TaskExecutor` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span>

Executes tasks with dependency resolution.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `max_parallel` | `usize` |  |
| `tasks` | `HashMap < String , (Task , PyObject) >` |  |

#### Methods

##### `new` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span>

```rust
fn new (max_parallel : usize) -> Result < Self , ExecutorError >
```



##### `register` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span>

```rust
fn register (& mut self , task : Task , handler : PyObject) -> Result < () , ExecutorError >
```



##### `run` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span>

```rust
fn run (& self , task_name : & str , dry_run : bool) -> Result < RunResult , ExecutorError >
```



##### `list_tasks` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span>

```rust
fn list_tasks (& self) -> Vec < String >
```





### `struct RunResult` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span>

Result of running tasks.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `success` | `bool` |  |
| `tasks_run` | `usize` |  |
| `duration` | `Duration` |  |
| `failed` | `Vec < String >` |  |



## Enums

### `enum ExecutorError` <span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: #4caf50; color: white;">pub</span>

Errors that can occur during execution.

#### Variants

- **`InvalidConfig`**
- **`DuplicateTask`**
- **`TaskNotFound`**
- **`CircularDependency`**
- **`ExecutionFailed`**



