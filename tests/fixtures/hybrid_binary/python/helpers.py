"""Pure Python helpers for the task runner.

This module provides convenience utilities that wrap the core
Rust functionality with a more Pythonic API.
"""

from typing import Callable, List, Optional
from hybrid_binary import Runner, Task


class TaskBuilder:
    """Fluent builder for creating tasks.

    Provides a chainable API for task configuration:

        task = (TaskBuilder("build")
            .description("Build the project")
            .depends_on("setup")
            .async_()
            .build())

    Attributes:
        name: The task name being built.
    """

    def __init__(self, name: str):
        """Create a new TaskBuilder.

        Args:
            name: Unique identifier for the task.
        """
        self._name = name
        self._description: Optional[str] = None
        self._dependencies: List[str] = []
        self._is_async: bool = False

    @property
    def name(self) -> str:
        """The task name."""
        return self._name

    def description(self, desc: str) -> "TaskBuilder":
        """Set the task description.

        Args:
            desc: Human-readable description of what the task does.

        Returns:
            Self for chaining.
        """
        self._description = desc
        return self

    def depends_on(self, *task_names: str) -> "TaskBuilder":
        """Add dependencies on other tasks.

        Args:
            *task_names: Names of tasks this depends on.

        Returns:
            Self for chaining.
        """
        self._dependencies.extend(task_names)
        return self

    def async_(self) -> "TaskBuilder":
        """Mark this task as async-capable.

        Returns:
            Self for chaining.
        """
        self._is_async = True
        return self

    def build(self) -> Task:
        """Build the configured Task.

        Returns:
            A new Task instance with the configured settings.
        """
        task = Task(self._name, self._description)
        for dep in self._dependencies:
            task.depends_on(dep)
        if self._is_async:
            task.set_async(True)
        return task


def run_task(
    runner: Runner,
    name: str,
    handler: Callable,
    *,
    description: Optional[str] = None,
    depends: Optional[List[str]] = None,
    dry_run: bool = False,
):
    """Convenience function to register and run a task in one call.

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
    """
    task = TaskBuilder(name)
    if description:
        task.description(description)
    if depends:
        task.depends_on(*depends)

    runner.register(task.build(), handler)
    return runner.run(name, dry_run=dry_run)
