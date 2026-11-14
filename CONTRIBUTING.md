# Contributing

**This repository does not accept outside contributions.** Pull requests from anyone
other than the maintainer are closed without review. The maintainer is the sole author
of this repository's own code, which keeps the right to relicense that code. Outside
contributions would end that right unless every author agreed.

**Fork instead.** The licence, GPL-3.0-only, lets anyone fork the bridge and change it
under its terms. If you fork it, you are encouraged to open a
[fork report](https://github.com/sgeos/ikria-bridge/issues/new?template=fork_report.md)
saying why, such as the defect, the missing feature or the need that led you to fork.
Those reports are welcome and read.

## Reporting needs, and strict provenance

**Every change here is implemented independently, from first principles, so that the
provenance of all code in this repository stays strict.** That is why outside code is
not taken in any form, and why reports carry needs rather than implementations.

- **Report the need.** The fork report, feature request and bug report templates ask
  for what the bridge must do, why it matters, the behaviour observed and expected, how
  to tell when the need is met, and the constraints a solution must respect. A report
  that states those well gives the maintainer what is needed to meet it upstream.
- **Leave out implementations.** Do not include code, patches, diffs or links to commits
  that implement a change, from a fork or anywhere else. Logs, error messages, measured
  values and descriptions of behaviour are welcome.
- **Expect an independent solution.** The maintainer does not read a fork's code to
  implement a report. A change made upstream may differ from what a fork does, and the
  maintainer may meet a need in another way, or not at all.
- **Material that arrives anyway is set aside.** If a report includes code or a link to
  an implementation, the maintainer does not use it, and may edit it out of the report.

Participation in the project's spaces follows the
[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

## Reporting security issues

Do not open a public issue for a vulnerability. Follow [`SECURITY.md`](SECURITY.md).

## For the maintainer's own work

These rules govern the maintainer's own branches, including those opened by agents
working for the maintainer.

1. Run `./scripts/check.sh` and confirm that it exits zero.
2. Keep each pull request to one change, on a branch named
   `<scope>/<short-description>`, such as `fix/parser-error-recovery`.
3. Write commit messages as conventional commits, such as `feat:`, `fix:`, `docs:`,
   `test:` or `chore:`.
4. Add an entry under `[Unreleased]` in `CHANGELOG.md` for any change a user would notice.
5. Do not weaken a check to make it pass. If a check is wrong, say so in the pull request
   and fix it deliberately.
