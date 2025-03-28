use super::name_node;
use super::*;
use crate::ui::menus::*;
use crate::ui::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (spawn_color_widget, update_swatch, color_wheel_buttons),
    )
    .register_required_components::<ColorWidget, AnchorWidget>()
    .register_required_components_with::<ColorWidget, Node>(color_widget_node);
}

pub struct ColorWidget {
    pub current: LinearRgba,
    pub on_submit: SystemId<In<Color>>,
}

#[derive(Component)]
struct Swatch;

impl Component for ColorWidget {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_remove(|mut world, ctx| {
            let widget = world
                .get::<ColorWidget>(ctx.entity)
                .expect("About to remove")
                .on_submit;
            world.commands().unregister_system(widget);
        });
    }
    type Mutability = bevy::ecs::component::Mutable;
}

fn spawn_color_widget(
    mut widgets: Query<
        (Entity, &ColorWidget, Option<&Name>, &mut BackgroundColor),
        Added<ColorWidget>,
    >,
    mut commands: Commands,
    palette: Res<UiPalette>,
) {
    for (root, widget, name, mut bg) in &mut widgets {
        let swatch = commands
            .spawn((
                Node {
                    width: Val::Percent(50.),
                    height: Val::Percent(50.),
                    ..Default::default()
                },
                BackgroundColor(widget.current.into()),
                Name::new("Swatch"),
            ))
            .id();
        bg.0 = palette.background;
        commands.entity(root).add_child(swatch);
        if let Some(name) = name {
            commands.spawn((
                name_node(),
                MyText(name.to_uppercase().into()),
                MyFont::Custom(10.),
                ChildOf { parent: root },
            ))
        } else {
            commands.spawn((
                name_node(),
                MyText("Color Widget".into()),
                MyFont::Custom(10.),
                ChildOf { parent: root },
            ))
        };
        let c = palette.background.to_linear();
        let set_red =
            commands.register_system(move |val: In<f32>, mut wheels: Query<&mut ColorWidget>| {
                let Ok(mut wheel) = wheels.get_mut(root) else {
                    error!("{:?} is not wheel", root);
                    return;
                };
                wheel.current.red = *val;
            });
        let set_green =
            commands.register_system(move |val: In<f32>, mut wheels: Query<&mut ColorWidget>| {
                let Ok(mut wheel) = wheels.get_mut(root) else {
                    error!("{:?} is not wheel", root);
                    return;
                };
                wheel.current.green = *val;
            });
        let set_blue =
            commands.register_system(move |val: In<f32>, mut wheels: Query<&mut ColorWidget>| {
                let Ok(mut wheel) = wheels.get_mut(root) else {
                    error!("{:?} is not wheel", root);
                    return;
                };
                wheel.current.blue = *val;
            });
        commands.entity(root).with_children(|commands| {
            commands.spawn((
                Node {
                    width: Val::Percent(80.),
                    margin: UiRect::horizontal(Val::Auto),
                    ..Default::default()
                },
                SliderWidget {
                    min: 0.,
                    max: 1.,
                    current: c.red,
                    on_change: set_red,
                },
                BackgroundColor(Color::srgb(0.5, 0., 0.)),
            ));
            commands.spawn((
                Node {
                    width: Val::Percent(80.),
                    margin: UiRect::horizontal(Val::Auto),
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0., 0.5, 0.)),
                SliderWidget {
                    min: 0.,
                    max: 1.,
                    current: c.green,
                    on_change: set_green,
                },
            ));
            commands.spawn((
                Node {
                    width: Val::Percent(80.),
                    margin: UiRect::horizontal(Val::Auto),
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0., 0., 0.5)),
                SliderWidget {
                    min: 0.,
                    max: 1.,
                    current: c.blue,
                    on_change: set_blue,
                },
            ));
            commands
                .spawn(Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(20.),
                    ..Default::default()
                })
                .with_children(|c| {
                    c.spawn((
                        menu_button_node(),
                        menu_boarder(),
                        Buttons::Submit,
                        Button,
                        MyFont::Custom(10.),
                        MyText("Submit".into()),
                    ));
                    c.spawn((
                        menu_button_node(),
                        menu_boarder(),
                        Buttons::Close,
                        Button,
                        MyFont::Custom(10.),
                        MyText("Cancel".into()),
                    ));
                });
        });
    }
}

#[derive(Component)]
enum Buttons {
    Submit,
    Close,
}

fn update_swatch(
    mut swatchs: Query<&mut BackgroundColor, With<Swatch>>,
    widgets: Query<(&ColorWidget, &Children), Changed<ColorWidget>>,
) {
    for (color, children) in &widgets {
        for child in children {
            if let Ok(mut swatch) = swatchs.get_mut(*child) {
                swatch.0 = color.current.into();
            }
        }
    }
}

fn color_widget_node() -> Node {
    Node {
        position_type: PositionType::Absolute,
        #[cfg(debug_assertions)]
        top: Val::Px(30.),
        #[cfg(not(debug_assertions))]
        top: Val::Px(0.),
        left: Val::Px(0.),
        flex_direction: FlexDirection::Column,
        width: Val::Percent(25.),
        height: Val::Percent(25.),
        ..Default::default()
    }
}

fn color_wheel_buttons(
    root: Query<&ColorWidget>,
    hierarchy: Query<&ChildOf>,
    buttons: Query<(Entity, &Buttons, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (entity, button, interaction) in &buttons {
        let Interaction::Pressed = interaction else {
            continue;
        };
        let root_entity = hierarchy.root_ancestor(entity);
        match button {
            Buttons::Submit => {
                if let Ok(root) = root.get(root_entity) {
                    commands.run_system_with(root.on_submit, root.current.into());
                }
                commands.entity(root_entity).despawn();
            }
            Buttons::Close => {
                commands.entity(root_entity).despawn();
            }
        }
    }
}
