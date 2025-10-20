use bevy::prelude::*;
use super::move_State::*;
//collect data from every frame booleans

#[derive(Component, Clone,)]
pub enum BotState{
    idel,
    right,
    left,
    jump,
    // jump_r,
    // jump_l,
}

impl BotState {
    pub fn new() -> Self{
        BotState:: idel
    }
    
}


#[derive(Component, Clone )]
pub struct StateMachine {
    pub current: BotState,
    //pub prev: BotState,
    //action: 
}

impl StateMachine {
    pub fn new(Init: BotState) -> Self {
        //this as the constructor for tree
        Self {
            current: Init,
        }
    }

    pub fn transition(&mut self, new_state: BotState){
        self.current = new_state;
    }
    
}