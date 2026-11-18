# Reverse Prompt

> **Navigation**: [Process](README.md)

Written by Codex on 2026-11-18 in Coordinated Universal Time.

The operator requested the components needed for Keleusma flight-control loops. This
branch implements the separate Rust bridge for one guarded verification profile.
Ikria's paired branch implements the harness and programs. The operator approved
committing, pushing and merging Ikria first, followed by the bridge. Ikria pull
request 34 passed all required checks and merged as `583e1d4`. The pinned
implementation revision `504d4c4` is preserved in main.

The bridge checks the actual save, vessel, crew and revision B configuration, binds
one session, exchanges bounded version 2 frames and advances held physics ticks.
The whole command is authorized before any application. Command calls are sequential,
so partial application remains possible and lost acknowledgements are never retried.
A cancelled remote procedure call causes connection discard. Healthy shutdown attempts
neutralization and tick release. Solid motors cannot be stopped by neutralization.

Live verification completed a normal flight and a coast abort with eight-part recovery.
Ikria VR-0010 preserves journals, hashes and failed development runs. A moving-frame
velocity mistake lost an earlier empty article, then was corrected. A later touchdown
bounce fault was fixed in the harness. No crew flew and no career file changed.

The local gate now checks formatting, lint, three asynchronous framing and timeout
tests, and repository hygiene. Shared policy and runtime tests are in Ikria. No
remote continuous integration result was observed. The workflow explicitly checks out
the matching Ikria revision beside the bridge. Cargo updates remain manual while the
records dependency uses that sibling path.

The code was written independently from the stated need and released dependency
interfaces. No outside issue, pull-request or fork implementation was used. Keleusma
was not changed. Career qualification, active attitude actuation, general vessels,
multiple vessels, separation, persistent identity and batching remain future work.

The bridge publication is authorized for this session and follows the completed
Ikria merge. The local gate was rerun successfully. Merge only after remote required
checks pass. This is a pre-merge report, with completion recorded by the remote
pull request and merge ancestry. No new standing authority is inferred.
