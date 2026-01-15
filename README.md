# EPANET-RS

A very fast, modern and *safe* re-implementation of the EPANET2 hydraulic solver, written in Rust.

## Features

- **Global Gradient Algorithm** (Todini & Pilati, 1987) for hydraulic simulation
- **Hazen-Williams** and **Darcy-Weisbach** headloss formulas
- **Pump curves** with single-point curve support
- **Demand patterns** for extended period simulation
- **Parallel solving** for independent timesteps using Rayon
- **Sparse matrix solver** using faer with symbolic Cholesky factorization
- **INP file format** compatible with EPANET2

## Usage

```bash
# Run simulation
epanet-rs <network_file.inp>

# Run with output file (JSON or MessagePack)
epanet-rs <network_file.inp> output.json

# Run with verbose output
epanet-rs <network_file.inp> --verbose

# Run with parallel solving (for extended period simulations)
epanet-rs <network_file.inp> --parallel

# Print results to console
epanet-rs <network_file.inp> --print-results
```

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

## Testing

```bash
# Run all tests
cargo test

# Run solver tests
cargo test --test solver_test
```

## Supported Network Elements

| Element | Status |
|---------|--------|
| Junctions | Supported |
| Reservoirs | Supported |
| Tanks | Not yet implemented |
| Pipes | Supported |
| Pumps | Supported (single-point curves) |
| Valves | Partial |

## Dependencies

- [faer](https://crates.io/crates/faer) - Sparse linear algebra
- [rayon](https://crates.io/crates/rayon) - Parallel iteration
- [hashbrown](https://crates.io/crates/hashbrown) - Fast hash maps
- [serde](https://crates.io/crates/serde) - Serialization
- [clap](https://crates.io/crates/clap) - Command line parsing

## License

See [LICENSE](LICENSE) file.

## Author

Abel Heinsbroek (Vitens N.V.)
