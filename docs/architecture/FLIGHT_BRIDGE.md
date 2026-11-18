# Flight Bridge

> **Navigation**: [Architecture](README.md)

**Implemented by Codex, 2026-11-17.** A single-session Rust bridge connects the Ikria
flight harness to kRPC 0.7.0. It uses the released `krpc-client` crate, pinned to
0.7.0, and Ikria's version 2 flight records. The bridge contains no control law.

## Build and launch

Keep this checkout beside a checkout named `ikria`. The records dependency is
`../ikria/crates/ikria-records`. Continuous integration checks out the matching
Ikria revision explicitly. Cargo dependency updates are manual while this path
crosses repositories, because Dependabot cannot resolve it alone.

```sh
RUSTC_WRAPPER= cargo build --locked
./target/debug/ikria-bridge "$SAVES_DIRECTORY" IKR-TS-001B 50120 12005
```

This admits a telemetry-only session on loopback port 50120. Add
`--arm-verification` only for the prepared empty revision B. Use Ikria's `fly`
executable for `monitor`, `pad-abort` or `hop`. The bridge accepts one connection,
then exits. Its maximum tick count must exceed the harness count so the harness can
close after its final acknowledged command. Details of preparation, program loading,
abort requests and replay are in Ikria's `docs/architecture/FLIGHT_CONTROL.md`.

## Boundaries

A unique temporary save snapshot establishes the actual loaded save. The bridge
requires `Ikria Physical Verification`, a flight scene, the expected active vessel
and zero crew. Arming additionally checks the eight expected starting parts, 35 units
of solid fuel, the 20 percent thrust limit, prelaunch situation, an inactive motor
and a stowed parachute. Built-in attitude assistance must be off.

The save-load generation, scene, active vessel and warp settings are checked each
tick. Vessel identity 1 and part identities 1 and 2 are local to this connection.
They are not persistent game identifiers. The shared profile policy validates the
whole command before any write, including exact vessel and part identities, available
control authority and the parachute deployment envelope. Staging, arbitrary events,
career writes, debug setters and autopilot are not exposed.

The transport is local and unauthenticated. It assumes trusted processes on the local
account. Strict message, capability and identity checks do not protect against a
compromised account or another direct kRPC client.

## Frames and stepping

The local up vector comes from the vessel's surface frame and is transformed into
vessel coordinates. Velocity comes from the rotating body frame. Angular velocity is
measured relative to that body frame and then transformed into vessel coordinates.
The vessel frame moves and rotates with the vessel, so reading velocity directly
relative to it can return zero. The first verification flight exposed that mistake.
The record in Ikria's VR-0010 preserves the failed run rather than treating it as a pass.

Each cycle uses `NextTick`, reads state, exchanges a sample and command, checks that
game time still matches, applies validated commands and acknowledges them. The
700 millisecond cycle deadline is below the observed one-second server hold timeout.
The server timeout must retain that configuration. Calls are sequential, not an atomic
batch. A partial application or lost acknowledgement is therefore uncertain and is
never retried. The client exposes generated calls but not their batching builders as
public methods, so batching remains an optimization task.

A healthy session attempts neutralization and tick release on exit. Neutralization
cannot stop a burning solid motor. A cancelled or failed remote procedure call can
leave a partial response, so its connection is discarded without further calls.
Disconnect releases the server hold. A failed cleanup also terminates the connection.
No streams are awaited during held ticks.

The kRPC release documentation describes the same hold and disconnect semantics in
its [control-loop guide](https://krpc.github.io/krpc/0.7.0/tutorials/control-loops.html).
They are installation and server assumptions, not a guarantee supplied by this bridge.

## Verification limits

The gate runs formatting, Clippy with warnings denied, three asynchronous framing
and deadline tests, and repository checks. The Ikria gate owns shared policy, virtual
machine, protocol and replay tests. Live tests require the prepared game and are
recorded separately. No network or game test is silently counted as an offline pass.

This first bridge supports one verification profile. Multiple vessels, persistent
identity across loads or separation, generic part inventories, career qualification,
atomic batches and authenticated transport remain future work. Changes were written
from the stated need and the released dependencies' interfaces. No outside issue,
pull request or fork implementation was used.
