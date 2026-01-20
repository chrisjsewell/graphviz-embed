#!/bin/bash
# Generate parser files for graphviz-embed
#
# This script generates the Bison/Flex parser files from Graphviz source.
# Requires Bison >= 3.0 and Flex to be installed.
#
# On macOS: brew install bison flex
# On Linux: apt-get install bison flex

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VENDOR_DIR="$SCRIPT_DIR/vendor/graphviz"
GENERATED_DIR="$SCRIPT_DIR/generated"

# Check for bison
if command -v /opt/homebrew/opt/bison/bin/bison &> /dev/null; then
    BISON="/opt/homebrew/opt/bison/bin/bison"
elif command -v /usr/local/opt/bison/bin/bison &> /dev/null; then
    BISON="/usr/local/opt/bison/bin/bison"
elif command -v bison &> /dev/null; then
    BISON="bison"
else
    echo "Error: bison not found. Please install bison >= 3.0"
    exit 1
fi

# Check bison version
BISON_VERSION=$($BISON --version | head -1 | grep -oE '[0-9]+\.[0-9]+' | head -1)
BISON_MAJOR=$(echo $BISON_VERSION | cut -d. -f1)
if [ "$BISON_MAJOR" -lt 3 ]; then
    echo "Error: bison >= 3.0 required, found $BISON_VERSION"
    echo "On macOS, install with: brew install bison"
    exit 1
fi

# Check for flex
if ! command -v flex &> /dev/null; then
    echo "Error: flex not found. Please install flex"
    exit 1
fi

echo "Using bison: $BISON ($BISON_VERSION)"
echo "Using flex: $(flex --version)"
echo ""

# Generate cgraph parser
echo "Generating cgraph parser files..."
cd "$VENDOR_DIR/lib/cgraph"
$BISON -d -o grammar.c grammar.y
flex --case-insensitive -o scan.c scan.l
cp grammar.c grammar.h scan.c "$GENERATED_DIR/cgraph/"
echo "  ✓ grammar.c, grammar.h, scan.c"

# Generate common (HTML) parser
echo "Generating common (HTML label) parser files..."
cd "$VENDOR_DIR/lib/common"
$BISON -d -o htmlparse.c htmlparse.y 2>/dev/null || true
cp htmlparse.c htmlparse.h "$GENERATED_DIR/common/"
echo "  ✓ htmlparse.c, htmlparse.h"

# Generate expr parser
echo "Generating expr parser files..."
cd "$VENDOR_DIR/lib/expr"
$BISON --yacc -Wno-yacc --defines --debug --verbose -o exparse.c exparse.y 2>/dev/null || true
cp exparse.c exparse.h "$GENERATED_DIR/expr/"
echo "  ✓ exparse.c, exparse.h"

# Cleanup generated files in vendor directory
echo ""
echo "Cleaning up vendor directory..."
rm -f "$VENDOR_DIR/lib/cgraph/grammar.c" "$VENDOR_DIR/lib/cgraph/grammar.h" "$VENDOR_DIR/lib/cgraph/scan.c"
rm -f "$VENDOR_DIR/lib/common/htmlparse.c" "$VENDOR_DIR/lib/common/htmlparse.h"
rm -f "$VENDOR_DIR/lib/expr/exparse.c" "$VENDOR_DIR/lib/expr/exparse.h" "$VENDOR_DIR/lib/expr/exparse.output"

echo ""
echo "Done! Parser files generated in $GENERATED_DIR"
echo ""
echo "Generated files:"
ls -la "$GENERATED_DIR"/*/*.c "$GENERATED_DIR"/*/*.h
