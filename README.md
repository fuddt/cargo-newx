# cargo-newx

A cargo extension that creates new Rust projects with best practice configuration files.

## Overview

`cargo-newx` is a custom CLI command for initializing Rust projects. In addition to the standard `cargo new`, it automatically generates best practice configuration files like `rustfmt.toml`.

## Installation

```bash
cargo install --path .
```

## Basic Usage

```bash
cargo newx myproject
```

This command:
- Runs `cargo new myproject` internally
- Automatically generates `myproject/rustfmt.toml` (default)
- Does not include other additional files

## Options

| Option      | Description                                                    |
|-------------|----------------------------------------------------------------|
| `--clippy`  | Add `clippy.toml` configuration                                |
| `--all`     | Generate both `rustfmt.toml` and `clippy.toml`               |
| `--lib`     | Initialize as a library project (default is `--bin`)          |

## Examples

```bash
# Basic project with rustfmt.toml
cargo newx myproject

# Add clippy configuration
cargo newx myproject --clippy

# Library project with both configurations
cargo newx myproject --all --lib
```

## Project Structure

```
cargo-newx/
├── src/
│   └── main.rs
├── templates/
│   ├── rustfmt.toml
│   └── clippy.toml
├── Cargo.toml
├── README.md
```



