#!/usr/bin/env bash
# Run the release build with Air-friendly defaults. Anything you pass goes to lightcraft.
#   LIGHTCRAFT_MEMORY_MB=768 scripts/air-run.sh   # override the memory budget (default 1024 on 8 GB)
#   LIGHTCRAFT_THREADS=2 scripts/air-run.sh       # override worker threads (default: physical cores)
#   LIGHTCRAFT_PROFILE=1 scripts/air-run.sh       # print per-stage pipeline timings to stderr
cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 1
exec target/release/lightcraft "$@"
