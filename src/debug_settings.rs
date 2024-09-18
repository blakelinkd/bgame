// debug_settings.rs

use bevy::prelude::*;
use bevy::input::ButtonInput;

#[derive(Resource)]
pub struct DebugSettings {
    pub debug_colliders: bool,
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self {
            debug_colliders: false,
        }
    }
}

// Toggle function for DebugSettings
pub fn toggle_debug(mut debug_settings: ResMut<DebugSettings>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        debug_settings.debug_colliders = !debug_settings.debug_colliders;
        println!("Debug colliders: {}", debug_settings.debug_colliders);
    }
}
