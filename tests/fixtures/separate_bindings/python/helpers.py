"""Pure Python helpers for the data pipeline.

This module provides Pythonic convenience utilities on top of
the core Rust pipeline functionality.
"""

from typing import Any, Callable, Dict, Iterator, List, Optional, TypeVar
from separate_bindings import Pipeline, DataBatch, PipelineResult

T = TypeVar("T")


class PipelineBuilder:
    """Fluent builder for creating pipelines.

    Provides a chainable API for pipeline construction:

        pipeline = (PipelineBuilder("etl")
            .extract(fetch_data)
            .transform(clean_data)
            .transform(enrich_data)
            .load(save_data)
            .build())

    Attributes:
        name: The pipeline name being built.
    """

    def __init__(self, name: str):
        """Create a new pipeline builder.

        Args:
            name: Identifier for the pipeline.
        """
        self._name = name
        self._stages: List[tuple[str, Callable]] = []

    @property
    def name(self) -> str:
        """The pipeline name."""
        return self._name

    def stage(
        self, name: str, processor: Callable[[DataBatch], DataBatch]
    ) -> "PipelineBuilder":
        """Add a generic stage.

        Args:
            name: Unique stage identifier.
            processor: Function to process batches.

        Returns:
            Self for chaining.
        """
        self._stages.append((name, processor))
        return self

    def extract(self, processor: Callable[[DataBatch], DataBatch]) -> "PipelineBuilder":
        """Add an extraction stage.

        Convention: extract stages are named "extract" or "extract_N".

        Args:
            processor: Extraction function.

        Returns:
            Self for chaining.
        """
        count = sum(1 for name, _ in self._stages if name.startswith("extract"))
        name = "extract" if count == 0 else f"extract_{count}"
        return self.stage(name, processor)

    def transform(
        self, processor: Callable[[DataBatch], DataBatch]
    ) -> "PipelineBuilder":
        """Add a transformation stage.

        Args:
            processor: Transformation function.

        Returns:
            Self for chaining.
        """
        count = sum(1 for name, _ in self._stages if name.startswith("transform"))
        name = "transform" if count == 0 else f"transform_{count}"
        return self.stage(name, processor)

    def load(self, processor: Callable[[DataBatch], DataBatch]) -> "PipelineBuilder":
        """Add a load stage.

        Args:
            processor: Load function.

        Returns:
            Self for chaining.
        """
        count = sum(1 for name, _ in self._stages if name.startswith("load"))
        name = "load" if count == 0 else f"load_{count}"
        return self.stage(name, processor)

    def build(self) -> Pipeline:
        """Build the configured pipeline.

        Returns:
            A new Pipeline instance.
        """
        pipeline = Pipeline(self._name)
        for name, processor in self._stages:
            pipeline.add_stage(name, processor)
        return pipeline


def batch_from_iter(
    items: Iterator[Dict[str, Any]], chunk_size: int = 1000
) -> Iterator[DataBatch]:
    """Create DataBatches from an iterator of dictionaries.

    Useful for processing large datasets that don't fit in memory.

    Args:
        items: Iterator yielding row dictionaries.
        chunk_size: Number of rows per batch.

    Yields:
        DataBatch objects with up to chunk_size rows each.

    Example:
        >>> def read_csv_rows(path):
        ...     with open(path) as f:
        ...         reader = csv.DictReader(f)
        ...         yield from reader
        >>> for batch in batch_from_iter(read_csv_rows("data.csv")):
        ...     result = pipeline.run(batch)
    """
    chunk: List[Dict[str, Any]] = []
    for item in items:
        chunk.append(item)
        if len(chunk) >= chunk_size:
            yield DataBatch.from_dicts(chunk)
            chunk = []
    if chunk:
        yield DataBatch.from_dicts(chunk)


def run_parallel(
    pipeline: Pipeline,
    batches: List[DataBatch],
    max_workers: Optional[int] = None,
) -> List[PipelineResult]:
    """Run a pipeline on multiple batches in parallel.

    Uses Python's concurrent.futures for parallelism.

    Args:
        pipeline: The pipeline to run.
        batches: List of input batches.
        max_workers: Maximum parallel workers (default: CPU count).

    Returns:
        List of PipelineResult, one per input batch.

    Raises:
        RuntimeError: If any batch fails to process.
    """
    from concurrent.futures import ThreadPoolExecutor, as_completed

    results: List[Optional[PipelineResult]] = [None] * len(batches)

    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        future_to_idx = {
            executor.submit(pipeline.run, batch): idx
            for idx, batch in enumerate(batches)
        }

        for future in as_completed(future_to_idx):
            idx = future_to_idx[future]
            results[idx] = future.result()

    return [r for r in results if r is not None]
