"""Validation utilities for PySnake.

This demonstrates `pysnake.utils.validation` - a nested single-file module.
"""

from typing import Optional


class ValidationError(Exception):
    """Raised when validation fails.

    Attributes:
        field: The field that failed validation.
        message: Description of the failure.
    """

    def __init__(self, field: str, message: str):
        """Create a validation error.

        Args:
            field: The field name.
            message: Error description.
        """
        self.field = field
        self.message = message
        super().__init__(f"{field}: {message}")


def validate_length(length: int, min_val: int = 1, max_val: int = 1000) -> bool:
    """Validate a snake length value.

    Args:
        length: The length to validate.
        min_val: Minimum allowed value.
        max_val: Maximum allowed value.

    Returns:
        True if valid.

    Raises:
        ValidationError: If length is out of range.

    Examples:
        >>> validate_length(50)
        True
        >>> validate_length(-1)
        Traceback (most recent call last):
        ...
        ValidationError: length: must be >= 1
    """
    if length < min_val:
        raise ValidationError("length", f"must be >= {min_val}")
    if length > max_val:
        raise ValidationError("length", f"must be <= {max_val}")
    return True


def validate_name(name: str) -> bool:
    """Validate a snake name.

    Args:
        name: The name to validate.

    Returns:
        True if valid.

    Raises:
        ValidationError: If name is empty or too long.
    """
    if not name or not name.strip():
        raise ValidationError("name", "cannot be empty")
    if len(name) > 50:
        raise ValidationError("name", "must be 50 characters or less")
    return True
