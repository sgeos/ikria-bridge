# Handoff

> **Navigation**: [Process](README.md) | [Documentation Root](../README.md)

## State on 2026-11-18

Written by Codex for Claude on `docs/codex-claude-bridge-handoff`, based on main
`5d63344`. The operator requested handoff preparation after the authorized ordered
publication. Ikria [pull request 34](https://github.com/sgeos/ikria/pull/34) merged as
`583e1d4`, then bridge [pull request 5](https://github.com/sgeos/ikria-bridge/pull/5)
merged as `5d63344`. Both main checkouts were synchronized and clean. These new
handoff changes are local and unpushed. No implementation publication remains open.

The Rust bridge uses released `krpc-client` 0.7.0 and Ikria's version 2 flight records.
It admits one empty IKR-TS-001B in the verification save, supports telemetry-only and
explicitly armed sessions, holds and steps physics ticks, validates whole commands,
and supplies rotating-body velocity and transformed vessel-frame rates. It contains
no flight law. Career writes, staging, debug setters and autopilot are unsupported.

Ikria owns the Keleusma normal and independent abort programs, policy tests, journals
and replay. Its VR-0010 records normal and coast-abort flights recovered with all eight
parts. I-0001 retains an earlier empty-article loss caused by moving-frame velocity.
I-0002 retains a touchdown bounce fault. Both were corrected before completed runs.
The last game observation was on 2026-11-17 at the verification Space Center, with
Jebediah available and the career unchanged. This task did not query or change the game.

## Receiving Claude line and open work

1. Follow `CLAUDE.md` startup and read `REVERSE_PROMPT.md`. The paired Ikria handoff is
   on `docs/codex-claude-handoff` under the main Ikria checkout's
   `tmp/worktrees/docs-codex-claude-handoff`. Review both local handoff commits before
   publication under the receiving line's applicable authority. No mailbox is assigned.
2. Keep repositories as siblings named `ikria-bridge` and `ikria`, or provide equivalent
   paths. Worktrees under Ikria's tmp use a sibling `ikria` symlink. The manifest's
   `../ikria/crates/ikria-records` dependency is intentional. The workflow pins Ikria
   revision `504d4c4`, preserved in merged ancestry. Retain earlier worktrees and logs.
3. Career qualification remains open. The empty capsule has no control source, and
   successful part-level commands are verification instrumentation only. Active
   attitude actuation, general and multiple vessels, persistent identity, separation
   and atomic batching remain future work. Ikria's original T12 sandbox run remains open.
4. Recheck the server hold timeout before changing the installation. The bridge's
   700 millisecond deadline assumes the observed one-second server hold. kRPC uses
   localhost ports 50000 and 50001. Bridge loopback port 50120 was exercised. JavaScript
   Object Notation cost and batched throughput remain measurement and optimization work.
5. Read Ikria's operating guide and incident evidence before live use. Recheck actual
   save, scene, crew and craft. Use a fresh abort-request path for a normal flight.
   Whole-command authorization precedes sequential application, which is not atomic.
   Never retry an uncertain outcome. A failed or cancelled remote procedure call
   discards its connection. Healthy shutdown attempts neutralization and tick release.
   Solid motors cannot be stopped. Transport loss does not guarantee recovery.

## Preserved constraints and publication evidence

The repository remains GPL-3.0-only, with the process boundary in `BRIDGE_SCOPE.md`.
Outside contributions are not accepted. The implementation was written from the need
and released dependency interfaces, without issue, pull-request or fork code.
Keleusma itself was not modified. Local transport trusts the local account and is not
authenticated. Acknowledgement establishes returned calls, not physical outcome.

Main remains protected and requires a pull request with a green `CI result`. Cargo
dependency updates remain manual because Dependabot cannot resolve the sibling path.
The first publication workflow audit rejected the template's example toolchain pin.
The corrected pin `683b588c487a69d01f6c6ddf40f036ff682cc2c8` was verified against the
official repository. No audit was suppressed. Corrected pre-merge run `44255914853`
passed, followed by post-merge run `44256254190`. Ikria's post-merge run `44255410252`
also passed. The operator's publication approval was session-specific.

## Validity

```sh
git status --short
git log --oneline -5
RUSTC_WRAPPER= ./scripts/check.sh
```

The checkout should be clean, its newest commit should describe this state, and the
gate should exit zero. The inherited report was stale about publication. Local gates
pass on macOS with Rust 1.100.0, including formatting, Clippy with warnings denied,
repository hygiene and three offline asynchronous framing and deadline tests, none
ignored. Ikria separately reaches 60 Rust and 13 Python tests. Live observations are
separate evidence. Remote results cover merged implementations, not these local
handoff commits. Report checks not run and commits after this rewrite.

## Mailbox

None assigned. Coordination remains through the operator.
