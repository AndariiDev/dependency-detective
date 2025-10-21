# dependency-detective

A CLI tool to check a list of dependencies from a file and checking for their
existence in the file system.

It currently works with C files, but will eventually be file-agnostic (via cli arguments).


## Disclaimer

This is a learning project, and as such will likely have plenty of suboptimal or
subpar implementations. It's only my second ever attempt at writing Rust.


## Roadmap

1. Clean up code + final output formatting (clear list, owo-color or termcolor) --done
2. Comprehensive error handling (custom enum) --done
3. Implement recursive checking ('src/lib' etc) with dir traversal --done
4. .toml config for parsing rules and different project types with serde and
   toml crates
5. Multiple Language Standard support (e.g. Node.js imports, Rust 'use'
   statements, etc) based on loaded .toml config

Advanced:
6. Runtime dependency detection dynamic analysis (with tools like Strace on
   Linux) to indentify dependencies that are checked at runtime, not just those
   declared statically.
