use bevy::{prelude::*, utils::hashbrown::HashSet};
use indexmap::IndexSet;

use crate::{deck::PlayerTarget, prelude::*, GameState};

// create a resouse that holds the current boaed state linke each position to an entity or none if it is empty
#[derive(Resource)]
pub struct Board {
    width: i32,
    hight: i32,
    board: Vec<Option<Entity>>,
    changed: HashSet<IVec2>,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            width: 10,
            hight: 20,
            board: vec![None; 10 * 20],
            changed: HashSet::new(),
        }
    }
}
impl Board {
    pub fn get(&self, block: IVec2) -> BlockState {
        let IVec2 { x, y } = block;
        if x >= self.width || y >= self.hight {
            return BlockState::OutOfBounds;
        }
        if x < 0 || y < 0 {
            return BlockState::OutOfBounds;
        }
        self.board[(y * self.width + x) as usize].map_or(BlockState::Empty, BlockState::Contains)
    }
    pub fn set(&mut self, block: IVec2, entity: Entity) {
        self.changed.insert(block);
        self.board[(block.y * self.width + block.x) as usize] = Some(entity);
    }
    pub fn clear(&mut self, block: IVec2) {
        self.changed.insert(block);
        self.board[(block.y * self.width + block.x) as usize] = None;
    }
    pub fn take(&mut self, block: IVec2) -> Option<Entity> {
        self.changed.insert(block);
        std::mem::take(&mut self.board[(block.y * 10 + block.x) as usize])
    }
}

#[derive(PartialEq, Eq)]
pub enum BlockState {
    Empty,
    OutOfBounds,
    Contains(Entity),
}

#[derive(Clone, Copy)]
pub struct Block {
    pub shape: Entity,
    pub moved: bool,
}

fn clear_moved(mut blocks: Query<&mut Block>) {
    for mut block in &mut blocks {
        block.moved = false;
    }
}

fn update_moved(mut blocks: Query<&mut Block, Changed<Transform>>) {
    for mut block in &mut blocks {
        block.moved = true;
    }
}

impl Component for Block {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        // hooks.on_insert(|mut world, entity, _| {
        //     let pos = *world.get::<Block>(entity).expect("Just Inserted");
        //     world
        //         .get_resource_mut::<BoardState>()
        //         .expect("Board always exists")
        //         .set(pos.position, entity);
        // });
        // hooks.on_replace(|mut world, entity, _| {
        //     let pos = *world.get::<Block>(entity).expect("About to replace");
        //     world
        //         .get_resource_mut::<BoardState>()
        //         .expect("Board always exists")
        //         .clear(pos.position.x, pos.position.y);
        // });
    }
}

#[derive(Resource)]
pub struct BlockImage(Handle<Image>);
impl BlockImage {
    #[inline(always)]
    fn get(&self) -> Handle<Image> {
        self.0.clone()
    }
}

impl FromWorld for BlockImage {
    fn from_world(world: &mut World) -> Self {
        BlockImage(world.resource::<AssetServer>().load("block.png"))
    }
}

#[derive(Component, Clone)]
pub struct Shape {
    pub split: bool,
    pub center: IVec2,
    pub blocks: Vec<IVec2>,
    pub color: Color,
}

impl Shape {
    pub fn can_translate(&self, board: &Board, offset: IVec2) -> bool {
        let mut can_move = true;
        for block in self.blocks.iter() {
            let mut next = block + offset;
            if self.blocks.contains(&next) {
                continue;
            };
            next += self.center;
            match board.get(next) {
                BlockState::Empty => {}
                _ => can_move = false,
            }
        }
        can_move
    }
    pub fn translate(&mut self, board: &mut Board, offset: IVec2) -> bool {
        if !self.can_translate(board, offset) {
            return false;
        }
        let mut old = Vec::new();
        for block in self.blocks.iter() {
            let block = self.center + block;
            old.push(board.take(block));
        }
        self.center += offset;
        for (block, target) in self.blocks.iter().zip(old) {
            let block = self.center + block;
            if let Some(target) = target {
                // todo .expect("Blocks Should not be empty")
                board.set(block, target);
            }
        }
        true
    }

    pub fn can_rotate(&self, board: &Board) -> bool {
        let mut can_move = true;
        for block in self.blocks.iter() {
            let mut next = IVec2 {
                x: -block.y,
                y: block.x,
            };
            if self.blocks.contains(&next) {
                continue;
            };
            next += self.center;
            match board.get(next) {
                BlockState::Empty => {}
                _ => can_move = false,
            }
        }
        can_move
    }

    pub fn rotate(&mut self, board: &mut Board) -> bool {
        if !self.can_rotate(board) {
            return false;
        }
        let mut old = Vec::new();
        for block in self.blocks.iter() {
            let block = self.center + block;
            old.push(board.take(block));
        }
        for (block, target) in self.blocks.iter_mut().zip(old) {
            let x = -block.y;
            block.y = block.x;
            block.x = x;
            let block = self.center + *block;
            if let Some(target) = target {
                // todo .expect("Blocks Should not be empty")
                board.set(block, target);
            }
        }
        true
    }

    pub fn can_spawn(&self, board: &crate::board::Board) -> bool {
        for block in self.blocks.iter() {
            let block = self.center + block;
            if BlockState::Empty != board.get(block) {
                return false;
            }
        }
        true
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_shape)
        .add_systems(PostUpdate, (update_board, update_moved, clear_line).chain())
        .add_systems(
            FixedUpdate,
            (apply_gravity, spawn_next).run_if(in_state(GameState::Playing)),
        )
        .add_systems(FixedFirst, clear_moved)
        .init_resource::<BlockImage>()
        .add_systems(PostUpdate, split_shape);

    app.register_required_components::<Block, Sprite>();
}

fn apply_gravity(
    mut shapes: Query<(Entity, &mut Shape)>,
    mut target: Query<&mut PlayerTarget>,
    mut board: ResMut<crate::board::Board>,
) {
    for (entity, mut shape) in &mut shapes {
        if shape.translate(&mut board, IVec2::NEG_Y) {
            if let Ok(mut target) = target.get_mut(entity) {
                target.last_y = 0;
                target.moved = true;
            }
        }
    }
}

fn spawn_shape(
    mut commands: Commands,
    shapes: Query<(Entity, &Shape), Added<Shape>>,
    mut board: ResMut<crate::board::Board>,
    block_image: Res<BlockImage>,
) {
    for (e, shape) in &shapes {
        if shape.split {
            continue;
        }
        for block in shape.blocks.iter() {
            let block = shape.center + block;
            let id = commands
                .spawn((
                    Block {
                        shape: e,
                        moved: true,
                    },
                    Transform::from_translation((block * 64).as_vec2().extend(1.)),
                    Sprite {
                        image: block_image.get(),
                        color: shape.color,
                        ..Default::default()
                    },
                ))
                .id();
            board.set(block, id);
        }
    }
}

fn update_board(mut blocks: Query<&mut Transform>, mut board: ResMut<Board>) {
    for cell in std::mem::take(&mut board.changed) {
        let BlockState::Contains(entity) = board.get(cell) else {
            continue;
        };
        let Ok(mut block) = blocks.get_mut(entity) else {
            warn!("{cell} has invalid entity {entity}");
            continue;
        };
        block.translation = (cell * 32).extend(1).as_vec3();
    }
}

fn spawn_next(
    active: Query<(), With<PlayerTarget>>,
    mut deck: ResMut<crate::deck::CurrentDeck>,
    board: Res<Board>,
    mut commands: Commands,
) {
    if active.get_single().is_ok() {
        return;
    };
    let mut shape = deck.next();
    let center = IVec2::new(board.width / 2, board.hight - 1);
    for y in 0..board.hight {
        for x in 0..(board.width / 2) + 1 {
            let next = center - IVec2::new(x, y);
            shape.center = next;
            if shape.can_spawn(&board) {
                commands.spawn((shape, PlayerTarget::default()));
                return;
            }
            let next = center - IVec2::new(-x, y);
            shape.center = next;
            if shape.can_spawn(&board) {
                commands.spawn((shape, PlayerTarget::default()));
                return;
            }
        }
    }
    warn!("Failed to find valid spawn for shape");
}

fn clear_line(
    mut board: ResMut<Board>,
    mut shapes: Query<&mut Shape>,
    blocks: Query<&Block>,
    player: Query<(), With<PlayerTarget>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
) {
    let mut found = 0;
    'y: for y in 0..board.hight {
        for x in 0..board.width {
            match board.get(IVec2::new(x, y)) {
                BlockState::Empty => {
                    continue 'y;
                }
                BlockState::Contains(block) => {
                    let Ok(block) = blocks.get(block) else {
                        error!("{block} is not a block");
                        continue 'y;
                    };
                    if block.moved {
                        continue 'y;
                    }
                    let Err(_) = player.get(block.shape) else {
                        continue 'y;
                    };
                }
                BlockState::OutOfBounds => {
                    continue 'y;
                }
            }
        }
        for x in 0..board.width {
            let pos = IVec2::new(x, y);
            let BlockState::Contains(entity) = board.get(pos) else {
                error!("Line Has Empty Space");
                continue;
            };
            board.clear(pos);
            let Ok(block) = blocks.get(entity) else {
                error!("{entity} is not a block");
                continue;
            };
            commands.entity(entity).despawn_recursive();
            let Ok(mut shape) = shapes.get_mut(block.shape) else {
                error!("{} is not a shape", block.shape);
                continue;
            };
            let mut index = 0;
            let pos = pos - shape.center;
            for block in shape.blocks.iter() {
                if *block == pos {
                    break;
                }
                index += 1;
            }
            shape.blocks.swap_remove(index);
            if shape.blocks.is_empty() {
                commands.entity(block.shape).despawn_recursive();
            }
        }
        found += 1;
    }
    if found > 0 {
        score.0 += found * found;
    }
}

fn split_shape(
    mut shapes: Query<&mut Shape, Changed<Shape>>,
    board: Res<Board>,
    mut blocks: Query<&mut Block>,
    mut commands: Commands,
) {
    for mut shape in &mut shapes {
        if shape.blocks.is_empty() {
            continue;
        };
        let mut to_check = IndexSet::with_capacity(shape.blocks.len());
        let mut checked = HashSet::new();
        let mut valid = Vec::new();
        to_check.insert(shape.blocks.first().cloned().expect("At least one block"));
        while let Some(current) = to_check.pop() {
            if shape.blocks.contains(&current) {
                valid.push(current);
                checked.insert(current);
                let up = current + IVec2::Y;
                if !checked.contains(&up) {
                    to_check.insert(up);
                }
                let down = current + IVec2::NEG_Y;
                if !checked.contains(&down) {
                    to_check.insert(down);
                }
                let right = current + IVec2::X;
                if !checked.contains(&right) {
                    to_check.insert(right);
                }
                let left = current + IVec2::NEG_X;
                if !checked.contains(&left) {
                    to_check.insert(left);
                }
            }
        }
        if shape.blocks.len() != valid.len() {
            std::mem::swap(&mut shape.blocks, &mut valid);
            valid.retain(|block| !shape.blocks.contains(block));
            let new = commands
                .spawn(Shape {
                    split: true,
                    center: shape.center,
                    blocks: valid.clone(),
                    color: shape.color,
                })
                .id();
            for block in valid.iter() {
                let block = block + shape.center;
                let BlockState::Contains(entity) = board.get(block) else {
                    error!("Block {block:?} not in board");
                    continue;
                };
                let Ok(mut block) = blocks.get_mut(entity) else {
                    error!("{entity:?} is not a block");
                    continue;
                };
                block.shape = new;
            }
        }
    }
}
