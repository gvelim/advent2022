#!/bin/bash

# Exit on error
set -e

# Check if mdbook is installed
if ! command -v mdbook &> /dev/null
then
    echo "mdbook not found, installing..."
    cargo install mdbook
fi

# Build the book
echo "Building mdbook..."
mdbook build

echo "Book built successfully in ./book directory"