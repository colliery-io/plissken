"""Async task execution support.

Provides async/await versions of task execution for use with
asyncio-based applications.
"""

from __future__ import annotations

import asyncio
from dataclasses import dataclass
from datetime import datetime
from typing import Any, Awaitable, Callable, Optional, TypeVar

from .task import TaskResult, TaskStatus

T = TypeVar("T")


@dataclass
class AsyncTaskResult(TaskResult):
    """Result of an async task execution.

    Extends TaskResult with async-specific information.

    Attributes:
        cancelled: Whether the task was cancelled.
        timeout_remaining: Remaining timeout when completed.
    """

    cancelled: bool = False
    timeout_remaining: Optional[float] = None


async def run_async(
    func: Callable[[], Awaitable[T]],
    *,
    timeout_seconds: Optional[float] = None,
    max_retries: int = 0,
) -> AsyncTaskResult:
    """Run an async function with timeout and retry support.

    Args:
        func: An async callable to execute.
        timeout_seconds: Maximum time to wait for completion.
        max_retries: Number of retry attempts on failure.

    Returns:
        AsyncTaskResult with execution details.

    Example:
        >>> async def fetch_data():
        ...     async with aiohttp.ClientSession() as session:
        ...         return await session.get(url)
        >>> result = await run_async(fetch_data, timeout_seconds=30)
    """
    started_at = datetime.now()
    attempts = 0
    last_error = None

    while attempts <= max_retries:
        try:
            if timeout_seconds is not None:
                return_value = await asyncio.wait_for(
                    func(), timeout=timeout_seconds
                )
            else:
                return_value = await func()

            ended_at = datetime.now()
            return AsyncTaskResult(
                status=TaskStatus.SUCCESS,
                started_at=started_at,
                ended_at=ended_at,
                duration_seconds=(ended_at - started_at).total_seconds(),
                return_value=return_value,
            )

        except asyncio.TimeoutError:
            ended_at = datetime.now()
            return AsyncTaskResult(
                status=TaskStatus.TIMEOUT,
                started_at=started_at,
                ended_at=ended_at,
                duration_seconds=(ended_at - started_at).total_seconds(),
                error=f"Task exceeded timeout of {timeout_seconds}s",
            )

        except asyncio.CancelledError:
            ended_at = datetime.now()
            return AsyncTaskResult(
                status=TaskStatus.SKIPPED,
                started_at=started_at,
                ended_at=ended_at,
                duration_seconds=(ended_at - started_at).total_seconds(),
                cancelled=True,
            )

        except Exception as e:
            last_error = str(e)
            attempts += 1

    ended_at = datetime.now()
    return AsyncTaskResult(
        status=TaskStatus.FAILED,
        started_at=started_at,
        ended_at=ended_at,
        duration_seconds=(ended_at - started_at).total_seconds(),
        error=last_error,
    )


async def gather_tasks(
    *funcs: Callable[[], Awaitable[Any]],
    return_exceptions: bool = False,
) -> list[AsyncTaskResult]:
    """Run multiple async tasks concurrently.

    Args:
        *funcs: Async callables to execute concurrently.
        return_exceptions: If True, exceptions are returned as results.

    Returns:
        List of AsyncTaskResult for each task.

    Example:
        >>> results = await gather_tasks(
        ...     fetch_users,
        ...     fetch_orders,
        ...     fetch_products,
        ... )
    """
    tasks = [run_async(func) for func in funcs]
    return await asyncio.gather(*tasks, return_exceptions=return_exceptions)
