use crate::ship::engine::Engine;
use crate::util::Math;
use crate::{CameraShake, MainCamera};
use bevy::prelude::*;
use bevy_parallax::ParallaxMoveEvent;
use crate::gameplay::player::PlayerComponent;

pub fn camera_follow(
    time: Res<Time>,
    player_q: Query<&Transform, (With<Engine>, With<PlayerComponent>)>,
    mut camera_q: Query<
        (Entity, &Transform, &mut CameraShake),
        (With<Transform>, With<MainCamera>),
    >,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    if let Ok((camera_entity, camera_transform, mut shake)) = camera_q.get_single_mut() {
        // info!("camera transform: {:?}", camera_transform);
        if let Ok(player_transform) = player_q.get_single() {
            // info!("player transform: {:?}", player_transform);
            // Calculate the new camera position based on the player's position
            let target_position = Vec2::new(
                player_transform.translation.x + 1.0,
                player_transform.translation.y,
            );

            let current_position = camera_transform.translation.truncate();

            let smooth_move_position = current_position
                .lerp(target_position, 5.0 * time.delta_secs())
                + shake.trauma * Math::random_2d_unit_vector();

            shake.trauma = f32::max(shake.trauma - shake.decay * time.delta_secs(), 0.0);

            move_event_writer.send(ParallaxMoveEvent {
                translation: smooth_move_position - current_position,
                camera: camera_entity,
                rotation: camera_transform.rotation.z,
            });
        }
    }
}
