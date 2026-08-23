# MEMORY.md — motatool

Last updated: 2026-08-22

Rust CLI for MeshCore `.mota` firmware containers: build, verify, inspect, serve, delta encode. MeshEnvy fork of [vk496/motatool](https://github.com/vk496/motatool). Release branch: **`envyos/main`**. Integration: **`envyos/dev`** (tracks release until WIP diverges).

## Releases

| Item | Location |
|------|----------|
| Version | `Cargo.toml` `[package].version` |
| User notes | `CHANGELOG.md` (`## [Unreleased]` → promote at cut) |
| Policy | `docs/change-management.md` |
| Release CI | `.github/workflows/release.yml` on `v*` tag |
| Test CI | `.github/workflows/ci.yml` on `envyos/main`, `main`, PRs |
| Cut workflow | `.cursor/skills/motatool-release/SKILL.md` |

GitHub Releases: https://github.com/MeshEnvy/motatool/releases

Assets per tag: `motatool-<ver>-x86_64-unknown-linux-gnu.tar.gz`, `motatool-<ver>-aarch64-apple-darwin.tar.gz`, `motatool-<ver>-x86_64-apple-darwin.tar.gz`.

EnvyOS bench pins semver via envyos `ENVYOS_VERSIONS` `motatool=` when bundled in a distro release.

## Build / test

```bash
cargo build --release          # target/release/motatool
cargo test --locked
make dev-setup                 # optional: detools oracle for delta tests
```

## Layout

| Path | Role |
|------|------|
| `src/` | CLI + library (encode, verify, serve) |
| `tests/` | Integration tests |
| `scripts/changelog.sh` | Release notes extract/check |
| `scripts/seeder.sh` | USB seeder wrapper (`motatool serve`) |
| `third_party/detools/` | Test oracle only |

## Active threads

- vk496 upstream PR: role-aware OTA staging ceiling (`feature/ota-stage-ceiling`)
