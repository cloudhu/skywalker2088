use crate::gameplay::gamelogic::game_not_paused;
use crate::screens::AppState;
use crate::util::Colour;
use crate::AppSet;
use bevy::prelude::*;

#[derive(Component)]
pub struct FloatingText {
    pub ttl: Timer,
    pub rise_distance: f32,
}

impl Default for FloatingText {
    fn default() -> Self {
        Self {
            ttl: Timer::from_seconds(1.0, TimerMode::Once),
            rise_distance: 10.0,
        }
    }
}

#[derive(Component)]
pub struct HitFlash {
    pub timer: Timer,
    pub flash_colour: Color,
    pub original_colour: Option<Color>,
}

impl HitFlash {
    pub fn hit(&mut self) {
        self.timer.reset();
        self.timer.unpause();
    }
}

impl Default for HitFlash {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(0.1, TimerMode::Once);
        timer.pause();
        Self {
            timer,
            flash_colour: Colour::RED,
            original_colour: None,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (floating_text_system, hit_flash_system)
            .chain()
            .in_set(AppSet::Update)
            .distributive_run_if(game_not_paused)
            .distributive_run_if(in_state(AppState::InGame)),
    );
}

pub fn floating_text_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut FloatingText, &mut Text)>,
) {
    for (entity, mut transform, mut floating_text, mut text) in &mut query {
        floating_text.ttl.tick(time.delta());

        transform.translation.y += time.delta().as_secs_f32()
            / floating_text.ttl.duration().as_secs_f32()
            * floating_text.rise_distance;
        text.sections.iter_mut().for_each(|section| {
            section
                .style
                .color
                .set_alpha(floating_text.ttl.fraction_remaining());
        });

        if floating_text.ttl.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn hit_flash_system(time: Res<Time>, mut query: Query<(&mut Text, &mut HitFlash)>) {
    for (mut text, mut hit_flash) in &mut query {
        // First time
        if !hit_flash.timer.paused() && hit_flash.timer.elapsed().is_zero() {
            // Store the actual colour once
            if hit_flash.original_colour.is_none() {
                hit_flash.original_colour = text
                    .sections
                    .first()
                    .and_then(|section| Some(section.style.color));
            }
            // Set to flash colour
            text.sections
                .iter_mut()
                .for_each(|section| section.style.color = hit_flash.flash_colour);
        }

        hit_flash.timer.tick(time.delta());

        // End
        if hit_flash.timer.just_finished() {
            // Reset to original colour
            text.sections.iter_mut().for_each(|section| {
                section.style.color = hit_flash.original_colour.unwrap_or(Colour::PURPLE)
            });
            // Stop the timer
            hit_flash.timer.pause();
        }
    }
}
