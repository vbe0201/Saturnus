# Saturnus

An open-source reimplementation of the proprietary RTOS microkernel "HorizonKernel"
that is used in the Nintendo Switch.

## About

The original Horizon kernel is written in modern C++, using a recent LLVM toolchain.

It is a fully custom design heavily based around reference-counted C++ object. Using
a custom IPC system, most of the work in the system is orchestrated between sandboxed
processes following a microservice model.

This project tries to combine Horizon's modern design choices with the capabilities
Rust provides as a systems programming language.

We try to achieve semantic equivalence in reimplementation, less effort is put in
trying to generate matching assembly. Nonetheless, it is a goal for the kernel to
work as a drop-in replacement on real hardware.

Eventually, it may be ported to more architectures than just AArch64 in the future.

## Code Organisation

TODO: Write me.

## Building and Testing

The `cargo kernel` command shall be used for building and testing Saturnus.

Most commonly, one will want to use the `cargo kernel -t aarch64-qemu run --release`
which builds the Kernel for the `aarch64-qemu` target and runs it in QEMU.

The `build` and `run` commands will always produce a ready kernel image as
`target/dist/kernel.bin`.

See `cargo kernel help` for a full list of usable commands and their options.

### Testing

TODO: Write once we have proper tests.

## Porting

TODO: Write me.

## Contributing

Due to the required familiarity with the mechanics of the proprietary kernel that
is being reimplemented here, this project is unlikely to accept significant code
contributions from outsiders without prior bikeshedding discussion.

## License

Saturnus consists of many small crates which compose the full kernel.

Generally, each of these crates may be licensed under either the
[ISC License](./LICENSE-ISC) or the [GNU GPLv2](./LICENSE-GPL).

See the `Cargo.toml` file of a particular crate to learn its license. All
code in a crate shall be licensed under the license stated there unless
where specifically noted.
