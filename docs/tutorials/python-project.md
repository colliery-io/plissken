# Tutorial: Document a Python Project

This tutorial walks you through generating documentation for a pure Python
project. By the end, you'll have a browsable documentation site with
auto-generated API reference pages.

## Prerequisites

- [plissken installed](../how-to/install.md)
- Python 3.8+ installed
- pip installed (for MkDocs)

## What You'll Build

You'll create a small Python package and generate a complete documentation
site that includes:

- Module overview pages
- Class and function documentation extracted from docstrings
- Type information from annotations
- A navigable MkDocs Material site

## Step 1: Create a Python Package

Create a new directory and set up a minimal Python package:

```bash
mkdir scheduler-demo && cd scheduler-demo
```

Create `pyproject.toml`:

```toml
[project]
name = "scheduler"
version = "0.1.0"
```

Create the package directory:

```bash
mkdir -p scheduler
```

Create `scheduler/__init__.py`:

```python
"""A simple task scheduler.

This package provides a lightweight interface for scheduling
and running tasks at specified intervals.
"""

from .task import Task
from .runner import Runner

__all__ = ["Task", "Runner"]
```

Create `scheduler/task.py`:

```python
"""Task definitions for the scheduler."""

from dataclasses import dataclass
from typing import Callable, Optional


@dataclass
class Task:
    """A schedulable unit of work.

    Tasks represent individual operations that can be scheduled
    and executed by a Runner.

    Attributes:
        name: Human-readable task identifier.
        callback: The function to execute.
        interval_seconds: How often to run, in seconds.
        max_retries: Maximum retry attempts on failure. Defaults to 3.

    Examples:
        >>> task = Task("cleanup", lambda: print("done"), 60)
        >>> task.name
        'cleanup'
    """

    name: str
    callback: Callable[[], None]
    interval_seconds: int
    max_retries: int = 3

    def execute(self) -> bool:
        """Execute the task's callback.

        Returns:
            True if the callback completed without raising an exception.

        Raises:
            RuntimeError: If max_retries is negative.
        """
        if self.max_retries < 0:
            raise RuntimeError("max_retries must be non-negative")
        try:
            self.callback()
            return True
        except Exception:
            return False
```

Create `scheduler/runner.py`:

```python
"""Task runner with scheduling support."""

from typing import Optional
from .task import Task


class Runner:
    """Executes tasks on a schedule.

    The Runner manages a collection of tasks and executes them
    according to their configured intervals.

    Args:
        name: Runner identifier for logging.

    Attributes:
        name: The runner's identifier.
        tasks: List of registered tasks.

    Examples:
        >>> runner = Runner("main")
        >>> runner.add_task(Task("ping", lambda: None, 30))
        >>> len(runner.tasks)
        1
    """

    def __init__(self, name: str) -> None:
        self.name = name
        self.tasks: list[Task] = []

    def add_task(self, task: Task) -> None:
        """Register a task with the runner.

        Args:
            task: The task to schedule.

        Raises:
            ValueError: If a task with the same name already exists.
        """
        if any(t.name == task.name for t in self.tasks):
            raise ValueError(f"Task '{task.name}' already registered")
        self.tasks.append(task)

    def remove_task(self, name: str) -> Optional[Task]:
        """Remove a task by name.

        Args:
            name: Name of the task to remove.

        Returns:
            The removed task, or None if not found.
        """
        for i, task in enumerate(self.tasks):
            if task.name == name:
                return self.tasks.pop(i)
        return None

    async def run_once(self) -> dict[str, bool]:
        """Execute all registered tasks once.

        Returns:
            Dictionary mapping task names to success/failure booleans.
        """
        results = {}
        for task in self.tasks:
            results[task.name] = task.execute()
        return results
```

## Step 2: Initialize plissken

Run the init command:

```bash
plissken init
```

plissken auto-detects your project and creates `plissken.toml`:

```toml
[project]
name = "scheduler"
version_from = "pyproject"

[output]
format = "markdown"
path = "docs/api"
template = "mkdocs-material"

[python]
package = "scheduler"
```

## Step 3: Validate the Configuration

Check that everything is correct:

```bash
plissken check
```

You should see output confirming the configuration is valid. If there are
warnings about missing modules, that's fine — plissken will discover them
automatically.

## Step 4: Generate Documentation

```bash
plissken render
```

This parses your Python source files and generates Markdown documentation:

```
docs/api/
  scheduler.md
  scheduler/task.md
  scheduler/runner.md
  _nav.yml
```

Each `.md` file contains the extracted docstrings (in Google style — the
format used in the code above), type information, and function signatures
formatted as Markdown. The `_nav.yml` file contains navigation entries
that can be included in your `mkdocs.yml` for larger projects; for this
tutorial, we'll write the navigation manually.

## Step 5: Set Up MkDocs

Install MkDocs Material:

```bash
pip install mkdocs-material
```

Create `mkdocs.yml` in the project root:

```yaml
site_name: scheduler
theme:
  name: material

nav:
  - Home: index.md
  - API Reference:
    - scheduler: api/scheduler.md
    - scheduler.task: api/scheduler/task.md
    - scheduler.runner: api/scheduler/runner.md
```

Create a landing page at `docs/index.md`:

```markdown
# scheduler

A simple task scheduler for Python.

## API Reference

- [scheduler](api/scheduler.md) — Package overview
- [scheduler.task](api/scheduler/task.md) — Task definitions
- [scheduler.runner](api/scheduler/runner.md) — Task execution
```

## Step 6: Serve and Browse

```bash
mkdocs serve
```

Open [http://localhost:8000](http://localhost:8000). You should see:

- A landing page with links to each module
- Module pages with class and function documentation
- Parameter tables extracted from docstrings
- Type annotations from your Python code
- Badges for `async` functions (like `run_once`)

## Step 7: Iterate

As you add more code, regenerate the docs:

```bash
plissken render
mkdocs serve
```

plissken will discover new modules automatically and update the generated
files.

## What You Learned

- How to initialize plissken for a Python project
- How to write Google-style docstrings that plissken extracts
- How to generate Markdown documentation from source code
- How to serve the result with MkDocs Material

## Next Steps

- [Tutorial: Document a Rust Project](rust-project.md) — Rust-specific documentation
- [Tutorial: Document a Hybrid Project](hybrid-project.md) — PyO3 cross-references
- [How-To: Customize Templates](../how-to/customize-templates.md) — Change the look and feel
- [Configuration Reference](../reference/configuration.md) — All configuration options
