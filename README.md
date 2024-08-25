# Utility Collection

This repository is a Rust workspace containing a collection of small, reusable
utilities. It serves as a single source for various tools and libraries that
can be used across different projects.

Currently included utilities:

- **TripleBuffer**: A container for a triple buffer pattern.

## Project Structure

The workspace contains the following crates:

- **tc**: The main crate that re-exports utilities from other crates.
- **tc_triple_buffer**: Contains the implementation of the `TripleBuffer`.

