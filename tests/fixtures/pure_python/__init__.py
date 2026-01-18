"""A task scheduling library.

Provides a simple, decorator-based API for defining and running
scheduled tasks with support for cron expressions and intervals.

Example:
    >>> from pure_python import Scheduler, every, cron
    >>>
    >>> scheduler = Scheduler()
    >>>
    >>> @scheduler.task(every(minutes=5))
    >>> def check_health():
    ...     print("Health check OK")
    >>>
    >>> @scheduler.task(cron("0 9 * * MON"))
    >>> def weekly_report():
    ...     print("Generating weekly report...")
    >>>
    >>> scheduler.run()
"""

from .scheduler import Scheduler
from .schedule import Schedule, every, cron, at
from .task import Task, TaskResult, TaskStatus
from .async_tasks import AsyncTaskResult, run_async, gather_tasks
from .protocols import Runnable, Schedulable, Serializable, Deserializable

#: Library version.
VERSION: str = "1.0.0"

#: Default maximum retries for failed tasks.
DEFAULT_MAX_RETRIES: int = 3

#: Default timeout in seconds for task execution.
DEFAULT_TIMEOUT_SECONDS: float = 300.0

__all__ = [
    # Core classes
    "Scheduler",
    "Schedule",
    "Task",
    "TaskResult",
    "TaskStatus",
    # Schedule builders
    "every",
    "cron",
    "at",
    # Async support
    "AsyncTaskResult",
    "run_async",
    "gather_tasks",
    # Protocols
    "Runnable",
    "Schedulable",
    "Serializable",
    "Deserializable",
    # Constants
    "VERSION",
    "DEFAULT_MAX_RETRIES",
    "DEFAULT_TIMEOUT_SECONDS",
]
