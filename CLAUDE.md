# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

ASCII roguelike dungeon crawler built with Rust and `bracket-lib` (~0.8.1). Learning project following "Hands On Rust" book, with custom deviations.

## Build & Run

```bash
cargo build          # debug build
cargo run            # run the game
cargo build --release  # optimized build
cargo check          # type-check without building (fastest feedback)
cargo clippy         # lint
cargo test           # run tests (none currently)
```

No workspace members, no feature flags, no custom build scripts.

## Architecture

**Game loop**: `State` struct implements bracket-lib's `GameState` trait. The `tick()` method runs each frame at 30 FPS cap: player input -> map render (console 0) -> player render (console 1).

**Coordinate system**: Grid world stored as flat `Vec<CellType>` (1D indexing via `map_idx(x, y)`). Two coordinate spaces:
- Internal screen: 80x50 (`SCREEN_WIDTH/HEIGHT`)
- Playable cells: 78x48 (`CELLS_WIDTH/HEIGHT`)
- Display window: 40x25 (`DISPLAY_WIDTH/HEIGHT`)

**Modules**:
- `main.rs` — `State` struct (owns Map, Player, Camera), game loop, BTerm initialization, prelude with shared constants
- `map.rs` — `Map` with `CellType` enum (Vacuum=wall, Platform=walkable), rendering, collision checks
- `map_builder.rs` — Procedural generation: random rectangular rooms with corridors connecting them (horizontal/vertical bridges between sorted island centers)
- `player.rs` — `Player` entity with arrow-key movement and collision validation
- `camera.rs` — Viewport centered on player, translates world coords to screen coords

**Rendering**: Two bracket-lib console layers. Map tiles use `dungeonfont.png` (32x32 tile sheet in `resources/`).

## Conventions

- `Point` type from bracket-lib used throughout for positions
- Room collision avoidance uses `outset_rect()` to expand rects by a buffer before intersection checks
- Code has partial hex-grid support commented out in `map.rs`
