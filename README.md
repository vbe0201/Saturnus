# Saturnus

An open-source reimplementation of the proprietary RTOS microkernel "HorizonKernel"
that is used in the Nintendo Switch.

## About

The original Horizon kernel is written in modern C++ using a design heavily based on
reference-counted C++ objects. It has a custom IPC system and the full system is split
up into many sandboxed processes beyond the actual kernel. Few design choices from UNIX
and POSIX are borrowed.

This project tries to combine Horizon's secure design with the capabilities Rust
provides us with to achieve a semantically equivalent reimplementation of the original
kernel that is capable to serve as a drop-in replacement on real hardware.

## Code Organisation

TODO: Write me.

## Building and Testing

TODO: Write me.

### Testing

TODO: Write once we have proper tests

## Contributing

As a research project and the required familiarity with the mechanics of the proprietary
kernel we're reimplementing, this project is unlikely to accept significant code
contributions from people that are not project developers without prior bikeshedding
discussion.

Code quality improvements, additional documentation and bugfixes to the existing codebase
are encouraged and heavily appreciated anytime!

## License

Saturnus notably consists of two major components:

- The kernel and its corresponding bootstrap loader under [`saturnus/`](./saturnus/)

- The library ecosystem under [`crates/`](./crates/)

Generally speaking, we'd like to keep most of the generic library ecosystem reusable
in the embedded space. As such, we dual-license it under the terms of either the
Apache License (Version 2.0) or the MIT license, at the user's choice. See
[LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.

Code that reimplements mechanisms specific to Horizon OS is licensed under the terms
of the GNU General Public License v2. See [`LICENSE-GPL`](./LICENSE-GPL) for more details.

Review the `Cargo.toml` file of each crate to learn about specific licensing.
