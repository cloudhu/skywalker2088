//! A credits screen that can be accessed from the title screen.
use bevy::prelude::*;

use crate::assets::game_assets::Fonts;
use crate::components::states::AppStates;
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppStates::Credits), spawn_credits_screen);
}

fn spawn_credits_screen(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .ui_root()
        .insert(StateScoped(AppStates::Credits))
        .with_children(|children| {
            children.header("Made by", fonts.primary.clone());
            children.label("Developer", fonts.primary.clone());

            children.header("Assets", fonts.primary.clone());
            children.label("logo", fonts.primary.clone());

            children
                .button("Back", fonts.primary.clone())
                .observe(enter_title_screen);
        });
}

fn enter_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<AppStates>>) {
    next_screen.set(AppStates::MainMenu);
}
