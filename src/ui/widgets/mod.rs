mod anchor_widget;
mod color_widget;
mod slider_widget;
use bevy::prelude::*;

pub use anchor_widget::AnchorWidget;
pub use color_widget::ColorWidget;
pub use slider_widget::SliderWidget;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        color_widget::plugin,
        anchor_widget::plugin,
        slider_widget::plugin,
    ));
}

pub fn name_node() -> Node {
    Node {
        width: Val::Percent(95.),
        height: Val::Percent(5.),
        min_height: Val::Px(10.),
        top: Val::Px(0.),
        left: Val::Percent(5.),
        ..Default::default()
    }
}
