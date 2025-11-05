# Rust TypeScript LSP Server

A lightweight Language Server Protocol (LSP) implementation written in **Rust**, designed to provide fast and reliable language features for **TypeScript** and **JavaScript** projects.

## Overview

This project aims to build a **modern, high-performance alternative** to the official TypeScript Language Server (`tsserver`) with better performance suitable for larger projects.

## Features (Planned)

- Speed

## Architecture

The architecture follows the **LSP protocol** and consists of several modules:

- **Lexer** — Converts source code into tokens.
- **Parser** — Generates an Abstract Syntax Tree (AST) from tokens.
- **AST** — Contains TypeScript AST structures.
- **Server** — Implements the LSP API and serves as a wrapper for the Analyzer.
- **Analyzer** — Performs code analysis and provides language features.

## Why Rust?

Rust provides:

- Performance

## Development

TBD

## License

MIT License — see [LICENSE](LICENSE) for details.

---

> ⚙️ *Work in progress — contributions and ideas are welcome!*