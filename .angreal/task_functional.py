"""Functional SSG tests for plissken documentation rendering."""

import os
import subprocess
import shutil
from pathlib import Path

import angreal
from angreal.integrations.flox import flox_required


def get_project_root():
    """Get project root directory."""
    return str(Path(angreal.get_root()).parent)


def get_functional_dir():
    """Get functional test directory."""
    return os.path.join(get_project_root(), "tests", "functional")


def get_fixture_dir():
    """Get fixture directory."""
    return os.path.join(get_project_root(), "tests", "fixtures", "hybrid_binary")


def get_plissken_binary():
    """Get path to plissken binary, building if necessary."""
    project_root = get_project_root()
    binary = os.path.join(project_root, "target", "release", "plissken")
    if not os.path.exists(binary):
        # Try debug build
        binary = os.path.join(project_root, "target", "debug", "plissken")
    return binary


def run_cmd(cmd, cwd=None, check=True):
    """Run a command and return result."""
    result = subprocess.run(
        cmd,
        cwd=cwd or get_project_root(),
        capture_output=True,
        text=True,
    )
    if check and result.returncode != 0:
        print(f"Command failed: {' '.join(cmd)}")
        print(f"stderr: {result.stderr}")
        return None
    return result


# Command group for functional tests
functional = angreal.command_group(name="functional", about="Functional SSG tests")


@functional()
@angreal.command(name="build", about="Build plissken for functional tests")
@angreal.argument(
    name="release",
    long="release",
    is_flag=True,
    takes_value=False,
    help="Build in release mode"
)
def functional_build(release=False):
    """Build plissken binary."""
    cmd = ["cargo", "build", "-p", "plissken"]
    if release:
        cmd.append("--release")

    print(f"Building plissken ({'release' if release else 'debug'})...")
    result = run_cmd(cmd)
    if result is None:
        return 1

    print("Build complete!")
    return 0


@flox_required(get_project_root())
@functional()
@angreal.command(name="mkdocs", about="Test MkDocs Material integration")
@angreal.argument(
    name="serve",
    long="serve",
    is_flag=True,
    takes_value=False,
    help="Start local server after building"
)
@angreal.argument(
    name="open_browser",
    long="open",
    is_flag=True,
    takes_value=False,
    help="Open browser (with --serve)"
)
def functional_mkdocs(serve=False, open_browser=False):
    """Test MkDocs Material integration."""
    functional_dir = get_functional_dir()
    fixture_dir = get_fixture_dir()
    mkdocs_dir = os.path.join(functional_dir, "mkdocs-material")
    docs_dir = os.path.join(mkdocs_dir, "docs")

    # Get plissken binary
    plissken = get_plissken_binary()
    if not os.path.exists(plissken):
        print("Error: plissken not built. Run 'angreal functional build' first.")
        return 1

    # Clean old generated docs first (project-first structure)
    hybrid_binary_dir = os.path.join(docs_dir, "hybrid_binary")
    if os.path.exists(hybrid_binary_dir):
        shutil.rmtree(hybrid_binary_dir)

    # Render docs using mkdocs-material's plissken.toml config
    print("Rendering docs with mkdocs-material template...")
    result = run_cmd([plissken, "render", mkdocs_dir, "-o", docs_dir])
    if result is None:
        return 1
    print(result.stderr)

    # Verify output structure (project-first: hybrid_binary/python/Task.md)
    python_class = os.path.join(docs_dir, "hybrid_binary", "python", "Task.md")
    if os.path.exists(python_class):
        with open(python_class) as f:
            content = f.read()
            if "var(--md-" not in content:
                print("Error: MkDocs Material CSS variables not found in output")
                return 1
        print("MkDocs Material CSS variables verified!")
    else:
        print(f"Warning: Expected {python_class} not found")

    # Build site
    print("Building MkDocs site...")
    result = run_cmd(["mkdocs", "build"], cwd=mkdocs_dir)
    if result is None:
        return 1

    site_dir = os.path.join(mkdocs_dir, "site")
    if not os.path.isdir(site_dir):
        print("Error: MkDocs site directory not created")
        return 1

    print("MkDocs Material test PASSED!")

    if serve:
        print("\nStarting MkDocs server...")
        cmd = ["mkdocs", "serve"]
        if open_browser:
            cmd.append("--open")
        subprocess.run(cmd, cwd=mkdocs_dir)

    return 0


@flox_required(get_project_root())
@functional()
@angreal.command(name="mdbook", about="Test mdBook integration")
@angreal.argument(
    name="serve",
    long="serve",
    is_flag=True,
    takes_value=False,
    help="Start local server after building"
)
@angreal.argument(
    name="open_browser",
    long="open",
    is_flag=True,
    takes_value=False,
    help="Open browser (with --serve)"
)
def functional_mdbook(serve=False, open_browser=False):
    """Test mdBook integration."""
    functional_dir = get_functional_dir()
    fixture_dir = get_fixture_dir()
    mdbook_dir = os.path.join(functional_dir, "mdbook")
    src_dir = os.path.join(mdbook_dir, "src")

    # Get plissken binary
    plissken = get_plissken_binary()
    if not os.path.exists(plissken):
        print("Error: plissken not built. Run 'angreal functional build' first.")
        return 1

    # Render docs
    print("Rendering docs with mdbook template...")
    result = run_cmd([plissken, "render", fixture_dir, "-o", src_dir, "-t", "mdbook"])
    if result is None:
        return 1
    print(result.stderr)

    # Verify CSS variables (should NOT have mkdocs variables)
    helpers_md = os.path.join(src_dir, "helpers.md")
    if os.path.exists(helpers_md):
        with open(helpers_md) as f:
            content = f.read()
            if "var(--md-" in content:
                print("Error: MkDocs CSS variables found in mdBook output (should use mdBook variables)")
                return 1
            if "var(--" not in content:
                print("Error: CSS variables not found in output")
                return 1
        print("mdBook CSS variables verified!")

    # Build book
    print("Building mdBook site...")
    result = run_cmd(["mdbook", "build"], cwd=mdbook_dir)
    if result is None:
        return 1

    book_dir = os.path.join(mdbook_dir, "book")
    if not os.path.isdir(book_dir):
        print("Error: mdBook book directory not created")
        return 1

    print("mdBook test PASSED!")

    if serve:
        print("\nStarting mdBook server...")
        cmd = ["mdbook", "serve"]
        if open_browser:
            cmd.append("--open")
        subprocess.run(cmd, cwd=mdbook_dir)

    return 0


@functional()
@angreal.command(name="all", about="Run all SSG functional tests")
@angreal.argument(
    name="release",
    long="release",
    is_flag=True,
    takes_value=False,
    help="Build in release mode"
)
def functional_all(release=False):
    """Run all SSG functional tests."""
    print("=" * 50)
    print("Running all SSG functional tests")
    print("=" * 50)

    # Build first
    if functional_build(release=release) != 0:
        print("\nBuild failed!")
        return 1

    print()

    # Run MkDocs test
    if functional_mkdocs() != 0:
        print("\nMkDocs Material test FAILED!")
        return 1

    print()

    # Run mdBook test
    if functional_mdbook() != 0:
        print("\nmdBook test FAILED!")
        return 1

    print()
    print("=" * 50)
    print("All SSG functional tests PASSED!")
    print("=" * 50)

    return 0


@functional()
@angreal.command(name="clean", about="Clean generated SSG output")
def functional_clean():
    """Clean generated SSG output files."""
    functional_dir = get_functional_dir()
    dirs_to_clean = [
        # MkDocs Material (project-first structure)
        os.path.join(functional_dir, "mkdocs-material", "site"),
        os.path.join(functional_dir, "mkdocs-material", "docs", "hybrid_binary"),
        # mdBook (project-first structure)
        os.path.join(functional_dir, "mdbook", "book"),
        os.path.join(functional_dir, "mdbook", "src", "hybrid_binary"),
    ]

    for path in dirs_to_clean:
        if os.path.exists(path):
            if os.path.isdir(path):
                shutil.rmtree(path)
                print(f"Removed directory: {path}")
            else:
                os.remove(path)
                print(f"Removed file: {path}")

    print("Clean complete!")
    return 0
