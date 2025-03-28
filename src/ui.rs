use std::borrow::Cow;

use crate::prelude::*;
use bevy::{ecs::system::SystemId, prelude::*};
use bevy_pkv::PkvStore;

pub fn plugin(app: &mut App) {
    app.init_resource::<FontData>()
        .init_resource::<UiPalette>()
        .add_plugins((menus::plugin, widgets::plugin))
        .add_systems(
            Update,
            (
                (update_font_size, save_font_size).run_if(resource_changed::<FontData>),
                save_palette.run_if(resource_changed::<UiPalette>),
            ),
        )
        .add_systems(
            Update,
            (fill_text, ui_hover, run_button_clicks, update_palette),
        )
        .add_systems(PostUpdate, set_font_size)
        .add_systems(OnEnter(GameState::Playing), spawn_score)
        .add_systems(Update, update_score.run_if(resource_changed::<Score>))
        .register_type::<MyText>()
        .register_type::<MyFont>();
}

#[derive(Resource)]
struct FontData {
    image: Handle<Image>,
    atlas: Handle<TextureAtlasLayout>,
    font_size: FontSize,
}

impl FromWorld for FontData {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let store = world.resource::<bevy_pkv::PkvStore>();
        let font_size = if let Ok(data) = store.get::<FontSize>(DataKeys::FontSize) {
            data
        } else {
            FontSize::Normal
        };
        FontData {
            image: asset_server.load("font.png"),
            atlas: asset_server.add(FontIndex::layout()),
            font_size,
        }
    }
}

#[derive(
    Clone,
    Copy,
    serde::Deserialize,
    serde::Serialize,
    strum_macros::FromRepr,
    strum_macros::EnumCount,
)]
enum FontSize {
    Small,
    Normal,
    Large,
    XL,
}

impl From<FontSize> for f32 {
    fn from(value: FontSize) -> Self {
        match value {
            FontSize::Small => 18.,
            FontSize::Normal => 30.,
            FontSize::Large => 45.,
            FontSize::XL => 60.,
        }
    }
}

impl FontSize {
    fn next(self) -> FontSize {
        FontSize::from_repr(self as usize + 1).unwrap_or(self)
    }

    fn prev(self) -> FontSize {
        let next = self as usize;
        if next == 0 {
            self
        } else {
            FontSize::from_repr(self as usize - 1).unwrap_or(self)
        }
    }
}

#[derive(strum_macros::FromRepr)]
enum FontIndex {
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Space = 36,
    Plus,
    Sub,
    Dot,
    SemiColin = 45,
    LowerA,
    LowerB,
    LowerC,
    LowerD,
    LowerE,
    LowerF,
    LowerG,
    LowerH,
    LowerI,
    LowerJ,
    LowerK,
    LowerL,
    LowerM,
    LowerN,
    LowerO,
    LowerP,
    LowerQ,
    LowerR,
    LowerS,
    LowerT,
    LowerU,
    LowerV,
    LowerW,
    LowerX,
    LowerY,
    LowerZ,
    Comma = 72,
    Empty = 100,
}

impl FontIndex {
    fn layout() -> TextureAtlasLayout {
        let mut textures = Vec::new();
        let mut start = UVec2::new(0, 0);
        for y in 0..10 {
            for x in 0..10 {
                let Some(letter) = FontIndex::from_repr(y * 10 + x) else {
                    textures.push(URect::new(start.x, start.y, start.x + 18, start.y + 18));
                    continue;
                };
                textures.push(URect::new(
                    start.x,
                    start.y,
                    start.x + letter.kerning(),
                    start.y + 18,
                ));
                start.x += 18;
            }
            start.x = 0;
            start.y += 18;
        }

        TextureAtlasLayout {
            size: UVec2::new(180, 180),
            textures,
        }
    }
    fn kerning(&self) -> u32 {
        match self {
            FontIndex::Digit0 => 14,
            FontIndex::Digit1 => 10,
            FontIndex::Digit2 => 13,
            FontIndex::Digit3 => 13,
            FontIndex::Digit4 => 13,
            FontIndex::Digit5 => 13,
            FontIndex::Digit6 => 13,
            FontIndex::Digit7 => 13,
            FontIndex::Digit8 => 16,
            FontIndex::Digit9 => 14,
            FontIndex::Space => 10,
            FontIndex::I => 10,
            FontIndex::Y => 16,
            FontIndex::N => 16,
            FontIndex::M => 16,
            FontIndex::W => 16,
            FontIndex::T => 16,
            FontIndex::O => 17,
            FontIndex::D => 15,
            FontIndex::E => 13,
            FontIndex::L => 13,
            FontIndex::X => 17,
            _ => 18,
        }
    }
}

impl From<char> for FontIndex {
    fn from(value: char) -> Self {
        match value {
            '0' => FontIndex::Digit0,
            '1' => FontIndex::Digit1,
            '2' => FontIndex::Digit2,
            '3' => FontIndex::Digit3,
            '4' => FontIndex::Digit4,
            '5' => FontIndex::Digit5,
            '6' => FontIndex::Digit6,
            '7' => FontIndex::Digit7,
            '8' => FontIndex::Digit8,
            '9' => FontIndex::Digit9,
            'A' => FontIndex::A,
            'B' => FontIndex::B,
            'C' => FontIndex::C,
            'D' => FontIndex::D,
            'E' => FontIndex::E,
            'F' => FontIndex::F,
            'G' => FontIndex::G,
            'H' => FontIndex::H,
            'I' => FontIndex::I,
            'J' => FontIndex::J,
            'K' => FontIndex::K,
            'L' => FontIndex::L,
            'M' => FontIndex::M,
            'N' => FontIndex::N,
            'O' => FontIndex::O,
            'P' => FontIndex::P,
            'Q' => FontIndex::Q,
            'R' => FontIndex::R,
            'S' => FontIndex::S,
            'T' => FontIndex::T,
            'U' => FontIndex::U,
            'V' => FontIndex::V,
            'W' => FontIndex::W,
            'X' => FontIndex::X,
            'Y' => FontIndex::Y,
            'Z' => FontIndex::Z,
            'a' => FontIndex::LowerA,
            'b' => FontIndex::LowerB,
            'c' => FontIndex::LowerC,
            'd' => FontIndex::LowerD,
            'e' => FontIndex::LowerE,
            'f' => FontIndex::LowerF,
            'g' => FontIndex::LowerG,
            'h' => FontIndex::LowerH,
            'i' => FontIndex::LowerI,
            'j' => FontIndex::LowerJ,
            'k' => FontIndex::LowerK,
            'l' => FontIndex::LowerL,
            'm' => FontIndex::LowerM,
            'n' => FontIndex::LowerN,
            'o' => FontIndex::LowerO,
            'p' => FontIndex::LowerP,
            'q' => FontIndex::LowerQ,
            'r' => FontIndex::LowerR,
            's' => FontIndex::LowerS,
            't' => FontIndex::LowerT,
            'u' => FontIndex::LowerU,
            'v' => FontIndex::LowerV,
            'w' => FontIndex::LowerW,
            'x' => FontIndex::LowerX,
            'y' => FontIndex::LowerY,
            'z' => FontIndex::LowerZ,
            '.' => FontIndex::Dot,
            '+' => FontIndex::Plus,
            '-' => FontIndex::Sub,
            ';' => FontIndex::SemiColin,
            ',' => FontIndex::Comma,
            ' ' => FontIndex::Space,
            _ => FontIndex::Empty,
        }
    }
}

#[derive(Component)]
struct ScoreBoard;

#[derive(Component, Clone, Copy, Reflect)]
enum MyFont {
    Default,
    Custom(f32),
}

#[derive(Component, Reflect)]
#[require(Node)]
struct MyText(Cow<'static, str>);

fn update_font_size(mut fonts: Query<(&mut Node, &MyFont)>, font_data: Res<FontData>) {
    let fs = font_data.font_size.into();
    for (mut font, size) in &mut fonts {
        let fs = match size {
            MyFont::Default => fs,
            MyFont::Custom(c) => *c,
        };
        font.height = Val::Px(fs);
    }
}

fn set_font_size(
    mut fonts: Query<(&mut Node, &MyFont), Changed<MyFont>>,
    font_data: Res<FontData>,
) {
    let fs = font_data.font_size.into();
    for (mut font, size) in &mut fonts {
        let fs = match size {
            MyFont::Default => fs,
            MyFont::Custom(c) => *c,
        };
        font.height = Val::Px(fs);
    }
}

pub mod menus;

#[derive(Resource, serde::Deserialize, serde::Serialize)]
pub struct UiPalette {
    pub background: Color,
    pub text_color: Color,
    pub button_color: Color,
    pub hover_color: Color,
    pub click_color: Color,
}

#[allow(clippy::type_complexity)]
fn update_palette(
    mut elements: ParamSet<(
        Query<&mut BackgroundColor, With<Button>>,
        Query<&mut BackgroundColor, With<StateScoped<menus::Menu>>>,
    )>,
    palette: Res<UiPalette>,
) {
    if !palette.is_changed() {
        return;
    }
    for mut bg in &mut elements.p0() {
        bg.0 = palette.button_color;
    }
    for mut bg in &mut elements.p1() {
        bg.0 = palette.background;
    }
}

fn save_palette(mut store: ResMut<PkvStore>, palette: Res<UiPalette>) {
    if let Err(e) = store.set(DataKeys::UiPalette, &*palette) {
        error!("Failed to save palette: {e:?}");
    };
}
fn save_font_size(mut store: ResMut<PkvStore>, font_data: Res<FontData>) {
    if let Err(e) = store.set(DataKeys::FontSize, &font_data.font_size) {
        error!("Failed to save palette: {e:?}");
    };
}

impl UiPalette {
    fn new() -> Self {
        UiPalette {
            background: Color::srgb(0.45, 0.45, 0.45),
            button_color: Color::srgb(0.33, 0.33, 0.33),
            hover_color: Color::srgb(0.25, 0.25, 0.25),
            click_color: Color::srgb(0.50, 0.50, 0.50),
            text_color: Color::srgb(1., 0.1, 0.1),
        }
    }
}

impl FromWorld for UiPalette {
    fn from_world(world: &mut World) -> Self {
        let store = world.resource::<bevy_pkv::PkvStore>();
        if let Ok(old) = store.get(DataKeys::UiPalette) {
            old
        } else {
            UiPalette::new()
        }
    }
}

#[derive(Clone, Copy)]
struct MenuButton {
    cleanup: bool,
    on_click: SystemId,
}

impl Component for MenuButton {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_replace(|mut world, ctx| {
            let buttons = *world
                .get::<MenuButton>(ctx.entity)
                .expect("About to remove");
            if buttons.cleanup {
                world.commands().unregister_system(buttons.on_click);
            }
        });
    }
    type Mutability = bevy::ecs::component::Immutable;
}

fn fill_text(
    text: Query<(Entity, &MyText, Option<&MyFont>), Changed<MyText>>,
    font: Res<FontData>,
    palette: Res<UiPalette>,
    mut commands: Commands,
) {
    for (parent, MyText(text), size) in &text {
        let size = size.cloned().unwrap_or(MyFont::Default);
        if let MyFont::Custom(size) = size {
            info!("{}", size);
        }

        commands
            .entity(parent)
            .despawn_related::<Children>()
            .with_children(|p| {
                for mut letter in text.chars() {
                    if letter.is_lowercase() {
                        letter = letter.to_ascii_uppercase()
                    }
                    let index = FontIndex::from(letter);
                    p.spawn((
                        ImageNode {
                            color: palette.text_color,
                            image: font.image.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: font.atlas.clone(),
                                index: index as usize,
                            }),
                            ..Default::default()
                        },
                        Node {
                            margin: UiRect::vertical(Val::Auto),
                            ..Default::default()
                        },
                        size,
                    ));
                }
            });
    }
}

fn ui_hover(
    mut elements: Query<(&mut BackgroundColor, &Interaction), Changed<Interaction>>,
    palette: Res<UiPalette>,
) {
    for (mut element, state) in &mut elements {
        match state {
            Interaction::Pressed => element.0 = palette.click_color,
            Interaction::Hovered => element.0 = palette.hover_color,
            Interaction::None => element.0 = palette.button_color,
        }
    }
}

fn run_button_clicks(
    buttons: Query<(&MenuButton, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (button, interaction) in &buttons {
        let Interaction::Pressed = interaction else {
            continue;
        };
        commands.run_system(button.on_click);
    }
}

mod widgets;

fn spawn_score(mut commands: Commands, score: Res<Score>) {
    commands.spawn((
        ScoreBoard,
        MyText(score.0.to_string().into()),
        Node {
            left: Val::Percent(5.),
            top: Val::Percent(20.),
            ..Default::default()
        },
        BorderRadius::all(Val::Px(20.)),
    ));
}

fn update_score(score: Res<Score>, mut board: Query<&mut MyText, With<ScoreBoard>>) {
    for mut board in &mut board {
        board.0 = score.0.to_string().into()
    }
}
