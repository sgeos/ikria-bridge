# Handoff

> **Navigation**: [Process](README.md) | [Documentation Root](../README.md)

## State on 2026-11-17

Written by Codex on `feat/codex-live-bridge`, based on main `551bb7b`. Remote main was
refreshed and remains that commit. The branch is unpushed. The operator explicitly
required consultation before pushing. No merge or publication is authorized here.

The Rust bridge is implemented with released `krpc-client` 0.7.0 and Ikria's version 2
flight records. It admits one empty IKR-TS-001B in the verification save, supports
telemetry-only and explicitly armed sessions, holds and steps physics ticks, validates
whole commands, and supplies rotating-body velocity and transformed vessel-frame
rates. It has no flight law, career writes, staging, debug setters or autopilot.

The paired Ikria branch is `feat/codex-flight-control`. It owns the Keleusma hop and
abort programs, policy tests, journals and replay. Its VR-0010 records a completed
normal flight and a coast-abort flight, recovered with all eight parts. Earlier
failures are retained in I-0001 and I-0002. The first lost an empty article because
of moving-frame velocity. Both defects were corrected before the completed runs.
The game ended at the verification Space Center, with Jebediah available and the
career file unchanged.

## Dependencies and next step

1. Review both branches and ask before pushing. Publish Ikria first if approved,
   because this repository's continuous integration pins its records revision.
2. Keep the repositories as siblings named `ikria-bridge` and `ikria`, or provide
   the equivalent local path layout. The worktree under Ikria's tmp uses a sibling
   symlink named `ikria`. The manifest path dependency is intentional and documented.
3. Career qualification remains open. The empty capsule has no control source, so
   successful part-level commands are verification instrumentation only. Attitude
   actuation, general vessels, multiple vessels, persistent identity, separation and
   atomic batching remain future work.
4. Recheck the server hold timeout before changing the installation. The bridge's
   700 millisecond deadline assumes the observed one-second server timeout. The
   game was reached at localhost ports 50000 and 50001. The bridge was exercised on
   loopback port 50120. JSON cost and batched request throughput remain measurement
   and optimization work.

## Preserved constraints

The repository remains GPL-3.0-only, with the process boundary and scope from
`BRIDGE_SCOPE.md`. Outside contributions are not accepted. This implementation was
written from the need and released dependency interfaces, without using issue,
pull-request or fork code. Keleusma itself was not modified.

Main remains protected and changes require a pull request with a green `CI result`.
The gate now includes Rust formatting, Clippy with warnings denied and three offline
asynchronous framing and deadline tests. Cargo dependency updates remain manual
because Dependabot cannot resolve a sibling path dependency. The workflow pins the
matching Ikria source. No remote continuous integration result has been observed.

## Validity

```sh
git status --short
git log --oneline -5
RUSTC_WRAPPER= ./scripts/check.sh
```

The checkout should be clean, the newest commit should describe this state, and the
gate should exit zero. Baseline hygiene passed at `551bb7b`. The final gate passes on
macOS with Rust 1.100.0 and three tests, none ignored. Ikria separately covers 60 Rust
and 13 Python tests. Live observations are reported separately, not counted as offline
passes. Report checks not run and commits after the last rewrite of this file.

## Mailbox

None assigned. Coordination remains through the operator.
