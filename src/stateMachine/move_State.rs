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
    pub flip_timer: f32,
    pub flip_period: f32,
    pub last_pos: Vec2,
    pub still_time: f32,
    pub move_eps: f32,
    pub flip_if_still: f32,
}

impl PatrolMemory {
    pub fn new() -> Self {
        Self {
            flip_timer: 0.0,
            flip_period: 1.0,
            dir: 1,
            last_pos: Vec2::ZERO,
            still_time: 0.0,
            move_eps: 1.0,
            flip_if_still: 0.5,
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
    // decide next patrol point
    let dt = time.delta_secs();

    mem.flip_timer += dt;
    if mem.flip_timer >= mem.flip_period {
        mem.flip_timer = 0.0;
        mem.dir = -mem.dir;
    }
    let pos = tf.translation().truncate();
    let moved = pos.distance(mem.last_pos);
    if moved < mem.move_eps {
        mem.still_time += dt;
        if mem.still_time >= mem.flip_if_still {
            mem.still_time = 0.0;
            mem.dir = -mem.dir;
        }
    } else {
        mem.last_pos = pos;
        mem.still_time = 0.0;
    }

    keys.release(KeyCode::ArrowLeft);
    keys.release(KeyCode::ArrowRight);
    keys.release(KeyCode::ArrowUp);

    let mut rng = rand::thread_rng();

    let r: u8 = rng.gen_range(0..=5);
    match r {
        0 => {
            keys.press(KeyCode::ArrowLeft);
            BotState::left
        }
        1 => {
            keys.press(KeyCode::ArrowRight);
            BotState::right
        }
        2 => {
            keys.press(KeyCode::ArrowLeft);
            keys.press(KeyCode::ArrowUp);
            BotState::jump
        }
        3 => {
            keys.press(KeyCode::ArrowRight);
            keys.press(KeyCode::ArrowUp);
            BotState::jump
        }
        _ => {
            keys.press(KeyCode::ArrowDown);
            BotState::idel
        }
    }
}