use bevy::prelude::*;
use rand::prelude::*;
use super::state::*;

#[derive(Component, Clone)]
pub struct Bot {
    pub state_machine: StateMachine,

}

#[derive(Event)]
pub struct PlayerEvent{
    pub entity: Entity,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}


impl Bot{
    pub fn new() -> Self {
        Self {
            state_machine: StateMachine::new(BotState::idel),
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
        mut keys: &mut ButtonInput<KeyCode>,
        // timer: Res<Time>,
    ) -> BotState{
        //temporary random movement to change state
        let rng = rand::rng();
        let mut input;
        //remove when done please
        let next = match self.state_machine.current{ // bevy timer repeating
            //idel change to 
            BotState::idel =>{
                input = rand::rng().random_range(0..=4);
                println!("print idel {}", input);
                if input == 0{
                    keys.press(KeyCode::ArrowRight);
                    // timer.0.reset(); // Reset the timer when the key is pressed
                    // timer.0.set_duration(Duration::from_secs(2)); // Set the desired duration
                    // timer.0.set_mode(TimerMode::Once); // Set to once
                    BotState::right
                }
                else if input == 1{
                    keys.press(KeyCode::ArrowLeft);
                    BotState::left
                }
                else if input == 2{
                    keys.press(KeyCode::ArrowUp);
                    BotState::jump
                }
                else if input == 3{
                    keys.press(KeyCode::ArrowDown);
                    BotState::idel
                }
                else if input == 4{
                    keys.press(KeyCode::ArrowLeft);
                    BotState::left
                }
                else{
                    //println!("print Hurt you");
                    BotState::idel
                }
            }

            BotState::right =>{
                println!("print righj");
                input = rand::rng().random_range(0..=3);
                if input == 0{
                    keys.press(KeyCode::ArrowRight);
                    BotState::right
                }
                else if input == 1{
                    keys.press(KeyCode::ArrowLeft);
                    BotState::left
                }

                else if input == 3{
                    keys.press(KeyCode::ArrowDown);
                    BotState::idel
                }
                else{
                    keys.press(KeyCode::ArrowDown);
                    BotState::idel
                }
            }

             BotState::left =>{
                println!("print lkeft");
                input = rand::rng().random_range(0..=3);
                if input == 10{
                    keys.press(KeyCode::ArrowRight);
                    BotState::right
                }
                else if input == 1{
                    keys.press(KeyCode::ArrowLeft);
                    BotState::left
                }
                else if input == 100{
                    keys.press(KeyCode::ArrowDown);
                    BotState::idel
                }
                else{
                    keys.press(KeyCode::ArrowDown);
                    BotState::idel
                }
            }
             BotState::jump =>{
                println!("print jump");
                input = rand::rng().random_range(0..=2);
                if input == 2{
                    keys.press(KeyCode::ArrowDown);
                    BotState::idel
                }
                else{
                    keys.press(KeyCode::ArrowDown);
                    BotState::idel
                }
            }
            
            
            };
            //return next;
            self.state_machine.current = next.clone();
            next
        }
    }