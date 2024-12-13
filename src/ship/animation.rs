//! Exposes a component to tag animatable sprite sheets, along with the plugin with associated
//! systems to animate + clean up 2D sprite-based animations.
use crate::components::events::AnimationCompletedEvent;
use crate::gameplay::GameStates;
use crate::screens::AppStates;
use bevy::sprite::Sprite;
use bevy::{
    app::{App, Update},
    asset::Assets,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventWriter,
        schedule::IntoSystemConfigs,
        system::{Query, Res},
    },
    prelude::error,
    sprite::TextureAtlasLayout,
    state::condition::in_state,
    time::{Time, Timer},
};
use serde::Deserialize;

/// The main behavior to animate sprite sheets while the game is not paused. Without this plugin,
/// sprite animations will stay on their first frame.
pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        animate_sprite_system
            .run_if(in_state(AppStates::Game))
            .run_if(in_state(GameStates::Playing)),
    )
    .add_systems(
        Update,
        animate_sprite_system.run_if(in_state(AppStates::MainMenu)),
    );

    app.add_event::<AnimationCompletedEvent>();
}

/// Describes how to change frames of animation
#[derive(Deserialize, Clone)]
pub enum AnimationDirection {
    None,
    Forward,
    PingPong(PingPongDirection),
}

/// Current direction of a pingping animation
#[derive(Deserialize, Clone)]
pub enum PingPongDirection {
    Forward,
    Backward,
}

/// Describes an animation
// #[derive(Deserialize)]
// pub struct AnimationData {
//     pub direction: AnimationDirection,
//     pub frame_duration: f32,
// }

/// A tag on entities that need to be animated
#[derive(Component)]
pub struct AnimationComponent {
    /// Timer to track frame duration,
    pub timer: Timer,
    /// Direction of the animation
    pub direction: AnimationDirection,
}

/// Increments (or decrements) the indexes of all sprite atlases on every frame. Ticks timers
/// within the components to keep track of when animations are completed.
fn animate_sprite_system(
    time: Res<Time>,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    mut animation_complete_event_writer: EventWriter<AnimationCompletedEvent>,
    mut query: Query<(Entity, &mut AnimationComponent, &mut Sprite)>,
) {
    for (entity, mut animation, mut sprite) in query.iter_mut() {
        // tick the animation timer
        animation.timer.tick(time.delta());

        // check if frame has completed
        if animation.timer.finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                // get the texture atlas
                if let Some(texture_atlas_layout) = texture_atlas_layouts.get(atlas.layout.id()) {
                    // update animation based on the animation direction
                    match &animation.direction {
                        AnimationDirection::None => {}
                        AnimationDirection::Forward => {
                            let new_idx = (atlas.index + 1) % texture_atlas_layout.textures.len();
                            if new_idx == 0 {
                                animation_complete_event_writer
                                    .send(AnimationCompletedEvent(entity));
                            }
                            atlas.index = new_idx;
                        }
                        AnimationDirection::PingPong(direction) => match direction {
                            PingPongDirection::Forward => {
                                if atlas.index < (texture_atlas_layout.textures.len() - 1) {
                                    atlas.index += 1;
                                }

                                if atlas.index == (texture_atlas_layout.textures.len() - 1) {
                                    animation.direction =
                                        AnimationDirection::PingPong(PingPongDirection::Backward)
                                }
                            }
                            PingPongDirection::Backward => {
                                if atlas.index > 0 {
                                    atlas.index -= 1;
                                }

                                if atlas.index == 0 {
                                    animation.direction =
                                        AnimationDirection::PingPong(PingPongDirection::Forward)
                                }
                            }
                        },
                    };
                } else {
                    error!(
                        "Could not get texture atlas layout for id: {}.",
                        sprite.clone().texture_atlas.unwrap().layout.id()
                    );
                }
            }
        }
    }
}
