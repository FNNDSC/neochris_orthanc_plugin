# Orthanc Plugins in Rust

This [cargo workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html) contains:

| Crate                            | Version                                                                                          | Description                                                              |
|----------------------------------|--------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------|
| [orthanc_sdk](./orthanc_sdk)     | [![Crates.io Version](https://img.shields.io/crates/v/orthanc_sdk)][orthanc_sdk]                 | Abstractions for developing a Rust Orthanc plugin                        |
| [orthanc_client_ogen][ogen]      | [![Crates.io Version](https://img.shields.io/crates/v/orthanc_client_ogen)][orthanc_client_ogen] | Automatically generated Orthanc client using OpenAPI                     |
| [orthanc_api](./orthanc_api)     | [![Crates.io Version](https://img.shields.io/crates/v/orthanc_api)][orthanc_api]                 | Hand-written types for the Orthanc API                                   |
| [example_plugin][example]        | N/A                                                                                              | Example Orthanc plugin using [orthanc_sdk][orthanc_sdk]                  |
| [blt](./blt)                     | ![Cargo.toml Version][blt-badge]                                                                 | Orthanc plugin for automating the Boston Children's Hospital BLT project |

[ogen]: ./orthanc_client_ogen_overlay
[example]: ./example_plugin/src/plugin.rs
[orthanc_api]: https://crates.io/crates/orthanc_api
[orthanc_sdk]: https://crates.io/crates/orthanc_sdk
[orthanc_client_ogen]: https://crates.io/crates/orthanc_client_ogen
[blt-badge]: https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fgithub.com%2FFNNDSC%2Forthanc-rs%2Fraw%2Frefs%2Fheads%2Fmaster%2Fblt%2FCargo.toml&query=package.version&label=Cargo.toml

## Development

Dependencies for codegen and testing are listed in [./flake.nix](flake.nix) and can be setup automatically
using [nix develop](https://nix.dev/manual/nix/2.30/command-ref/new-cli/nix3-develop.html).

```shell
nix develop -c just
```

<details>
<summary>

### Instructions for Debian, Ubuntu, or Other

</summary>

> [!WARNING]
> I use Nix myself, so these instructions are untested.

If you don't want to use [Nix](https://nixos.org), install the packages specified
in the `outputs.devShell.buildInputs` section of `flake.nix` manually.

On Ubuntu or Debian, some basic dependencies can be installed using `apt`:

```shell
sudo apt update
sudo apt install just fd-find xh podman-compose
```

You will also need these, which are trickier to install:

- Rust: https://rustup.rs
- Bindgen: https://rust-lang.github.io/rust-bindgen/command-line-usage.html
- OpenAPI Generator: https://openapi-generator.tech/docs/installation
- Podman: https://podman.io/docs/installation

</details>

### Codegen

This repository depends on automatic code generation (codegen) for:

- Rust bindings to Orthanc's C plugin header
- Orthanc API models (and client) generated from the [OpenAPI specification](https://orthanc.uclouvain.be/api/)

```shell
just
```

### Testing

[example_plugin](/example_plugin) is both a well-documented example plugin and
a test for the [orthanc_sdk](./orthan_sdk) and [orthanc_api](orthanc_api) crates.

```shell
cd example_plugin
just up -d
sleep 1

just test

just down
```
