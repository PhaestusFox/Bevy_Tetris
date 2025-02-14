use crate::ui::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_anchor);
}

#[derive(Component, Default)]
pub struct AnchorWidget;

#[derive(Component)]
pub struct Anchor;

pub fn spawn_anchor(mut anchors: Query<Entity, Added<AnchorWidget>>, mut commands: Commands) {
    for entity in &mut anchors {
        commands
            .spawn((
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                anchor_node(),
                Anchor,
            ))
            .set_parent(entity)
            .observe(move_question_mark);
    }
}

pub fn move_question_mark(
    event: Trigger<Pointer<Drag>>,
    parent: Query<&Parent>,
    mut root: Query<&mut Node>,
) {
    let Ok(parent) = parent.get(event.entity()) else {
        error!("{:?} has no parent", event.entity());
        return;
    };
    let Ok(mut root) = root.get_mut(parent.get()) else {
        error!("{:?} has no node", parent);
        return;
    };
    if let Val::Px(ref mut x) = root.left {
        *x += event.delta.x;
    } else {
        warn!("root Node should have left as Val::Px");
    }
    if let Val::Px(ref mut y) = root.top {
        *y += event.delta.y;
    } else {
        warn!("root Node should have top as Val::Px");
    }
}

pub fn anchor_node() -> Node {
    Node {
        position_type: PositionType::Absolute,
        height: Val::Percent(5.),
        width: Val::Percent(5.),
        min_width: Val::Px(10.),
        min_height: Val::Px(10.),
        top: Val::Px(0.),
        left: Val::Px(0.),
        ..Default::default()
    }
}
