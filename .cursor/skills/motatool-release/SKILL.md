---
name: motatool-release
description: >-
  Cut a motatool GitHub release (version bump, tag, binary publish).
  Use when preparing or publishing v* releases.
---

# motatool release

Policy: [`docs/change-management.md`](../../docs/change-management.md). Workflow: [`.github/workflows/release.yml`](../../.github/workflows/release.yml). Index: [MEMORY.md](../../MEMORY.md) § Releases.

## Preconditions

1. `envyos/main` is green (`cargo test --locked`).
2. User-facing changes since last tag are in [`CHANGELOG.md`](../../CHANGELOG.md) (not only in commits).
3. `version` in [`Cargo.toml`](../../Cargo.toml) matches the tag you are about to cut.

## Changelog promote

1. Move **`## [Unreleased]`** content to **`## [vX.Y.Z] - YYYY-MM-DD`** (optional one-line summary under the heading).
2. Insert a fresh empty **`## [Unreleased]`** at the top.
3. Verify:

```bash
./scripts/changelog.sh check vX.Y.Z
./scripts/changelog.sh notes vX.Y.Z   # preview GitHub body
```

Skill for day-to-day entries: **`motatool-changelog`**.

## Version bump

Set `version` in [`Cargo.toml`](../../Cargo.toml) `[package]`.

Commit changelog promote + version bump on `envyos/main` before tagging.

## Publish

```bash
git tag v0.1.2
git push meshenvy v0.1.2
```

Push the tag only after the promoted changelog and version bump are on the tagged commit.

## What CI does

On `v*` tag push, GitHub Actions:

1. Builds `motatool` release binaries for `x86_64-unknown-linux-gnu`, `aarch64-apple-darwin`, `x86_64-apple-darwin`.
2. Runs `./scripts/changelog.sh check` and publishes that section as the release body.
3. Uploads `motatool-<version>-<target>.tar.gz` assets.

GitHub attaches **Source code (zip)** and **Source code (tar.gz)** automatically.

## Verify

1. Open https://github.com/MeshEnvy/motatool/releases
2. Confirm release notes match `CHANGELOG.md` for that version (not a commit list).
3. Confirm three platform archives plus source archives.
4. Smoke-test one binary: `motatool --help`

## After release

When the version ships in an EnvyOS distro bundle, bump `motatool=` in envyos [`ENVYOS_VERSIONS`](https://github.com/MeshEnvy/envyos).

## Notes

- No Windows target in the matrix yet.
- Default branch on MeshEnvy is **`envyos/main`**; tag from merged commits there.
- Bench builds still compile from the envyos submodule checkout; release binaries are for operators who want a pinned download.
