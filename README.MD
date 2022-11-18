
# Example Project Structure for Rust

This repository is to show the structure of a Rust project as part of our Newsletter see [Rust-Trends.com](https://rust-trends.com/)

It starts with the example given in the Cargo Book. The structure in this repo is similar but not the same.

## The Cargo Book <sup>[1](https://doc.rust-lang.org/stable/cargo/guide/project-layout.html)</sup>

Cargo uses conventions for file placement to make it easy to dive into a new Cargo package:

```
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

- `Cargo.toml` and `Cargo.lock` are stored in the root of your package (package root).
- Source code goes in the `src` directory.
- The default library file is `src/lib.rs`.
- The default executable file is `src/main.rs`.
- Other executables can be placed in `src/bin/`.
- Benchmarks go in the `benches` directory.
- Examples go in the `examples` directory.
- Integration tests go in the `tests` directory. i.e. tests that are meant to test **your library**.

If a binary, example, bench, or integration test consists of multiple source files, place a `main.rs` file along with the extra modules within a subdirectory of the src/bin, examples, benches, or tests directory.

## To run the tests
Running tests is as simple as executing `cargo test`. This will run both the integration tests in your `tests/` directory as the unit tests in your source files.<sup>[2](https://doc.rust-lang.org/stable/rust-by-example/testing.html)</sup> Running a test that contains a term like e.g. dummy `cargo test dummy` only runs the unit test in `src/bin/emulator.rs`

```
#[cfg(test)]
mod tests {
    #[test]
    fn test_dummy() {
        assert_eq!(true, true);
    }
}
```

## To run the examples
In order to run a particular example from the `examples` directory you can use the command `cargo run --example simple` on root level, this will run the example named `examples/simple.rs`. Or try `cargo run --example multi-file-example`.

## To run the binary
If you have a project that needs to compile into multiple binaries you need to add that in the `Cargo.toml` file e.g:

```
[[lib]]
name = "mylib"
path = "src/lib.rs"

[[bin]]
name = "emulator"
src = "src/bin/emulator.rs"
```

You can run the additional binary with `cargo run --bin emulator`

# Important notes:
- Use underscore `_` (snake case) over hyphens `-` in module names. Naming conventions <sup>[3](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md)</sup>
- When running benches with `cargo bench` you might want to install gnuplot, e.g. for Mac you can install it with brew: `brew install gnuplot`. After running the benchmark you can find the HTML report in `targets\criterion`


# References
1. Cargo Book https://doc.rust-lang.org/stable/cargo/guide/project-layout.html
2. Testing Rust by Example book https://doc.rust-lang.org/stable/rust-by-example/testing.html
3. Naming conventions https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md


# Links
1. Blog post on Hackernoon by Jose Javi Asilis https://hackernoon.com/including-files-and-deeply-directories-in-rust-q35o3yer


Join our Newsletter at [Rust-Trends.com](https://rust-trends.com/), for your biweekly dose of Rust Trends.