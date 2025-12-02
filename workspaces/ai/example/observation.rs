pub fn observation_system(
    player: Query<(&Transform, &Velocity), With<MainPlayer>>,
    envirnment: Query<(&TotalCoin, &GroundState), With<MainPlayer>>,
    mut obs_writer: EventWriter<Observation>,
) {
    let (transform, velocity) = player_state(&player);
    let (coin, is_ground) = env_reward(&envirnment);
    // x, y, vx, vy
    let obs_vec = to_state(&transform, &velocity);
    // coin, height, level_bonus, wall_detect
    let rew_vec = to_reward(&transform, &velocity, &coin, &is_ground);

    obs_writer.write(Observation {
        observation: obs_vec,
        reward: rew_vec,
    });
}