# Verdi

Welcome to **Verdi**, a modern, elegant, and fresh Wayland compositor that aims
to reshape the Wayland ecosystem from the ground up. Verdi isn't just another
compositor; it's a complete reimagining of what a Wayland compositor can be,
built to empower developers and users alike with performance and flexibility
without sacrificing on stability.

## Overview

Unlike many other Wayland compositors, **Verdi** is crafted from scratch, free
from reliance on existing libraries, providing a fresh and innovative
architecture. At its core, Verdi is powered by this cutting-edge technologies:

- **[Tokio](https://github.com/tokio-rs/tokio)**: A state-of-the-art
  asynchronous runtime for the Rust programming language, enabling Verdi to
  handle the async nature of the protocol without fear.
- **[wgpu](https://github.com/gfx-rs/wgpu)**: A modern and powerful graphics API
  that ensures Verdi is not only performant but also compatible with a broad
  range of hardware, including more esoteric setups like NVIDIA GPUs.

## Documentation

Comprehensive documentation for Verdi is available at
[docs.verdi.rocks](https://docs.verdi.rocks). Keep in mind that documentation is
still work in progress and stuff might be missing

## Community and Support

Join our growing community on [Discord](https://chat.verdi.rocks). This is the
primary hub for discussion, support, and collaboration. Whether you're a
developer interested in contributing or a user eager to test the latest
features, our community is here to help.

## Getting Started

### Configuration

Verdi is highly configurable via a simple TOML file. The main configuration file
can be found at:

`$XDG_CONFIG_HOME/verdi/verdi.toml` (Typically `~/.config/verdi/verdi.toml` on
most systems).

For detailed configuration options, visit our
[documentation](https://docs.verdi.rocks/configuration).

### Installation

If you're using an Arch-based distribution, Verdi is available as a development
package in the AUR. You can easily install it using your favorite AUR helper:

```bash
paru -S verdi-git
```

Support for additional distributions is on the roadmap and will be available
soon.

### Building from Source

Before you start building Verdi, ensure that your environment meets the
following prerequisites:

- **[just](https://just.system)**: Must be installed and available in your
  system's `PATH`.
- **Rust**: Installed via [rustup](https://rustup.rs/). Note that
  distro-provided versions of Rust are not supported.
- **System Libraries**: Ensure that `libinput` and `libudev` libraries are
  installed, along with their corresponding headers.

Once the prerequisites are in place, you can build and install Verdi with:

```bash
just && just install
```

For more detailed build instructions, check the
[documentation](https://docs.verdi.rocks/building).

## Roadmap and Future Development

Verdi is still in its early stages, with many exciting features and improvements
planned. Our focus is on stability, performance, and expanding support for a
wider range of hardware. The mission of Verdi extends beyond building a stable
and feature-rich compositor. Verdi is an ambitious project that seeks to
contribute to the overall advancement of the Wayland ecosystem. Every line of
code in Verdi is written in Rust, ensuring memory safety, concurrency without
data races, and reliability.

## Contributing

<!-- We welcome contributions from the community! Whether you're interested in
coding, reporting bugs, improving documentation, or just providing feedback,
your input is invaluable. Check out our
[contributing guidelines]() to get started. -->

TBA

## License

This project is licensed under the
[Apache-2.0 License](http://www.apache.org/licenses/LICENSE-2.0). For more
information, please see the [LICENSE](LICENSE) file.
