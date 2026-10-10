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

## The Darkroom UI

The look is the GROOVEHAUS "Darkroom Editor" design (dark variant): obsidian canvas, bone type,
one sun-yellow accent per frame, hairline rules, square corners, no gloss.

| Piece | Where |
|---|---|
| Palette (bone @ 55/50/30/25/18 % over black, sun #E9B547), metrics, radii, motion | `crates/ui-egui/src/theme.rs` (`Tokens`, `apply`) |
| Fonts: Rajdhani (UI), Bebas Neue (headlines), Cormorant Garamond Italic (notes) | `assets/fonts/`, licence `OFL-Groovehaus-brand-fonts.txt`, `assets/ATTRIBUTION.md` |
| Tracked-caps text, bar-thumb slider with centre tick, `+`/`−` section headers, outlined chips | `crates/ui-egui/src/widgets.rs` |
| `01 — LIGHT` eyebrow + Bebas `EDIT` header | `crates/ui-egui/src/panels/edit.rs` |
| GROOVEHAUS · DARKROOM wordmark, tracked search field | `crates/ui-egui/src/panels/topbar.rs` |
| Copy-settings chip | `crates/ui-egui/src/panels/bottombar.rs` |
| Right panel 340 pt wide (mockup) | `crates/ui-egui/src/state.rs` |

Deliberate deviations from the mockup, for a working editor: the histogram keeps its red/green/blue
channels (a monochrome histogram hides clipping per channel); the reject flag and mask overlay stay
flame red (state needs a distinct colour); dim text is bone @ 50 % instead of 30 % (50 % is the
lowest step with WCAG AA 4.5:1 contrast on black; 30 % is about 2.4:1); the tool strip keeps icons rather
than three-letter labels.

Not done: the **Bone** variant (cream surface, 32 px rounded floating cards, pill buttons, round slider
thumbs). It is a layout change, not a palette swap, so it needs its own pass.

## Licence note

Code is MIT OR Apache-2.0. `docs/brand/` (ArtCraft name and logos) is *not* open source: keep it
unmodified, or remove it if you publish a derivative under another name.
