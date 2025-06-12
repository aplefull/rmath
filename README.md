# RMath

## Running tests
### All tests
```shell
cargo test
```

### Specific file
```shell
cargo test --test <test_file_name>
# Example: cargo test --test test_power
```

### Running any test with println output
```shell
cargo test <test_name> -- --nocapture
# Example: cargo test test_power -- --nocapture 
```
