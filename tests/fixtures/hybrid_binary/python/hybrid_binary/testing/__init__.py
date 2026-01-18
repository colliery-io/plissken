"""Testing utilities for hybrid_binary task code.

This module provides mock objects, fixtures, and assertion helpers
for testing code that uses the hybrid_binary task runner.

Example:
    >>> from hybrid_binary.testing import MockRunner, assert_task_ran
    >>> runner = MockRunner()
    >>> # ... register and run tasks ...
    >>> assert_task_ran(runner, "build")
"""

from typing import Any, Callable, Dict, List, Optional


class MockTask:
    """A mock Task for testing without the Rust backend.

    Simulates Task behavior for unit testing task registration
    and dependency logic.

    Attributes:
        name: The task name.
        description: Optional task description.
        dependencies: List of dependency task names.
        is_async: Whether the task is async.
    """

    def __init__(self, name: str, description: Optional[str] = None):
        """Create a mock task.

        Args:
            name: Unique task identifier.
            description: Optional human-readable description.
        """
        self.name = name
        self.description = description
        self.dependencies: List[str] = []
        self.is_async = False

    def depends_on(self, task_name: str) -> None:
        """Add a dependency.

        Args:
            task_name: Name of the dependency task.
        """
        self.dependencies.append(task_name)

    def set_async(self, is_async: bool) -> None:
        """Set async flag.

        Args:
            is_async: Whether the task runs asynchronously.
        """
        self.is_async = is_async


class MockRunResult:
    """A mock RunResult for testing.

    Attributes:
        success: Whether execution succeeded.
        tasks_run: Number of tasks executed.
        duration_secs: Simulated duration.
        failed: List of failed task names.
    """

    def __init__(
        self,
        success: bool = True,
        tasks_run: int = 1,
        duration_secs: float = 0.1,
        failed: Optional[List[str]] = None,
    ):
        """Create a mock result.

        Args:
            success: Whether all tasks succeeded.
            tasks_run: Number of tasks that ran.
            duration_secs: Simulated execution time.
            failed: List of failed task names.
        """
        self.success = success
        self.tasks_run = tasks_run
        self.duration_secs = duration_secs
        self.failed = failed or []


class MockRunner:
    """A mock Runner for testing task logic.

    Records all registrations and run calls for later assertion.

    Attributes:
        registered_tasks: Dict mapping task names to (task, handler) tuples.
        run_history: List of (task_name, dry_run) tuples for each run call.
    """

    def __init__(self):
        """Create a new mock runner."""
        self.registered_tasks: Dict[str, tuple] = {}
        self.run_history: List[tuple] = []
        self._next_result: Optional[MockRunResult] = None

    def register(self, task: MockTask, handler: Callable) -> None:
        """Register a task with its handler.

        Args:
            task: The task to register.
            handler: The callable to execute.

        Raises:
            ValueError: If task name already registered.
        """
        if task.name in self.registered_tasks:
            raise ValueError(f"Task '{task.name}' already registered")
        self.registered_tasks[task.name] = (task, handler)

    def run(self, task_name: str, dry_run: bool = False) -> MockRunResult:
        """Run a task (mock execution).

        Args:
            task_name: Name of task to run.
            dry_run: Whether to simulate without executing.

        Returns:
            MockRunResult with execution details.

        Raises:
            RuntimeError: If task not found.
        """
        if task_name not in self.registered_tasks:
            raise RuntimeError(f"Task '{task_name}' not found")

        self.run_history.append((task_name, dry_run))

        if self._next_result:
            result = self._next_result
            self._next_result = None
            return result

        return MockRunResult(success=True, tasks_run=1)

    def list_tasks(self) -> List[str]:
        """List registered task names.

        Returns:
            List of task names.
        """
        return list(self.registered_tasks.keys())

    def set_next_result(self, result: MockRunResult) -> None:
        """Set the result for the next run() call.

        Useful for testing error handling.

        Args:
            result: The MockRunResult to return.
        """
        self._next_result = result


def assert_task_ran(runner: MockRunner, task_name: str) -> None:
    """Assert that a task was run.

    Args:
        runner: The MockRunner to check.
        task_name: Expected task name.

    Raises:
        AssertionError: If task was not run.
    """
    ran_tasks = [name for name, _ in runner.run_history]
    assert task_name in ran_tasks, f"Task '{task_name}' was not run. Ran: {ran_tasks}"


def assert_task_registered(runner: MockRunner, task_name: str) -> None:
    """Assert that a task was registered.

    Args:
        runner: The MockRunner to check.
        task_name: Expected task name.

    Raises:
        AssertionError: If task was not registered.
    """
    assert task_name in runner.registered_tasks, (
        f"Task '{task_name}' not registered. Registered: {list(runner.registered_tasks.keys())}"
    )
