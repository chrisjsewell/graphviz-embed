#!/bin/bash
# Prepare graphviz-sys crate for publishing to crates.io
#
# This script copies vendored sources from git submodules into the crate
# directory so they can be included in the published crate.
#
# Usage: ./scripts/prepare-publish.sh
#
# After running this script:
# 1. Review the changes
# 2. Run `cargo publish --dry-run -p graphviz-sys` to test
# 3. Run `cargo publish -p graphviz-sys` to publish
# 4. Run `cargo publish -p graphviz-embed` to publish the main crate
# 5. Run `./scripts/cleanup-publish.sh` to remove copied sources

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
SYS_CRATE="$ROOT_DIR/graphviz-sys"

echo "Preparing graphviz-sys for publishing..."

# Check that submodules are initialized
if [ ! -f "$ROOT_DIR/vendor/graphviz/CMakeLists.txt" ]; then
    echo "Error: Graphviz submodule not initialized"
    echo "Run: git submodule update --init --recursive"
    exit 1
fi

if [ ! -f "$ROOT_DIR/vendor/expat/expat/CMakeLists.txt" ]; then
    echo "Error: Expat submodule not initialized"
    echo "Run: git submodule update --init --recursive"
    exit 1
fi

# Create vendor directory in crate
mkdir -p "$SYS_CRATE/vendor"

# Copy Graphviz source (excluding .git and unnecessary files)
echo "Copying Graphviz source..."
rsync -av --delete \
    --exclude='.git' \
    --exclude='.github' \
    --exclude='tests' \
    --exclude='rtest' \
    --exclude='doc' \
    --exclude='*.md' \
    --exclude='*.txt' \
    --exclude='*.html' \
    --exclude='*.in' \
    --exclude='Makefile.am' \
    --exclude='configure.ac' \
    --exclude='autogen.sh' \
    --exclude='debian' \
    --exclude='redhat' \
    --exclude='windows' \
    --exclude='macosx' \
    --exclude='share' \
    --exclude='contrib' \
    --exclude='tclpkg' \
    --exclude='dot.demo' \
    --exclude='plugin.demo' \
    --exclude='graphs' \
    "$ROOT_DIR/vendor/graphviz/" "$SYS_CRATE/vendor/graphviz/"

# Copy Expat source (just the expat subdirectory, excluding tests)
echo "Copying Expat source..."
rsync -av --delete \
    --exclude='.git' \
    --exclude='tests' \
    --exclude='doc' \
    --exclude='*.md' \
    --exclude='*.txt' \
    "$ROOT_DIR/vendor/expat/expat/" "$SYS_CRATE/vendor/expat/"

# Copy generated parser files
echo "Copying generated parser files..."
mkdir -p "$SYS_CRATE/generated"
cp -r "$ROOT_DIR/generated/"* "$SYS_CRATE/generated/"

# Copy patches
echo "Copying patches..."
mkdir -p "$SYS_CRATE/patches"
cp -r "$ROOT_DIR/patches/"* "$SYS_CRATE/patches/"

# Calculate sizes
GRAPHVIZ_SIZE=$(du -sh "$SYS_CRATE/vendor/graphviz" | cut -f1)
EXPAT_SIZE=$(du -sh "$SYS_CRATE/vendor/expat" | cut -f1)
TOTAL_SIZE=$(du -sh "$SYS_CRATE/vendor" | cut -f1)

echo ""
echo "Done! Vendored sources copied to graphviz-sys crate:"
echo "  - Graphviz: $GRAPHVIZ_SIZE"
echo "  - Expat: $EXPAT_SIZE"
echo "  - Total vendor/: $TOTAL_SIZE"
echo ""
echo "Next steps:"
echo "  1. Test with: cargo publish --dry-run -p graphviz-sys"
echo "  2. Publish with: cargo publish -p graphviz-sys"
echo "  3. Then publish: cargo publish -p graphviz-embed"
echo "  4. Cleanup with: ./scripts/cleanup-publish.sh"
