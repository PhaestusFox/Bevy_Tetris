mod lightning;

use bevy::prelude::*;
pub use lightning::*;

use super::Block;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Lightning);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Effect {
    Fast,
}
