//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::{audio::SoundEffect, demo::enemy::EnemyAssets, AppSet};

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.register_type::<EnemyAnimation>();
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSet::TickTimers),
            (
                update_animation_movement,
                update_animation_atlas,
                trigger_step_sound_effect,
            )
                .chain()
                .run_if(resource_exists::<EnemyAssets>)
                .in_set(AppSet::Update),
        ),
    );
}

/// Update the sprite direction and animation state (idling/walking).
fn update_animation_movement(mut enemy_query: Query<(&mut Sprite, &mut EnemyAnimation)>) {
    for (mut sprite, mut animation) in &mut enemy_query {
        // let dx = controller.intent.x;
        // if dx != 0.0 {
        //     sprite.flip_x = dx < 0.0;
        // }
        //
        // let animation_state = if controller.intent == Vec2::ZERO {
        //     EnemyAnimationState::Idling
        // } else {
        //     EnemyAnimationState::Walking
        // };
        // animation.update_state(animation_state);
    }
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut EnemyAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&EnemyAnimation, &mut TextureAtlas)>) {
    for (animation, mut atlas) in &mut query {
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

/// If the player is moving, play a step sound effect synchronized with the
/// animation.
fn trigger_step_sound_effect(
    mut commands: Commands,
    player_assets: Res<EnemyAssets>,
    mut step_query: Query<&EnemyAnimation>,
) {
    for animation in &mut step_query {
        if animation.state == EnemyAnimationState::Walking
            && animation.changed()
            && (animation.frame == 2 || animation.frame == 5)
        {
            let rng = &mut rand::thread_rng();
            let random_step = player_assets.steps.choose(rng).unwrap();
            commands.spawn((
                AudioBundle {
                    source: random_step.clone(),
                    settings: PlaybackSettings::DESPAWN,
                },
                SoundEffect,
            ));
        }
    }
}

/// Component that tracks player's animation state.
/// It is tightly bound to the texture atlas we use.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EnemyAnimation {
    timer: Timer,
    frame: usize,
    state: EnemyAnimationState,
}

#[derive(Reflect, PartialEq)]
pub enum EnemyAnimationState {
    Idling,
    Walking,
}

impl EnemyAnimation {
    /// The number of idle frames.
    const IDLE_FRAMES: usize = 2;
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(500);
    /// The number of walking frames.
    const WALKING_FRAMES: usize = 6;
    /// The duration of each walking frame.
    const WALKING_INTERVAL: Duration = Duration::from_millis(50);

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: EnemyAnimationState::Idling,
        }
    }

    fn walking() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: EnemyAnimationState::Walking,
        }
    }

    pub fn new() -> Self {
        Self::idling()
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                EnemyAnimationState::Idling => Self::IDLE_FRAMES,
                EnemyAnimationState::Walking => Self::WALKING_FRAMES,
            };
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: EnemyAnimationState) {
        if self.state != state {
            match state {
                EnemyAnimationState::Idling => *self = Self::idling(),
                EnemyAnimationState::Walking => *self = Self::walking(),
            }
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.finished()
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        match self.state {
            EnemyAnimationState::Idling => self.frame,
            EnemyAnimationState::Walking => 6 + self.frame,
        }
    }
}
