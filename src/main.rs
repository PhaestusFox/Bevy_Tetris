use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};

use prelude::*;

mod blocks;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        leafwing_input_manager::prelude::InputManagerPlugin::<deck::PlayerInputs>::default(),
    ))
    .insert_resource(bevy_pkv::PkvStore::new("Phox", "Tetris"))
    .add_systems(OnEnter(GameState::Playing), make_board)
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, scroll_camera)
    .init_resource::<board::Board>()
    .add_plugins((board::plugin, deck::plugin, ui::plugin, blocks::plugin))
    .insert_resource(Time::<Fixed>::from_hz(3.))
    .insert_resource(Score(0))
    .init_state::<GameState>();
    #[cfg(debug_assertions)]
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_translation(Vec3::new(320., 0., 0.)),
        IsDefaultUiCamera,
    ));
}

fn make_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let block_image = asset_server.load("block.png");
    commands
        .spawn((
            Name::new("Board"),
            Transform::IDENTITY,
            Visibility::Visible,
            StateScoped(GameState::Playing),
        ))
        .with_children(|commands| {
            // setup the map
            for x in 0..12 {
                commands.spawn((
                    Sprite {
                        image: block_image.clone(),
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(x as f32 * 32.0 - 32., -32., 0.0)),
                ));
                commands.spawn((
                    Sprite {
                        image: block_image.clone(),
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(x as f32 * 32.0 - 32., 640.0, 0.0)),
                ));
            }
            for y in 0..20 {
                commands.spawn((
                    Sprite {
                        image: block_image.clone(),
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(320., 32. * y as f32, 0.0)),
                ));
                commands.spawn((
                    Sprite {
                        image: block_image.clone(),
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(-32., 32. * y as f32, 0.0)),
                ));
            }
        });
}

fn scroll_camera(
    scroll: Res<AccumulatedMouseScroll>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    for mut camera_transform in cameras.iter_mut() {
        camera_transform.translation.y += scroll.delta.y * 25.;
        camera_transform.translation.y = camera_transform.translation.y.clamp(0.0, 500.0);
    }
}

mod board;
mod deck;
mod ui;

pub mod prelude {
    use bevy::prelude::*;

    pub(crate) use super::GameState;

    #[derive(Resource, Deref, DerefMut)]
    pub struct Score(pub i32);

    #[derive(strum_macros::AsRefStr)]
    pub enum DataKeys {
        UiPalette,
        FontSize,
    }
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    InMenu,
    Playing,
}
