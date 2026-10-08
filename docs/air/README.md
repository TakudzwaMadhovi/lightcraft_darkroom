# LightCraft — Air edition

A personal build of LightCraft tuned for a **2020 Intel MacBook Air (8 GB, Core i3/i5)**.
Everything else is upstream LightCraft; the diff is small on purpose.

## Get it running

**Option A — no Rust needed.** Push this repo to GitHub, open *Actions → Air DMG → Run workflow*,
download `lightcraft-air-x86_64-dmg`, open the `.dmg`, drag LightCraft to Applications.
The app is ad-hoc signed, not notarized: the first time, right-click it → **Open** → **Open**.

**Option B — build on the Air.**
```
xcode-select --install                       # once
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # once
scripts/air-build.sh                         # 2 compile jobs so 8 GB doesn't swap
scripts/air-run.sh                           # starts with the procedural demo library
```
`scripts/air-build.sh --native` also tunes for your exact CPU (opt-in; see "Measured" below);
`--dmg` packages `LightCraft.app` + `.dmg`.

## What the Air edition changes

| Area | Upstream | Air edition | Why |
|---|---|---|---|
| Memory budget | RAM ÷ 4, max 1.5 GiB | RAM ≤ 8 GiB: RAM ÷ 8 (**1 GiB**); above: unchanged | iGPU, macOS and other apps share the 8 GB |
| Worker threads | one per logical thread | one per **physical** core when hyper-threaded (min 2) | Less heat on a thin chassis that throttles under sustained load |
| Build | `--locked` release, default jobs | 2 compile jobs, no incremental | Keeps compiles inside 8 GB |

Overrides, all environment variables: `LIGHTCRAFT_MEMORY_MB`, `LIGHTCRAFT_THREADS` (or
`RAYON_NUM_THREADS`), `LIGHTCRAFT_PROFILE=1` for per-stage timings. The GPU path is upstream's
(wgpu → Metal on the Intel Iris Plus); nothing was changed there.

Code: `crates/engine/src/hardware.rs` (policy + unit tests), `apps/lightcraft/src/main.rs`
(one call at startup).

## Measured

An AVX2 (`x86-64-v3`) build was benchmarked against the generic x86-64 build on a 24 MP image
with a heavy edit stack (CPU render path, x86_64 Linux, AVX2/AVX-512 Xeon, 5 alternating runs):
~3.29 s vs ~3.25 s after warm-up, i.e. within noise, and the output bytes differed slightly
(FMA rounding). So AVX2 is **not** a default. `--native` stays available for you to test on the
Air itself. The memory and thread defaults are *not* benchmarked: they are conservative limits
for an 8 GB, thermally limited machine, unit-tested for their arithmetic, and overridable.
Nothing here was run on a Mac; the macOS build and Metal path are unverified until you run them.

## Plugging in your UI

The entire look lives in `crates/ui-egui/src/theme.rs`: a `Tokens` struct (~30 colours: chrome,
canvas, text levels, accent, …), `apply()` for egui visuals/shadows/rounding, and font setup.
Send a mockup or palette + fonts and only that file changes; panels read colours from `Tokens`.
Keep fonts OFL/CC0 and add an entry to `assets/ATTRIBUTION.md` (`cargo xtask assets` enforces it).

## Licence note

Code is MIT OR Apache-2.0. `docs/brand/` (ArtCraft name and logos) is *not* open source: keep it
unmodified, or remove it if you publish a derivative under another name.
