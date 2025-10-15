use bevy::prelude::*;
use rand::prelude::*;
use super::state::*;

#[derive(Component)]
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

    }

    pub fn change(
        &mut self, /*input: &Input*/
        mut keys: &mut ButtonInput<KeyCode>,
    ) -> (BotState, i32){
        //temporary random movement to change state
        let rng = rand::rng();
        let mut input;
        //remove when done please
        let next = match self.state_machine.current{
            //idel change to 
            BotState::idel =>{
                input = rand::rng().random_range(0..3);
                if input == 0{
                    keys.press(KeyCode::ArrowRight);
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
                else{
                    BotState::idel
                }
            }

            BotState::right =>{
                input = rand::rng().random_range(0..3);
                if input == 0{
                    keys.press(KeyCode::ArrowRight);
                    BotState::right
                }
                else if input == 1{
                    keys.press(KeyCode::ArrowLeft);
                    BotState::left
                }
                else if input == 2{
                     keys.press(KeyCode::ArrowUp);
                     keys.press(KeyCode::ArrowRight);
                    BotState::jump_r
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
                input = rand::rng().random_range(0..3);
                if input == 0{
                    keys.press(KeyCode::ArrowRight);
                    BotState::right
                }
                else if input == 1{
                    keys.press(KeyCode::ArrowLeft);
                    BotState::left
                }
                else if input == 2{
                    keys.press(KeyCode::ArrowUp);
                    keys.press(KeyCode::ArrowLeft);
                    BotState::jump_l
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
             BotState::jump =>{
                input = rand::rng().random_range(0..2);
                if input == 0{
                    keys.press(KeyCode::ArrowUp);
                    keys.press(KeyCode::ArrowRight);
                    BotState::jump_r
                }
                else if input == 1{
                    keys.press(KeyCode::ArrowUp);
                    keys.press(KeyCode::ArrowLeft);
                    BotState::jump_l
                }
                else if input == 2{
                    keys.press(KeyCode::ArrowDown);
                    BotState::idel
                }
                else{
                    keys.press(KeyCode::ArrowDown);
                    BotState::idel
                }
            }
            BotState::jump_r =>{
                input = rand::rng().random_range(0..2);
                if input == 0{
                    keys.press(KeyCode::ArrowRight);
                    BotState::right
                }
                else if input == 1{
                    keys.press(KeyCode::ArrowLeft);
                    BotState::left
                }
                else if input == 2{
                    keys.press(KeyCode::ArrowDown);
                    BotState::idel
                }
                else{
                    keys.press(KeyCode::ArrowDown);
                    BotState::idel
                }
            }
            BotState::jump_l =>{
                input = rand::rng().random_range(0..2);
                if input == 0{
                    keys.press(KeyCode::ArrowRight);
                    BotState::right
                }
                else if input == 1{
                    keys.press(KeyCode::ArrowLeft);
                    BotState::left
                }
                else if input == 2{
                    keys.press(KeyCode::ArrowDown);
                    BotState::idel
                }
                else{
                    keys.press(KeyCode::ArrowDown);
                    BotState::idel
                }
            }
            };
            return (next,input);
            
        }
    }