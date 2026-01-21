#!/bin/bash
# Clean up vendored sources after publishing
#
# This removes the copied vendor/, generated/, and patches/ directories
# from the graphviz-sys crate that were created by prepare-publish.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
SYS_CRATE="$ROOT_DIR/graphviz-sys"

echo "Cleaning up graphviz-sys publish artifacts..."

# Remove copied directories
if [ -d "$SYS_CRATE/vendor" ]; then
    echo "Removing $SYS_CRATE/vendor/"
    rm -rf "$SYS_CRATE/vendor"
fi

if [ -d "$SYS_CRATE/generated" ]; then
    echo "Removing $SYS_CRATE/generated/"
    rm -rf "$SYS_CRATE/generated"
fi

if [ -d "$SYS_CRATE/patches" ]; then
    echo "Removing $SYS_CRATE/patches/"
    rm -rf "$SYS_CRATE/patches"
fi

echo "Done! Vendored sources removed from graphviz-sys crate."
