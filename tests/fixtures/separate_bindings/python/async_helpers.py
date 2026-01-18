"""Async helpers for the data pipeline.

Provides async/await versions of pipeline operations for use with
asyncio-based applications.
"""

from __future__ import annotations

import asyncio
from dataclasses import dataclass, field
from datetime import datetime
from typing import (
    Any,
    AsyncIterator,
    Awaitable,
    Callable,
    Generic,
    List,
    Optional,
    TypeVar,
)

from separate_bindings import DataBatch, Pipeline, PipelineResult

T = TypeVar("T")


@dataclass
class AsyncPipelineResult:
    """Result of an async pipeline run.

    Attributes:
        result: The underlying PipelineResult.
        started_at: When execution started.
        completed_at: When execution completed.
        was_cancelled: Whether the run was cancelled.
    """

    result: Optional[PipelineResult]
    started_at: datetime
    completed_at: datetime
    was_cancelled: bool = False

    @property
    def success(self) -> bool:
        """Check if the run was successful."""
        return self.result is not None and not self.was_cancelled

    @property
    def duration_secs(self) -> float:
        """Get the total duration in seconds."""
        return (self.completed_at - self.started_at).total_seconds()


async def run_async(
    pipeline: Pipeline,
    batch: DataBatch,
    *,
    timeout: Optional[float] = None,
) -> AsyncPipelineResult:
    """Run a pipeline asynchronously.

    Wraps synchronous pipeline execution in an async context,
    allowing integration with async applications.

    Args:
        pipeline: The pipeline to run.
        batch: Input data batch.
        timeout: Optional timeout in seconds.

    Returns:
        AsyncPipelineResult with execution details.

    Example:
        >>> async def main():
        ...     result = await run_async(pipeline, batch, timeout=30.0)
        ...     if result.success:
        ...         print(f"Processed in {result.duration_secs:.2f}s")
    """
    started_at = datetime.now()

    try:
        loop = asyncio.get_event_loop()

        if timeout is not None:
            result = await asyncio.wait_for(
                loop.run_in_executor(None, pipeline.run, batch),
                timeout=timeout,
            )
        else:
            result = await loop.run_in_executor(None, pipeline.run, batch)

        return AsyncPipelineResult(
            result=result,
            started_at=started_at,
            completed_at=datetime.now(),
        )

    except asyncio.TimeoutError:
        return AsyncPipelineResult(
            result=None,
            started_at=started_at,
            completed_at=datetime.now(),
            was_cancelled=True,
        )

    except asyncio.CancelledError:
        return AsyncPipelineResult(
            result=None,
            started_at=started_at,
            completed_at=datetime.now(),
            was_cancelled=True,
        )


async def run_concurrent(
    pipeline: Pipeline,
    batches: List[DataBatch],
    *,
    max_concurrency: int = 4,
) -> List[AsyncPipelineResult]:
    """Run a pipeline on multiple batches concurrently.

    Uses a semaphore to limit concurrent executions.

    Args:
        pipeline: The pipeline to run.
        batches: List of input batches.
        max_concurrency: Maximum concurrent pipeline runs.

    Returns:
        List of results in the same order as input batches.

    Example:
        >>> results = await run_concurrent(pipeline, batches, max_concurrency=8)
        >>> successful = sum(1 for r in results if r.success)
    """
    semaphore = asyncio.Semaphore(max_concurrency)

    async def run_with_semaphore(batch: DataBatch) -> AsyncPipelineResult:
        async with semaphore:
            return await run_async(pipeline, batch)

    return await asyncio.gather(*[run_with_semaphore(b) for b in batches])


@dataclass
class StreamContext(Generic[T]):
    """Context for async stream processing.

    Attributes:
        value: The current value being processed.
        index: The index in the stream (0-based).
        metadata: Optional metadata dict.
    """

    value: T
    index: int
    metadata: dict[str, Any] = field(default_factory=dict)


async def stream_batches(
    source: Callable[[], Awaitable[Optional[DataBatch]]],
    *,
    max_batches: Optional[int] = None,
) -> AsyncIterator[StreamContext[DataBatch]]:
    """Create an async iterator of batches from a source.

    Args:
        source: Async callable that returns batches or None when exhausted.
        max_batches: Optional limit on number of batches.

    Yields:
        StreamContext wrapping each batch with index.

    Example:
        >>> async def fetch_batch():
        ...     # Fetch from external source
        ...     return DataBatch.from_dicts([...])
        >>> async for ctx in stream_batches(fetch_batch, max_batches=100):
        ...     print(f"Processing batch {ctx.index}")
    """
    index = 0
    while max_batches is None or index < max_batches:
        batch = await source()
        if batch is None:
            break
        yield StreamContext(value=batch, index=index)
        index += 1


async def collect_results(
    results: AsyncIterator[AsyncPipelineResult],
) -> List[AsyncPipelineResult]:
    """Collect async results into a list.

    Args:
        results: Async iterator of pipeline results.

    Returns:
        List of all results.
    """
    collected = []
    async for result in results:
        collected.append(result)
    return collected
