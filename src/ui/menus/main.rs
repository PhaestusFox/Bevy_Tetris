use crate::ui::*;

use super::{menu_boarder, menu_button_node, Menu};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(menus::Menu::Main), open_main_menu);
}

fn open_main_menu(mut commands: Commands, palette: Res<UiPalette>) {
    let play = commands.register_system(|mut state: ResMut<NextState<GameState>>| {
        state.set(GameState::Playing);
    });
    let options = commands.register_system(|mut state: ResMut<NextState<Menu>>| {
        state.set(Menu::Options);
    });
    let quit = commands.register_system(|mut state: EventWriter<AppExit>| {
        state.send(AppExit::Success);
    });

    commands
        .spawn((
            Node {
                height: Val::Percent(75.),
                min_width: Val::Percent(30.),
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceAround,
                ..Default::default()
            },
            BackgroundColor(palette.background),
            BorderRadius::all(Val::Px(10.)),
            StateScoped(Menu::Main),
        ))
        .with_children(|commands| {
            commands.spawn((
                menu_button_node(),
                menu_boarder(),
                Button,
                MenuButton {
                    cleanup: true,
                    on_click: play,
                },
                BackgroundColor(palette.button_color),
                MyText("PLAY".into()),
            ));
            commands.spawn((
                menu_button_node(),
                menu_boarder(),
                Button,
                MenuButton {
                    cleanup: true,
                    on_click: options,
                },
                BackgroundColor(palette.button_color),
                MyText("OPTIONS".into()),
            ));
            commands.spawn((
                menu_button_node(),
                menu_boarder(),
                Button,
                MenuButton {
                    cleanup: true,
                    on_click: quit,
                },
                BackgroundColor(palette.button_color),
                MyText("EXIT".into()),
            ));
        });
}
