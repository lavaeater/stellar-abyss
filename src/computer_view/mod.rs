
pub struct ComputerViewPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ComputerViewPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<Actions>().add_systems(
        //     Update,
        //     set_movement_actions.run_if(in_state(GameState::Playing)),
        // );
    }
}