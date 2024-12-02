use crate::components::states::AppStates;
use crate::gameplay::gamelogic::game_not_paused;
use crate::AppSet;
use bevy::prelude::*;

#[derive(Component)]
pub struct BaseRotation {
    pub rotation: Quat,
}

#[derive(Component)]
pub struct Rotator {
    pub speed: f32,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        rotator_system
            .chain()
            .in_set(AppSet::Update)
            .distributive_run_if(game_not_paused)
            .distributive_run_if(in_state(AppStates::Game)),
    );
}

pub fn rotator_system(time: Res<Time>, mut query: Query<(&mut Transform, &Rotator)>) {
    for (mut transform, rotator) in &mut query {
        transform.rotate(Quat::from_rotation_z(rotator.speed * time.delta_seconds()));
    }
}
