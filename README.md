# RMath

A small mathematics library in Rust with interactive graphical plotter.

## Quick Start

```bash
# Run the interactive function plotter
cargo run

# Build the library
cargo build --release

# Run all tests
cargo test
```

## Function Categories

- **Trigonometric**: sin, cos, tan, arcsin, arccos, arctan, sinc
- **Hyperbolic**: sinh, cosh, tanh and inverses
- **Power**: sqrt, cbrt, general power functions
- **Logarithmic**: ln, log2, log10, general logarithms
- **Special**: Lambert W, error functions (erf, erfc)
- **Easing**: smoothstep, smootherstep, lerp, clamp
- **Waves**: square, sawtooth, triangle, pulse waves
- **Geometry**: step, sign, floating-point modulus
- **Animation**: quadratic easing, bounce, elastic effects
- **Noise**: pseudo-random and hash functions

## Usage

```rust
use rmath::*;

// Basic trigonometric functions
let result = sin(std::f64::consts::PI / 2.0); // 1.0

// Easing functions for animations
let smooth = smoothstep(0.5); // 0.5

// Wave functions for signal processing
let wave = square_wave(1.5); // -1.0

// Statistical functions
let avg = mean(&[1.0, 2.0, 3.0, 4.0, 5.0]); // 3.0
```

## Testing

```bash
# All tests
cargo test

# Specific module
cargo test --test trigonometric
cargo test --test easing

# With output
cargo test test_name -- --nocapture
```
