"""Test tasks for plissken."""

import subprocess
from pathlib import Path

import angreal


def get_project_root():
    """Get project root directory."""
    return str(Path(angreal.get_root()).parent)


# Command group for tests
test = angreal.command_group(name="test", about="Testing commands")


@test()
@angreal.command(name="unit", about="Run unit tests")
@angreal.argument(
    name="package",
    short="p",
    long="package",
    help="Specific package to test (e.g., plissken-core)"
)
def test_unit(package=None):
    """Run Rust unit tests."""
    project_root = get_project_root()
    cmd = ["cargo", "test"]
    if package:
        cmd.extend(["-p", package])
    else:
        cmd.append("--workspace")

    print(f"Running unit tests{f' for {package}' if package else ''}...")
    result = subprocess.run(cmd, cwd=project_root)
    return result.returncode


@test()
@angreal.command(name="doc", about="Run documentation tests")
def test_doc():
    """Run documentation tests."""
    project_root = get_project_root()
    cmd = ["cargo", "test", "--doc", "--workspace"]

    print("Running doc tests...")
    result = subprocess.run(cmd, cwd=project_root)
    return result.returncode


@test()
@angreal.command(name="all", about="Run all tests")
def test_all():
    """Run all tests (unit + doc)."""
    print("=" * 50)
    print("Running all tests")
    print("=" * 50)

    # Unit tests
    if test_unit() != 0:
        print("\nUnit tests failed!")
        return 1

    print()

    # Doc tests
    if test_doc() != 0:
        print("\nDoc tests failed!")
        return 1

    print()
    print("=" * 50)
    print("All tests passed!")
    print("=" * 50)

    return 0
