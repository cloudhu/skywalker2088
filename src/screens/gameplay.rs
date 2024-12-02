//! The screen state for the main gameplay.
use crate::assets::game_assets::Fonts;
use crate::components::states::*;
use crate::gameplay::loot::Points;
use crate::theme::interaction::OnPress;
use crate::theme::prelude::*;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameStates::GameOver), setup_game_over);
    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(AppStates::Game).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<AppStates>>) {
    next_screen.set(AppStates::MainMenu);
}

fn setup_game_over(mut commands: Commands, points: Res<Points>, fonts: Res<Fonts>) {
    commands
        .ui_root()
        .insert(StateScoped(GameStates::GameOver))
        .with_children(|children| {
            children.content(format!("{}", points.into_inner()));
            children.label("points", fonts.primary.clone());
            children
                .button("Return To Title", fonts.primary.clone())
                .observe(return_title_screen);
        });
}

fn return_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<AppStates>>) {
    next_screen.set(AppStates::MainMenu);
}
