use bevy::prelude::*;
use bevy_ascii_terminal::{AutoCamera, Border, Terminal, TerminalBundle, TerminalPlugin};
use crate::beats::data::FactsOfTheWorld;
use crate::GameState;

pub const BOARD_WIDTH: usize = 60;
pub const BOARD_HEIGHT: usize = 120;
pub const BOARD_SIZE: UVec2 = UVec2::from_array([BOARD_WIDTH as u32, BOARD_HEIGHT as u32]);


#[derive(Component)]
struct ComputerTerminal;

#[derive(Resource)]
struct ShipInfo {
    pub health: u32,
    pub started: bool
}

impl Default for ShipInfo {
    fn default() -> Self {
        ShipInfo {
            health: 100,
            started: true
        }
    }
}


pub struct ComputerViewPlugin;

impl Plugin for ComputerViewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TerminalPlugin)
            .insert_resource(FactsOfTheWorld::new())
            .insert_resource(ShipInfo::default())
            .add_systems(
                OnEnter(GameState::ComputerView),
                setup_computer_view,
            )
            .add_systems(
                OnExit(GameState::ComputerView),
                tear_down_computer_view,
            )
            .add_systems(
                Update,
                update_computer_view.run_if(in_state(GameState::ComputerView)),
            )
        ;
    }
}

fn setup_computer_view(
    mut commands: Commands,
) {

    let term = Terminal::new(BOARD_SIZE + 2).with_border(Border::double_line());

    commands
        .spawn(TerminalBundle::from(term))
        .insert(ComputerTerminal);
}


fn tear_down_computer_view() {

}

fn update_computer_view(mut q_term: Query<&mut Terminal, With<ComputerTerminal>>,
                        mut ship_info: ResMut<ShipInfo>) {
    if q_term.is_empty() {
        return;
    }

    if ship_info.started || ship_info.is_changed() {
        ship_info.started = false;
        let mut term = q_term.single_mut();

        term.clear();
        term.put_string([1, 6], "Health:");
        term.put_string([2, 5], ship_info.health.to_string());
    }

}
