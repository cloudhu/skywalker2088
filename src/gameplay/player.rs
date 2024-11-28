use crate::assets::player_assets::PlayerAssets;
use crate::components::abilities::{
    AbilitiesResource, AbilityDescriptionsResource, ActivateAbilityEvent, ChargeAbilityBundle,
    SlotOneAbilityType, SlotTwoAbilityType, StandardWeaponAbilityBundle,
};
use crate::components::character::CharactersResource;
use crate::components::input::{InputsResource, PlayerAction};
use crate::components::player::{
    InputRestrictionsAtSpawn, PlayerBundle, PlayerComponent, PlayerIDComponent, PlayerInput,
    PlayersResource,
};
use crate::options::resources::GameParametersResource;
use crate::{
    components::{health::Health, health::ShipBundle},
    gameplay::{
        gamelogic::{game_not_paused, Allegiance, PlayerLevel, Targettable, WillTarget},
        loot::{Cargo, Magnet},
        physics::{BaseGlyphRotation, Collider, Physics},
        GameState,
    },
    screens::AppStates,
    ship::engine::Engine,
    util::RenderLayer,
    AppSet, CameraShake, MainCamera,
};
use bevy::input::mouse::MouseWheel;
use bevy::window::WindowMode;
use bevy::{
    app::App,
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};
use bevy_rapier2d::dynamics::{ExternalImpulse, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::{ActiveEvents, ColliderMassProperties, Restitution};
use leafwing_input_manager::prelude::*;
use ron::de::from_bytes;
use std::f32::consts::PI;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerComponent>();
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    app.add_event::<ActivateAbilityEvent>();

    app.insert_resource(
        from_bytes::<CharactersResource>(include_bytes!("../../assets/data/characters.ron"))
            .unwrap(),
    );

    app.insert_resource(
        from_bytes::<AbilitiesResource>(include_bytes!("../../assets/data/abilities.ron")).unwrap(),
    );

    app.insert_resource(
        from_bytes::<AbilityDescriptionsResource>(include_bytes!(
            "../../assets/data/ability_descriptions.ron"
        ))
        .unwrap(),
    );

    app.insert_resource(PlayersResource::default())
        .insert_resource(InputRestrictionsAtSpawn::default());

    app.add_systems(
        Update,
        (
            pause_control,
            zoom_control,
            handle_mouse_wheel_input,
            toggle_fullscreen,
        )
            .chain()
            .in_set(AppSet::Update)
            .run_if(in_state(AppStates::InGame)),
    );
    app.add_systems(
        Update,
        (player_control, level_up_system)
            .chain()
            .in_set(AppSet::Update)
            .distributive_run_if(game_not_paused)
            .distributive_run_if(in_state(AppStates::InGame)),
    );
}

/// A command to spawn the player character.
#[derive(Debug)]
pub struct SpawnPlayer {
    /// 可配置的飞船参数.
    pub max_speed: f32,
    pub drag: f32,
    pub power: f32,
    pub steering_factor: f32,
    pub max_health: usize,
    pub max_shield: usize,
    pub radius: f32,
}

impl Default for SpawnPlayer {
    fn default() -> Self {
        SpawnPlayer::new(16.0, 5.0, 8.0, 10.0, 100, 100, 10.0)
    }
}

impl SpawnPlayer {
    pub fn new(
        max_speed: f32,
        drag: f32,
        power: f32,
        steering_factor: f32,
        max_health: usize,
        max_shield: usize,
        radius: f32,
    ) -> SpawnPlayer {
        SpawnPlayer {
            max_speed,
            drag,
            power,
            steering_factor,
            max_health,
            max_shield,
            radius,
        }
    }
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_player);
    }
}

trait PlayerAbilityChildBuilderExt {
    fn spawn_slot_1_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: &Option<SlotOneAbilityType>,
    );

    fn spawn_slot_2_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: &Option<SlotTwoAbilityType>,
    );
}

impl PlayerAbilityChildBuilderExt for ChildBuilder<'_> {
    fn spawn_slot_1_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: &Option<SlotOneAbilityType>,
    ) {
        if let Some(ability_type) = ability_type {
            match ability_type {
                SlotOneAbilityType::StandardBlast => self.spawn(StandardWeaponAbilityBundle::from(
                    &abilities_res.standard_blast_ability,
                )),
                SlotOneAbilityType::StandardBullet => self.spawn(
                    StandardWeaponAbilityBundle::from(&abilities_res.standard_bullet_ability),
                ),
            };
        }
    }

    fn spawn_slot_2_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: &Option<SlotTwoAbilityType>,
    ) {
        if let Some(ability_type) = ability_type {
            match ability_type {
                SlotTwoAbilityType::Charge => {
                    self.spawn(ChargeAbilityBundle::from(&abilities_res.charge_ability))
                }
                SlotTwoAbilityType::MegaBlast => self.spawn(StandardWeaponAbilityBundle::from(
                    &abilities_res.mega_blast_ability,
                )),
            };
        }
    }
}

// Spawn the player
fn spawn_player(
    In(config): In<SpawnPlayer>,
    mut commands: Commands,
    characters: Res<CharactersResource>,
    game_parameters: Res<GameParametersResource>,
    player_assets: Res<PlayerAssets>,
    players_resource: Res<PlayersResource>,
    inputs_res: Res<InputsResource>,
    abilities_res: Res<AbilitiesResource>,
) {
    // check if more than one player is playing
    let is_multiplayer = players_resource.player_data.get(1).is_some();
    println!("spawning player is_multiplayer:{},player_data:{}",is_multiplayer,players_resource.player_data.get(0).is_some());
    for (player_id, maybe_player_data) in players_resource
        .player_data
        .iter()
        .enumerate()
        .map(|(id, pd)| (PlayerIDComponent::from(id), pd))
    {
        if let Some(player_data) = maybe_player_data {
            // choose a character
            let character = &characters.characters[&player_data.character];

            // scale collider to align with the sprite
            let collider_size_hx =
                character.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
            let collider_size_hy =
                character.collider_dimensions.y * game_parameters.sprite_scale / 2.0;
            println!("spawning Player");
            // create player component from character
            let player_bundle = PlayerBundle::from(character).with_id(player_id);

            // spawn the player
            let mut player_entity = commands.spawn_empty();
            player_entity
                .insert(SpriteBundle {
                    texture: player_assets.get_asset(&character.character_type),
                    transform: Transform::from_translation(Vec3 {
                        x: 100.0,
                        y: 100.0,
                        z: RenderLayer::Player.as_z(),
                    }),
                    ..Default::default()
                })
                .insert(RigidBody::Dynamic)
                .insert(LockedAxes::ROTATION_LOCKED_Z)
                // .insert(Transform {
                //     translation: if is_multiplayer {
                //         Vec3::new(
                //             if matches!(player_id, PlayerIDComponent::One) {
                //                 -game_parameters.player_spawn_distance-100.0
                //             } else {
                //                 game_parameters.player_spawn_distance+100.0
                //             },
                //             100.0,
                //             if matches!(player_id, PlayerIDComponent::One) {
                //                 RenderLayer::Player.as_z()
                //             } else {
                //                 RenderLayer::Player.as_z() + 0.2
                //             },
                //         )
                //     } else {
                //         Vec3 {
                //             x: 100.0,
                //             y: 100.0,
                //             z: RenderLayer::Player.as_z(),
                //         }
                //     },
                //     scale: Vec3::new(
                //         game_parameters.sprite_scale*8.0,
                //         game_parameters.sprite_scale*8.0,
                //         1.0,
                //     ),
                //     ..Default::default()
                // })
                .insert(InputManagerBundle::<PlayerAction> {
                    action_state: ActionState::default(),
                    input_map: match player_data.input {
                        PlayerInput::Keyboard => inputs_res.player_keyboard.clone(),
                        PlayerInput::Gamepad(id) => inputs_res
                            .player_gamepad
                            .clone()
                            .set_gamepad(Gamepad { id })
                            .to_owned(),
                    },
                })
                .insert(bevy_rapier2d::geometry::Collider::cuboid(
                    collider_size_hx,
                    collider_size_hy,
                ))
                .insert(Velocity::default())
                .insert(Restitution::new(1.0))
                .insert(ColliderMassProperties::Density(character.collider_density))
                .insert(player_bundle)
                .insert(Health::from(character))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(ExternalImpulse::default())
                .insert(Name::new("Player"))
                .with_children(|parent| {
                    parent.spawn_slot_1_ability(&abilities_res, &character.slot_1_ability);
                    parent.spawn_slot_2_ability(&abilities_res, &character.slot_2_ability);
                })
                .insert(
                    ShipBundle {
                    physics: Physics::new(config.drag),
                    engine: Engine::new_with_steering(
                        config.power,
                        config.max_speed,
                        config.steering_factor,
                    ),
                    collider: Collider {
                        radius: config.radius,
                    },
                    targettable: Targettable(Allegiance::Friend),
                    will_target: WillTarget(vec![Allegiance::Enemy]),
                    ..default()
                })
                .insert(BaseGlyphRotation {
                    rotation: Quat::from_rotation_z(PI / 2.0),
                })
                .insert(Cargo::default())
                .insert(Magnet::default())
                .insert(StateScoped(AppStates::InGame));

            // add colored outline to player if multiplayer
            if is_multiplayer {
                player_entity.with_children(|parent| {
                    parent
                        .spawn(SpriteBundle {
                            texture: player_assets.get_outline_asset(&character.character_type),
                            sprite: Sprite {
                                color: if matches!(player_id, PlayerIDComponent::One) {
                                    Color::srgb(0.7, 0.0, 0.0)
                                } else {
                                    Color::srgb(0.0, 0.0, 1.0)
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Transform::from_translation(Vec3::new(
                            100.0,
                            100.0,
                            RenderLayer::Player.as_z() + 0.1,
                        )));
                });
            }
        }
    }
}

pub fn player_control(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut query: Query<(&PlayerComponent, &mut Engine), (With<PlayerComponent>, With<Engine>)>,
) {
    for (_, mut engine) in &mut query {
        if mouse_button_input.pressed(MouseButton::Left) {
            // Calculate current position to mouse position
            let (camera, camera_transform) = camera_q.single();
            let window = windows.get_single().expect("no primary window");

            engine.target = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate());
            // println!("Player controlled at {:?}", engine.target);
        } else {
            engine.target = None;
        }
    }
}

pub fn pause_control(
    key_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut change_game_state: ResMut<NextState<GameState>>,
    mut query: Query<&mut CameraShake>,
) {
    if key_input.just_pressed(KeyCode::Space) {
        match game_state.get() {
            GameState::Running => change_game_state.set(GameState::Paused),
            GameState::Paused => change_game_state.set(GameState::Running),
            _ => (),
        }
    }

    // Debug camera shake
    if key_input.just_pressed(KeyCode::KeyR) {
        for mut shake in &mut query {
            shake.trauma = 5.0;
        }
    }
}

pub fn level_up_system(
    mut level: ResMut<PlayerLevel>,
    mut query: Query<&mut Cargo, With<PlayerComponent>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for mut cargo in &mut query {
        if cargo.amount >= level.required_cargo_to_level() {
            cargo.amount -= level.required_cargo_to_level();
            level.value += 1;
            next_state.set(GameState::Selection);
        }
    }
}

pub fn zoom_control(
    key_input: Res<ButtonInput<KeyCode>>,
    mut camera_q: Query<
        &mut OrthographicProjection,
        (With<OrthographicProjection>, With<MainCamera>),
    >,
) {
    let scale_factor = 0.25;

    if key_input.just_pressed(KeyCode::PageUp) {
        if let Ok(mut projection) = camera_q.get_single_mut() {
            projection.scale = (projection.scale - scale_factor).max(1.);
        }
    }

    if key_input.just_pressed(KeyCode::PageDown) {
        if let Ok(mut projection) = camera_q.get_single_mut() {
            projection.scale = (projection.scale + scale_factor).min(3.);
        }
    }
}

/// 处理鼠标滚轮事件
fn handle_mouse_wheel_input(
    mut mouse_wheel_input: EventReader<MouseWheel>,
    mut camera_q: Query<
        &mut OrthographicProjection,
        (With<OrthographicProjection>, With<MainCamera>),
    >,
) {
    for event in mouse_wheel_input.read() {
        if let Ok(mut projection) = camera_q.get_single_mut() {
            projection.scale = (projection.scale + event.y).clamp(1., 3.);
        }
    }
}

fn toggle_fullscreen(mut window_query: Query<&mut Window>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::F11) {
        let mut window = window_query.single_mut();
        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::SizedFullscreen,
            WindowMode::BorderlessFullscreen => WindowMode::Windowed,
            WindowMode::SizedFullscreen => WindowMode::Windowed,
            WindowMode::Fullscreen => WindowMode::Windowed,
        };
    }
}
