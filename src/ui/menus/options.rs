use crate::ui::*;

use super::{menu_boarder, menu_button_node, Menu};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(menus::Menu::Options), spawn_options_menu);
}

fn spawn_options_menu(mut commands: Commands, palette: Res<UiPalette>) {
    let back = commands.register_system(|mut state: ResMut<NextState<Menu>>| {
        state.set(Menu::Main);
    });
    let bind = commands.register_system(|mut state: ResMut<NextState<Menu>>| {
        state.set(Menu::KeyBinding);
    });
    let palette_id = commands.register_system(|mut state: ResMut<NextState<Menu>>| {
        state.set(Menu::UiPalette);
    });
    let text_up = commands.register_system(|mut text_size: ResMut<FontData>| {
        text_size.font_size = text_size.font_size.next();
    });
    let text_down = commands.register_system(|mut text_size: ResMut<FontData>| {
        text_size.font_size = text_size.font_size.prev();
    });

    commands
        .spawn((
            Node {
                height: Val::Percent(75.),
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceAround,
                ..Default::default()
            },
            BackgroundColor(palette.background),
            BorderRadius::all(Val::Px(10.)),
            StateScoped(Menu::Options),
        ))
        .with_children(|commands| {
            commands
                .spawn(Node {
                    height: Val::Percent(25.),
                    margin: UiRect::horizontal(Val::Auto),
                    ..Default::default()
                })
                .with_children(|commands| {
                    commands.spawn((
                        Node {
                            width: Val::Percent(15.),
                            border: UiRect::all(Val::Px(10.)).with_right(Val::Px(0.)),
                            ..menu_button_node()
                        },
                        BorderRadius::left(Val::Px(20.)),
                        Button,
                        MenuButton {
                            cleanup: true,
                            on_click: text_down,
                        },
                        BackgroundColor(palette.button_color),
                        MyText("-".into()),
                    ));
                    commands.spawn((
                        Node {
                            border: UiRect::vertical(Val::Px(10.)),
                            ..menu_button_node()
                        },
                        BackgroundColor(Color::srgb(0.33, 0.33, 0.33)),
                        MyText("FONT SIZE".into()),
                    ));
                    commands.spawn((
                        Node {
                            width: Val::Percent(15.),
                            border: UiRect::all(Val::Px(10.)).with_left(Val::Px(0.)),
                            ..menu_button_node()
                        },
                        Button,
                        MenuButton {
                            cleanup: true,
                            on_click: text_up,
                        },
                        BorderRadius::right(Val::Px(20.)),
                        BackgroundColor(palette.button_color),
                        MyText("+".into()),
                    ));
                });
            commands.spawn((
                menu_button_node(),
                Button,
                MenuButton {
                    cleanup: true,
                    on_click: bind,
                },
                menu_boarder(),
                BackgroundColor(palette.button_color),
                MyText("Key Binging".into()),
            ));
            commands.spawn((
                menu_button_node(),
                Button,
                MenuButton {
                    cleanup: true,
                    on_click: palette_id,
                },
                menu_boarder(),
                BackgroundColor(palette.button_color),
                MyText("Palette".into()),
            ));
            commands.spawn((
                menu_button_node(),
                Button,
                MenuButton {
                    cleanup: true,
                    on_click: back,
                },
                menu_boarder(),
                BackgroundColor(palette.button_color),
                MyText("Back".into()),
            ));
        });
}
