# regolith-doctor

A tool that diagnoses problems in a [Regolith Desktop](https://regolith-desktop.com) installation and fixes them where that is safe. It knows the common ways a Regolith system goes wrong (apt sources, package sets, window manager partials, Xresources, looks, session environment, bars, displays, keyboard, power) and explains each finding with the file and line it comes from.

It has a command-line interface; a GTK4 window is planned on top of the same core.

## Status

Early development. The workspace, the command-line skeleton, the core types and one check (ENV-001, unsupported platform) exist. The other checks are documented in [docs/checks](docs/checks) but not implemented yet; commands that are not implemented exit with status 4.

- [docs/architecture.md](docs/architecture.md): how it is meant to work.
- [docs/cli.md](docs/cli.md): commands, options, exit codes and JSON output.
- [docs/checks](docs/checks): one page per check.

## Build and test

You need a Rust toolchain; `rust-toolchain.toml` pins the version and `rustup` installs it on first use.

```sh
cargo build --release
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

Run it:

```sh
target/release/regolith-doctor check
target/release/regolith-doctor explain ENV-001
target/release/regolith-doctor --root tests/fixtures/env-001/arch --json check
```

`--root` makes the doctor read another system tree instead of `/`, which is how the tests run checks against the fixtures in `tests/fixtures/`.

## Layout

| Path | Contents |
|---|---|
| `crates/doctor-core` | Check ids, findings, reports, the check registry, file access under `--root` |
| `crates/doctor-checks` | The checks and their compiled-in documentation |
| `crates/regolith-doctor` | The command-line program |
| `data/` | Tables the checks use, with the source of each fact |
| `docs/` | Design, command line and per-check documentation |
| `tests/fixtures/` | System trees the tests run against |

## License

Apache-2.0, see [LICENSE](LICENSE).
