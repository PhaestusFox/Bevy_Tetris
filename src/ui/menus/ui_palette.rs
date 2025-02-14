use super::{menu_boarder, menu_button_node, Menu};
use crate::ui::widgets::ColorWidget;
pub use crate::ui::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::UiPalette), spawn_palette_menu);
}

fn spawn_palette_menu(mut commands: Commands, palette: Res<UiPalette>) {
    let back = commands.register_system(|mut state: ResMut<NextState<Menu>>| {
        state.set(Menu::Options);
    });
    let change_bg = commands.register_system(|mut commands: Commands, ui: Res<UiPalette>| {
        let on_submit = commands.register_system(|input: In<Color>, mut ui: ResMut<UiPalette>| {
            ui.background = *input;
        });
        commands.spawn((
            ColorWidget {
                current: ui.background.to_linear(),
                on_submit,
            },
            Name::new("BG Color"),
        ));
    });
    let change_button = commands.register_system(|mut commands: Commands, ui: Res<UiPalette>| {
        let on_submit = commands.register_system(|input: In<Color>, mut ui: ResMut<UiPalette>| {
            ui.button_color = *input;
        });
        commands.spawn((
            ColorWidget {
                current: ui.button_color.to_linear(),
                on_submit,
            },
            Name::new("Button Color"),
        ));
    });
    let change_hover = commands.register_system(|mut commands: Commands, ui: Res<UiPalette>| {
        let on_submit = commands.register_system(|input: In<Color>, mut ui: ResMut<UiPalette>| {
            ui.hover_color = *input;
        });
        commands.spawn((
            ColorWidget {
                current: ui.hover_color.to_linear(),
                on_submit,
            },
            Name::new("Selected Color"),
        ));
    });
    let change_click = commands.register_system(|mut commands: Commands, ui: Res<UiPalette>| {
        let on_submit = commands.register_system(|input: In<Color>, mut ui: ResMut<UiPalette>| {
            ui.click_color = *input;
        });
        commands.spawn((
            ColorWidget {
                current: ui.click_color.to_linear(),
                on_submit,
            },
            Name::new("Click Color"),
        ));
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
            StateScoped(Menu::UiPalette),
        ))
        .with_children(|commands| {
            commands.spawn((
                menu_button_node(),
                Button,
                MenuButton {
                    cleanup: true,
                    on_click: change_bg,
                },
                menu_boarder(),
                MyText("Background".into()),
            ));
            commands.spawn((
                menu_button_node(),
                Button,
                MenuButton {
                    cleanup: true,
                    on_click: change_button,
                },
                menu_boarder(),
                MyText("Button".into()),
            ));
            commands.spawn((
                menu_button_node(),
                Button,
                MenuButton {
                    cleanup: true,
                    on_click: change_hover,
                },
                menu_boarder(),
                MyText("Selected".into()),
            ));
            commands.spawn((
                menu_button_node(),
                Button,
                MenuButton {
                    cleanup: true,
                    on_click: change_click,
                },
                menu_boarder(),
                MyText("Click".into()),
            ));
            commands.spawn((
                menu_button_node(),
                Button,
                MenuButton {
                    cleanup: true,
                    on_click: back,
                },
                menu_boarder(),
                BackgroundColor(Color::srgb(0.33, 0.33, 0.33)),
                MyText("Back".into()),
            ));
        });
}
