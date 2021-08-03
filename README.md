# Saturnus

An open-source reimplementation of the modern RTOS microkernel "HorizonKernel" that is
used in the Nintendo Switch.

## About

The original Horizon kernel is written in modern C++ using a design heavily based on
reference-counted C++ objects. As a microkernel, it has a custom IPC system and being
a custom design, it has zero POSIX compatibility elements.

A combination of these factors makes up for a very interesting research project to see
how well the Rust language can handle the design of this kernel in its essence, while
also providing semantic equivalence to the original kernel.

That being said, the goal is not to generate matching binaries to the original but to
have an idiomatic open source reference of this masterpiece while pushing Rust's boundaries
and exploring the capabilities of this language in the process. Maybe we will also extinguish
Linux from the world of embedded software for good as a side effect.

## Code Organisation

TODO: Write me.

## Building and Testing

### Dependencies and `cargo-xtask`

Saturnus uses [cargo-xtask] as it's build system which can be invoked by running `cargo xtask`
in the project directory. Execute `cargo xtask help` to see all possible commands xtask can execute.

This has the advantage that the only dependency required to build Saturnus is Rust itself and all the
components of the Rust toolchain specified in the `rust-toolchain.toml` file. These will automatically
be installed by [rustup] if you enter the project directory.

### Building and Running

To build and run the Saturnus Kernel in [QEMU] execute the following command
```bash
cargo kernel run --release
```

`cargo kernel` is an alias for `cargo xtask -p kernel` (see [`.cargo/config.toml`](./.cargo/config.toml) for all aliases)
and thus can be used to execute each action provided by `xtask`:

```bash
# Only build the kernel in debug mode
cargo kernel build

# Run one of the LLVM bintools on the produced kernel binary
cargo kernel llvm size
```

Since Saturnus is composed of multiple packages, all these commands also work for other packages
(e.g. the `loader`: `cargo loader build` / `cargo xtask -p loader build`).

For a complete and up-to-date overview run `cargo xtask help`.

### Testing

TODO: Write once we have proper tests

## Contributing

Because of the official status as a research project and the required familiarity with the
original system, this project is unlikely to accept significant code contributions from people
that are not project developers without prior bikeshedding discussion.

Code quality improvements, additional documentation and bugfixes to the existing codebase
are encouraged and heavily appreciated anytime!

## License

Saturnus is distributed under the terms of either the Apache License (Version 2.0) or the
MIT license, at the user's choice.

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
Contributions to the Saturnus project must be made under the terms of both licenses.

[cargo-xtask]: https://github.com/matklad/cargo-xtask/
[QEMU]: https://www.qemu.org/
[rustup]: https://rustup.rs
