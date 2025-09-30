# Rum

`rum` is a compact Rust implementation of the Universal Machine (UM), the
register-based virtual computer defined in the ICFP 2006 contest. It loads a UM
binary, emulates the machine's segmented memory and instruction set, and writes
any output produced by the program to standard output. The interpreter was
originally written as part of a class homework assignment.

## Run

```bash
cargo run --release -- <path-to-um-binary>
```
