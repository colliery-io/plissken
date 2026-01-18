"""Event handlers for snake actions.

This is a single-file submodule demonstrating `pysnake.handlers` namespace.
"""

from typing import Callable, Any

HandlerFunc = Callable[[str, Any], None]
"""Type alias for handler functions."""


class EventHandler:
    """Handles events emitted by snakes.

    Attributes:
        handlers: Registered event handlers.
    """

    def __init__(self):
        """Create a new event handler registry."""
        self.handlers: dict[str, list[HandlerFunc]] = {}

    def register(self, event: str, handler: HandlerFunc) -> None:
        """Register a handler for an event.

        Args:
            event: The event name to listen for.
            handler: The callback function.
        """
        if event not in self.handlers:
            self.handlers[event] = []
        self.handlers[event].append(handler)

    def emit(self, event: str, data: Any) -> None:
        """Emit an event to all registered handlers.

        Args:
            event: The event name.
            data: Data to pass to handlers.
        """
        for handler in self.handlers.get(event, []):
            handler(event, data)


def on_slither(callback: HandlerFunc) -> HandlerFunc:
    """Decorator to register a slither event handler.

    Args:
        callback: The handler function.

    Returns:
        The decorated function.

    Examples:
        >>> @on_slither
        ... def my_handler(event, data):
        ...     print(f"Snake slithered: {data}")
    """
    return callback
