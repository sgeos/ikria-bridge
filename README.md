# Ikria Bridge

[![CI](https://github.com/sgeos/ikria-bridge/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sgeos/ikria-bridge/actions/workflows/ci.yml)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

<!--
Add a badge for each place the project is published, after the first release, for example:
[![Crates.io](https://img.shields.io/crates/v/ikria-bridge.svg)](https://crates.io/crates/ikria-bridge)
[![Docs.rs](https://docs.rs/ikria-bridge/badge.svg)](https://docs.rs/ikria-bridge)
-->

The bridge between the [Ikria](https://github.com/sgeos/ikria) harness and the kRPC mod
for Kerbal Space Program 1. It runs as its own process and is licensed GPL-3.0, because
the kRPC client and service definitions it uses are GPL-3.0.

## Status

**Seeded.** Nothing is implemented yet. The design it follows is Ikria's
[`HARNESS_DESIGN.md`](https://github.com/sgeos/ikria/blob/HEAD/docs/architecture/HARNESS_DESIGN.md),
and the decisions that created this repository are in
[`docs/decisions/BRIDGE_SCOPE.md`](docs/decisions/BRIDGE_SCOPE.md).

## Development

```sh
./scripts/check.sh                       # the local gate, which continuous integration also runs
scripts/worktree.sh new feat/some-thing  # an isolated worktree for a parallel line
scripts/scratch.sh status                # whether build output is linked to a scratch volume
```

## Documentation

The documentation index is [`docs/README.md`](docs/README.md). Agent guidance is in
[`CLAUDE.md`](CLAUDE.md), and [`AGENTS.md`](AGENTS.md) points other assistants to it.
Participation is governed by the [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

## Contributing

Outside contributions are not accepted. Fork the bridge instead, and open a fork report
saying why, as [`CONTRIBUTING.md`](CONTRIBUTING.md) explains.

## License

[GPL-3.0-only](LICENSE).
