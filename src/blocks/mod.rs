mod powers;
use bevy::{platform_support::collections::HashSet, prelude::*};
pub use powers::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(powers::plugin);
}

#[derive(Clone, Component)]
pub struct Block {
    pub shape: Entity,
    pub moved: bool,
    pub effects: HashSet<Effect>,
}
