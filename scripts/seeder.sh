#!/usr/bin/env bash
# Serve a folder of .mota files to a MeshCore seeder over USB serial (motatool serve).
#
# Usage:
#   ./scripts/seeder.sh /dev/cu.usbmodem1444301
#   ./scripts/seeder.sh usbmodem1444301
#   ./scripts/seeder.sh /dev/cu.usbmodem1444301 /path/to/motas/v0.1.1
#
# Requires: motatool on PATH (release binary or `cargo build --release`), or MOTATOOL=/path.

set -euo pipefail

DIR="${2:-.}"

usage() {
  cat >&2 <<EOF
usage: $0 <serial-device> [motas-dir]

  serial-device   USB serial port, e.g. /dev/cu.usbmodem1444301
  motas-dir       folder of .mota files (default: current directory)

environment:
  MOTATOOL   Override motatool binary path

examples:
  $0 /dev/cu.usbmodem1444301
  $0 usbmodem1444301 ./motas/v0.1.1
EOF
  exit 2
}

resolve_motatool() {
  if [[ -n "${MOTATOOL:-}" && -x "$MOTATOOL" ]]; then
    printf '%s\n' "$MOTATOOL"
    return 0
  fi
  if command -v motatool >/dev/null 2>&1; then
    command -v motatool
    return 0
  fi
  local repo_bin
  repo_bin="$(cd "$(dirname "$0")/.." && pwd)/target/release/motatool"
  if [[ -x "$repo_bin" ]]; then
    printf '%s\n' "$repo_bin"
    return 0
  fi
  echo "error: motatool not found (install release binary, cargo build --release, or set MOTATOOL=)" >&2
  return 1
}

resolve_serial() {
  local dev="$1"
  if [[ "$dev" == /* ]]; then
    printf '%s' "$dev"
    return
  fi
  if [[ -e "/dev/cu.$dev" ]]; then
    printf '/dev/cu.%s' "$dev"
    return
  fi
  if [[ -e "/dev/tty.$dev" ]]; then
    printf '/dev/tty.%s' "$dev"
    return
  fi
  printf '/dev/cu.%s' "$dev"
}

[[ $# -ge 1 && $# -le 2 ]] || usage

SERIAL="$(resolve_serial "$1")"
[[ -e "$SERIAL" ]] || {
  echo "error: serial device not found: $SERIAL" >&2
  exit 1
}
[[ -d "$DIR" ]] || {
  echo "error: motas dir not found: $DIR" >&2
  exit 1
}

MT="$(resolve_motatool)"
exec "$MT" serve --dir "$DIR" --serial "$SERIAL" -v
