use bevy::{
    input::{keyboard::KeyboardInput, mouse::AccumulatedMouseScroll},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_editor_pls::EditorPlugin::default(),
            leafwing_input_manager::prelude::InputManagerPlugin::<deck::PlayerInputs>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, scroll_camera)
        .init_resource::<board::Board>()
        .add_plugins((board::plugin, deck::plugin))
        .insert_resource(Time::<Fixed>::from_hz(3.))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Transform::from_translation(Vec3::new(320., 0., 0.)),
    ));

    let block_image = asset_server.load("block.png");
    // setup the map
    for x in 0..12 {
        commands.spawn((
            Sprite {
                image: block_image.clone(),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(x as f32 * 64.0 - 64., -64., 0.0)),
        ));
        commands.spawn((
            Sprite {
                image: block_image.clone(),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(x as f32 * 64.0 - 64., 1280.0, 0.0)),
        ));
    }
    for y in 0..20 {
        commands.spawn((
            Sprite {
                image: block_image.clone(),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(640., 0. + 64. * y as f32, 0.0)),
        ));
        commands.spawn((
            Sprite {
                image: block_image.clone(),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(-64., 0. + 64. * y as f32, 0.0)),
        ));
    }
}

fn scroll_camera(
    scroll: Res<AccumulatedMouseScroll>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    for mut camera_transform in cameras.iter_mut() {
        camera_transform.translation.y += scroll.delta.y * 25.;
        camera_transform.translation.y = camera_transform.translation.y.clamp(0.0, 1000.0);
    }
}

mod board;
mod deck;
