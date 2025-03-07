# Verdi

Verdi is a Wayland compositor written in Rust, designed with a focus on
performance, stability, and extensibility. The project implements its core
functionality from the ground up, taking a fresh approach to compositor
architecture while maintaining clean and maintainable code.

## Overview

Verdi takes an independent approach to Wayland compositor design, implementing
core functionality without relying on existing compositor libraries. The
architecture is built on two main foundations:

- **[Tokio](https://github.com/tokio-rs/tokio)**: An asynchronous runtime for
  Rust that provides robust handling of the Wayland protocol's asynchronous
  nature
- **[wgpu](https://github.com/gfx-rs/wgpu)**: A cross-platform graphics API
  providing broad hardware compatibility and high performance across different
  platforms

## Documentation

Documentation is available at [docs.verdi.rocks](https://docs.verdi.rocks).
Please note that the documentation is currently under development and may be
incomplete in some areas.

## Community and Support

For questions, discussions, and development updates, join our community on
[Discord](https://chat.verdi.rocks). Whether you're interested in contributing
to development or testing new features, the community is here to help.

## Getting Started

### Installation

For Arch-based distributions, Verdi is available in the AUR as a development
package:

```bash
paru -S verdi-git
```

Support for additional distributions is in development.

### Configuration

Verdi uses Corn for configuration. The default configuration file location is:

`$XDG_CONFIG_HOME/verdi/verdi.corn` (typically `~/.config/verdi/verdi.corn`)

For configuration options and examples, see the
[documentation](https://docs.verdi.rocks/configuration).

### Building from Source

Prerequisites:

- **Rust**: Install via [rustup](https://rustup.rs/) (distribution-provided Rust
  versions are not supported)
- **System Libraries**: `libinput` and `libudev` with development headers

Build and install commands:

```bash
cargo xtask build && cargo xtask install
```

Detailed build instructions are available in the
[documentation](https://docs.verdi.rocks/building).

## Development Status

Verdi is currently in pre-alpha stage, with active development focusing on core
functionality and stability.

The project aims to contribute to the Wayland ecosystem by providing a reliable,
performant implementation that can serve as a foundation for future development.
Each component is carefully designed and implemented with a focus on long-term
maintainability and reliability.

## License

This project is licensed under the
[Apache-2.0 License](http://www.apache.org/licenses/LICENSE-2.0). For more
information, please see the [LICENSE](LICENSE) file.
