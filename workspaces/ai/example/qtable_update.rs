pub fn qlearning_update(
    mut obs: EventReader<Observation>,
    mut qtable: ResMut<QTable>,
    mut last_info: Local<LastInfo>,
) {
    for o in obs.read() {
        let (s_next, r_next) = get_obs(&o);
        let (s, a, r) = get_last_info(&last_info);

        let reward = f_reward(r, r_next);
        let old_q  = qtable.get(s, a);
        let max_q  = qtable.max_q(s_next);
        // Q(s,a) <- Q(s,a)  +  α * (  R  +  γ  *  MaxQ(s') - Q(s,a))
        let new_q = old_q + ALPHA * (reward + GAMMA * max_q - old_q);
        qtable.set(s, a, new_q);

        let a_t = epsilon_greedy(&qtable, s_next, epsilon);
        update_last_info(&s_next, &a_next, &r_next);
    }
}