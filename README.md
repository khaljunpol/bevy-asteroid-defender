# Bevy Asteroid Defender

A roguelike asteroid shooter built with Rust and the [Bevy](https://bevyengine.org/) engine (v0.11).
Inspired by the arcade classic Asteroids — survive waves of meteors, earn upgrades, and see how far you can go.

---

## Running Locally on Windows

### 1. Install Rust

Download and run the installer from [https://rustup.rs](https://rustup.rs).
Accept the default options. This installs `cargo` and `rustc`.

### 2. Install Visual Studio C++ Build Tools

Rust on Windows requires the MSVC linker. Install it from:
[https://visualstudio.microsoft.com/visual-cpp-build-tools/](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

During install, select **"Desktop development with C++"**.

### 3. Clone the Repository

Open **Command Prompt** or **PowerShell** and run:

```powershell
git clone https://github.com/khaljunpol/bevy-asteroid-defender.git
cd bevy-asteroid-defender
git checkout claude/add-claude-documentation-okwtt
```

### 4. Run the Game

```powershell
# Fast dev build (recommended for testing)
cargo run --features dev

# Or standard build
cargo run
```

> The first build will take a few minutes — Rust is compiling all dependencies.
> Subsequent builds are much faster.

---

## Controls

| Key | Action |
|-----|--------|
| Arrow Up | Hold to accelerate |
| Arrow Left | Rotate left |
| Arrow Right | Rotate right |
| Space | Shoot |
| Space / Enter | Confirm (menus, upgrade selection, restart) |
| Arrow Left / Right (or A / D) | Navigate upgrade choices |

---

## How to Play

1. **Survive** — destroy all meteors on screen to clear the level.
2. **After each level** — pick 1 of 3 random upgrades to power up your ship.
3. **Don't die** — if a meteor hits you, you lose HP. Reach 0 and it's game over.
4. **HP packs** — blue star pickups restore 1 HP. They spawn mid-level.
5. **Meteors split** — large meteors break into smaller ones when destroyed.

### Upgrades (10 total)

| Upgrade | Effect |
|---------|--------|
| Split Shot | Fire extra bullets in a spread |
| Rapid Fire | Reduce shoot cooldown |
| Heavy Rounds | Increase bullet damage |
| Ricochet | Bullets bounce off screen edges |
| Extra Armor | Gain +1 max HP and +1 current HP |
| Afterburner | Increase top speed |
| Quick Reflexes | Increase turn speed |
| Overclock | Asteroids move at 60% speed |
| Chain Reaction | Killing an asteroid triggers 3s of ultra-rapid fire |
| Asteroid Magnet | HP packs drift toward you |

---

## Level Scaling

Difficulty increases each level — more asteroids and higher HP on large ones.

---

## Dependencies

- [bevy_tweening](https://github.com/djeedai/bevy_tweening)
- [bevy_framepace](https://github.com/aevyrie/bevy_framepace)

## Assets

[Kenney Space Shooter Redux](https://kenney.nl/assets/space-shooter-redux) — credit to [kenney.nl](https://kenney.nl/)
