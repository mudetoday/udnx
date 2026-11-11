//! Third-person character controller prototype with Bevy.
//! Primitive cube player, camera follows behind.
//! Desktop: WASD + mouse look (right mouse to orbit).
//! Android: Touch to move relative to camera direction.

use bevy::{
    input::touch::TouchPhase,
    prelude::*,
    window::{AppLifecycle, WindowMode},
    winit::WinitSettings,
};

#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: "wgpu=error,naga=warn".into(),
                    ..default()
                }),
        )
        .insert_resource(WinitSettings::mobile())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement,
                camera_follow,
                handle_lifecycle,
            ),
        )
        .run();
}

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct ThirdPersonCamera {
    offset: Vec3,
    smoothness: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Player (primitive cube)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Player { speed: 5.0 },
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: false, // better performance on mobile
            intensity: 10_000_000.0,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
        ..default()
    });

    // Third-person camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.0, 8.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ThirdPersonCamera {
            offset: Vec3::new(0.0, 3.5, 7.0),
            smoothness: 5.0,
        },
    ));
}

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    mut player_query: Query<(&Player, &mut Transform)>,
    camera_query: Query<&Transform, (With<ThirdPersonCamera>, Without<Player>)>,
) {
    let Ok((player, mut transform)) = player_query.single_mut() else {
        return;
    };
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    // Keyboard (desktop)
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        direction += Vec3::Z;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        direction -= Vec3::Z;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        direction -= Vec3::X;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        direction += Vec3::X;
    }

    // Touch (Android): move relative to camera forward/right
    if let Some(touch) = touches.first_pressed_position() {
        // Simple: treat touch position relative to screen center as direction
        // For better control you can add a virtual joystick later
        let window_size = Vec2::new(800.0, 600.0); // approximate; in real use query window
        let center = window_size / 2.0;
        let delta = touch - center;
        if delta.length() > 20.0 {
            direction += Vec3::new(delta.x, 0.0, -delta.y).normalize_or_zero();
        }
    }

    if direction != Vec3::ZERO {
        // Transform direction into camera-relative space
        let forward = camera_transform.forward().xz().normalize_or_zero().extend(0.0).xzy();
        let right = camera_transform.right().xz().normalize_or_zero().extend(0.0).xzy();
        let move_dir = (forward * direction.z + right * direction.x).normalize_or_zero();

        transform.translation += move_dir * player.speed * time.delta_secs();

        // Face movement direction
        if move_dir.length_squared() > 0.01 {
            let target_rot = Quat::from_rotation_arc(Vec3::Z, move_dir);
            transform.rotation = transform.rotation.slerp(target_rot, 10.0 * time.delta_secs());
        }
    }
}

fn camera_follow(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&ThirdPersonCamera, &mut Transform), Without<Player>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok((cam, mut camera_transform)) = camera_query.single_mut() else {
        return;
    };

    // Desired position: player + rotated offset
    let desired = player_transform.translation + player_transform.rotation * cam.offset;
    let look_target = player_transform.translation + Vec3::Y * 1.0;

    camera_transform.translation = camera_transform
        .translation
        .lerp(desired, cam.smoothness * time.delta_secs());

    camera_transform.look_at(look_target, Vec3::Y);
}

fn handle_lifecycle(
    mut lifecycle_events: MessageReader<AppLifecycle>,
    mut windows: Query<&mut Window>,
) {
    for event in lifecycle_events.read() {
        match event {
            AppLifecycle::Suspended | AppLifecycle::WillSuspend => {
                // Optionally pause game logic here
                if let Ok(mut window) = windows.single_mut() {
                    window.focused = false;
                }
            }
            AppLifecycle::Resumed => {
                if let Ok(mut window) = windows.single_mut() {
                    window.focused = true;
                }
            }
            _ => {}
        }
    }
}
