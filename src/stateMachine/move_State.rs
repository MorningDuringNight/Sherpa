use bevy::prelude::*;
use rand::prelude::*;
use super::state::*;

#[derive(Component, Clone)]
pub struct Bot {
    pub state_machine: StateMachine,
    pub patrol_memory: PatrolMemory,
}

#[derive(Event)]
pub struct PlayerEvent{
    pub entity: Entity,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}

#[derive(Default, Clone)]
pub struct PatrolMemory {
    pub dir: i8,
    // pub flip_timer: f32,
    // pub flip_period: f32,
    pub last_pos: Vec2,
    pub still_time: f32,
    pub move_eps: f32,
    pub flip_if_still: f32,
    // pub player_dir: i8, // -1 left, 1 right, 0 unknown
    // pub player_jump: bool,
}

impl PatrolMemory {
    pub fn new() -> Self {
        Self {
            // flip_timer: 0.0,
            // flip_period: 1.0,
            dir: 1,
            last_pos: Vec2::ZERO,
            still_time: 0.0,
            move_eps: 1.0,
            flip_if_still: 0.05,
        }
    }
}

impl Bot{
    pub fn new() -> Self {
        Self {
            state_machine: StateMachine::new(BotState::idel),
            patrol_memory: PatrolMemory::new(),
        }
    }
    // brach this 
    pub fn playerToEvent(
        player_events: EventReader<PlayerEvent>,
        mut state_query: Query<&mut StateMachine>,
    ){
        // timer mode 
        // different timer resource
        // run in a fixed update schedual
        // first statement of state transition if timer not trigger return nothing
        // other state transition
        
    }


    pub fn change(
        &mut self, /*input: &Input*/
        time: &Time,
        tf: &GlobalTransform,
        keys: &mut ButtonInput<KeyCode>,
        // timer: Res<Time>,
    ) -> BotState{
        let next = decide_next_patrol(
            self.state_machine.current.clone(),
            time,
            tf,
            keys,
            &mut self.patrol_memory,
        );
        self.state_machine.current = next.clone();
        next
        //temporary random movement to change state
        // let rng = rand::rng();
        // let mut input;
        // //remove when done please
        // let next = match self.state_machine.current{ // bevy timer repeating
        //     //idel change to 
        //     BotState::idel =>{
        //         input = rand::rng().random_range(0..=4);
        //         println!("print idel {}", input);
        //         if input == 0{
        //             keys.press(KeyCode::ArrowRight);
        //             // timer.0.reset(); // Reset the timer when the key is pressed
        //             // timer.0.set_duration(Duration::from_secs(2)); // Set the desired duration
        //             // timer.0.set_mode(TimerMode::Once); // Set to once
        //             BotState::right
        //         }
        //         else if input == 1{
        //             keys.press(KeyCode::ArrowLeft);
        //             BotState::left
        //         }
        //         else if input == 2{
        //             keys.press(KeyCode::ArrowUp);
        //             BotState::jump
        //         }
        //         else if input == 3{
        //             keys.press(KeyCode::ArrowDown);
        //             BotState::idel
        //         }
        //         else if input == 4{
        //             keys.press(KeyCode::ArrowLeft);
        //             BotState::left
        //         }
        //         else{
        //             //println!("print Hurt you");
        //             BotState::idel
        //         }
        //     }

        //     BotState::right =>{
        //         println!("print righj");
        //         input = rand::rng().random_range(0..=3);
        //         if input == 0{
        //             keys.press(KeyCode::ArrowRight);
        //             BotState::right
        //         }
        //         else if input == 1{
        //             keys.press(KeyCode::ArrowLeft);
        //             BotState::left
        //         }

        //         else if input == 3{
        //             keys.press(KeyCode::ArrowDown);
        //             BotState::idel
        //         }
        //         else{
        //             keys.press(KeyCode::ArrowDown);
        //             BotState::idel
        //         }
        //     }

        //      BotState::left =>{
        //         println!("print lkeft");
        //         input = rand::rng().random_range(0..=3);
        //         if input == 10{
        //             keys.press(KeyCode::ArrowRight);
        //             BotState::right
        //         }
        //         else if input == 1{
        //             keys.press(KeyCode::ArrowLeft);
        //             BotState::left
        //         }
        //         else if input == 100{
        //             keys.press(KeyCode::ArrowDown);
        //             BotState::idel
        //         }
        //         else{
        //             keys.press(KeyCode::ArrowDown);
        //             BotState::idel
        //         }
        //     }
        //      BotState::jump =>{
        //         println!("print jump");
        //         input = rand::rng().random_range(0..=2);
        //         if input == 2{
        //             keys.press(KeyCode::ArrowDown);
        //             BotState::idel
        //         }
        //         else{
        //             keys.press(KeyCode::ArrowDown);
        //             BotState::idel
        //         }
        //     }
            
            
        //     };
        //     //return next;
        //     self.state_machine.current = next.clone();
        //     next
    }
}

pub fn decide_next_patrol(
    current: BotState,
    time: &Time,
    tf: &GlobalTransform,
    keys: &mut ButtonInput<KeyCode>,
    mem: &mut PatrolMemory,
) -> BotState {
    // 打印mem指针
    // info!("PatrolMemory ptr: {:p}", mem);
    // decide next patrol point
    let dt = time.delta_secs();

    // mem.flip_timer += dt;
    // if mem.flip_timer >= mem.flip_period {
    //     mem.flip_timer = 0.0;
    //     mem.dir = -mem.dir;
    // }
    // 解决bot卡住不动的问题
    let pos = tf.translation().truncate();
    let moved = pos.distance(mem.last_pos);
    let mut changed = false;
    if moved < mem.move_eps {
        // info!("Bot seems stuck, moved distance: {}", moved);
        mem.still_time += dt;
        // info!("Still time: {}", mem.still_time);
        if mem.still_time >= mem.flip_if_still {
            // info!("Bot is stuck, flipping direction");
            mem.still_time = 0.0;
            mem.dir = -mem.dir;
            changed = true;
        }
    } else {
        mem.last_pos = pos;
        mem.still_time = 0.0;
    }

    
    keys.release(KeyCode::ArrowLeft);
    keys.release(KeyCode::ArrowRight);
    keys.release(KeyCode::ArrowUp);
    keys.release(KeyCode::ArrowDown);


    let mut rng = rand::thread_rng();

    // 1. 被困状态：优先解救
    if changed {
        // info!("[1] Bot is stuck, trying to escape...");
        let r: u8 = rng.gen_range(0..=1);
        match r {
            0 => {
                if mem.dir == -1 {
                    keys.press(KeyCode::ArrowLeft);
                    // info!("  Bot is jumping left to escape");
                } else {
                    keys.press(KeyCode::ArrowRight);
                    // info!("  Bot is jumping right to escape");
                }
                keys.press(KeyCode::ArrowUp);
                return BotState::jump;
            }
            _ => {
                if mem.dir == -1 {
                    keys.press(KeyCode::ArrowLeft);
                    // info!("  Bot is moving left to escape");
                    return BotState::left;
                } else {
                    keys.press(KeyCode::ArrowRight);
                    // info!("  Bot is moving right to escape");
                    return BotState::right;
                }
            }
        }
    }

    // 2. 一定概率进入玩家跟随状态
    if rng.gen_bool(0.6){
        // info!("[2] Bot is following the player...");

        let player_left = keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
        let player_right = keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
        let player_jump = keys.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
        if player_left {
            if mem.dir != -1 {
                mem.dir = -1;
            }
            keys.press(KeyCode::ArrowLeft);
            if player_jump {
                keys.press(KeyCode::ArrowUp);
                // info!("  Bot is moving left and jumping");
                return BotState::jump;
            } else {
                // info!("  Bot is moving left");
                return BotState::left;
            }
        } else if player_right {
            if mem.dir != 1 {
                mem.dir = 1;
            }
            keys.press(KeyCode::ArrowRight);
            if player_jump {
                keys.press(KeyCode::ArrowUp);
                // info!("  Bot is moving right and jumping");
                return BotState::jump;
            } else {
                // info!("  Bot is moving right");
                return BotState::right;
            }
        } else if player_jump {
            keys.press(KeyCode::ArrowUp);
            // info!("  Bot is jumping");
            return BotState::jump;
        } else {
            // info!("  Bot is idling");
        }
    }

    // 3. 常规巡逻状态
    let r: u8 = rng.gen_range(0..=5);
    // info!("[3] Bot is patrolling, random choice: {}", r);
    match r {
        0 => {
            keys.press(KeyCode::ArrowLeft);
            mem.dir = -1;
            // info!("  Bot is moving left");
            BotState::left
        }
        1 => {
            keys.press(KeyCode::ArrowRight);
            mem.dir = 1;
            // info!("  Bot is moving right");
            BotState::right
        }
        2 => {
            keys.press(KeyCode::ArrowLeft);
            keys.press(KeyCode::ArrowUp);
            mem.dir = -1;
            // info!("  Bot is jumping left");
            BotState::jump
        }
        3 => {
            keys.press(KeyCode::ArrowRight);
            keys.press(KeyCode::ArrowUp);
            mem.dir = 1;
            // info!("  Bot is jumping right");
            BotState::jump
        }
        _ => {
            keys.press(KeyCode::ArrowDown);
            // info!("  Bot is idling");
            BotState::idel
        }
    }
}