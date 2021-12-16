use bevy::{
    prelude::*,
    render::pass::ClearColor,
};

/// An implementation of the classic game "Breakout"
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system(paddle_movement_system.system())
        .run();
}

struct Ball {
    speed: f32,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    //asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // paddle
    commands
    .spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
        transform: Transform::from_xyz(0.0, -50.0, 1.0),
        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
        ..Default::default()
    })
        .insert(Ball { speed: 500.0 });
}

fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Ball, &mut Transform)>,
) {
    if let Ok((paddle, mut transform)) = query.single_mut() {
        let mut direction_x = 0.0;
        let mut direction_y = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction_x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction_x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction_y -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction_y += 1.0;
        }

        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.x += time.delta_seconds() * direction_x * paddle.speed;
        translation.y += time.delta_seconds() * direction_y * paddle.speed;
        // bound the paddle within the walls
        //translation.x = translation.x.min(380.0).max(-380.0);
    }
}