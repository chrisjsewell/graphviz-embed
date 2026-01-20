# Pre-generated Parser Files

This directory contains pre-generated Bison/Flex parser outputs for Graphviz.
These files are generated from the `.y` (Bison) and `.l` (Flex) source files
in the vendored Graphviz source tree.

## Why Pre-generate?

By pre-generating these files, users don't need Bison ≥3.0 and Flex installed
to build the crate. This significantly simplifies the build process, especially
on Windows where these tools are less commonly available.

## Regenerating Files

If you update the vendored Graphviz version, you'll need to regenerate these
files. Run the following commands from a system with Bison ≥3.0 and Flex installed:

```bash
# Navigate to the vendored Graphviz source
cd vendor/graphviz

# Generate cgraph parser files
cd lib/cgraph
bison -d -o grammar.c grammar.y
flex --case-insensitive -o scan.c scan.l
cp grammar.c grammar.h scan.c ../../../generated/cgraph/

# Generate common parser files (HTML labels)
cd ../common
bison -d -o htmlparse.c htmlparse.y
cp htmlparse.c htmlparse.h ../../../generated/common/

# Generate expr parser files
cd ../expr
bison --yacc -Wno-yacc --defines --debug --verbose -o exparse.c exparse.y
cp exparse.c exparse.h ../../../generated/expr/
```

## Directory Structure

```
generated/
├── README.md           # This file
├── cgraph/
│   ├── grammar.c       # DOT grammar parser
│   ├── grammar.h       # Parser header
│   └── scan.c          # Lexer
├── common/
│   ├── htmlparse.c     # HTML label parser
│   └── htmlparse.h     # Parser header
└── expr/
    ├── exparse.c       # Expression parser
    └── exparse.h       # Parser header
```

## Graphviz Version

These files were generated from Graphviz version: **14.1.1**

Expat version: **2.7.1**

Last regenerated: 2026-01-20
