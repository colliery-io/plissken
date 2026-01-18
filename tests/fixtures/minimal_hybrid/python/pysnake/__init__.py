"""PySnake - A minimal Python package for testing plissken documentation.

This package demonstrates various Python symbol types and nested module structures.
"""

from enum import Enum
from typing import Optional, List

__version__ = "0.1.0"
__all__ = ["Snake", "SnakeColor", "create_snake", "MAX_LENGTH"]

MAX_LENGTH: int = 100
"""Maximum allowed snake length."""


class SnakeColor(Enum):
    """Colors available for snakes."""

    GREEN = "green"
    YELLOW = "yellow"
    RED = "red"
    BROWN = "brown"


class Snake:
    """A snake entity with configurable properties.

    Attributes:
        name: The snake's name.
        length: Length in centimeters.
        color: The snake's color.

    Examples:
        >>> snake = Snake("Monty", 150, SnakeColor.GREEN)
        >>> snake.slither()
        'Monty slithers forward'
    """

    def __init__(self, name: str, length: int, color: SnakeColor = SnakeColor.GREEN):
        """Create a new snake.

        Args:
            name: The snake's name.
            length: Length in centimeters.
            color: The snake's color.
        """
        self.name = name
        self.length = length
        self.color = color

    def slither(self) -> str:
        """Make the snake slither forward.

        Returns:
            A description of the slithering action.
        """
        return f"{self.name} slithers forward"

    def shed(self) -> bool:
        """Shed the snake's skin.

        Returns:
            True if shedding was successful.
        """
        return True


def create_snake(name: str, length: int = 50) -> Snake:
    """Factory function to create a snake with defaults.

    Args:
        name: The snake's name.
        length: Length in centimeters. Defaults to 50.

    Returns:
        A new Snake instance.

    Examples:
        >>> snake = create_snake("Slinky")
        >>> snake.length
        50
    """
    return Snake(name, length)
