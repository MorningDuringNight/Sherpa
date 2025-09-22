use bevy::prelude::*;

pub fn run() {
    App::new()
        .insert_resource(DebugColliders(false))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, apply_gravity, player_platform_collision, toggle_debug))
        .add_systems(Update, draw_colliders.run_if(debug_colliders_on))
        .run();
}

#[derive(Component)]
struct Player {
    controls: Controls,
}

struct Controls {
    left: KeyCode,
    right: KeyCode,
    jump: KeyCode,
}

#[derive(Component)]
struct Platform;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Collider {
    size: Vec2,
}

#[derive(Component)]
struct Grounded(pub bool);

#[derive(Component)]
struct JumpState {
    jump_count: u8,     // how many jumps have been used
    max_jumps: u8,      // usually 2 for double jump
    coyote_timer: f32,  // seconds left for grace period
}


#[derive(Resource)]
struct DebugColliders(pub bool);


#[derive(Bundle)]
struct PlayerBundle {
    sprite: Sprite,
    transform: Transform,
    player: Player,
    velocity: Velocity,
    collider: Collider,
    grounded: Grounded,
    jump_state: JumpState,
}

impl PlayerBundle {
    fn new(asset_server: &Res<AssetServer>, position: Vec3, controls: Controls, asset_path: &str) -> Self {
        Self {
            sprite: Sprite::from_image(asset_server.load(asset_path)),
            transform: Transform {
                translation: position,
                scale: Vec3::splat(0.5),
                ..default()
            },
            player: Player { controls },
            velocity: Velocity { x: 0.0,  y: 0.0 },
            collider: Collider {
                size: Vec2::new(100.0, 110.0),
            },
            grounded: Grounded(true),
            jump_state: JumpState{jump_count: 0, max_jumps: 2, coyote_timer: 0.1},
        }
    }
}

// Press enter to toggle debug mode
// Debug mode will 
fn toggle_debug(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut debug: ResMut<DebugColliders>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        debug.0 = !debug.0;
    }
}

fn debug_colliders_on(debug: Res<DebugColliders>) -> bool {
    debug.0
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // make player 1
    commands.spawn(PlayerBundle::new(
        &asset_server,
        Vec3::new(-100., -200., 0.0),
        Controls {
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            jump: KeyCode::KeyW,
        },
        "spriteguy.png"
    ));

    // make player 2
    commands.spawn(PlayerBundle::new(
        &asset_server,
        Vec3::new(-200., -200., 0.0),
        Controls {
            left: KeyCode::ArrowLeft,
            right: KeyCode::ArrowRight,
            jump: KeyCode::ArrowUp,
        },
        "spriteguy.png"
    ));
    
    // platform
    commands.spawn((
            Sprite {
                color: Color::srgb(0.8, 0.6, 0.2), // orange rectangle
                custom_size: Some(Vec2::new(200.0, 20.0)),
                ..default()
            },
            Transform {
                translation: Vec3::new(500.0, -290.0, 0.0),
                ..default()
            },
            Platform,
            Collider {
                size: Vec2::new(200.0, 20.0),
            },
    ));
}

fn draw_colliders(
    mut gizmos: Gizmos,
    players: Query<(&Transform, &Collider), With<Player>>,
    platforms: Query<(&Transform, &Collider), With<Platform>>,
    debug: Res<DebugColliders>,
) {
    if !debug.0 {
        return;
    }

    for (tf, col) in &players {
        let pos = tf.translation.truncate();
        gizmos.rect_2d(pos, col.size, Color::srgb(0.8, 0.1, 0.9));
    }

    for (tf, col) in &platforms {
        let pos = tf.translation.truncate();
        gizmos.rect_2d(pos, col.size, Color::srgb(1.0, 0.1, 0.6));
    }
}


fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Grounded, &mut JumpState, &Player), With<Player>>,
) {
    let speed = 200.0;
    let jump_strength = 400.0;

    for (mut velocity, mut grounded, mut jump_state, player) in &mut query {
        // Reset horizontal velocity each frame
        velocity.x = 0.0;

        // Horizontal movement
        if keyboard_input.pressed(player.controls.left) {
            velocity.x -= speed;
        }
        if keyboard_input.pressed(player.controls.right) {
            velocity.x += speed;
        }

        // Jump
        if keyboard_input.just_pressed(player.controls.jump) {
            let can_jump_from_ground = grounded.0 || jump_state.coyote_timer > 0.0;
            let can_double_jump = jump_state.jump_count < jump_state.max_jumps;

            if can_jump_from_ground || can_double_jump {
                velocity.y = jump_strength;
                grounded.0 = false;
                jump_state.jump_count += 1;
                jump_state.coyote_timer = 0.0; // reset grace after jumping
            }
        }
    }
}

// this also applies velocity
// That is maybe problematic
fn apply_gravity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut Grounded, &mut JumpState), With<Player>>,
) {
    let gravity = -900.0;
    let ground_y = -300.0;

    for (mut transform, mut velocity, mut grounded, mut jump_state) in &mut query {
        // Apply gravity
        velocity.y += gravity * time.delta_secs();

        // Apply velocity to position
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();

        // World ground clamp
        if transform.translation.y < ground_y {
            transform.translation.y = ground_y;
            velocity.y = 0.0;

            // ✅ Landing state reset
            grounded.0 = true;
            jump_state.jump_count = 0;      // reset jumps
            jump_state.coyote_timer = 0.1;  // give grace period
        } else {
            grounded.0 = false;
        }
    }
}


// Lets rethink how collisions should work
// Platforms positions are checked against player posiitons.
fn player_platform_collision(
    time: Res<Time>,
    mut players: Query<(
        &mut Transform,
        &mut Velocity,
        &Collider,
        &mut Grounded,
        &mut JumpState,
    ), With<Player>>,
    platforms: Query<(&Transform, &Collider), (With<Platform>, Without<Player>)>,
) {
    for (mut player_tf, mut velocity, player_col, mut grounded, mut jump_state) in &mut players {
        grounded.0 = false;

        let player_half = player_col.size * 0.5;

        // Apply velocity
        player_tf.translation.x += velocity.x * time.delta_secs();
        player_tf.translation.y += velocity.y * time.delta_secs();

        for (plat_tf, plat_col) in &platforms {
            let plat_half = plat_col.size * 0.5;

            let player_min = player_tf.translation.truncate() - player_half;
            let player_max = player_tf.translation.truncate() + player_half;

            let plat_min = plat_tf.translation.truncate() - plat_half;
            let plat_max = plat_tf.translation.truncate() + plat_half;

            let overlap_x = player_min.x < plat_max.x && player_max.x > plat_min.x;
            let overlap_y = player_min.y < plat_max.y && player_max.y > plat_min.y;

            if overlap_x && overlap_y {
                // Compute penetration depths
                let pen_left = player_max.x - plat_min.x;
                let pen_right = plat_max.x - player_min.x;
                let pen_x = pen_left.min(pen_right);

                let pen_down = player_max.y - plat_min.y;
                let pen_up = plat_max.y - player_min.y;
                let pen_y = pen_down.min(pen_up);

                // Resolve along smaller penetration axis
                if pen_x < pen_y {
                    // ✅ Resolve X
                    if pen_left < pen_right {
                        player_tf.translation.x -= pen_left;
                    } else {
                        player_tf.translation.x += pen_right;
                    }
                    velocity.x = 0.0;
                } else {
                    // ✅ Resolve Y — but only if falling
                    if velocity.y <= 0.0 {
                        // Landing
                        player_tf.translation.y = plat_max.y + player_half.y;
                        grounded.0 = true;
                        velocity.y = 0.0;
                        jump_state.jump_count = 0;
                        jump_state.coyote_timer = 0.1;
                    } else {
                        // 🚫 If jumping, ignore vertical correction
                        // (prevents teleport-down bug)
                    }
                }
            }
        }

        // Tick down coyote timer if not grounded
        if !grounded.0 {
            jump_state.coyote_timer -= time.delta_secs();
        }
    }
}
