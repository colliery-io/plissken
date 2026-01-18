"""Development environment setup for plissken."""

import subprocess
from pathlib import Path

import angreal


def get_project_root():
    """Get project root directory."""
    return str(Path(angreal.get_root()).parent)


@angreal.command(name="setup", about="Setup development environment")
@angreal.argument(
    name="release",
    long="release",
    is_flag=True,
    takes_value=False,
    help="Build in release mode"
)
def setup(release=False):
    """Setup the plissken development environment."""
    project_root = get_project_root()
    print("Setting up plissken development environment...")

    # Build the project
    cmd = ["cargo", "build", "--workspace"]
    if release:
        cmd.append("--release")

    print(f"Building project ({'release' if release else 'debug'} mode)...")
    result = subprocess.run(cmd, cwd=project_root)
    if result.returncode != 0:
        print("Build failed!")
        return 1

    print("Development environment ready!")
    print("\nAvailable commands:")
    print("  cargo test --workspace     - Run all tests")
    print("  cargo clippy --workspace   - Run linter")
    print("  angreal functional all     - Run SSG functional tests")

    return 0
