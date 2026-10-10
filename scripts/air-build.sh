#!/usr/bin/env bash
# Build LightCraft on a 2020 Intel MacBook Air (8 GB). Run from the repo root:
#   scripts/air-build.sh            # fast, memory-safe build  -> target/release/lightcraft
#   scripts/air-build.sh --native   # also tune for this exact CPU (not portable to other Macs)
#   scripts/air-build.sh --dmg      # build and package LightCraft.app + .dmg (ad-hoc signed)
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

[ "$(uname -s)" = Darwin ] || { echo "error: run this on the Mac" >&2; exit 1; }
[ "$(uname -m)" = x86_64 ] || echo "warning: this is not an Intel Mac ($(uname -m)); the Air edition targets x86_64" >&2

if ! command -v cargo >/dev/null; then
  echo "Rust not found. Install it first:  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" >&2
  exit 1
fi
if ! xcode-select -p >/dev/null 2>&1; then
  echo "Xcode command line tools not found. Install:  xcode-select --install" >&2
  exit 1
fi

NATIVE=0; DMG=0
for a in "$@"; do
  case "$a" in
    --native) NATIVE=1 ;;
    --dmg) DMG=1 ;;
    -h | --help) sed -n '2,6p' "$0"; exit 0 ;;
    *) echo "unknown argument: $a" >&2; exit 2 ;;
  esac
done

# 2 parallel compile jobs: rustc for wgpu/egui crates peaks at ~1-2 GB each, and 8 GB shared with
# macOS and the iGPU starts swapping above that. Override with CARGO_BUILD_JOBS.
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}"
export CARGO_INCREMENTAL=0
[ "$NATIVE" = 1 ] && export RUSTFLAGS="${RUSTFLAGS:-} -C target-cpu=native"

if [ "$DMG" = 1 ]; then
  exec packaging/macos/package.sh --arch x86_64
fi
cargo build --release --locked -p lightcraft -p lightcraft-cli
echo
echo "Built: target/release/lightcraft   (run: scripts/air-run.sh)"
