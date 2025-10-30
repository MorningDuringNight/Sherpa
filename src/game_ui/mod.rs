use bevy::prelude::*;
pub mod ui;

use ui::*;

pub struct UIPlugin;

impl Plugin for UIPlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource( TotalCoin {amount:0,})
            .insert_resource(MaxHeight{amount:0,})
            .add_systems(Startup, loadUI)
            .add_systems(Update, updateHeight)
            .add_systems(Update, updateUI);
    }
}
