use bevy::prelude::*;
use bevy_ascii_terminal::{Border, Terminal, TerminalBundle};
use crate::beats::data::FactsOfTheWorld;
use crate::GameState;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
pub const BOARD_SIZE: UVec2 = UVec2::from_array([BOARD_WIDTH as u32, BOARD_HEIGHT as u32]);


#[derive(Component)]
struct ComputerTerminal;

#[derive(Resource, Default)]
struct ShipInfo {
    pub health: u32
}


pub struct ComputerViewPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ComputerViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FactsOfTheWorld::new())
            .insert_resource(ShipInfo::default())
            .add_systems(
                OnEnter(GameState::ComputerView),
                (setup_computer_view),
            )
            .add_systems(
                OnExit(GameState::ComputerView),
                (tear_down_computer_view),
            )
            .add_systems(
                Update,
                (
                    update_computer_view
                ).run_if(in_state(GameState::ComputerView)),
            )
        ;
    }
}

fn setup_computer_view(
    mut commands: Commands,
) {

    let term = Terminal::new(BOARD_SIZE + 2);

    commands
        .spawn(TerminalBundle::from(term))
        .insert(ComputerTerminal);
}


fn tear_down_computer_view() {

}

fn update_computer_view(mut q_term: Query<&mut Terminal, With<ComputerTerminal>>,
                        ship_info: Res<ShipInfo>) {
    if q_term.is_empty() {
        return;
    }

    if ship_info.is_changed() {
        let mut term = q_term.single_mut();

        term.clear();
        let glyphs = Border::double_line();
        term.set_border(glyphs);
        term.put_string([1, 6], "Health:");
        term.put_string([2, 5], ship_info.health.to_string());
    }

}
