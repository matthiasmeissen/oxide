# Release Workflow

How changes get from a branch to a tagged release. Kept deliberately light.

## Branches

- **`feature/*`** — one branch per feature or fix.
- **`develop`** — integration branch. Features are merged here (via PR) as they're finished.
- **`main`** — always equals the latest **tagged, verified release**. It only advances when we cut a release.

```
feature/*  →  PR into develop  →  (collect + test)  →  release: develop → main + tag
```

## Releasing

We **batch** releases rather than tagging every feature. Because this is a hardware
project, a tag should mark a build that was actually flashed and verified on the device
(the Linux/OLED paths can't be fully tested on the dev machine). Collect a coherent,
tested increment on `develop`, then:

1. Verify the build on the Raspberry Pi.
2. Bump `version` in `Cargo.toml` (see Versioning).
3. In `CHANGELOG.md`, rename `## [Unreleased]` to `## [X.Y.Z] - YYYY-MM-DD` and start a
   fresh empty `Unreleased` section on top.
4. Merge `develop` → `main`.
5. Tag it: `git tag -a vX.Y.Z -m "vX.Y.Z"` and push tags.
6. Create the GitHub release from the tag, using the changelog entry as the notes.

## Versioning (SemVer)

`MAJOR.MINOR.PATCH`, kept in sync with the git tag:

- **PATCH** (`0.1.0 → 0.1.1`) — backward-compatible bug fixes only.
- **MINOR** (`0.1.0 → 0.2.0`) — new, backward-compatible functionality.
- **MAJOR** (`→ 1.0.0`) — breaking changes. "Big" is not the same as "breaking".

## Changelog

`CHANGELOG.md` follows [Keep a Changelog](https://keepachangelog.com). Append to the
`## [Unreleased]` section as each feature merges into `develop` (under `Added` /
`Changed` / `Removed` / `Fixed`). At release time it becomes the version's notes.
