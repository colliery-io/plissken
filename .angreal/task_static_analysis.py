"""Static analysis tasks for plissken."""

import subprocess
from pathlib import Path

import angreal


def get_project_root():
    """Get project root directory."""
    return str(Path(angreal.get_root()).parent)


# Command group for linting/analysis
lint = angreal.command_group(name="lint", about="Static analysis commands")


@lint()
@angreal.command(name="clippy", about="Run Clippy linter")
@angreal.argument(
    name="fix",
    long="fix",
    is_flag=True,
    takes_value=False,
    help="Automatically fix warnings where possible"
)
def lint_clippy(fix=False):
    """Run Clippy for Rust static analysis."""
    project_root = get_project_root()
    cmd = ["cargo", "clippy", "--workspace"]
    if fix:
        cmd.extend(["--fix", "--allow-dirty"])
    cmd.extend(["--", "-D", "warnings"])

    print("Running Clippy...")
    result = subprocess.run(cmd, cwd=project_root)
    return result.returncode


@lint()
@angreal.command(name="fmt", about="Check code formatting")
@angreal.argument(
    name="fix",
    long="fix",
    is_flag=True,
    takes_value=False,
    help="Automatically format code"
)
def lint_fmt(fix=False):
    """Check or fix Rust code formatting."""
    project_root = get_project_root()
    cmd = ["cargo", "fmt", "--all"]
    if not fix:
        cmd.append("--check")

    action = "Formatting" if fix else "Checking formatting..."
    print(action)
    result = subprocess.run(cmd, cwd=project_root)
    return result.returncode


@lint()
@angreal.command(name="all", about="Run all linters")
@angreal.argument(
    name="fix",
    long="fix",
    is_flag=True,
    takes_value=False,
    help="Automatically fix issues where possible"
)
def lint_all(fix=False):
    """Run all static analysis checks."""
    print("=" * 50)
    print("Running all static analysis")
    print("=" * 50)

    # Format check/fix
    if lint_fmt(fix=fix) != 0:
        print("\nFormatting check failed!")
        return 1

    print()

    # Clippy
    if lint_clippy(fix=fix) != 0:
        print("\nClippy failed!")
        return 1

    print()
    print("=" * 50)
    print("All static analysis passed!")
    print("=" * 50)

    return 0
