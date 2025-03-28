use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use rand::seq::SliceRandom;

use crate::{
    blocks::Block,
    board::{self, Shape},
    GameState,
};

#[derive(Resource)]
pub struct Deck {
    shapes: Vec<Shape>,
}

impl FromWorld for Deck {
    fn from_world(_world: &mut World) -> Self {
        let mut deck = Deck {
            shapes: vec![
                Shape {
                    split: false,
                    center: IVec2::new(0, 1),
                    blocks: vec![
                        IVec2::new(0, 0),
                        IVec2::new(0, -1),
                        IVec2::new(0, -2),
                        IVec2::new(0, 1),
                    ],
                    color: bevy::color::palettes::css::LIGHT_BLUE.into(),
                    center_of_mass: Vec2::ZERO,
                },
                Shape {
                    split: false,
                    center: IVec2::new(0, 0),
                    blocks: vec![
                        IVec2::new(-1, 0),
                        IVec2::new(0, 0),
                        IVec2::new(0, -1),
                        IVec2::new(1, -1),
                    ],
                    color: bevy::color::palettes::css::RED.into(),
                    center_of_mass: Vec2::ZERO,
                },
                Shape {
                    split: false,
                    center: IVec2::new(0, 0),
                    blocks: vec![
                        IVec2::new(-1, 0),
                        IVec2::new(0, 0),
                        IVec2::new(0, -1),
                        IVec2::new(-1, -1),
                    ],
                    color: bevy::color::palettes::css::YELLOW.into(),
                    center_of_mass: Vec2::ZERO,
                },
                Shape {
                    split: false,
                    center: IVec2::new(0, 0),
                    blocks: vec![
                        IVec2::new(0, 1),
                        IVec2::new(0, 0),
                        IVec2::new(1, 0),
                        IVec2::new(1, -1),
                    ],
                    color: bevy::color::palettes::css::LIGHT_GREEN.into(),
                    center_of_mass: Vec2::ZERO,
                },
                Shape {
                    split: false,
                    center: IVec2::new(0, 0),
                    blocks: vec![
                        IVec2::new(0, 0),
                        IVec2::new(-1, 0),
                        IVec2::new(1, 0),
                        IVec2::new(0, 1),
                    ],
                    color: bevy::color::palettes::css::PURPLE.into(),
                    center_of_mass: Vec2::ZERO,
                },
                Shape {
                    split: false,
                    center: IVec2::new(0, 0),
                    blocks: vec![
                        IVec2::new(0, 2),
                        IVec2::new(0, 1),
                        IVec2::new(0, 0),
                        IVec2::new(-1, 0),
                    ],
                    color: bevy::color::palettes::css::DARK_BLUE.into(),
                    center_of_mass: Vec2::ZERO,
                },
                Shape {
                    split: false,
                    center: IVec2::new(0, 0),
                    blocks: vec![
                        IVec2::new(0, 2),
                        IVec2::new(0, 1),
                        IVec2::new(0, 0),
                        IVec2::new(1, 0),
                    ],
                    color: bevy::color::palettes::css::ORANGE.into(),
                    center_of_mass: Vec2::ZERO,
                },
            ],
        };
        // let mut deck = Deck {
        //     shapes: vec![Shape {
        //     split: false,
        //         center: IVec2::new(0, 1),
        //         blocks: vec![
        //             IVec2::new(0, 0),
        //             IVec2::new(0, -1),
        //             IVec2::new(0, -2),
        //             IVec2::new(0, 1),
        //         ],
        //         color: bevy::color::palettes::css::LIGHT_BLUE.into(),
        //center_of_mass: Vec2::ZERO,
        //     }],
        // };
        // deck.shapes.clear();
        // deck.shapes.push(Shape {
        //     split: false,
        //     center: IVec2::new(0, 0),
        //     blocks: vec![
        //         IVec2::new(0, 0),
        //         IVec2::new(-1, 0),
        //         IVec2::new(1, 0),
        //         IVec2::new(0, 1),
        //     ],
        //     color: bevy::color::palettes::css::PURPLE.into(),
        //     center_of_mass: Vec2::ZERO,
        // });
        for shape in deck.shapes.iter_mut() {
            shape.calc_center();
        }
        deck
    }
}

#[derive(Resource)]
pub struct CurrentDeck {
    shapes: Vec<Shape>,
}

impl FromWorld for CurrentDeck {
    fn from_world(world: &mut World) -> Self {
        let deck = world.resource::<Deck>();
        let mut deck = CurrentDeck {
            shapes: deck.shapes.to_vec(),
        };
        deck.shapes.shuffle(&mut rand::thread_rng());
        deck
    }
}

impl CurrentDeck {
    pub fn next(&mut self) -> Shape {
        self.shapes.pop().expect("Always at least one shape")
    }
}

fn refill_deck(mut current: ResMut<CurrentDeck>, deck: Res<Deck>) {
    for shape in deck.shapes.iter() {
        current.shapes.push(shape.clone());
    }
}

#[derive(Component, Clone, Copy)]
pub struct PlayerTarget {
    pub last_y: u8,
    pub moved: bool,
}

impl Default for PlayerTarget {
    fn default() -> Self {
        PlayerTarget {
            last_y: 0,
            moved: true,
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (player_moves,).run_if(in_state(GameState::Playing)))
        .add_systems(
            FixedLast,
            remove_player_target.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            First,
            refill_deck.run_if(|deck: Res<CurrentDeck>| deck.shapes.is_empty()),
        )
        .add_systems(FixedFirst, clear_moved)
        .init_resource::<Deck>()
        .init_resource::<CurrentDeck>()
        .insert_resource(ActionState::<PlayerInputs>::default())
        .insert_resource(InputMap::new([
            (PlayerInputs::MoveLeft, KeyCode::KeyA),
            (PlayerInputs::MoveRight, KeyCode::KeyD),
            (PlayerInputs::MoveDown, KeyCode::KeyS),
            (PlayerInputs::Rotate, KeyCode::KeyW),
            (PlayerInputs::MoveLeft, KeyCode::ArrowLeft),
            (PlayerInputs::MoveRight, KeyCode::ArrowRight),
            (PlayerInputs::MoveDown, KeyCode::ArrowDown),
            (PlayerInputs::Rotate, KeyCode::ArrowUp),
        ]));
}

fn player_moves(
    time: Res<Time>,
    settings: Res<ActionState<PlayerInputs>>,
    mut target: Query<(Entity, &mut Shape, &mut PlayerTarget)>,
    mut board: ResMut<board::Board>,
    mut last: Local<f32>,
    mut commands: Commands,
) {
    for (entity, mut shape, mut target) in &mut target {
        if settings.just_pressed(&PlayerInputs::MoveLeft) {
            shape.translate(&mut board, IVec2::NEG_X);
            *last = 0.;
            target.moved = true;
        }
        if settings.just_pressed(&PlayerInputs::MoveRight) {
            shape.translate(&mut board, IVec2::X);
            *last = 0.;
            target.moved = true;
        }
        if settings.just_pressed(&PlayerInputs::Rotate) {
            shape.rotate(&mut board);
            *last = 0.;
            target.moved = true;
        }
        if settings.pressed(&PlayerInputs::MoveDown) {
            if shape.translate(&mut board, IVec2::NEG_Y) {
                target.last_y = 0;
                target.moved = true;
            } else {
                commands.entity(entity).remove::<PlayerTarget>();
            }
        }
        if !settings.get_pressed().is_empty() {
            *last += time.delta_secs();
        }
        if *last < 0.5 {
            return;
        } else if settings.pressed(&PlayerInputs::MoveLeft) {
            shape.translate(&mut board, IVec2::NEG_X);
            *last = 0.;
        } else if settings.pressed(&PlayerInputs::MoveRight) {
            shape.translate(&mut board, IVec2::X);
            *last = 0.;
        } else if settings.pressed(&PlayerInputs::Rotate) {
            shape.rotate(&mut board);
            *last = 0.;
        }
        target.moved = true;
    }
}

fn clear_moved(mut target: Query<&mut PlayerTarget>) {
    for mut target in &mut target {
        target.last_y += 1;
        target.moved = false;
    }
}

fn remove_player_target(
    player: Query<(Entity, &PlayerTarget), With<PlayerTarget>>,
    mut commands: Commands,
) {
    for (entity, shape) in &player {
        if shape.last_y > 3 || !shape.moved {
            commands.entity(entity).remove::<PlayerTarget>();
        }
    }
}

#[derive(leafwing_input_manager::Actionlike, Reflect, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum PlayerInputs {
    MoveLeft,
    MoveRight,
    MoveDown,
    Rotate,
}
