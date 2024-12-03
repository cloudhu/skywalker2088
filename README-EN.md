# skywalker2088
Vampire survivor like game, cosmic science fiction wasteland, players drive the spaceship to roam in the universe, fight monsters to collect equipment to upgrade the spaceship, research technology tree, explore the remains of civilization.
______
[中文](https://github.com/cloudhu/skywalker2088)
______
# 0. Introduce
- Game type: Vampire survivor like game
- Game background: The player is a space patrol, in a patrol mission found a pirate ship, during the pursuit process accidentally sucked into a black hole, into a cosmic relic, the universe number is 2088.
  In order to return to their own space, players need to drive spaceships to fight monsters, collect equipment, research technology tree, and explore the remains of civilization, and in the process, players gradually uncover the reason of the destruction of this cosmic civilization.
  The final goal is that player needs to find a way to return to his own universe as soon as possible, and bring the case of 2088 to cosmic scientists, perhaps they have a way to avoid the collapse of their own cosmic civilization.
______
# 1. Game Design
______
1. Energy collection: The spaceship has a conversion core that converts space ore into energy, and can also absorb the energy crystal core on the transformed space monster, which provides a steady supply of energy for the spaceship and the energy required for upgrading;
2. Spaceship upgrade: The core and components of Mecha monsters can be used as materials for spaceship upgrade, and the direction of upgrade mainly includes engine upgrade (faster moving speed), weapon upgrade, defense upgrade... Upgraded ships can deal with more powerful monsters;
3. Science and technology research: Through the exploration of civilization relics, players can collect science and technology points, unlock the science and technology tree, so that the spacecraft has more powerful skills, such as laser rays, gravitational wave detectors, drone clusters, super particle cannons...
4. Civilization Relics: As the level increases, players can explore more advanced and dangerous civilization relics, to obtain more powerful cores, more energy and technology points;
5. Art style: Space, science fiction, wasteland, civilization ruins.

## 1.1 Control
______
| Action             | Keyboard 🖮                            | Controller 🎮    |
|--------------------|----------------------------------------|------------------|
| 🕹️ Movement       | 'WASD' / Arrow Keys /left mouse button | D-Pad            |
| 🔫 Fire Weapon     | Auto                                   | Auto             |
| 💥 Special Ability | Right Click / Shift                    | Left Bumper (LB) |
| Pause/Resume       | space bar                              | Start            |
| back               | ESC                                    | B button         |
| Full screen        | F11                                    | Settings         |
| Zoom               | mouse wheel /PageUp, PageDown          | Triggers         |

______
##  1.2. Level Design
The levels are mainly divided into: 1. Drama level; 2. Relic exploration level; 3. Infinite Abyss Level.
###  1.2.1  Drama level
todo……
###  1.2.2  Relic exploration level
todo……
###  1.2.3  Infinite Abyss Level
todo……

______
# 2.  Develop Log
## 2.1  Finished
- [x] Use `cargo generate thebevyflock/bevy_new_2d` to Generated 2d game templates,reference [bevy_new_2d](https://github.com/TheBevyFlock/bevy_new_2d)；
- [x] Core code is copied from [ASCII Space Shooter](https://github.com/JamesHDuffield/ascii-rust) And optimized the structure to adapt Bevy_new_2d template；
- [x] Pause/Resume
- [x] support wasm
- [x] Enemy AI
- [x] Game audio
- [x] Use [bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader) to optimize the resource loading process
- [x] Use [bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio) to Optimize sound play logic and performance
- [x] Supports full screen/window switching and mouse wheel zooming (November 15, 2024)
- [x] Configure property values and multiple languages with csv (English and Chinese are supported first)
- [x] Merging [Thetawave](https://github.com/thetawavegame/thetawave)'s codes and resources to improve code structure and gameplay logics (November 29, 2024)
- [x] Support gamepad handle operation input （employ [leafwing-input-manager](https://github.com/leafwing-studios/leafwing-input-manager) as input plugin）
- [x] Spawner for different type of enemies
- [x] Collision detection with a physics engine（bevy_rapier2d）
- [x] Animation module
- [x] Game UI

## 2.2 待实现列表 TODO List
- [ ] Use [LDTK](https://github.com/Trouv/bevy_ecs_ldtk) to develop game levels
- [ ] Level loading
- [ ] Level switching
- [ ] Game saving, file reading, auto save(employ [bevy_pkv](https://github.com/johanhelsing/bevy_pkv) plugin)
- [ ] Multiplayer support

# 3.  For Developers
## 3.1  Game Engine
Game Engine:[Bevy](https://bevyengine.org/)
## 3.2  Develop Template
- `cargo generate thebevyflock/bevy_new_2d`
- [ASCII Space Shooter](https://github.com/JamesHDuffield/ascii-rust)

## 3.3  Compile Optimism
- [Sccache](https://github.com/mozilla/sccache) : `cargo install sccache --locked`

## 3.4  cmd
- Rust formation： `cargo fmt`
- clippy：`cargo clippy --locked --workspace --all-targets --all-features -- --deny warnings`
- develop run：`cargo run`

# 4.  Showcase
[Itch.io](https://cloudhu.itch.io/skywalker2088)

# 5. External Assets

## 🎵 Music
[Joel Schuman](https://joelhasa.site/) - Original Game Soundtrack

## 📢 Sound Effects
[*Space Ultimate Megapack*](https://gamesupply.itch.io/ultimate-space-game-mega-asset-package) - Comprehensive Space Audio Collection

## 🎨 Art
[Kadith's icons](https://kadith.itch.io/kadiths-free-icons) - Game Iconography

## 📜 Fonts
[*Space Madness*](https://modernmodron.itch.io/) - Font Design by Rose Frye

# 6. Merged Repositories 
- [ASCII Space Shooter](https://github.com/JamesHDuffield/ascii-rust)
- [Thetawave](https://github.com/thetawavegame/thetawave)