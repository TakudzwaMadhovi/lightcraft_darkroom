# lightcraft_darkroom

LightCraft (an open-source, pure-Rust photo library and raw developer) in the **GROOVEHAUS
Darkroom** look, tuned for a **2020 Intel MacBook Air (8 GB)**.

- **Download the app:** Actions → *Air DMG* → latest run → artifact `lightcraft-air-x86_64-dmg`.
  Open the `.dmg`, drag LightCraft to Applications, then the first time right-click it → **Open**
  (it is ad-hoc signed, not notarized).
- **Build on the Air instead:** `scripts/air-build.sh`, then `scripts/air-run.sh`.
- **What is different from upstream, and why:** [`docs/air/README.md`](docs/air/README.md)
  (memory and thread defaults for 8 GB, the Darkroom theme, measurements, deviations).
- **Upstream LightCraft README:** [`README.upstream.md`](README.upstream.md).

Code: MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`). Fonts: SIL OFL 1.1
(`assets/fonts/`). `docs/brand/` holds ArtCraft trademarks under their own terms.
