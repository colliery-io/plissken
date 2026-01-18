"""Timing utilities.

Functions for measuring and formatting time durations.
"""

from __future__ import annotations

import time
from contextlib import contextmanager
from dataclasses import dataclass
from enum import Enum
from typing import Any, Callable, Generator, TypeVar

T = TypeVar("T")

#: Default precision for duration formatting.
DEFAULT_PRECISION: int = 2

#: Maximum duration to display in seconds before switching to minutes.
SECONDS_THRESHOLD: float = 60.0


class TimeUnit(Enum):
    """Units of time for duration formatting."""

    NANOSECONDS = "ns"
    """Nanoseconds (10^-9 seconds)."""

    MICROSECONDS = "us"
    """Microseconds (10^-6 seconds)."""

    MILLISECONDS = "ms"
    """Milliseconds (10^-3 seconds)."""

    SECONDS = "s"
    """Seconds."""

    MINUTES = "m"
    """Minutes."""

    HOURS = "h"
    """Hours."""


@dataclass
class TimingResult:
    """Result of a timing measurement.

    Attributes:
        elapsed_seconds: The measured duration in seconds.
        result: The return value of the timed function.
    """

    elapsed_seconds: float
    result: Any = None

    @property
    def elapsed_ms(self) -> float:
        """Get elapsed time in milliseconds."""
        return self.elapsed_seconds * 1000


@contextmanager
def measure_time() -> Generator[TimingResult, None, None]:
    """Context manager to measure execution time.

    Yields:
        A TimingResult that will be populated with the duration.

    Example:
        >>> with measure_time() as timing:
        ...     do_work()
        >>> print(f"Took {timing.elapsed_seconds:.2f}s")
    """
    result = TimingResult(elapsed_seconds=0.0)
    start = time.perf_counter()
    try:
        yield result
    finally:
        result.elapsed_seconds = time.perf_counter() - start


def timed(func: Callable[..., T]) -> Callable[..., tuple[T, float]]:
    """Decorator to measure function execution time.

    Args:
        func: The function to time.

    Returns:
        A wrapper that returns (result, elapsed_seconds).

    Example:
        >>> @timed
        ... def slow_function():
        ...     time.sleep(1)
        ...     return "done"
        >>> result, elapsed = slow_function()
    """

    def wrapper(*args: Any, **kwargs: Any) -> tuple[T, float]:
        start = time.perf_counter()
        result = func(*args, **kwargs)
        elapsed = time.perf_counter() - start
        return result, elapsed

    wrapper.__name__ = func.__name__
    wrapper.__doc__ = func.__doc__
    return wrapper


def format_duration(
    seconds: float,
    *,
    unit: TimeUnit | None = None,
    precision: int = DEFAULT_PRECISION,
) -> str:
    """Format a duration as a human-readable string.

    Args:
        seconds: The duration in seconds.
        unit: Force a specific unit, or auto-detect if None.
        precision: Number of decimal places.

    Returns:
        Formatted duration string like "1.50s" or "250ms".

    Example:
        >>> format_duration(0.5)
        '500.00ms'
        >>> format_duration(90.5)
        '1.51m'
        >>> format_duration(0.001, unit=TimeUnit.MICROSECONDS)
        '1000.00us'
    """
    if unit is None:
        if seconds < 0.001:
            unit = TimeUnit.MICROSECONDS
        elif seconds < 1.0:
            unit = TimeUnit.MILLISECONDS
        elif seconds < SECONDS_THRESHOLD:
            unit = TimeUnit.SECONDS
        elif seconds < 3600:
            unit = TimeUnit.MINUTES
        else:
            unit = TimeUnit.HOURS

    conversions = {
        TimeUnit.NANOSECONDS: 1e9,
        TimeUnit.MICROSECONDS: 1e6,
        TimeUnit.MILLISECONDS: 1e3,
        TimeUnit.SECONDS: 1,
        TimeUnit.MINUTES: 1 / 60,
        TimeUnit.HOURS: 1 / 3600,
    }

    value = seconds * conversions[unit]
    return f"{value:.{precision}f}{unit.value}"
