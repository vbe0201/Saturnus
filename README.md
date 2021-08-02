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

TODO: Write me.

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
