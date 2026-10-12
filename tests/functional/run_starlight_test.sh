#!/bin/bash
# Functional test for the Starlight template.
#
# This script:
# 1. Builds plissken
# 2. Makes a site from the starlight-aurora starter (hyper on)
# 3. Renders the hybrid_binary fixture into it with the starlight template
# 4. Builds the site at / and under a base path
# 5. Checks every internal link and anchor (starlight-aurora's check-links)
#
# Usage:
#   ./run_starlight_test.sh
#
# Requirements:
#   - Rust toolchain (cargo), git, Node 22 or later (npm)

set -euo pipefail

STARLIGHT_AURORA_REF="${STARLIGHT_AURORA_REF:-v0.2.1}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
FIXTURE_DIR="$PROJECT_ROOT/tests/fixtures/hybrid_binary"

# A real path: the check scripts do nothing when run through a symlink
WORK_DIR="$(cd "$(mktemp -d)" && pwd -P)"
trap 'rm -rf "$WORK_DIR"' EXIT

log_info() {
    echo "[INFO] $1"
}

log_info "Building plissken..."
cargo build -p plissken --manifest-path "$PROJECT_ROOT/Cargo.toml"
PLISSKEN="$PROJECT_ROOT/target/debug/plissken"

log_info "Making a site from the starlight-aurora $STARLIGHT_AURORA_REF starter..."
git clone --quiet --depth 1 --branch "$STARLIGHT_AURORA_REF" \
    https://github.com/colliery-io/starlight-aurora "$WORK_DIR/starlight-aurora"
cp -R "$WORK_DIR/starlight-aurora/template/docs" "$WORK_DIR/docs"
SITE="$WORK_DIR/docs"
API_DIR="$SITE/src/content/docs/reference/api"

log_info "Rendering the fixture with the starlight template..."
"$PLISSKEN" render "$FIXTURE_DIR" -o "$API_DIR" -t starlight

for file in helpers.md rust/hybrid_binary.md rust/hybrid_binary/internal.md _sidebar.mjs; do
    if [[ ! -f "$API_DIR/$file" ]]; then
        echo "[ERROR] Missing file: $API_DIR/$file"
        exit 1
    fi
done

# Import the sidebar group, put it in the Reference group, and turn on hyper
node - "$SITE/astro.config.mjs" <<'EOF'
const fs = require('node:fs');
const path = process.argv[2];
let s = fs.readFileSync(path, 'utf8');
s = s.replace(
    "import starlightAurora from 'starlight-aurora';",
    "import starlightAurora from 'starlight-aurora';\nimport apiSidebar from './src/content/docs/reference/api/_sidebar.mjs';",
);
s = s.replace('starlightAurora()', 'starlightAurora({ hyper: true })');
s = s.replace(
    "{ autogenerate: { directory: 'reference' } }]",
    "{ autogenerate: { directory: 'reference' } }, apiSidebar]",
);
if (!s.includes('apiSidebar]')) throw new Error('astro.config.mjs: sidebar not changed');
fs.writeFileSync(path, s);
EOF

log_info "Installing the site..."
(cd "$SITE" && npm install --no-audit --no-fund)
(cd "$WORK_DIR/starlight-aurora" && npm install --no-audit --no-fund)
CHECK_LINKS="$WORK_DIR/starlight-aurora/scripts/check-links.mjs"

log_info "Building the site at /..."
(cd "$SITE" && DOCS_SITE= DOCS_BASE= npm run build)
grep -q "TaskBuilder" "$SITE/dist/reference/api/helpers/index.html"
node "$CHECK_LINKS" "$SITE/dist" | tee "$WORK_DIR/links.log"
grep -q " 0 broken" "$WORK_DIR/links.log"

log_info "Building the site under /project/..."
rm -rf "$SITE/dist"
(cd "$SITE" && DOCS_SITE=https://example.github.io DOCS_BASE=/project npm run build)
node "$CHECK_LINKS" "$SITE/dist" --base /project/ | tee "$WORK_DIR/links.log"
grep -q " 0 broken" "$WORK_DIR/links.log"

log_info "Starlight test PASSED"
