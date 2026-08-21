# Changelog

MeshEnvy motatool. Upstream origin: [vk496/motatool](https://github.com/vk496/motatool). Do not copy upstream notes here.

## [Unreleased]

### Changed

- Sync `src/targets.rs` with firmware OTA env table (cache/storage slugs; drop `*_superseeder`).

### Improved

- `motatool serve -v`: split `[host]` (seeder protocol) vs `[tag]` (firmware serial); `DESCRIBE` logs mid/target/version/codec for each registered index.

## [0.1.1] - 2026-08-14

Declared MeshEnvy-canonical (`repository` → MeshEnvy/motatool). Binary name unchanged.

### Improved

- In-place encode builds one suffix array of the shifted base and filters it per segment (same patch bytes as 0.1.0).
- Auto `memory_size` returns the converged patch instead of encoding a third time.

## [0.1.0] - 2026-08-03

First EnvyOS-tracked motatool. Shipped with distro v0.1.2.
