# Project Structure

**One workspace + three crates**

- `game_core` (library): the **shared game kernel**—components, resources, events, systems (toggleable).
- `server` (binary): reuses `game_core`, runs in **authoritative / headless** mode.
- `client` (binary): reuses `game_core`, runs in **rendering + client-prediction** mode. 

------

# Responsibilities

## **server (authoritative)**

- Rules/validation: collision results, score calculation
- Authoritative physics: final position/velocity updates
- Level generation and map loading
- Run bot AI directly on the server
- Decide player spawn/despawn
- Send view-culled snapshots to clients

> Principle: **Any “result” that could be cheated must be decided by the server.** 

## **client (rendering + feel/control)**

- Rendering / UI / audio
- Reconcile local physics with server
- Subscribe to other players’ state
- Gather input and upload intents
- Menu animation/state control
- Camera control

> Principle: **Client only affects presentation and input**; final state still follows the server. 

## **game_core (shared library)**

#### Entity modules

- `player`: spawns player entities. Only reacts to external `event` inputs; does not self-spawn.
- `rope`: spawns rope entities and sends physics requests to `physics`.
- `platform`: spawns platform entities.
- `coin`: spawns coins/rewards. 

#### Solver modules

- `score`: scoring system; consumes coin-collection info.
- `collision`: detects/solves collisions; passes geometric data to `physics` for physical resolution.
- `physics`: physics solver with encapsulated updates (no leakage). 

#### Interaction modules

- `control`: control logic; implements separate interfaces for players and AI; outputs to physics. `PlayerIntentEvent` carries player intent—no physics solving here.
- `camera`: camera creation & tracking; client-only.
- `ui`: read-only data for client display.
- `menu`: game menu system on the initial screen; collects input only. 

#### Kernel modules

- `state_machine`: game states (menu, in-game, game-over).
- `player_lifetime`: player spawning and life-state tracking.
- `map_loading`: map generation.
- `net`: multiplayer networking; server–client interaction. 

#### System infrastructure

- `event`: inter-module messaging.
- `system_set`: per-frame system execution order conventions. 

------

# Directory Layout

```
/katsuo
├─ Cargo.toml
├─ game_core/
│  ├─ src/
│  │  ├─ player
│  │  ├─ rope
│  │  ├─ platform
│  │  ├─ coin
│  │  ├─ score
│  │  ├─ collision
│  │  ├─ physics
│  │  ├─ control
│  │  ├─ camera
│  │  ├─ ui
│  │  ├─ menu
│  │  ├─ state_machine
│  │  ├─ player_lifetime
│  │  ├─ map_loading
│  │  ├─ net
│  │  ├─ event.rs
│  │  ├─ system_set.rs
│  │  └─ lib.rs
│  └─ Cargo.toml
│
├─ server/
│  ├─ src/main.rs
│  └─ Cargo.toml
│
└─ client/
   ├─ src/main.rs
   └─ Cargo.toml
```