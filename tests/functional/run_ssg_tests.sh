#!/bin/bash
# Functional tests for plissken SSG integrations
#
# This script:
# 1. Builds plissken
# 2. Renders documentation for the hybrid_binary fixture
# 3. Builds the docs with each supported SSG
# 4. Validates the output
#
# Usage:
#   ./run_ssg_tests.sh [--mkdocs-only|--mdbook-only]
#
# Requirements:
#   - Flox (flox activate provides mkdocs-material and mdbook)
#   - Rust toolchain (cargo)

set -e

# Ensure we're running in flox environment
if [[ -z "$FLOX_ENV" ]]; then
    echo "This script must be run within a flox environment."
    echo "Run: flox activate"
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
FIXTURE_DIR="$PROJECT_ROOT/tests/fixtures/hybrid_binary"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Parse arguments
RUN_MKDOCS=true
RUN_MDBOOK=true

while [[ $# -gt 0 ]]; do
    case $1 in
        --mkdocs-only)
            RUN_MDBOOK=false
            shift
            ;;
        --mdbook-only)
            RUN_MKDOCS=false
            shift
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Build plissken
log_info "Building plissken..."
cd "$PROJECT_ROOT"
cargo build --release -p plissken
PLISSKEN="$PROJECT_ROOT/target/release/plissken"

if [[ ! -f "$PLISSKEN" ]]; then
    log_error "plissken binary not found at $PLISSKEN"
    exit 1
fi

log_info "plissken built successfully"

# =============================================================================
# MkDocs Material Test
# =============================================================================

test_mkdocs_material() {
    log_info "Testing MkDocs Material integration..."

    local MKDOCS_DIR="$SCRIPT_DIR/mkdocs-material"
    local DOCS_DIR="$MKDOCS_DIR/docs"

    # mkdocs is provided by flox environment
    if ! command -v mkdocs &> /dev/null; then
        log_error "mkdocs not found. Make sure flox environment is activated."
        return 1
    fi

    # Render documentation with mkdocs-material template
    log_info "Rendering docs with mkdocs-material template..."
    "$PLISSKEN" render "$FIXTURE_DIR" -o "$DOCS_DIR" -t mkdocs-material

    # Verify rendered files exist
    local REQUIRED_FILES=(
        "helpers.md"
        "rust/hybrid_binary.md"
        "rust/hybrid_binary/internal.md"
    )

    for file in "${REQUIRED_FILES[@]}"; do
        if [[ ! -f "$DOCS_DIR/$file" ]]; then
            log_error "Missing required file: $DOCS_DIR/$file"
            return 1
        fi
    done

    log_info "All required files present"

    # Verify MkDocs Material CSS variables are used
    if ! grep -q "var(--md-" "$DOCS_DIR/helpers.md"; then
        log_error "MkDocs Material CSS variables not found in output"
        return 1
    fi

    log_info "MkDocs Material CSS variables verified"

    # Build MkDocs site
    log_info "Building MkDocs site..."
    cd "$MKDOCS_DIR"
    mkdocs build || {
        log_error "MkDocs build failed"
        return 1
    }

    # Verify built site
    if [[ ! -d "$MKDOCS_DIR/site" ]]; then
        log_error "MkDocs site directory not created"
        return 1
    fi

    # Check that HTML was generated with our content
    if ! grep -q "TaskBuilder" "$MKDOCS_DIR/site/helpers/index.html" 2>/dev/null; then
        log_error "TaskBuilder class not found in built HTML"
        return 1
    fi

    log_info "MkDocs Material test PASSED"
    return 0
}

# =============================================================================
# mdBook Test
# =============================================================================

test_mdbook() {
    log_info "Testing mdBook integration..."

    local MDBOOK_DIR="$SCRIPT_DIR/mdbook"
    local SRC_DIR="$MDBOOK_DIR/src"

    # mdbook is provided by flox environment
    if ! command -v mdbook &> /dev/null; then
        log_error "mdbook not found. Make sure flox environment is activated."
        return 1
    fi

    # Render documentation with mdbook template
    # Note: mdbook template creates full project structure (book.toml, src/, theme/)
    log_info "Rendering docs with mdbook template..."
    "$PLISSKEN" render "$FIXTURE_DIR" -o "$MDBOOK_DIR" -t mdbook

    # Verify rendered files exist
    local REQUIRED_FILES=(
        "helpers.md"
        "rust/hybrid_binary.md"
        "rust/hybrid_binary/internal.md"
    )

    for file in "${REQUIRED_FILES[@]}"; do
        if [[ ! -f "$SRC_DIR/$file" ]]; then
            log_error "Missing required file: $SRC_DIR/$file"
            return 1
        fi
    done

    log_info "All required files present"

    # Verify mdBook CSS variables are used (not MkDocs ones)
    if grep -q "var(--md-" "$SRC_DIR/helpers.md"; then
        log_error "MkDocs CSS variables found in mdBook output (should use mdBook variables)"
        return 1
    fi

    # Should have generic CSS variables
    if ! grep -q "var(--" "$SRC_DIR/helpers.md"; then
        log_error "CSS variables not found in output"
        return 1
    fi

    log_info "mdBook CSS variables verified"

    # Build mdBook site
    log_info "Building mdBook site..."
    cd "$MDBOOK_DIR"
    mdbook build || {
        log_error "mdBook build failed"
        return 1
    }

    # Verify built site
    if [[ ! -d "$MDBOOK_DIR/book" ]]; then
        log_error "mdBook book directory not created"
        return 1
    fi

    # Check that HTML was generated with our content
    if ! grep -q "TaskBuilder" "$MDBOOK_DIR/book/helpers.html" 2>/dev/null; then
        log_error "TaskBuilder class not found in built HTML"
        return 1
    fi

    log_info "mdBook test PASSED"
    return 0
}

# =============================================================================
# Main
# =============================================================================

FAILED=0

if [[ "$RUN_MKDOCS" == "true" ]]; then
    if ! test_mkdocs_material; then
        log_error "MkDocs Material test FAILED"
        FAILED=1
    fi
    echo
fi

if [[ "$RUN_MDBOOK" == "true" ]]; then
    if ! test_mdbook; then
        log_error "mdBook test FAILED"
        FAILED=1
    fi
    echo
fi

# Summary
echo "=============================================="
if [[ $FAILED -eq 0 ]]; then
    log_info "All SSG tests PASSED!"
else
    log_error "Some SSG tests FAILED"
fi
echo "=============================================="

exit $FAILED
