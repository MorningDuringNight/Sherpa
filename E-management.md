# Bevy Project Collaboration & Code Standards

*Goal: ensure **maintainability**, **extensibility**, and **stable frame behavior** under multi-person collaboration. These standards constrain the entry structure, plugins & scheduling, event design, `SystemSet` usage, module ownership & documentation, and team communication / request templates.*

------

## 0. Overview

1. `main.rs` rules
2. `event` rules
3. `SystemSet` rules
4. `plugin` rules
5. Module ownership
6. Documentation requirements
7. Naming conventions
8. PR/commit conventions
9. Communication template
10. Prohibited items (continuously updated)

------

## 1) Entry constraints: `main.rs` only assembles plugins

**Principle:** `main` only **builds the App + registers plugins**. Do not write any business systems or resource initialization details here.

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            CoreSchedulePlugin,    // Core scheduling (custom)
            PhysicsPlugin,         // Physics / fixed timestep
            PlayerPlugin,          // Player
            RopePlugin,            // Rope
            UiPlugin,              // UI / HUD
            NetPlugin,             // Net / snapshots
            // ... add plugins only
        ))
        .run();
}
```

- **Forbidden in `main.rs`:**
  - Adding systems directly (`.add_systems(...)`), spawning entities, writing event logic
  - Directly accessing/initializing business resources (should be encapsulated inside a plugin)

------

## 2) Scheduling model & `SystemSet` rules

### 2.1 Schedule stages

- Use Bevy’s standard stages: `First` → `PreUpdate` → `Update` → `PostUpdate` → `Last`
- **Fixed-step physics:** use `FixedUpdate`; fix `dt = 1/60`

### 2.2 `SystemSet` naming & layering

- Name sets with **module prefix** + **function**; derive an enum tag:

```rust
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PhysicsSet {
    PreIntegrate,   // Constraint sampling / force collection
    Integrate,      // Integrate (velocity / position)
    Resolve,        // Collision / constraint solving
    Sync,           // Sync to rendering / broadcast events
}
```

- **Ordering** (explicit in the plugin via `configure_sets`):

```rust
app
  .configure_sets(FixedUpdate, (
      PhysicsSet::PreIntegrate,
      PhysicsSet::Integrate,
      PhysicsSet::Resolve,
      PhysicsSet::Sync,
  ).chain());
```

- When **adding systems**, you must specify the stage and set:

```rust
app.add_systems(
    FixedUpdate,
    (
        collect_forces.in_set(PhysicsSet::PreIntegrate),
        integrate_momentum.in_set(PhysicsSet::Integrate),
        resolve_collisions.in_set(PhysicsSet::Resolve),
        sync_state_to_render.in_set(PhysicsSet::Sync),
    )
);
```

- **Run conditions:** for any switchable system, define a `run_if` with a toggle resource/state (e.g., `GameState::InGame`).

------

## 3) Event design rules

### 3.1 Definition

- Events are **pure data**; no logic, avoid heavy references (prefer not to hold `&T`)
- Recommended derives: `#[derive(Event, Debug, Clone, Copy)]` (if it contains an `Entity`, you usually *cannot* use `Copy`)
- **Naming:** `<Domain><Intent|Happened>`, e.g., `PlayerIntentJump`, `HitHappened`

```rust
#[derive(Event, Debug, Clone, Copy)]
pub struct PlayerIntentJump {
    pub player: Entity,
    pub strength: f32,
}
```

### 3.2 Registration & usage

```rust
// Register in a plugin
app.add_event::<PlayerIntentJump>();

// Send
fn handle_input(
    keyboard: Res<Input<KeyCode>>,
    mut evw: EventWriter<PlayerIntentJump>,
    q_player: Query<Entity, With<LocalPlayer>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if let Ok(player) = q_player.get_single() {
            evw.write(PlayerIntentJump { player, strength: 1.0 });
        }
    }
}

// Receive
fn apply_jump(
    mut evr: EventReader<PlayerIntentJump>,
    mut q: Query<&mut NetForce, With<PlayerTag>>,
) {
    for e in evr.read() {
        if let Ok(mut f) = q.get_mut(e.player) {
            f.0.y += e.strength * JUMP_FORCE;
        }
    }
}
```

### 3.3 Event channels & ownership

- **Source first**, **effect later**: Input/AI → Intent → Physics/State
- **Forbidden** to modify physics directly in UI/rendering stages; to change physics, send an event to the physics module.

### 3.4 When to Use Events

- **Use events for:**

  - **Instant, one-off happenings:** spawn/despawn, control handoff, damage taken, door opened, play SFX, logging—i.e., edge-triggered actions.
  - **Cross-module notifications:** sender doesn’t know receiver internals; send an **intent/request value** and let the target decide which components to add.
  - **Low-frequency, non-persistent signals:** event buffers are cleared after read; not meant to store long-lived state.

- **Don’t use events for:**

  - **High-frequency continuous data** (per-frame input axes, velocity, position sync). Prefer **Components/Resources**.
  - **Queryable persistent state** (HP, control mode, team). Keep these as **Components**; events should only announce that a change occurred.

- **Are lots of events bad?**

  - It’s about **granularity**, not count: *one action = one event*. Don’t turn per-frame noise into an event firehose.
  - Keep event names clear, payloads small, and payloads as **value objects**—never implementation components.
  - Performance: Bevy batches events efficiently for typical low-rate use. Flooding with per-frame axis values is inappropriate.

------

## 4) Plugin and module (crate/module) structure

### 4.1 Plugin skeleton

```rust
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        use crate::core::schedule::{PlayerSet, GameState};

        app
          .configure_sets(Update, (
              PlayerSet::Input,
              PlayerSet::EmitIntent,
          ).chain())
          .add_event::<PlayerIntentJump>()
          .add_systems(Update, (
              input_keyboard     .in_set(PlayerSet::Input)     .run_if(in_state(GameState::InGame)),
              emit_jump_intent   .in_set(PlayerSet::EmitIntent).run_if(in_state(GameState::InGame)),
          ));
    }
}
```

------

## 5) Module ownership & CODEOWNERS

| Module  | Owner | Backup | Review threshold         |
| ------- | ----- | ------ | ------------------------ |
| core    |       |        | 2 LGTM (incl. Owner)     |
| physics |       |        | 2 LGTM (incl. Owner)     |
| player  |       |        | 1 LGTM (Owner or Backup) |
| rope    |       |        | 2 LGTM                   |
| net     |       |        | 2 LGTM (incl. Net Owner) |
| ui      |       |        | 1 LGTM                   |
|         |       |        |                          |
|         |       |        |                          |
|         |       |        |                          |

- Any **cross-module interface** change must involve the corresponding Owner in review.

------

## 6) Documentation requirements (each module **must** include a `README.md`)

### 6.1 README template

```
# <Module Name>

**Responsibility:** One-sentence boundary of the module and what it explicitly does NOT do.

## Public API (prelude)
- Events: ...
- Components: ...
- Resources / States: ...

## Data model
- Component table: fields / units / invariants (e.g., velocity unit: m/s)
- Event flow: source → consumers (with stage / Set)

## Scheduling diagram
- List of SystemSets and execution order (before / after / chain)

## Invariants & constraints
- e.g., `Velocity` is writable only in `FixedUpdate::Integrate`

## Test checklist
- Unit tests: ...
- Scenario regression: ...

## Common change flows
- How to add a new intent event
- How to add a new constraint / force
```

------

## 7) Naming conventions

- **Filenames**:
- **Components**: `PascalCase` (nouns), e.g., `NetForce`, `Momentum`
- **Resources/States**: `PascalCase`, e.g., `GameState`, `WorldBounds`
- **Events**: `PascalCase` + suffix `Intent|Happened|Request|Response`
- **System functions**: `snake_case` starting with a verb, e.g., `collect_forces`, `emit_jump_intent`
- **Constants**: `UPPER_SNAKE_CASE`, e.g., `FIXED_DT`

------

## 8) PR/Commit conventions

- 

------

## 9) Communication / request template (also for Issue/discussion posts)

```
[Title] <Module>/<Interface> Request: <one sentence>

[Background / Context]

[Expected Outcome]

[Acceptance Criteria]

[Timeline & Ownership]
- Owner:
- Due:
```

------

## 10) Prohibited items

- Writing to authoritative physics components (velocity/position) in `Update`
- Directly operating on the World in `main.rs`
- Packing complex logic/large objects into events
- Exposing internals via broad `pub` access without documentation; cross-module direct access
- Changing interfaces without updating README/tests