# Handoff

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This is a state file. It is overwritten whole on every update and holds only what is in
flight, why, what is owed and the next step, within a budget of 8 KB. History lives in
git. A rewrite that drops an open item says in its commit message whether the item was
completed, moved or abandoned.

## State on 2026-11-14

- **Seeded** by the Ikria line from `sgeos/project-template` at `3954eba`, tag
  `v1.0.8`, at the operator's instruction of 2026-11-14. Nothing is implemented. The
  public repository is `https://github.com/sgeos/ikria-bridge`.
- **Why it exists.** [`docs/decisions/BRIDGE_SCOPE.md`](../decisions/BRIDGE_SCOPE.md).
- **What it is to do.** Ikria's `docs/architecture/HARNESS_DESIGN.md`, sections "The
  bridge to kRPC" and "Stepping contract".

## Next step

1. Wait for Ikria's record crate, which defines the messages between the bridge and the
   harness, before writing the bridge's interface.
2. Then define the first milestone in `docs/roadmap/`. It is expected to connect to
   kRPC 0.7.0 with `krpc-client`, hold and step the game's tick, and read one vessel's
   telemetry.

## Open

- **The default branch is protected** by the template ruleset once applied, so changes
  reach `main` only by pull request with a green `CI result` check.
- **Gate.** `scripts/check.sh` holds only the template's checks. Add Rust lint, format
  and test commands, and the `cargo` entry in `.github/dependabot.yml`, when the first
  crate arrives.
- **Licence version.** GPL-3.0-only was the seeding line's choice. See the decision
  record.

## Environment

Observed by the Ikria line on 2026-11-14.

- **kRPC 0.7.0** runs in the operator's game and listens on `127.0.0.1`, ports 50000 and
  50001, once a save is loaded but not at the main menu.
- **Tick hold works.** From the Python client, `hold_tick` with `next_tick` stepped the
  game 0.02 s of game time per tick, so 50 ticks per second, at about 19.9 ms of wall
  time per held cycle at the Space Center.

## Validity

```sh
git status --short          # clean
git log --oneline -5        # the newest commit should describe this state
./scripts/check.sh          # exits zero
```

Report any check not run, and any commit after the one that last rewrote this file.

## Mailbox

None assigned. The Ikria line coordinates through the operator.
