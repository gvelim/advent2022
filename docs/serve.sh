#!/bin/bash

# Exit on error
set -e

# Check if mdbook is installed
if ! command -v mdbook &> /dev/null
then
    echo "mdbook not found, installing..."
    cargo install mdbook
fi

# Serve the book
echo "Serving mdbook on http://localhost:3000"
mdbook serve --open