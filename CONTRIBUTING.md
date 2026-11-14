# Contributing

**This repository does not accept outside contributions.** Pull requests from anyone
other than the maintainer are closed without review. The maintainer is the sole author
of this repository's own code, which keeps the right to relicense that code. Outside
contributions would end that right unless every author agreed.

**Fork instead.** The licence, GPL-3.0-only, lets anyone fork the bridge and change it
under its terms. If you fork it, you are encouraged to open a
[fork report](https://github.com/sgeos/ikria-bridge/issues/new?template=fork_report.md)
saying why, such as the defect, the missing feature or the need that led you to fork.
Those reports are welcome and read. The maintainer may address what they describe in a
different way, or not at all.

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
