"""Task definition and execution."""

from __future__ import annotations

import time
import traceback
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum, auto
from typing import Any, Callable, Optional

from .schedule import Schedule


class TaskStatus(Enum):
    """Status of a task execution."""

    PENDING = auto()
    """Task has not yet run."""

    RUNNING = auto()
    """Task is currently executing."""

    SUCCESS = auto()
    """Task completed successfully."""

    FAILED = auto()
    """Task failed with an error."""

    TIMEOUT = auto()
    """Task exceeded its timeout."""

    SKIPPED = auto()
    """Task was skipped (e.g., already running)."""


@dataclass
class TaskResult:
    """Result of a task execution.

    Attributes:
        status: The execution status.
        started_at: When execution started.
        ended_at: When execution ended.
        duration_seconds: How long execution took.
        return_value: The function's return value, if successful.
        error: Error message, if failed.
        traceback: Full traceback, if failed.
    """

    status: TaskStatus
    started_at: datetime
    ended_at: datetime
    duration_seconds: float
    return_value: Any = None
    error: Optional[str] = None
    traceback: Optional[str] = None

    @property
    def success(self) -> bool:
        """Check if the task succeeded."""
        return self.status == TaskStatus.SUCCESS


@dataclass
class Task:
    """A scheduled task.

    Tasks wrap a callable function with scheduling and execution
    configuration.

    Attributes:
        name: Unique identifier for this task.
        func: The function to execute.
        schedule: When to run the task.
        max_retries: Number of retry attempts on failure.
        timeout_seconds: Maximum execution time.
        last_run: When the task last ran.
        last_result: Result of the last execution.
    """

    name: str
    func: Callable
    schedule: Schedule
    max_retries: int = 0
    timeout_seconds: Optional[float] = None
    last_run: Optional[datetime] = field(default=None, repr=False)
    last_result: Optional[TaskResult] = field(default=None, repr=False)

    def should_run(self, now: datetime) -> bool:
        """Check if this task should run now.

        Args:
            now: The current datetime.

        Returns:
            True if the task should be executed.
        """
        return self.schedule.should_run(now, self.last_run)

    def run(self) -> TaskResult:
        """Execute the task.

        Handles retries and timeout. Updates `last_run` and `last_result`.

        Returns:
            TaskResult with execution details.
        """
        started_at = datetime.now()
        attempts = 0
        last_error = None
        last_tb = None

        while attempts <= self.max_retries:
            try:
                return_value = self._execute_with_timeout()
                ended_at = datetime.now()

                result = TaskResult(
                    status=TaskStatus.SUCCESS,
                    started_at=started_at,
                    ended_at=ended_at,
                    duration_seconds=(ended_at - started_at).total_seconds(),
                    return_value=return_value,
                )
                self.last_run = started_at
                self.last_result = result
                return result

            except TimeoutError:
                ended_at = datetime.now()
                result = TaskResult(
                    status=TaskStatus.TIMEOUT,
                    started_at=started_at,
                    ended_at=ended_at,
                    duration_seconds=(ended_at - started_at).total_seconds(),
                    error=f"Task exceeded timeout of {self.timeout_seconds}s",
                )
                self.last_run = started_at
                self.last_result = result
                return result

            except Exception as e:
                last_error = str(e)
                last_tb = traceback.format_exc()
                attempts += 1

        ended_at = datetime.now()
        result = TaskResult(
            status=TaskStatus.FAILED,
            started_at=started_at,
            ended_at=ended_at,
            duration_seconds=(ended_at - started_at).total_seconds(),
            error=last_error,
            traceback=last_tb,
        )
        self.last_run = started_at
        self.last_result = result
        return result

    def _execute_with_timeout(self) -> Any:
        """Execute the function with optional timeout."""
        if self.timeout_seconds is None:
            return self.func()

        # Simplified: real implementation would use threading/signal
        start = time.monotonic()
        result = self.func()
        elapsed = time.monotonic() - start

        if elapsed > self.timeout_seconds:
            raise TimeoutError(f"Execution took {elapsed:.2f}s")

        return result
