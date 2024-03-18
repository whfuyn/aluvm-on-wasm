# AluVM on WASM

This project demonstrates the execution of a sample script using AluVM on WebAssembly (WASM).

## Usage

Use the provided Makefile for building and running the project. The available commands are:

```bash
make          # Defaults to make debug
make debug    # Compiles the project and starts it in debug mode
make release  # Compiles the project and starts it in release mode
make clean    # Removes the compiled files