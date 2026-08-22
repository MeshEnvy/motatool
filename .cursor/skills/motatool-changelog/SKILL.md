---
name: motatool-changelog
description: >-
  Maintain CHANGELOG.md Unreleased entries for user-facing motatool changes.
  Use when shipping features, fixes, or breaking changes; before release cut.
---

# motatool changelog

Policy: [`docs/change-management.md`](../../docs/change-management.md). Canonical file: [`CHANGELOG.md`](../../CHANGELOG.md).

## When to write

Add a bullet under **`## [Unreleased]`** in the **same change set** as user-visible code when you:

- Add or remove CLI flags or `serve` behavior bench operators depend on
- Change `.mota` packaging, delta encoding, or verify rules
- Fix incorrect pack/verify/serve behavior users would notice
- Ship breaking greenfield changes (delete old paths; note migration)

Skip changelog lines for refactors, tests, CI, docs-only, and agent hygiene unless bench operators care.

## How to write

- Keep a Changelog sections: `### Added`, `### Changed`, `### Fixed`, `### Removed`, `### Improved`.
- **Bold lead phrase** then em dash and detail: `- **Serve logging** — …`
- Group related commits into **one bullet** (the release is not a commit dump).
- Plain language for firmware/OTA bench users, not Rust module names unless they are the surface.

## Release cut (maintainers)

1. When ready to ship, promote **`## [Unreleased]`** to **`## [vX.Y.Z] - YYYY-MM-DD`** (optional one-line summary under the heading). Until then, all pending work stays under Unreleased only.
2. Insert a fresh empty **`## [Unreleased]`** at the top.
3. Run `./scripts/changelog.sh check vX.Y.Z`.
4. Tag per skill **`motatool-release`**.

Do not rely on commit subjects for GitHub Release text.
