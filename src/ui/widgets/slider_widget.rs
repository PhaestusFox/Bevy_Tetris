use crate::ui::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (spawn_slider, update_slider));
}

#[derive(Component)]
pub struct SliderWidget {
    /// value when current = 0
    pub min: f32,
    /// value when current = 1
    pub max: f32,
    /// current value on scale 0..1
    pub current: f32,
    /// the system that is called when slider is changed takes scaled value in
    pub on_change: SystemId<In<f32>>,
}

impl SliderWidget {
    fn value(&self) -> f32 {
        let range = self.max - self.min;
        self.min + range * self.current
    }
}

#[derive(Component)]
pub struct SliderNob {
    start: f32,
}

pub fn spawn_slider(
    mut anchors: Query<(Entity, &SliderWidget), Added<SliderWidget>>,
    mut commands: Commands,
) {
    for (entity, slider) in &mut anchors {
        commands
            .spawn((
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                Node {
                    left: Val::Percent(slider.current * 95.),
                    ..slider_node()
                },
                SliderNob { start: 0. },
                ChildOf { parent: entity },
            ))
            .observe(move_question_mark)
            .observe(set_start);
    }
}

pub fn set_start(event: Trigger<Pointer<DragStart>>, mut nob: Query<(&mut SliderNob, &Node)>) {
    let Ok((mut nob, node)) = nob.get_mut(event.target) else {
        error!("{:?} is not a nob", event.target);
        return;
    };
    if let Val::Percent(start) = node.left {
        nob.start = start;
        info!("Set Start to {}", start);
    } else {
        warn!("nob{:?} left must be in Val::Percent", event.target);
    }
}

pub fn move_question_mark(
    event: Trigger<Pointer<Drag>>,
    mut nob: Query<(&ChildOf, &ComputedNode, &SliderNob, &mut Node)>,
    mut root: Query<(&mut SliderWidget, &ComputedNode)>,
) {
    let Ok((parent, comp, nob, mut node)) = nob.get_mut(event.target) else {
        error!("{:?} has no parent", event.target);
        return;
    };
    let Ok((mut slider, computed)) = root.get_mut(parent.parent) else {
        error!("{:?} has no node", parent);
        return;
    };
    if event.distance.y.abs() > comp.size().y {
        return;
    }
    let len = computed.size().x;
    let moved = event.distance.x;
    let percent = ((moved / len) * 95. + nob.start).clamp(0., 95.);
    slider.current = percent * 0.010526316; // convert range 0..95 to 0..1
    node.left = Val::Percent(percent.clamp(0., 95.));
}

pub fn update_slider(sliders: Query<&SliderWidget, Changed<SliderWidget>>, mut commands: Commands) {
    for slider in &sliders {
        let value = slider.value();
        commands.run_system_with(slider.on_change, value);
    }
}

pub fn slider_node() -> Node {
    Node {
        height: Val::Percent(100.),
        width: Val::Percent(5.),
        min_width: Val::Px(10.),
        min_height: Val::Px(10.),
        left: Val::Percent(0.),
        ..Default::default()
    }
}
