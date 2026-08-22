# Change management policy

motatool has one user-facing changelog. Commits stay granular for bisect; release notes are curated in [`CHANGELOG.md`](../CHANGELOG.md).

## Two layers

| Layer | Purpose |
|-------|---------|
| **Git commits** | Conventional Commits for history, review, bisect. Fine-grained is fine. |
| **`CHANGELOG.md`** | User-facing bullets grouped by release. One line can cover many commits. |

## Rules per change set

- User-visible behavior change → bullet under **`## [Unreleased]`** in the same commit/PR as the code.
- Internal-only work (`refactor`, `test`, `chore`, `ci`, agent hygiene) → no changelog entry unless bench operators care.
- Breaking greenfield changes → **`### Changed`** or **`### Removed`** with a short migration note.

Keep a Changelog sections: `### Added`, `### Changed`, `### Fixed`, `### Removed`, `### Improved`. Omit empty sections.

## Release cut

1. `cargo test --locked` green on `envyos/main`.
2. `version` in [`Cargo.toml`](../Cargo.toml) matches the tag (`v0.1.2` → `0.1.2`).
3. Promote **`## [Unreleased]`** → **`## [vX.Y.Z] - YYYY-MM-DD`** (one-line summary under the heading if helpful). Do not create a version section before the tag exists.
4. Open a fresh empty **`## [Unreleased]`** at the top.
5. `./scripts/changelog.sh check vX.Y.Z` passes.
6. Tag and push: `git tag vX.Y.Z && git push meshenvy vX.Y.Z`.

CI publishes the `## [vX.Y.Z]` body to GitHub Releases. It does **not** auto-generate from commits.

Skill: [`.cursor/skills/motatool-release/SKILL.md`](../.cursor/skills/motatool-release/SKILL.md). Day-to-day entries: [`.cursor/skills/motatool-changelog/SKILL.md`](../.cursor/skills/motatool-changelog/SKILL.md).

## Enforcement

| When | Gate |
|------|------|
| Tag push (`release.yml`) | `./scripts/changelog.sh check "${GITHUB_REF_NAME}"` |
| Manual pre-tag | `./scripts/changelog.sh check vX.Y.Z` and `./scripts/changelog.sh notes vX.Y.Z` |

Missing or empty version section fails the release workflow.

## Distro coupling

EnvyOS [`ENVYOS_VERSIONS`](https://github.com/MeshEnvy/envyos) pins the motatool semver the bench expects. After cutting a motatool release, bump the manifest in envyos when that version ships in a distro bundle.
