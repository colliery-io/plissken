"""Utility functions and classes for PySnake.

This subpackage demonstrates `pysnake.utils` namespace with nested modules.
"""

from typing import Optional

DEFAULT_ENCODING = "utf-8"
"""Default encoding for serialization."""


class Serializer:
    """Serializes snake data to various formats.

    Attributes:
        encoding: Character encoding to use.
    """

    def __init__(self, encoding: str = DEFAULT_ENCODING):
        """Create a serializer.

        Args:
            encoding: Character encoding. Defaults to UTF-8.
        """
        self.encoding = encoding

    def to_json(self, data: dict) -> str:
        """Serialize data to JSON string.

        Args:
            data: Dictionary to serialize.

        Returns:
            JSON string representation.
        """
        import json
        return json.dumps(data)

    def to_bytes(self, data: str) -> bytes:
        """Convert string to bytes.

        Args:
            data: String to encode.

        Returns:
            Encoded bytes.
        """
        return data.encode(self.encoding)


def format_length(length: int, unit: str = "cm") -> str:
    """Format a length value with units.

    Args:
        length: The length value.
        unit: Unit string. Defaults to "cm".

    Returns:
        Formatted string like "50 cm".
    """
    return f"{length} {unit}"
