// debug_settings.rs
use bevy::{input::ButtonInput, prelude::{KeyCode, Res, ResMut, Resource}};

#[derive(Default, Resource)]
pub struct DebugSettings {
    pub enable_debug_rendering: bool,
    // Add more settings here as needed
}

pub fn toggle_debug(keyboard_input: Res<ButtonInput<KeyCode>>, mut debug_settings: ResMut<DebugSettings>) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        debug_settings.enable_debug_rendering = !debug_settings.enable_debug_rendering;
    }
}
