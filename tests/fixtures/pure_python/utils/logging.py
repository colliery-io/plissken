"""Logging utilities.

Provides structured logging helpers for the scheduler.
"""

from __future__ import annotations

import logging
import sys
from enum import IntEnum
from typing import Any, TextIO


class LogLevel(IntEnum):
    """Log level constants.

    Mirrors Python's logging levels for consistency.
    """

    DEBUG = logging.DEBUG
    """Detailed information for debugging."""

    INFO = logging.INFO
    """General operational information."""

    WARNING = logging.WARNING
    """Indication of potential issues."""

    ERROR = logging.ERROR
    """Error that prevented an operation."""

    CRITICAL = logging.CRITICAL
    """Severe error that may crash the application."""


#: Default log format string.
DEFAULT_FORMAT: str = "%(asctime)s [%(levelname)s] %(name)s: %(message)s"

#: Default date format for log timestamps.
DEFAULT_DATE_FORMAT: str = "%Y-%m-%d %H:%M:%S"


def get_logger(
    name: str,
    *,
    level: LogLevel = LogLevel.INFO,
    stream: TextIO | None = None,
    format_string: str = DEFAULT_FORMAT,
) -> logging.Logger:
    """Get or create a configured logger.

    Args:
        name: The logger name, typically __name__.
        level: Minimum log level to output.
        stream: Output stream (defaults to stderr).
        format_string: Log message format.

    Returns:
        A configured Logger instance.

    Example:
        >>> logger = get_logger(__name__, level=LogLevel.DEBUG)
        >>> logger.debug("Starting task")
    """
    logger = logging.getLogger(name)
    logger.setLevel(level)

    # Avoid duplicate handlers
    if not logger.handlers:
        handler = logging.StreamHandler(stream or sys.stderr)
        handler.setLevel(level)
        handler.setFormatter(
            logging.Formatter(format_string, datefmt=DEFAULT_DATE_FORMAT)
        )
        logger.addHandler(handler)

    return logger


class StructuredLogger:
    """A logger that outputs structured log data.

    Wraps a standard logger to add structured context fields
    to every log message.

    Attributes:
        logger: The underlying Python logger.
        context: Default context fields added to every message.
    """

    def __init__(
        self,
        name: str,
        *,
        level: LogLevel = LogLevel.INFO,
        **context: Any,
    ):
        """Create a new structured logger.

        Args:
            name: The logger name.
            level: Minimum log level.
            **context: Default context fields.
        """
        self.logger = get_logger(name, level=level)
        self.context: dict[str, Any] = context

    def with_context(self, **extra: Any) -> StructuredLogger:
        """Create a child logger with additional context.

        Args:
            **extra: Additional context fields.

        Returns:
            A new StructuredLogger with merged context.
        """
        new_logger = StructuredLogger(
            self.logger.name,
            level=LogLevel(self.logger.level),
            **self.context,
        )
        new_logger.context.update(extra)
        return new_logger

    def _format_message(self, msg: str, **kwargs: Any) -> str:
        """Format message with context."""
        all_context = {**self.context, **kwargs}
        if all_context:
            context_str = " ".join(f"{k}={v!r}" for k, v in all_context.items())
            return f"{msg} [{context_str}]"
        return msg

    def debug(self, msg: str, **kwargs: Any) -> None:
        """Log a debug message."""
        self.logger.debug(self._format_message(msg, **kwargs))

    def info(self, msg: str, **kwargs: Any) -> None:
        """Log an info message."""
        self.logger.info(self._format_message(msg, **kwargs))

    def warning(self, msg: str, **kwargs: Any) -> None:
        """Log a warning message."""
        self.logger.warning(self._format_message(msg, **kwargs))

    def error(self, msg: str, **kwargs: Any) -> None:
        """Log an error message."""
        self.logger.error(self._format_message(msg, **kwargs))

    def critical(self, msg: str, **kwargs: Any) -> None:
        """Log a critical message."""
        self.logger.critical(self._format_message(msg, **kwargs))
