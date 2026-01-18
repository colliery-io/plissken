"""Hybrid Binary - A task runner with Rust core and Python API.

This package provides task execution and scheduling capabilities
with a high-performance Rust backend exposed through Python bindings.

Modules:
    testing: Test utilities and mock objects for testing task code.
    cli: Command-line interface utilities and argument parsing.
"""

from typing import List

__version__ = "0.1.0"
__all__ = ["testing", "cli"]


def get_version() -> str:
    """Get the package version string.

    Returns:
        The semantic version string.
    """
    return __version__


def list_submodules() -> List[str]:
    """List available submodules.

    Returns:
        List of submodule names.
    """
    return __all__.copy()
