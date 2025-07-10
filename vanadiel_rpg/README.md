# Vana'diel RPG

A Bevy-powered top-down RPG inspired by Final Fantasy XI.

## Quick Start

```bash
git clone <repo-url>
cd vanadiel_rpg
./scripts/fetch_assets.sh
cargo run -p game
```

Run `./scripts/fetch_assets.sh` anytime to refresh the public-domain assets.

The textures originate from [Kenney](https://kenney.nl) and
[DeadlyDan/rustyjam2023](https://github.com/DeadlyDan/rustyjam2023).
The music loop is hosted on [OpenGameArt](https://opengameart.org/).

See [docs/design.md](docs/design.md) for the game design and tech spec.

## Custom Skillchains

Weapon skills, gear bonuses, and skillchain sequences are defined in the
`game/assets/data` folder. Modders can extend `weapon_skills.toml` and
`skillchain_table.toml` to script new chains. The engine loads these tables at
startup so additional combos require no code changes.
