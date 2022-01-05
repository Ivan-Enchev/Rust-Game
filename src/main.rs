use bevy::prelude::*;
use bevy_retrograde::prelude::*;

fn main() {
    App::build()
        .add_plugins(RetroPlugins)
        .add_startup_system(setup.system())
    	.add_system(move_player.system())
        .run();
}
struct Player;


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(CameraBundle {
        camera: Camera {
            size: CameraSize::FixedHeight(200),
            background_color: Color::new(1.0, 1.0, 1.0, 1.0),
            ..Default::default()
        },
        transform: Transform::from_xyz(0., -50., 0.),
        ..Default::default()
    });

    let player = asset_server.load("player.png");
    let block = asset_server.load("block.png");

    let mut n = -100.0;

    while n < 100.0 {
        commands
        // First we spawn a sprite bundle like normal
        .spawn_bundle(SpriteBundle {
            image: block.clone(),
            transform: Transform::from_xyz(n, 20.0, 0.),
            ..Default::default()
        })
        // Then we add a tesselated collider component. This will create a convex collision shape
        // from the provided image automatically.
        .insert(TesselatedCollider {
            // We want to use the same block we use for the visual for the collider shape
            image: block.clone(),
            ..Default::default()
        })
        // Make it a static body
        .insert(RigidBody::Static);
        n += 20.0;
    }

    commands
        .spawn_bundle(SpriteBundle {
            image: player.clone(),
            sprite: Sprite {
                pixel_perfect: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., -50., 0.),
            ..Default::default()
        })
        .insert(TesselatedCollider {
            image: player.clone(),
            tesselator_config: TesselatedColliderConfig {
                // We want the collision shape for the player to be highly accurate
                vertice_separation: 0.,
                ..Default::default()
            },
            ..Default::default()
        })
        // The player is also a dynamic body with rotations locked
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        // Disable friction and bounciness
        .insert(PhysicMaterial {
            friction: 0.,
            restitution: 0.,
            ..Default::default()
        })
        // Set the player speed to 0 initially
        .insert(Velocity::from_linear(Vec3::default()))
        .insert(Player);
    
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        const SPEED: f32 = 100.;

        let mut multipleCheck = -1;

        let mut direction = Vec3::new(0., 0., 0.);

        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0., 0.);
            multipleCheck += 1;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0., 0.);
            multipleCheck += 1;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0., -1.0, 0.);
            multipleCheck += 1;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0., 1.0, 0.);
            multipleCheck += 1;
        }
        if multipleCheck >= 1 {
            direction /= 2.0;
        }

        multipleCheck = -1;

        *velocity = Velocity::from_linear(direction * SPEED);
    }
}

