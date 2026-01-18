"""Command-line interface utilities for hybrid_binary.

This module provides argument parsing, output formatting, and
interactive prompts for building CLI applications with the task runner.

Example:
    >>> from hybrid_binary.cli import TaskCLI, OutputFormat
    >>> cli = TaskCLI()
    >>> cli.add_task_args()
    >>> args = cli.parse()
"""

from enum import Enum
from typing import Any, Dict, List, Optional, TextIO
import sys


class OutputFormat(Enum):
    """Output format options for CLI display.

    Attributes:
        TEXT: Plain text output (default).
        JSON: JSON formatted output.
        QUIET: Minimal output, errors only.
    """

    TEXT = "text"
    JSON = "json"
    QUIET = "quiet"


class ColoredOutput:
    """Helper for colored terminal output.

    Provides ANSI color codes for terminal styling.
    Automatically disables colors when output is not a TTY.

    Attributes:
        enabled: Whether colors are enabled.
    """

    # ANSI color codes
    RED = "\033[91m"
    GREEN = "\033[92m"
    YELLOW = "\033[93m"
    BLUE = "\033[94m"
    RESET = "\033[0m"
    BOLD = "\033[1m"

    def __init__(self, stream: Optional[TextIO] = None):
        """Create a colored output helper.

        Args:
            stream: Output stream to check for TTY. Defaults to stdout.
        """
        stream = stream or sys.stdout
        self.enabled = hasattr(stream, "isatty") and stream.isatty()

    def red(self, text: str) -> str:
        """Wrap text in red color.

        Args:
            text: Text to colorize.

        Returns:
            Colored text if enabled, otherwise plain text.
        """
        if self.enabled:
            return f"{self.RED}{text}{self.RESET}"
        return text

    def green(self, text: str) -> str:
        """Wrap text in green color.

        Args:
            text: Text to colorize.

        Returns:
            Colored text if enabled, otherwise plain text.
        """
        if self.enabled:
            return f"{self.GREEN}{text}{self.RESET}"
        return text

    def yellow(self, text: str) -> str:
        """Wrap text in yellow color.

        Args:
            text: Text to colorize.

        Returns:
            Colored text if enabled, otherwise plain text.
        """
        if self.enabled:
            return f"{self.YELLOW}{text}{self.RESET}"
        return text

    def blue(self, text: str) -> str:
        """Wrap text in blue color.

        Args:
            text: Text to colorize.

        Returns:
            Colored text if enabled, otherwise plain text.
        """
        if self.enabled:
            return f"{self.BLUE}{text}{self.RESET}"
        return text

    def bold(self, text: str) -> str:
        """Make text bold.

        Args:
            text: Text to style.

        Returns:
            Bold text if enabled, otherwise plain text.
        """
        if self.enabled:
            return f"{self.BOLD}{text}{self.RESET}"
        return text


class ProgressBar:
    """A simple progress bar for task execution.

    Displays progress in the terminal with percentage and ETA.

    Attributes:
        total: Total number of items.
        current: Current progress.
        width: Bar width in characters.
    """

    def __init__(self, total: int, width: int = 40):
        """Create a progress bar.

        Args:
            total: Total number of items to process.
            width: Width of the progress bar in characters.
        """
        self.total = total
        self.current = 0
        self.width = width
        self._colors = ColoredOutput()

    def update(self, amount: int = 1) -> None:
        """Update progress.

        Args:
            amount: Number of items completed.
        """
        self.current = min(self.current + amount, self.total)

    def render(self) -> str:
        """Render the progress bar as a string.

        Returns:
            Formatted progress bar string.
        """
        if self.total == 0:
            percent = 100.0
        else:
            percent = (self.current / self.total) * 100

        filled = int(self.width * self.current / max(self.total, 1))
        bar = "█" * filled + "░" * (self.width - filled)

        return f"[{bar}] {percent:5.1f}% ({self.current}/{self.total})"

    def display(self) -> None:
        """Print the progress bar to stdout with carriage return."""
        print(f"\r{self.render()}", end="", flush=True)

    def finish(self) -> None:
        """Complete the progress bar and move to next line."""
        self.current = self.total
        print(f"\r{self.render()}")


def format_task_list(tasks: List[str], descriptions: Optional[Dict[str, str]] = None) -> str:
    """Format a list of tasks for display.

    Args:
        tasks: List of task names.
        descriptions: Optional dict mapping task names to descriptions.

    Returns:
        Formatted string with task list.
    """
    if not tasks:
        return "No tasks registered."

    lines = ["Available tasks:", ""]
    descriptions = descriptions or {}

    for task in sorted(tasks):
        desc = descriptions.get(task, "")
        if desc:
            lines.append(f"  {task:<20} {desc}")
        else:
            lines.append(f"  {task}")

    return "\n".join(lines)
