use bevy::prelude::*;

use super::Block;

use super::Effect;

#[derive(Component)]
pub struct Lightning;

impl Plugin for Lightning {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_lightning)
            .init_resource::<LightningAssets>();
    }
}

#[derive(Resource)]
struct LightningAssets {
    image: Handle<Image>,
}

impl FromWorld for LightningAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        LightningAssets {
            image: asset_server.load("icons/bolt.png"),
        }
    }
}

fn add_lightning(
    mut commands: Commands,
    mut added: Query<(Entity, &mut Block), (With<Block>, Added<Lightning>)>,
    assets: Res<LightningAssets>,
) {
    for (entity, mut block) in &mut added {
        commands.entity(entity).with_children(|c| {
            c.spawn((
                Sprite {
                    image: assets.image.clone(),
                    color: bevy::color::palettes::css::YELLOW.into(),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::Z),
            ));
        });
        block.effects.insert(Effect::Fast);
    }
}
