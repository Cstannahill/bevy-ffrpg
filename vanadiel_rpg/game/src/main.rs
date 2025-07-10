//! Entry point for the Vana'diel RPG.

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::render::texture::ImagePlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore};

const TILE: f32 = 32.0;
const GRID_WIDTH: i32 = 40;
const GRID_HEIGHT: i32 = 25;
const SPEED: f32 = 200.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Floor;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct DebugInfo {
    visible: bool,
}


/// Launches the game application.
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (640.0, 360.0).into(),
                    title: "Vana'diel RPG".into(),
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(DebugInfo { visible: true })
        .add_systems(Startup, (setup_floor, setup_walls, setup_player, setup_camera, setup_ui, startup_log))
        .add_systems(Update, (player_movement, camera_follow, update_debug_text, toggle_debug))
        .run();
}

fn startup_log() {
    info!("Vana'diel Awaits...");
}

fn setup_floor(mut commands: Commands) {
    let start_x = -(GRID_WIDTH as f32 * TILE) / 2.0 + TILE / 2.0;
    let start_y = -(GRID_HEIGHT as f32 * TILE) / 2.0 + TILE / 2.0;
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let color = if (x + y) % 2 == 0 {
                Color::rgb(0.2, 0.2, 0.2)
            } else {
                Color::rgb(0.25, 0.25, 0.25)
            };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(TILE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        start_x + x as f32 * TILE,
                        start_y + y as f32 * TILE,
                        0.0,
                    ),
                    ..default()
                },
                Floor,
            ));
        }
    }
}

fn setup_walls(mut commands: Commands) {
    let walls = [
        (Vec2::new(-160.0, 0.0), Vec2::new(32.0, 160.0)),
        (Vec2::new(160.0, 0.0), Vec2::new(32.0, 160.0)),
        (Vec2::new(0.0, 80.0), Vec2::new(160.0, 32.0)),
        (Vec2::new(-80.0, -80.0), Vec2::new(64.0, 32.0)),
        (Vec2::new(80.0, -80.0), Vec2::new(32.0, 64.0)),
    ];

    for (pos, size) in walls {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.4, 0.0, 0.0),
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 1.0),
                ..default()
            },
            Wall,
        ));
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::splat(16.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            ..default()
        },
        Player,
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 14.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        },
        FpsText,
    ));
}

fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    walls: Query<(&Transform, &Sprite), With<Wall>>,
) {
    let mut transform = query.single_mut();
    let mut delta = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        delta.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        delta.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        delta.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        delta.y -= 1.0;
    }

    if delta == Vec2::ZERO {
        return;
    }

    delta = delta.normalize() * SPEED * time.delta_seconds();

    let size = Vec2::splat(16.0);
    let mut new_pos = transform.translation;

    // move on x
    new_pos.x += delta.x;
    for (wall_tr, sprite) in &walls {
        if aabb_collision(
            new_pos,
            size,
            wall_tr.translation,
            sprite.custom_size.unwrap_or(Vec2::splat(TILE)),
        ) {
            if delta.x > 0.0 {
                new_pos.x = wall_tr.translation.x -
                    (sprite.custom_size.unwrap_or(Vec2::splat(TILE)).x + size.x) / 2.0;
            } else {
                new_pos.x = wall_tr.translation.x +
                    (sprite.custom_size.unwrap_or(Vec2::splat(TILE)).x + size.x) / 2.0;
            }
            break;
        }
    }

    // move on y
    new_pos.y += delta.y;
    for (wall_tr, sprite) in &walls {
        if aabb_collision(
            new_pos,
            size,
            wall_tr.translation,
            sprite.custom_size.unwrap_or(Vec2::splat(TILE)),
        ) {
            if delta.y > 0.0 {
                new_pos.y = wall_tr.translation.y -
                    (sprite.custom_size.unwrap_or(Vec2::splat(TILE)).y + size.y) / 2.0;
            } else {
                new_pos.y = wall_tr.translation.y +
                    (sprite.custom_size.unwrap_or(Vec2::splat(TILE)).y + size.y) / 2.0;
            }
            break;
        }
    }

    transform.translation = new_pos;
}

fn camera_follow(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let player_pos = player.single().translation;
    let mut cam = camera.single_mut();
    cam.translation.x = player_pos.x;
    cam.translation.y = player_pos.y;
}

fn update_debug_text(
    diagnostics: Res<DiagnosticsStore>,
    player: Query<&Transform, With<Player>>,
    mut query: Query<&mut Text, With<FpsText>>,
    info: Res<DebugInfo>,
) {
    if !info.visible {
        return;
    }
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.average())
        .unwrap_or(0.0);
    let pos = player.single().translation;
    let mut text = query.single_mut();
    text.sections[0].value = format!("FPS: {:.0}\nPos: {:.1}, {:.1}", fps, pos.x, pos.y);
}

fn toggle_debug(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut info: ResMut<DebugInfo>,
    mut vis_query: Query<&mut Visibility, With<FpsText>>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        info.visible = !info.visible;
        for mut v in &mut vis_query {
            v.visible = info.visible;
        }
    }
}

fn aabb_collision(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> bool {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;
    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    a_min.x < b_max.x
        && a_max.x > b_min.x
        && a_min.y < b_max.y
        && a_max.y > b_min.y
}
