"""Task scheduler implementation."""

from __future__ import annotations

import logging
import signal
import time
from datetime import datetime
from typing import Callable, Dict, List, Optional

from .schedule import Schedule
from .task import Task, TaskResult, TaskStatus

logger = logging.getLogger(__name__)


class Scheduler:
    """A task scheduler that runs tasks on configured schedules.

    The scheduler supports both interval-based and cron-based scheduling.
    Tasks are registered using the `@scheduler.task()` decorator.

    Attributes:
        tasks: Dictionary of registered tasks by name.
        running: Whether the scheduler is currently running.

    Example:
        >>> scheduler = Scheduler()
        >>> @scheduler.task(every(seconds=30))
        ... def heartbeat():
        ...     print("alive")
        >>> scheduler.run()
    """

    def __init__(self, timezone: Optional[str] = None):
        """Initialize a new scheduler.

        Args:
            timezone: IANA timezone name (e.g., "America/New_York").
                     If None, uses the system timezone.
        """
        self._tasks: Dict[str, Task] = {}
        self._running: bool = False
        self._timezone = timezone

    @property
    def tasks(self) -> Dict[str, Task]:
        """Get all registered tasks."""
        return self._tasks.copy()

    @property
    def running(self) -> bool:
        """Check if the scheduler is running."""
        return self._running

    def task(
        self,
        schedule: Schedule,
        *,
        name: Optional[str] = None,
        max_retries: int = 0,
        timeout_seconds: Optional[float] = None,
    ) -> Callable[[Callable], Callable]:
        """Decorator to register a function as a scheduled task.

        Args:
            schedule: When to run the task (use `every()` or `cron()`).
            name: Optional task name. Defaults to the function name.
            max_retries: Number of retry attempts on failure.
            timeout_seconds: Maximum execution time before killing the task.

        Returns:
            A decorator that registers the function.

        Raises:
            ValueError: If a task with this name already exists.

        Example:
            >>> @scheduler.task(every(hours=1), max_retries=3)
            ... def sync_data():
            ...     external_api.sync()
        """

        def decorator(func: Callable) -> Callable:
            task_name = name or func.__name__

            if task_name in self._tasks:
                raise ValueError(f"Task '{task_name}' already registered")

            task = Task(
                name=task_name,
                func=func,
                schedule=schedule,
                max_retries=max_retries,
                timeout_seconds=timeout_seconds,
            )
            self._tasks[task_name] = task

            return func

        return decorator

    def run(self, *, blocking: bool = True) -> None:
        """Start the scheduler.

        Args:
            blocking: If True (default), blocks until stopped.
                     If False, runs in a background thread.

        Raises:
            RuntimeError: If the scheduler is already running.
        """
        if self._running:
            raise RuntimeError("Scheduler is already running")

        self._running = True
        logger.info("Scheduler started with %d tasks", len(self._tasks))

        if blocking:
            self._setup_signal_handlers()
            self._run_loop()
        else:
            import threading

            thread = threading.Thread(target=self._run_loop, daemon=True)
            thread.start()

    def stop(self) -> None:
        """Stop the scheduler gracefully."""
        logger.info("Stopping scheduler...")
        self._running = False

    def run_task(self, name: str) -> TaskResult:
        """Manually run a task by name.

        Args:
            name: The task name to run.

        Returns:
            TaskResult with execution details.

        Raises:
            KeyError: If no task with this name exists.
        """
        if name not in self._tasks:
            raise KeyError(f"Unknown task: {name}")

        return self._tasks[name].run()

    def _run_loop(self) -> None:
        """Main scheduler loop."""
        while self._running:
            now = datetime.now()

            for task in self._tasks.values():
                if task.should_run(now):
                    try:
                        result = task.run()
                        if result.status == TaskStatus.FAILED:
                            logger.error(
                                "Task %s failed: %s", task.name, result.error
                            )
                    except Exception as e:
                        logger.exception("Unexpected error running task %s: %s", task.name, e)

            time.sleep(1)

    def _setup_signal_handlers(self) -> None:
        """Set up signal handlers for graceful shutdown."""
        signal.signal(signal.SIGINT, lambda *_: self.stop())
        signal.signal(signal.SIGTERM, lambda *_: self.stop())
