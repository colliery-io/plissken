# Python Projects

Guide for documenting pure Python projects with plissken.

## Configuration

```toml
[project]
name = "mypackage"
version_from = "pyproject"

[output]
path = "docs/api"
template = "mkdocs-material"

[python]
package = "mypackage"
```

## Docstring Formats

### Google Style (Recommended)

```python
def process_data(data: list[str], timeout: int = 30) -> dict:
    """Process input data and return results.

    Takes a list of strings and processes them with an optional
    timeout parameter.

    Args:
        data: List of strings to process.
        timeout: Maximum processing time in seconds. Defaults to 30.

    Returns:
        Dictionary containing processed results with keys:
        - 'count': Number of items processed
        - 'results': List of processed items

    Raises:
        ValueError: If data is empty.
        TimeoutError: If processing exceeds timeout.

    Examples:
        >>> process_data(['a', 'b', 'c'])
        {'count': 3, 'results': ['A', 'B', 'C']}
    """
```

### NumPy Style

```python
def process_data(data, timeout=30):
    """
    Process input data and return results.

    Parameters
    ----------
    data : list of str
        List of strings to process.
    timeout : int, optional
        Maximum processing time in seconds (default is 30).

    Returns
    -------
    dict
        Dictionary containing processed results.

    Raises
    ------
    ValueError
        If data is empty.
    """
```

## Type Hints

plissken extracts type information from:

1. **Function signatures** (preferred)
2. **Docstring type annotations** (fallback)

```python
# Types from signature (recommended)
def greet(name: str, times: int = 1) -> str:
    """Return a greeting message."""
    return f"Hello, {name}!" * times

# Types merged from docstring
def legacy_function(data):
    """Process data.

    Args:
        data (list): Input data list.

    Returns:
        dict: Processed results.
    """
```

## Class Documentation

```python
class DataProcessor:
    """Process and transform data.

    A flexible data processor supporting multiple input formats
    and transformation pipelines.

    Attributes:
        name: Processor identifier.
        config: Configuration dictionary.

    Examples:
        >>> processor = DataProcessor("main")
        >>> processor.process([1, 2, 3])
        [2, 4, 6]
    """

    def __init__(self, name: str, config: dict | None = None):
        """Initialize the processor.

        Args:
            name: Processor identifier.
            config: Optional configuration. Defaults to empty dict.
        """
        self.name = name
        self.config = config or {}

    def process(self, data: list) -> list:
        """Process input data.

        Args:
            data: Input data to process.

        Returns:
            Transformed data.
        """
        return [x * 2 for x in data]
```

## Decorators

plissken recognizes common decorators:

| Decorator | Badge |
|-----------|-------|
| `@property` | property |
| `@staticmethod` | staticmethod |
| `@classmethod` | classmethod |
| `async def` | async |

## Package Structure

For nested packages:

```
mypackage/
├── __init__.py
├── core/
│   ├── __init__.py
│   └── processor.py
└── utils/
    ├── __init__.py
    └── helpers.py
```

Generated docs:

```
docs/api/python/
├── mypackage/
│   ├── index.md
│   └── ...
├── mypackage/core/
│   ├── index.md
│   └── ...
└── mypackage/utils/
    ├── index.md
    └── ...
```
