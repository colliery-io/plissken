"""Protocol definitions for structural typing.

Defines protocols (interfaces) that describe expected behavior
without requiring inheritance.
"""

from __future__ import annotations

from datetime import datetime
from typing import Any, Protocol, TypeVar, runtime_checkable

T = TypeVar("T")
T_co = TypeVar("T_co", covariant=True)


@runtime_checkable
class Runnable(Protocol):
    """Protocol for objects that can be run.

    Any object with a `run()` method returning a result is considered
    Runnable, regardless of its actual class hierarchy.

    Example:
        >>> def execute(job: Runnable) -> Any:
        ...     return job.run()
    """

    def run(self) -> Any:
        """Execute this runnable and return the result."""
        ...


@runtime_checkable
class Schedulable(Protocol):
    """Protocol for objects that can be scheduled.

    Objects implementing this protocol can determine when they
    should next run based on their schedule configuration.
    """

    def should_run(self, now: datetime) -> bool:
        """Check if this should run at the given time.

        Args:
            now: The current datetime.

        Returns:
            True if this should run now.
        """
        ...

    def next_run(self, after: datetime) -> datetime:
        """Calculate the next run time after the given datetime.

        Args:
            after: Find the next run time after this point.

        Returns:
            The next datetime when this should run.
        """
        ...


class Serializable(Protocol[T_co]):
    """Protocol for objects that can be serialized.

    Generic protocol that defines serialization to a specific
    output type T_co (covariant).
    """

    def serialize(self) -> T_co:
        """Serialize this object.

        Returns:
            The serialized representation.
        """
        ...


class Deserializable(Protocol[T]):
    """Protocol for types that can be deserialized.

    Defines a class method for reconstructing objects from
    serialized data.
    """

    @classmethod
    def deserialize(cls, data: Any) -> T:
        """Deserialize from data.

        Args:
            data: The serialized data.

        Returns:
            A new instance of this type.
        """
        ...


@runtime_checkable
class SupportsComparison(Protocol):
    """Protocol for objects that support comparison operations."""

    def __lt__(self, other: Any) -> bool:
        """Less than comparison."""
        ...

    def __le__(self, other: Any) -> bool:
        """Less than or equal comparison."""
        ...

    def __gt__(self, other: Any) -> bool:
        """Greater than comparison."""
        ...

    def __ge__(self, other: Any) -> bool:
        """Greater than or equal comparison."""
        ...
