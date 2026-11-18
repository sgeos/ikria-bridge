# Reverse Prompt

> **Navigation**: [Process](README.md)

Written by Codex on 2026-11-18 for handoff to Claude.

The operator requested handoff preparation after the authorized publication. Ikria
pull request 34 merged as `583e1d4`, then bridge pull request 5 as `5d63344`. Both main
checkouts were synchronized and clean. The bridge workflow's Ikria revision `504d4c4`
is preserved in main ancestry. Implementation publication is complete.

This documentation-only task reconciles stale handoffs and roadmap status, preserves
open work and provides receiving-line instructions. The paired local branches are
`docs/codex-claude-handoff` in Ikria and `docs/codex-claude-bridge-handoff` here. They
have not been pushed. Review before publication under the receiving line's applicable
authority. No new standing authority is inferred.

The bridge checks the actual save, vessel, crew and revision B configuration, binds
one session, exchanges bounded version 2 frames and advances held physics ticks.
Whole commands are authorized before application. Calls are sequential, so partial
application remains possible. Lost acknowledgements are never retried. Failed or
cancelled remote procedure calls discard the connection. Healthy shutdown attempts
neutralization and tick release. Solid motors cannot be stopped by neutralization.

Ikria VR-0010 retains normal and coast-abort flights with eight-part recovery, exact
command replay, hashes and failed development runs. A moving-frame velocity mistake
lost an empty article, then was corrected. A touchdown bounce fault was fixed in the
harness. No crew flew and no career file changed. The game was last observed at the
verification Space Center on 2026-11-17. This task performed no game operations.

Local gates pass on macOS with Rust 1.100.0. The bridge reaches three asynchronous
framing and timeout tests, none ignored, plus formatting, lint and repository hygiene.
Ikria reaches 60 Rust and 13 Python tests. The first bridge workflow audit rejected
an invalid template toolchain pin. Corrective commit `03ea74b` substituted the verified
revision `683b588c487a69d01f6c6ddf40f036ff682cc2c8` without suppressing the audit.
Pre-merge run `44255914853` and post-merge run `44256254190` passed. Ikria post-merge
run `44255410252` passed. These local handoff commits have no remote check result.

The code was written independently from the need and released dependency interfaces.
No outside issue, pull-request or fork implementation was used. Keleusma was not
changed. Cargo updates remain manual while records use the sibling path dependency.
Career qualification, active attitude actuation, general and multiple vessels,
separation, persistent identity, timing measurements and atomic batching remain open.
The handoff preserves the dependency layout, server timeout assumption and live-use
precautions. No mailbox is assigned.
