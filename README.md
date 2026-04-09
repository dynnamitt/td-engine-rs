# TD-Engine-RS

TD = Tower Defence (eventually). Right now it's a roguelike tutorial assembly journal — working through [Hands-on Rust](https://pragprog.com/titles/hwrust/hands-on-rust/) by Herbert Wolverson with custom deviations, using [bracket-lib](https://github.com/amethyst/bracket-lib) for terminal rendering.

## Current state

- Procedural map generation (random rooms + corridor connections)
- Player movement with collision (arrow keys)
- Camera/viewport that tracks the player
- Crossterm terminal backend (no GL window needed)

## Running

```bash
cargo run
```

Requires a Rust toolchain. Renders in your terminal via crossterm.

## Controls

| Key | Action |
|-----|--------|
| Arrow keys | Move player |
| Ctrl+C | Quit |

## Book reference

Based on the **BasicDungeonCrawler** chapters. Tile font sourced from the [HandsOnRust repo](https://github.com/thebracket/HandsOnRust).
