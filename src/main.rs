use bevy::{input::system::exit_on_esc_system, prelude::*};

const GRID_SIZE: f32 = 64.0;

#[derive(Component)]
struct GridMarker;

#[derive(Component)]
struct Tower;

#[derive(Component)]
struct Enemy;

fn startup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(GRID_SIZE, GRID_SIZE, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.47, 0.87, 0.47),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GridMarker);
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(GRID_SIZE / 2.0, GRID_SIZE / 2.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.22, 0.22),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy);
}

fn grid_marker_mouse_system(
    windows: Res<Windows>,
    mut query: Query<(&GridMarker, &mut Transform)>,
) {
    let win = windows.get_primary().expect("no primary window");
    let size = Vec2::new(win.width() as f32, win.height() as f32);
    let default_orthographic_pos = size / 2.0;
    let mouse_pos = win.cursor_position();
    match mouse_pos {
        None => {}
        Some(vec2) => {
            let result = query.get_single_mut();
            match result {
                Result::Err(..) => {}
                Result::Ok((_grid_marker, mut transform)) => {
                    let world_pos = vec2 - default_orthographic_pos;
                    let grid_x = (world_pos.x / GRID_SIZE).floor() * GRID_SIZE;
                    let grid_y = (world_pos.y / GRID_SIZE).floor() * GRID_SIZE;
                    transform.translation.x = grid_x;
                    transform.translation.y = grid_y;
                }
            }
        }
    }
}

fn build_mouse_system(
    mouse_system: Res<Input<MouseButton>>,
    query: Query<(Entity, &Transform, &GridMarker)>,
    mut commands: Commands,
) {
    if mouse_system.just_pressed(MouseButton::Right) {
        let result = query.get_single();
        match result {
            Result::Err(..) => {}
            Result::Ok((_entity, transform, _grid_marker)) => {
                commands
                    .spawn_bundle(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(
                                transform.translation.x,
                                transform.translation.y,
                                0.0,
                            ),
                            scale: Vec3::new(GRID_SIZE, GRID_SIZE, 0.0),
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: Color::rgb(0.95, 0.95, 0.95),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Tower);
            }
        }
    }
}

fn tower_attack_system() {
    info!("tower_attack_system");   
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_system(grid_marker_mouse_system)
        .add_system(build_mouse_system)
        .add_system(tower_attack_system)
        .add_system(exit_on_esc_system)
        .run();
}
