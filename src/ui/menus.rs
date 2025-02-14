use crate::prelude::*;
use bevy::prelude::*;
mod main;
mod options;
mod ui_palette;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Component)]
pub enum Menu {
    #[default]
    Main,
    Options,
    KeyBinding,
    UiPalette,
    Pause,
    None,
}

pub fn plugin(app: &mut App) {
    app.enable_state_scoped_entities::<Menu>()
        .init_state::<Menu>()
        .add_systems(OnExit(GameState::InMenu), set_none)
        .add_systems(OnEnter(GameState::InMenu), pause_time)
        .add_plugins((main::plugin, options::plugin, ui_palette::plugin));
}

pub fn set_none(mut next: ResMut<NextState<Menu>>, mut time: ResMut<Time<Virtual>>) {
    next.set(Menu::None);
    time.unpause();
}

pub fn pause_time(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

pub fn menu_button_node() -> Node {
    Node {
        margin: UiRect::horizontal(Val::Auto),
        padding: UiRect::all(Val::Px(10.)),
        width: Val::Percent(90.),
        justify_content: JustifyContent::Center,
        border: UiRect::all(Val::Px(10.)),
        flex_grow: 1.,
        ..Default::default()
    }
}
pub fn menu_boarder() -> BorderRadius {
    BorderRadius::all(Val::Px(20.))
}
