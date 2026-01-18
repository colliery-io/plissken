"""Complex Python type annotations for parser stress testing.

This module contains intentionally complex type signatures
to test the parser's ability to handle edge cases.
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from collections.abc import (
    Awaitable,
    Callable,
    Coroutine,
    Generator,
    Iterable,
    Iterator,
    Mapping,
    MutableMapping,
    Sequence,
)
from dataclasses import dataclass
from typing import (
    TYPE_CHECKING,
    Any,
    ClassVar,
    Concatenate,
    Final,
    Generic,
    Literal,
    NamedTuple,
    NewType,
    NotRequired,
    Optional,
    ParamSpec,
    Protocol,
    Required,
    Self,
    TypeAlias,
    TypedDict,
    TypeGuard,
    TypeVar,
    Union,
    overload,
    runtime_checkable,
)

if TYPE_CHECKING:
    from typing import Never


# =============================================================================
# Type Variables and ParamSpecs
# =============================================================================

T = TypeVar("T")
T_co = TypeVar("T_co", covariant=True)
T_contra = TypeVar("T_contra", contravariant=True)
K = TypeVar("K")
V = TypeVar("V")

# Bounded type variables
Numeric = TypeVar("Numeric", int, float, complex)
Comparable = TypeVar("Comparable", bound="SupportsLessThan")

# ParamSpec for decorator typing
P = ParamSpec("P")
R = TypeVar("R")


# =============================================================================
# Nested Generic Types
# =============================================================================

def process_nested(
    data: list[dict[str, T | None]],
) -> list[T]:
    """Process deeply nested generic structures.

    Args:
        data: List of dicts with optional values.

    Returns:
        Extracted non-None values.
    """
    return [v for d in data for v in d.values() if v is not None]


def deep_nesting(
    data: dict[K, list[tuple[str, dict[str, list[V]]]]],
) -> Iterator[tuple[K, V]]:
    """Handle very deeply nested types.

    Args:
        data: Complex nested structure.

    Yields:
        Tuples of key and innermost values.
    """
    for k, outer_list in data.items():
        for _, inner_dict in outer_list:
            for values in inner_dict.values():
                for v in values:
                    yield k, v


# Recursive type alias
JSON: TypeAlias = dict[str, "JSON"] | list["JSON"] | str | int | float | bool | None


def parse_json(text: str) -> JSON:
    """Parse JSON with recursive type.

    Args:
        text: JSON string.

    Returns:
        Parsed JSON value.
    """
    import json
    return json.loads(text)


# =============================================================================
# Callable Types
# =============================================================================

def apply_callback(
    func: Callable[[Mapping[str, Any]], Sequence[tuple[int, str]]],
    data: Mapping[str, Any],
) -> Sequence[tuple[int, str]]:
    """Apply a callback with complex signature.

    Args:
        func: Callback taking a mapping and returning sequence of tuples.
        data: Input data.

    Returns:
        Result of callback application.
    """
    return func(data)


def higher_order(
    f: Callable[[Callable[[T], U]], Callable[[U], V]],
    g: Callable[[T], U],
) -> Callable[[U], V]:
    """Higher-order function with nested callables.

    Args:
        f: Function that transforms function g into another function.
        g: Inner function.

    Returns:
        Composed function.
    """
    return f(g)


# =============================================================================
# ParamSpec and Concatenate
# =============================================================================

def decorator_with_extra_arg(
    extra: str,
) -> Callable[[Callable[P, R]], Callable[Concatenate[str, P], R]]:
    """Decorator that adds an extra first argument.

    Args:
        extra: Value to prepend.

    Returns:
        Decorator function.
    """
    def decorator(func: Callable[P, R]) -> Callable[Concatenate[str, P], R]:
        def wrapper(first: str, *args: P.args, **kwargs: P.kwargs) -> R:
            _ = first
            return func(*args, **kwargs)
        return wrapper
    return decorator


def log_calls(func: Callable[P, R]) -> Callable[P, R]:
    """Decorator preserving full signature.

    Args:
        func: Function to wrap.

    Returns:
        Wrapped function with logging.
    """
    def wrapper(*args: P.args, **kwargs: P.kwargs) -> R:
        print(f"Calling {func.__name__}")
        return func(*args, **kwargs)
    return wrapper


# =============================================================================
# Protocols
# =============================================================================

@runtime_checkable
class SupportsLessThan(Protocol):
    """Protocol for types supporting < comparison."""

    def __lt__(self, other: Self) -> bool:
        """Compare with another instance."""
        ...


class SupportsAdd(Protocol[T_co]):
    """Protocol for types supporting addition."""

    def __add__(self, other: Self) -> T_co:
        """Add two instances."""
        ...


class SupportsGetItem(Protocol[K, V_co]):
    """Protocol for subscriptable types."""

    def __getitem__(self, key: K) -> V_co:
        """Get item by key."""
        ...


V_co = TypeVar("V_co", covariant=True)


class AsyncIteratorProtocol(Protocol[T_co]):
    """Protocol for async iterators."""

    def __aiter__(self) -> Self:
        """Return self."""
        ...

    async def __anext__(self) -> T_co:
        """Get next item."""
        ...


# =============================================================================
# TypedDict
# =============================================================================

class UserDict(TypedDict):
    """User data structure."""

    name: str
    email: str
    age: NotRequired[int]
    roles: Required[list[str]]


class ConfigDict(TypedDict, total=False):
    """Configuration with all optional fields."""

    debug: bool
    log_level: Literal["DEBUG", "INFO", "WARNING", "ERROR"]
    max_connections: int
    timeout_seconds: float
    tags: list[str]


class NestedConfig(TypedDict):
    """Config with nested TypedDicts."""

    database: DatabaseConfig
    cache: CacheConfig
    features: dict[str, bool]


class DatabaseConfig(TypedDict):
    """Database configuration."""

    host: str
    port: int
    username: str
    password: str
    options: NotRequired[dict[str, Any]]


class CacheConfig(TypedDict, total=False):
    """Cache configuration."""

    backend: Literal["redis", "memcached", "memory"]
    ttl: int
    prefix: str


# =============================================================================
# Generic Classes
# =============================================================================

class Container(Generic[T]):
    """Generic container class.

    Type Parameters:
        T: The type of items in the container.
    """

    def __init__(self, items: Iterable[T] | None = None) -> None:
        """Initialize container.

        Args:
            items: Initial items.
        """
        self._items: list[T] = list(items) if items else []

    def add(self, item: T) -> Self:
        """Add an item.

        Args:
            item: Item to add.

        Returns:
            Self for chaining.
        """
        self._items.append(item)
        return self

    def get(self, index: int) -> T:
        """Get item by index.

        Args:
            index: Item index.

        Returns:
            The item.

        Raises:
            IndexError: If index out of range.
        """
        return self._items[index]

    def map(self, func: Callable[[T], U]) -> "Container[U]":
        """Map a function over items.

        Args:
            func: Transformation function.

        Returns:
            New container with transformed items.
        """
        return Container(func(item) for item in self._items)


U = TypeVar("U")


class BiMap(Generic[K, V]):
    """Bidirectional map.

    Type Parameters:
        K: Key type.
        V: Value type (must be hashable).
    """

    def __init__(self) -> None:
        """Initialize empty bimap."""
        self._forward: dict[K, V] = {}
        self._reverse: dict[V, K] = {}

    def put(self, key: K, value: V) -> None:
        """Add a mapping.

        Args:
            key: The key.
            value: The value.
        """
        self._forward[key] = value
        self._reverse[value] = key

    def get_by_key(self, key: K) -> V | None:
        """Get value by key."""
        return self._forward.get(key)

    def get_by_value(self, value: V) -> K | None:
        """Get key by value."""
        return self._reverse.get(value)


# =============================================================================
# Overloads
# =============================================================================

@overload
def fetch(url: str, *, as_json: Literal[True]) -> dict[str, Any]: ...

@overload
def fetch(url: str, *, as_json: Literal[False] = False) -> str: ...

@overload
def fetch(url: str, *, as_json: bool = False) -> dict[str, Any] | str: ...

def fetch(url: str, *, as_json: bool = False) -> dict[str, Any] | str:
    """Fetch URL content.

    Args:
        url: URL to fetch.
        as_json: If True, parse response as JSON.

    Returns:
        Response content as string or parsed JSON.
    """
    _ = url
    if as_json:
        return {}
    return ""


# =============================================================================
# TypeGuard
# =============================================================================

def is_string_list(val: list[Any]) -> TypeGuard[list[str]]:
    """Check if all items are strings.

    Args:
        val: List to check.

    Returns:
        True if all items are strings.
    """
    return all(isinstance(x, str) for x in val)


def is_not_none(val: T | None) -> TypeGuard[T]:
    """Check if value is not None.

    Args:
        val: Value to check.

    Returns:
        True if not None.
    """
    return val is not None


# =============================================================================
# Async Types
# =============================================================================

async def async_process(
    items: Iterable[T],
    handler: Callable[[T], Awaitable[U]],
) -> list[U]:
    """Process items asynchronously.

    Args:
        items: Items to process.
        handler: Async handler for each item.

    Returns:
        List of results.
    """
    return [await handler(item) for item in items]


def create_async_generator(
    start: int,
    end: int,
) -> Coroutine[Any, Any, Generator[int, None, None]]:
    """Create an async generator.

    Args:
        start: Start value.
        end: End value.

    Returns:
        Coroutine that produces a generator.
    """
    async def inner() -> Generator[int, None, None]:
        return (x for x in range(start, end))
    return inner()
