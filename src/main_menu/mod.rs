mod components;
mod styles;
mod systems;

use systems::layout::*;

use bevy::prelude::*;

use crate::AppState;

use self::systems::interactions::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter State Systems
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            // Systems
            .add_systems(
                (interact_with_play_button, interact_with_quit_button)
                    .in_set(OnUpdate(AppState::MainMenu)),
            )
            // On Exit State Systems
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
