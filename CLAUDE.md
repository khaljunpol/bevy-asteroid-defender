# CLAUDE.md — Bevy Asteroid Defender

This file provides AI assistants with a comprehensive guide to the `bevy-asteroid-defender` codebase, conventions, and development workflows.

---

## Project Overview

**Bevy Meteor Defender** is a classic arcade-style asteroid shooter built with Rust and the [Bevy](https://bevyengine.org/) game engine (v0.11). The project serves as a learning exercise in Bevy's Entity Component System (ECS) architecture.

- **Language:** Rust (edition 2021)
- **Engine:** Bevy 0.11.2
- **Binary name:** `bevy-defender-game`

---

## Build & Run

```bash
# Development (fast compile via dynamic linking + opt-level=1)
cargo run

# Release build
cargo build --release
./target/release/bevy-defender-game

# Check for errors without building
cargo check
```

There is no Makefile. Use standard `cargo` commands. The `bevy` dependency uses the `dynamic_linking` feature in development for faster iteration.

---

## Directory Structure

```
bevy-asteroid-defender/
├── Cargo.toml            # Dependencies and build profiles
├── README.md
├── CLAUDE.md             # This file
├── assets/
│   ├── fonts/            # screen-diags-font.ttf
│   └── sprites/
│       ├── ships/        # 9 ship variants (3 types × 3 colors)
│       ├── laser/        # 3 colored projectiles (Blue, Green, Red)
│       ├── meteor/       # 3 size variants (Big, Med, Small)
│       ├── powerup/      # 12 power-up variation sprites
│       ├── effects/      # Particle effect and icon sprites (19 files)
│       └── ui/           # Life indicator sprites (4 colors)
└── src/
    ├── main.rs           # App entry point and plugin registration
    ├── lib.rs            # Global constants and shared type aliases
    ├── resources.rs      # Bevy Resources (GameSprites, WindowSize, Score, Life)
    ├── background.rs     # Placeholder (empty)
    ├── player/
    │   ├── mod.rs
    │   ├── player.rs     # Player movement, shooting, death, tween animations
    │   └── ship.rs       # ShipComponent, ShipType enum, Stats struct
    ├── objects/
    │   ├── mod.rs
    │   ├── meteor.rs     # MeteorComponent, spawning, splitting logic
    │   ├── projectile.rs # ProjectileComponent, firing, despawn timer
    │   └── powerup.rs    # PowerUpComponent, ship-type changing, spawning
    ├── common/
    │   ├── mod.rs
    │   ├── common_components.rs  # Velocity, Position, RotationAngle, HitBoxSize, bounds markers
    │   ├── common_systems.rs     # Movement, wrapping, despawning, transform sync
    │   └── collision.rs          # AABB collision detection for all game objects
    ├── state/
    │   ├── mod.rs
    │   └── states.rs     # GameStates enum and state transition systems
    ├── events/
    │   ├── mod.rs
    │   └── events.rs     # Custom Bevy events (PlayerDead, PlayerSpawn, StateStart/End)
    ├── ui/
    │   ├── mod.rs
    │   └── ui.rs         # Score display, life/health icon UI
    ├── effects/
    │   ├── mod.rs
    │   └── effects.rs    # Particle effects (bevy_hanabi, currently commented out)
    └── utils/
        ├── mod.rs
        ├── object_pool.rs  # Object pool skeleton (defined, not yet used)
        └── setup.rs        # Empty setup utility stubs
```

---

## Architecture

### ECS Pattern

The codebase follows Bevy's ECS strictly:

- **Components** hold data, never logic.
- **Systems** hold logic, query entities by component combinations.
- **Resources** hold global singleton state.
- **Events** communicate between systems without tight coupling.
- **Plugins** group related systems/events/resources for modular registration.

### Game State Machine

```
Menu → StartGame → InGame → Progression → EndGame → StartGame (loop)
```

| State | Behavior |
|-------|----------|
| `Menu` | Defined; no implementation yet |
| `StartGame` | Reset resources, tween player in, wait 1.5s |
| `InGame` | Meteor/powerup spawning, movement, collision, shooting active |
| `Progression` | Defined; no implementation yet |
| `EndGame` | Tween player out, cleanup all `CleanUpEndGame` entities, wait 1.5s |

State transitions fire `StateStartEvent` and `StateEndEvent` for cross-system coordination.

### Entity Lifecycle

Dynamic entities (meteors, projectiles, power-ups, player) are tagged with `CleanUpEndGame`. On `EndGame` exit a generic cleanup system despawns all of them, preventing memory leaks across loops.

---

## Key Components

| Component | Struct | Notes |
|-----------|--------|-------|
| Player identity | `PlayerComponent` | Marks the player entity |
| Ship config | `ShipComponent { ship_type, stats }` | Three types: Normal / Shield / Attack |
| Meteor | `MeteorComponent { size, rotation_speed }` | Three sizes: Large / Medium / Small |
| Projectile | `ProjectileComponent` | Despawns after 3 s via `ProjectileDespawnComponent(Timer)` |
| Power-up | `PowerUpComponent { powerup_type, change_target }` | Changes ship type or grants +1 HP |
| Physics | `Velocity(Vec2)`, `Position(Vec2)`, `RotationAngle(f32)` | Shared across all moving entities |
| Collision | `HitBoxSize(Vec2)` | Used in AABB checks |
| Boundary | `BoundsWarpable`, `BoundsDespawnable`, `BoundsDespawnableWithTimer` | Controls edge behavior |
| Collision marker | `CollisionDespawnableWithDamage` | Entity deals damage on player contact |
| Cleanup | `CleanUpEndGame { despawn_entity: bool }` | Marks entity for state-exit cleanup |

---

## Key Resources

| Resource | Type | Purpose |
|----------|------|---------|
| `GameSprites` | Struct of `Handle<Image>` | All preloaded texture handles |
| `WindowSize` | `{ w: f32, h: f32 }` | Current window dimensions (1280×720) |
| `WindowDespawnBorder` | Struct | Boundary thresholds for despawn systems |
| `Life` | `{ max_life, current_life }` | Player health (starts at 3) |
| `Score` | `{ current, high_score }` | Score tracking |

---

## Game Constants (`src/lib.rs`)

```rust
MAX_FRAMERATE          = 60.0
PLAYER_SIZE            = (50, 50)
PLAYER_START_HP        = 3
PLAYER_MAX_SPEED       = 5.0
PLAYER_SHOOT_COOLDOWN  = 0.15  // seconds
PROJECTILE_SPEED       = 10.0
METEOR_SPAWN_TIME      = 3.0   // seconds between spawns
POWERUP_SPAWN_TIME     = 5.0   // seconds between spawns
METEOR_MAX_COUNT       = 10
POWERUP_MAX_COUNT      = 3
```

Change game balance here. Do not hardcode magic numbers elsewhere.

---

## Player Controls

| Key | Action |
|-----|--------|
| Arrow Up | Accelerate forward |
| Arrow Left / Right | Rotate ship |
| Space | Fire projectile (respects cooldown) |
| X | Randomize ship type (debug / gameplay shortcut) |

---

## Collision System

Detection uses Bevy's built-in `bevy::sprite::collide_aabb::collide` (AABB).

| Pair | Result |
|------|--------|
| Projectile → Meteor | Projectile despawns; meteor splits into 3 smaller or despawns if smallest; score increases |
| Player → Meteor | Meteor despawns; player takes 1–3 damage based on meteor size |
| Player → PowerUp | PowerUp despawns; ship type changes or +1 HP if same type as current |

Collision loops use a `HashSet` to prevent double-processing the same entity pair.

---

## Naming Conventions

| Category | Convention | Example |
|----------|------------|---------|
| Components | `PascalCase` + `Component` suffix | `PlayerComponent`, `MeteorComponent` |
| Systems | `snake_case` + `_system` suffix | `collision_damage_system`, `spawn_meteor_system` |
| Resources | `PascalCase` (no suffix) | `WindowSize`, `GameSprites` |
| Enums | `PascalCase` | `GameStates`, `ShipType`, `MeteorSizeType` |
| Constants | `UPPER_SNAKE_CASE` | `PLAYER_MAX_SPEED`, `METEOR_SPAWN_TIME` |
| Events | `PascalCase` + `Event` suffix | `PlayerDeadEvent`, `StateStartEvent` |
| Plugins | `PascalCase` + `Plugin` suffix | `PlayerPlugin`, `MeteorPlugin` |

---

## Adding New Features

### New game object
1. Create a component struct in `src/objects/<name>.rs`.
2. Add a `Plugin` that registers spawn/movement/cleanup systems.
3. Tag entities with `CleanUpEndGame` so they are cleaned up on state exit.
4. Add texture handles to `GameSprites` in `resources.rs` and preload in `main.rs`.
5. Register the plugin in `main.rs`.

### New game state
1. Add variant to `GameStates` in `src/state/states.rs`.
2. Add transition systems with `.on_enter(GameStates::YourState)` and `.on_exit(...)`.
3. Gate active systems with `.run_if(in_state(GameStates::YourState))`.

### New collision type
- Add detection logic inside `src/common/collision.rs` following the existing AABB pattern.
- Use a `HashSet` to guard against duplicate processing in the same frame.

---

## Dependencies Summary

| Crate | Version | Use |
|-------|---------|-----|
| `bevy` | 0.11.2 | Core engine (ECS, rendering, input, assets) |
| `bevy_tweening` | 0.8.0 | Tween animations (player entry/exit) |
| `bevy_hanabi` | 0.7.0 | GPU particle effects (currently unused/commented) |
| `bevy_framepace` | 0.13.3 | Frame rate limiter (60 FPS cap) |
| `bevy-inspector-egui` | 0.19.0 | Runtime debug inspector (disabled in main) |
| `rand` | 0.8.5 | RNG for spawn positions, rotation speeds |

---

## Known Incomplete / Placeholder Areas

| Area | Status |
|------|--------|
| `src/background.rs` | Empty file, no background rendering |
| `src/utils/object_pool.rs` | Object pool struct defined, not used |
| `src/utils/setup.rs` | Empty utility stubs |
| `src/effects/effects.rs` | bevy_hanabi particle systems commented out |
| `Menu` state | No UI or logic implemented |
| `Progression` state | Defined but empty |
| Audio | No sound system present |
| Tests | No unit or integration tests |

---

## Testing

There are currently no automated tests. Debug workflows:

- `bevy-inspector-egui` can be re-enabled in `main.rs` to inspect ECS state at runtime.
- Key events (spawn, death, state changes) log via `println!` / `info!`.
- Collision damage values are printed to stdout on hit.

When adding tests in the future, place unit tests in `#[cfg(test)]` modules at the bottom of the relevant file, and integration tests in `tests/`.

---

## Common Pitfalls

- **Single-player query**: Systems that touch the player use `query.get_single_mut()`. Guard with `if let Ok(...) = ...` to avoid panics when the player is absent (e.g. during `EndGame`).
- **State-gated systems**: Systems that should only run in `InGame` must use `.run_if(in_state(GameStates::InGame))`. Forgetting this causes systems to run in wrong states.
- **Asset paths**: All asset paths are relative to the `assets/` directory (Bevy convention). The `assets/` folder must be present next to the binary at runtime.
- **Cleanup**: Any spawned entity that should not persist across game loops must have `CleanUpEndGame` inserted at spawn time.
