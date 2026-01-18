# /Users/dstorey/Desktop/plissken/tests/fixtures/hybrid_binary/python/helpers.py <span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-primary-fg-color); color: white;">🐍 Python</span>

Pure Python helpers for the task runner.

This module provides convenience utilities that wrap the core
Rust functionality with a more Pythonic API.


## Classes

### `class TaskBuilder`

Fluent builder for creating tasks.

    Provides a chainable API for task configuration:

        task = (TaskBuilder("build")
            .description("Build the project")
            .depends_on("setup")
            .async_()
            .build())

    Attributes:
        name: The task name being built.
    

#### Methods

##### `__init__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__init__</span>(self, name: str)</code>
</div>

Create a new TaskBuilder.

        Args:
            name: Unique identifier for the task.
        



##### `name` <span class="plissken-badge plissken-badge-property" style="display: inline-block; padding: 0.125em 0.5em; font-size: 0.75em; font-weight: 600; border-radius: 0.25em; background: var(--md-default-fg-color--light); color: white;">property</span>

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">name</span>(self) -> <span style="color: var(--md-default-fg-color--light);">str</span></code>
</div>

The task name.



##### `description`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">description</span>(self, desc: str) -> <span style="color: var(--md-default-fg-color--light);">&quot;TaskBuilder&quot;</span></code>
</div>

Set the task description.

        Args:
            desc: Human-readable description of what the task does.

        Returns:
            Self for chaining.
        



##### `depends_on`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">depends_on</span>(self, *task_names: str) -> <span style="color: var(--md-default-fg-color--light);">&quot;TaskBuilder&quot;</span></code>
</div>

Add dependencies on other tasks.

        Args:
            *task_names: Names of tasks this depends on.

        Returns:
            Self for chaining.
        



##### `async_`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">async_</span>(self) -> <span style="color: var(--md-default-fg-color--light);">&quot;TaskBuilder&quot;</span></code>
</div>

Mark this task as async-capable.

        Returns:
            Self for chaining.
        



##### `build`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">build</span>(self) -> <span style="color: var(--md-default-fg-color--light);">Task</span></code>
</div>

Build the configured Task.

        Returns:
            A new Task instance with the configured settings.
        





## Functions

### `run_task`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">run_task</span>(runner: Runner, name: str, handler: Callable, description: Optional[str] = None, depends: Optional[List[str]] = None, dry_run: bool = False)</code>
</div>

Convenience function to register and run a task in one call.

    This is useful for simple one-off tasks that don't need to be
    referenced by other tasks.

    Args:
        runner: The Runner instance to use.
        name: Unique name for the task.
        handler: Function to execute.
        description: Optional description.
        depends: Optional list of dependencies.
        dry_run: If True, don't actually execute.

    Returns:
        RunResult from the execution.

    Raises:
        RuntimeError: If execution fails.

    Example:
        >>> result = run_task(runner, "quick", lambda: print("done"))
        >>> print(f"Ran {result.tasks_run} tasks")
    



