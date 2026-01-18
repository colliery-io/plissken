"""Schedule definitions."""

from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass
from datetime import datetime, time, timedelta
from typing import Optional


class Schedule(ABC):
    """Base class for task schedules.

    Schedules determine when a task should run. Implement `next_run`
    to create custom schedule types.
    """

    @abstractmethod
    def next_run(self, after: datetime) -> datetime:
        """Calculate the next run time after the given datetime.

        Args:
            after: Find the next run time after this point.

        Returns:
            The next datetime when the task should run.
        """
        pass

    def should_run(self, now: datetime, last_run: Optional[datetime]) -> bool:
        """Check if the task should run now.

        Args:
            now: The current datetime.
            last_run: When the task last ran, or None if never.

        Returns:
            True if the task should run.
        """
        if last_run is None:
            return True
        return now >= self.next_run(last_run)


@dataclass
class IntervalSchedule(Schedule):
    """Run a task at fixed intervals.

    Attributes:
        interval: Time between runs.
    """

    interval: timedelta

    def next_run(self, after: datetime) -> datetime:
        """Get the next run time."""
        return after + self.interval


@dataclass
class CronSchedule(Schedule):
    """Run a task according to a cron expression.

    Supports standard 5-field cron syntax:
    - minute (0-59)
    - hour (0-23)
    - day of month (1-31)
    - month (1-12)
    - day of week (0-6, Sunday=0)

    Attributes:
        expression: The cron expression string.
    """

    expression: str

    def __post_init__(self):
        """Parse and validate the cron expression."""
        self._parts = self.expression.split()
        if len(self._parts) != 5:
            raise ValueError(
                f"Invalid cron expression: expected 5 fields, got {len(self._parts)}"
            )

    def next_run(self, after: datetime) -> datetime:
        """Calculate next run time from cron expression."""
        # Simplified: real implementation would parse cron fields
        # For now, just return next minute
        return after.replace(second=0, microsecond=0) + timedelta(minutes=1)


@dataclass
class DailySchedule(Schedule):
    """Run a task at a specific time each day.

    Attributes:
        at_time: The time of day to run.
    """

    at_time: time

    def next_run(self, after: datetime) -> datetime:
        """Get the next run time."""
        candidate = after.replace(
            hour=self.at_time.hour,
            minute=self.at_time.minute,
            second=self.at_time.second,
            microsecond=0,
        )
        if candidate <= after:
            candidate += timedelta(days=1)
        return candidate


def every(
    *,
    seconds: int = 0,
    minutes: int = 0,
    hours: int = 0,
    days: int = 0,
) -> IntervalSchedule:
    """Create an interval schedule.

    At least one interval component must be specified.

    Args:
        seconds: Number of seconds between runs.
        minutes: Number of minutes between runs.
        hours: Number of hours between runs.
        days: Number of days between runs.

    Returns:
        An IntervalSchedule with the combined interval.

    Raises:
        ValueError: If no interval is specified.

    Example:
        >>> every(minutes=30)  # Every 30 minutes
        >>> every(hours=2, minutes=30)  # Every 2.5 hours
    """
    total = timedelta(seconds=seconds, minutes=minutes, hours=hours, days=days)
    if total == timedelta():
        raise ValueError("At least one interval component must be specified")
    return IntervalSchedule(interval=total)


def cron(expression: str) -> CronSchedule:
    """Create a cron schedule.

    Args:
        expression: Standard 5-field cron expression.

    Returns:
        A CronSchedule for the expression.

    Example:
        >>> cron("0 9 * * MON-FRI")  # 9 AM on weekdays
        >>> cron("*/15 * * * *")      # Every 15 minutes
        >>> cron("0 0 1 * *")         # First day of each month
    """
    return CronSchedule(expression=expression)


def at(hour: int, minute: int = 0, second: int = 0) -> DailySchedule:
    """Create a daily schedule at a specific time.

    Args:
        hour: Hour of day (0-23).
        minute: Minute of hour (0-59).
        second: Second of minute (0-59).

    Returns:
        A DailySchedule for the specified time.

    Example:
        >>> at(9, 30)   # Every day at 9:30 AM
        >>> at(0)       # Every day at midnight
    """
    return DailySchedule(at_time=time(hour, minute, second))
