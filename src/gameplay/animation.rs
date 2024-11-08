use crate::gameplay::gamelogic::game_not_paused;
use crate::screens::AppState;
use crate::AppSet;
use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub(crate) first: usize,
    pub(crate) last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (animate_sprite,)
            .chain()
            .in_set(AppSet::Update)
            .distributive_run_if(game_not_paused)
            .distributive_run_if(in_state(AppState::Gameplay)),
    );
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
