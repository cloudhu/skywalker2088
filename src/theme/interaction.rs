use crate::assets::game_assets::AudioAssets;
use crate::config::GameConfig;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.add_systems(
        Update,
        (
            trigger_on_press,
            apply_interaction_palette,
            trigger_interaction_sound_effect,
        )
            .run_if(resource_exists::<AudioAssets>),
    );
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

/// Event triggered on a UI entity when the [`Interaction`] component on the same entity changes to
/// [`Interaction::Pressed`]. Observe this event to detect e.g. button presses.
#[derive(Event)]
pub struct OnPress;

fn trigger_on_press(
    interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (entity, interaction) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            commands.trigger_targets(OnPress, entity);
        }
    }
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

fn trigger_interaction_sound_effect(
    interaction_query: Query<&Interaction, Changed<Interaction>>,
    interaction_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    for interaction in &interaction_query {
        let source = match interaction {
            Interaction::Hovered => interaction_assets.hover.clone(),
            Interaction::Pressed => interaction_assets.press.clone(),
            _ => continue,
        };
        audio
            .play(source)
            .with_volume(Volume::Amplitude(config.sfx_volume as f64));
    }
}
