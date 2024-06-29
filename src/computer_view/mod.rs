use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::beats::data::{FactsOfTheWorld, FactUpdated, RuleUpdated, StoryBeatFinished, StoryEngine};
use crate::beats::systems::{button_system, fact_event_system, fact_update_event_broadcaster, rule_event_system, setup_stories, story_beat_effect_applier, story_evaluator};
use crate::GameState;
use crate::ui::banner_widget::BannerWidget;
use crate::ui::fps_widget;
use crate::ui::fps_widget::FpsWidget;

pub struct ComputerViewPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ComputerViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FactsOfTheWorld::new())
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

fn setup_computer_view() {

}


fn tear_down_computer_view() {

}

fn update_computer_view() {

}
