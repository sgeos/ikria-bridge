# Bridge Scope

> **Navigation**: [Decisions](./README.md) | [Documentation Root](../README.md)

**Status.** Decided.
**Date.** 2026-11-14.
**Decided by.** The operator, in the Ikria line's session, recorded by the Ikria line.

## Context

Ikria flies Kerbal Space Program 1 with Keleusma control loops through the kRPC mod.
The only Rust kRPC client built for kRPC 0.7.0, `krpc-client`, is GPL-3.0. kRPC's service
definition for vessel control is also GPL-3.0, while its protocol schema is MIT. Ikria's
charter keeps any GPL bridge in a small separate repository.

## Decision

1. **Three processes.** The bridge, the harness and the mission controller run as
   separate processes, so that Ikria's harness and controller stay 0BSD as built
   programs.
2. **A separate GPL-3.0 repository.** The bridge lives here, uses `krpc-client`, and is
   public.
3. **Licence version.** GPL-3.0-only, matching the GPL-3.0 code it depends on. The
   seeding line chose it, and the operator confirmed it on 2026-11-14 as the version
   closest to 0BSD for the operator, since it fixes the terms rather than admitting
   later versions that may be stricter. As the sole author of this repository's own
   code, the operator keeps the right to relicense that code, for example if kRPC's
   licensing loosens. Outside contributions would end that, unless their authors
   agree.

4. **No outside contributions.** The operator ruled on 2026-11-14 that the bridge accepts
   no outside contributions. Outside parties are encouraged to fork it and report the
   reason in an issue, rather than to open pull requests. This keeps the operator the
   sole author of the bridge's own code, and so keeps the right to relicense it.

## Consequences

- The interface with Ikria is Ikria's own record types, defined in a 0BSD crate in the
  Ikria repository, which this repository depends on.
- The licensing analysis behind these decisions is not legal advice.
