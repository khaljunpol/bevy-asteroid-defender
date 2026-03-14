# CLAUDE.md — Bevy Asteroid Defender

This file is the AI-assistant guide for the `bevy-asteroid-defender` codebase.
Keep it up to date when architecture or conventions change.

---

## Project Overview

**Bevy Asteroid Defender Roguelike** is an arcade asteroid shooter with per-level
roguelike upgrade picks, built with Rust and the [Bevy](https://bevyengine.org/)
game engine (v0.11).

- **Language:** Rust (edition 2021)
- **Engine:** Bevy 0.11.2
- **Binary:** `bevy-defender-game`
- **Targets:** native desktop + web (WASM via [Trunk](https://trunkrs.dev/))

---

## Build & Run

```bash
# Development – fast native compile (dynamic linking via the `dev` feature)
cargo run --features dev

# Standard native build (no dynamic linking)
cargo run

# Release build
cargo build --release

# Web build (requires: cargo install trunk + wasm32 target)
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve          # http://localhost:8080  (live-reload)
trunk build --release  # outputs to ./dist/
```

> `bevy_framepace` (frame-rate limiter) is a **native-only** dependency.
> On WASM the browser's own vsync handles frame pacing.
> Dynamic linking (`bevy/dynamic_linking`) is only activated with `--features dev`.

---

## Directory Structure

```
bevy-asteroid-defender/
├── Cargo.toml          # Dependencies and build profiles
├── index.html          # Trunk entry point for WASM builds
├── CLAUDE.md           # This file
├── assets/
│   ├── fonts/          # screen-diags-font.ttf
│   └── sprites/
│       ├── ships/      # 9 ship sprites (3 types × 3 colours)
│       ├── laser/      # 3 coloured projectile sprites
│       ├── meteor/     # 3 meteor size sprites
│       ├── powerup/    # HP-pack sprite (blue star)
│       ├── effects/    # Particle / icon sprites
│       └── ui/         # Life-indicator sprites
└── src/
    ├── main.rs         # App entry point, plugin wiring, PreStartup setup
    ├── lib.rs          # Global constants and shared types (separate crate)
    ├── resources.rs    # All Bevy Resources + asset-path constants
    ├── background.rs   # Placeholder (starfield could go here)
    ├── player/
    │   ├── player.rs   # PlayerComponent, movement, spawn/fly-out, tween
    │   └── ship.rs     # ShipComponent, ShipPlugin, texture sync
    ├── objects/
    │   ├── meteor.rs   # MeteorComponent (HP), level-start spawn, flash, level-complete check
    │   ├── projectile.rs  # ProjectileComponent (damage), split-shot, ricochet, chain-reaction timer
    │   └── powerup.rs  # PowerUpComponent (HP pack), magnet drift, timed spawn
    ├── common/
    │   ├── common_components.rs  # Shared ECS components (Velocity, Position, …)
    │   ├── common_systems.rs     # Movement, wrapping, bounds-despawn, transform sync
    │   └── collision.rs          # All AABB collision logic (projectile↔meteor, player↔meteor, player↔powerup)
    ├── state/
    │   └── states.rs   # GameStates enum + one Plugin per state
    ├── events/
    │   └── events.rs   # PlayerDeadEvent, EventsPlugin
    ├── upgrades/
    │   └── upgrades.rs # UpgradeType enum, UpgradePlugin, input/apply logic
    ├── ui/
    │   └── ui.rs       # All UI: HUD, countdown, level indicator, level-clear, upgrade selection, game-over
    └── utils/
        ├── cleanup.rs  # CleanUpOnGameOver / CleanUpOnLevelEnd + generic cleanup_system
        ├── manager.rs  # State-transition helper functions (goto_*)
        └── utils.rs    # Math helpers (get_angle_to_target, etc.)
```

---

## Game Loop

```
StartGame ──(1.8s tween)──► Countdown ──(3-2-1-GO!)──► InGame
   ▲                                                      │
   │                                              all meteors dead
   │                                                      ▼
   │                                             LevelComplete (2s)
   │                                                      │
   │                                             UpgradeSelection
   │                                           (pick 1 of 3 upgrades)
   │                                                      │
   │                                              Countdown again
   │                                                      │
   │                                              InGame (next level)
   │
   └──(Space/Enter)── GameOver ◄──(player HP = 0)──── InGame
```

Key transitions:
| From | To | Trigger |
|------|----|---------|
| `StartGame` | `Countdown` | 1.8 s timer (player tween completes) |
| `Countdown` | `InGame` | GO! timer expires |
| `InGame` | `LevelComplete` | all `MeteorComponent` entities gone |
| `InGame` | `GameOver` | `Life.current_life` reaches 0 |
| `LevelComplete` | `UpgradeSelection` | 2 s timer |
| `UpgradeSelection` | `Countdown` | player presses Space/Enter |
| `GameOver` | `StartGame` | player presses Space/Enter |

---

## Game States (src/state/states.rs)

```rust
pub enum GameStates {
    StartGame,        // reset + spawn player
    Countdown,        // 3–2–1–GO!
    InGame,           // active play
    LevelComplete,    // level cleared celebration
    UpgradeSelection, // roguelike pick
    GameOver,         // death screen
}
```

Each state has its own `Plugin` (e.g. `CountdownStatePlugin`) registered in `main.rs`.

---

## Key Resources (src/resources.rs)

| Resource | Purpose |
|----------|---------|
| `GameSprites` | Preloaded `Handle<Image>` for all sprites |
| `WindowSize` | Current window width/height |
| `WindowDespawnBorder` | Outer despawn boundary rects |
| `Life { max_life, current_life }` | Player HP |
| `Score { current, high_score }` | Score tracking |
| `LevelResource { current, total_asteroids_spawned }` | Level number + spawn guard |
| `CountdownResource { count, tick_timer, go_timer }` | Countdown state data |
| `PlayerUpgrades` | All active upgrades + runtime state (chain timer) |
| `UpgradeSelectionState { choices, selected }` | Current upgrade pick screen state |

---

## Level Scaling

| Level | Asteroids | Large HP |
|-------|-----------|----------|
| 1 | 3 | 1 |
| 2 | 4 | 1 |
| 3 | 5 | 1 |
| 4 | 6 | 2 |
| 7 | 9 | 3 |
| … | min(2+level, 12) | 1 + (level−1)÷3 |

Split children (Medium, Small) always spawn with **1 HP** regardless of level.

---

## Roguelike Upgrades (src/upgrades/upgrades.rs)

Three random eligible upgrades are shown after each level cleared.
Arrow keys or A/D navigate; Space/Enter confirms.

| Name | Category | Max Levels | Effect |
|------|----------|------------|--------|
| Split Shot | Offense | 3 | +2 bullets per level (spread pattern) |
| Rapid Fire | Offense | 3 | –25% fire cooldown per level |
| Heavy Rounds | Offense | 2 | +1 bullet damage per level |
| Ricochet | Offense | 1 | Bullets bounce off screen edges once |
| Extra Armor | Defense | 2 | +1 max HP, +1 current HP |
| Afterburner | Defense | 2 | +30% top speed per level |
| Quick Reflexes | Defense | 2 | +40% turn speed per level |
| Overclock | Special | 1 | Asteroids move at 60% speed |
| Chain Reaction | Special | 1 | Killing an asteroid triggers 3 s of ultra-rapid fire |
| Asteroid Magnet | Special | 1 | HP packs slowly drift toward the player |

An upgrade is **eligible** if `current_level < max_level`.
Apply via `UpgradeType::apply(&mut upgrades, &mut life)`.

---

## Entity Cleanup Strategy

| Component | Despawned when |
|-----------|----------------|
| `CleanUpOnGameOver` | `OnEnter(StartGame)` – full game reset; player carries this |
| `CleanUpOnLevelEnd` | `OnExit(InGame)` – between levels; projectiles and powerups |

Meteors also carry `CleanUpOnLevelEnd` via `BoundsDespawnable` and natural death,
but the real guard is the level-complete check (all meteors gone → `LevelComplete`).

---

## Collision System (src/common/collision.rs)

Detection method: `bevy::sprite::collide_aabb::collide` (AABB).

| Pair | Effect |
|------|--------|
| Projectile → Meteor | HP – damage; if HP ≤ 0: despawn + spawn `MeteorSplitEvent` |
| Meteor split | `meteor_split_system` reads `MeteorSplitEvent`, spawns 3 children (1 HP each) |
| Player → Meteor | Meteor despawns; `DamageCollision` marker spawned; `apply_damage_system` deducts HP |
| Player → Power-up | Power-up despawns; `PowerUpComponent::apply` adds 1 HP (capped at max) |

Chain Reaction upgrade is triggered in `player_projectile_hit_meteor_system` via
`trigger_chain_reaction(&mut upgrades)`.

---

## Player Upgrade Effects (runtime)

| Upgrade | How applied |
|---------|-------------|
| Split Shot | `PlayerUpgrades::shot_offsets()` returns angle offsets; projectile system fires one bullet per offset |
| Rapid Fire | `PlayerUpgrades::effective_shoot_cooldown()` multiplied by `0.75^level` |
| Heavy Rounds | `PlayerUpgrades::bullet_damage()` = `1 + heavy_rounds` |
| Ricochet | Projectile gets `ProjectileRicochet` component instead of `BoundsDespawnable` |
| Afterburner | `PlayerUpgrades::effective_max_speed()` |
| Quick Reflexes | `PlayerUpgrades::effective_turn_speed()` |
| Overclock | Applied at spawn time in `spawn_level_asteroids` |
| Chain Reaction | `chain_active` flag + `chain_timer` float; included in `effective_shoot_cooldown()` |
| Asteroid Magnet | `powerup_magnet_system` nudges powerup velocity toward player each frame |

---

## Naming Conventions

| Category | Convention | Example |
|----------|------------|---------|
| Components | `PascalCase` + `Component` suffix | `PlayerComponent`, `MeteorComponent` |
| Systems | `snake_case` + `_system` suffix | `collision_damage_system` |
| Resources | `PascalCase`, no suffix | `WindowSize`, `PlayerUpgrades` |
| Enums | `PascalCase` | `GameStates`, `UpgradeType` |
| Constants | `UPPER_SNAKE_CASE` | `PLAYER_MAX_SPEED`, `OVERCLOCK_SPEED_MULT` |
| Events | `PascalCase` + `Event` suffix | `PlayerDeadEvent` |
| Plugins | `PascalCase` + `Plugin` suffix | `MeteorPlugin`, `UpgradePlugin` |
| State managers | `PascalCase` + `StatePlugin` | `CountdownStatePlugin` |

---

## Adding New Features

### New upgrade
1. Add variant to `UpgradeType` in `src/upgrades/upgrades.rs`.
2. Implement `name()`, `description()`, `max_level()`, `current_level()`, `apply()`, `category()`.
3. Add a field to `PlayerUpgrades` in `src/resources.rs`.
4. Wire the runtime effect into the relevant system (movement, shooting, spawn, etc.).

### New game state
1. Add variant to `GameStates` in `src/state/states.rs`.
2. Create a `XxxStatePlugin` with `OnEnter` / `OnExit` / `Update` systems.
3. Register the plugin in `src/main.rs`.
4. Add any new UI entry/exit systems in `UIPlugin`.

### New asteroid/enemy type
1. Add a component and spawn function in `src/objects/`.
2. Tag with `CleanUpOnLevelEnd` for level cleanup.
3. Handle in `src/common/collision.rs`.
4. Include in the level-complete check if it should block progression.

---

## Web Hosting Notes

```bash
# Build WASM release
trunk build --release      # outputs to ./dist/

# Serve locally
trunk serve

# Deploy: copy ./dist/ to any static host (GitHub Pages, Cloudflare Pages, Netlify, etc.)
```

The `index.html` at the repo root is the trunk entry point. It references a
`<canvas id="bevy">` element that Bevy attaches to via `Window::canvas`.

To install trunk:
```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
```

---

## Common Pitfalls

- **Single-player query**: Use `query.get_single_mut()` and guard with `if let Ok(...)`.
  The player is absent during `GameOver` (physics components removed).
- **State-gated systems**: Systems that should only run in `InGame` need
  `.run_if(in_state(GameStates::InGame))`.
- **Level-complete race**: The guard `level.total_asteroids_spawned > 0` prevents
  a spurious level-complete trigger on the first frame before asteroids spawn.
- **Command flushing**: `OnEnter(StartGame)` uses `.chain()` + `apply_deferred`
  so the old player entity is fully despawned before the new one spawns.
- **Upgrade eligibility**: Always call `UpgradeType::is_eligible()` before showing
  an upgrade card. `generate_choices()` does this automatically.
- **WASM**: Do not add `bevy/dynamic_linking` to default features — it breaks WASM.
  Use `--features dev` for fast native builds only.
- **Asset paths**: All paths are relative to `assets/` (Bevy convention).
  The `assets/` folder must sit next to the binary at runtime.
