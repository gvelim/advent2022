# Advent of Code 2022 Documentation

This directory contains an [mdbook](https://rust-lang.github.io/mdBook/) documentation structure for the Advent of Code 2022 solutions.

## Getting Started

### Prerequisites

To build and view this documentation, you'll need to install mdbook:

```bash
cargo install mdbook
```

### Building the Documentation

From the project root, run:

```bash
cd docs
mdbook build
```

This will generate the HTML documentation in the `docs/book` directory.

### Viewing the Documentation

To view the documentation in your browser:

```bash
cd docs
mdbook serve --open
```

This will start a local web server and automatically open the documentation in your default web browser.

## Documentation Structure

- `src/` - Contains all the Markdown source files
  - `SUMMARY.md` - The table of contents for the book
  - `introduction.md` - Introduction to the project
  - `day*/` - Documentation for each day's solution
  - `patterns/` - Common programming patterns used in solutions
  - `utils/` - Utility functions and helpers

## Adding New Content

1. Update `src/SUMMARY.md` to include new sections or pages
2. Create corresponding Markdown files in appropriate directories
3. Rebuild the book with `mdbook build`

## Contributing

Feel free to enhance this documentation with:

- Better explanations of solutions
- Alternative approaches
- Visualizations
- Performance analysis
- Additional common patterns