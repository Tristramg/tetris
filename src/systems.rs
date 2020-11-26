use crate::components::*;
use crate::constants;
use crate::resources;
use bevy::prelude::*;

pub fn drop(
    time: Res<Time>,
    mut timer: ResMut<resources::SpeedTimer>,
    mut status: ResMut<resources::Status>,
) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        status.next_movements.insert(resources::Movement::Down);
    }
}

pub fn read_input(mut status: ResMut<resources::Status>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Left) {
        status.next_movements.insert(resources::Movement::Left);
    }
    if keyboard_input.pressed(KeyCode::Right) {
        status.next_movements.insert(resources::Movement::Right);
    }
    if keyboard_input.pressed(KeyCode::Down) {
        status.next_movements.insert(resources::Movement::Down);
    }
    if keyboard_input.just_pressed(KeyCode::Up) {
        status.next_movements.insert(resources::Movement::Rotation);
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        status.next_movements.insert(resources::Movement::Drop);
    }
}

pub fn apply_movement(
    time: Res<Time>,
    grid: Res<resources::Grid>,
    mut status: ResMut<resources::Status>,
    mut piece: ResMut<resources::Piece>,
    mut timer: ResMut<resources::ControlTimer>,
    blocks: Query<With<Active, (&GridPos,)>>,
    other: Query<Without<Active, (&GridPos,)>>,
) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        let blocked_left = blocks.iter().any(|(block,)| {
            block.x == 0 || other.iter().any(|(other,)| collides_left(other, block))
        });
        let blocked_right = blocks.iter().any(|(block,)| {
            block.x == grid.width - 1 || other.iter().any(|(other,)| collides_right(other, block))
        });
        piece.blocked_bottom = blocks.iter().any(|(block,)| {
            block.y == grid.height - 1 || other.iter().any(|(other,)| collides_bottom(other, block))
        });

        let mut down = 0;

        for movement in status.next_movements.drain() {
            match movement {
                resources::Movement::Left => {
                    if !blocked_left {
                        piece.x -= 1
                    }
                }
                resources::Movement::Right => {
                    if !blocked_right {
                        piece.x += 1
                    }
                }
                resources::Movement::Down => {
                    if !piece.blocked_bottom {
                        down = down.max(1)
                    }
                }
                resources::Movement::Rotation => {
                    piece.rotation = (piece.rotation + 1) % 4;
                }
                resources::Movement::Drop => {
                    down = piece.drop_height;
                }
            }
        }
        piece.y += down;
    }
}

pub fn movement_to_pixels(
    grid: Res<resources::Grid>,
    mut query: Query<(&mut Transform, &GridPos)>,
) {
    for (mut transform, grid_pos) in query.iter_mut() {
        transform.translation = grid.as_translation(grid_pos.x, grid_pos.y);
    }
}

fn collides_left(a: &GridPos, b: &GridPos) -> bool {
    a.y == b.y && a.x + 1 == b.x
}

fn collides_right(a: &GridPos, b: &GridPos) -> bool {
    a.y == b.y && a.x == b.x + 1
}

fn collides_bottom(a: &GridPos, b: &GridPos) -> bool {
    a.x == b.x && a.y == b.y + 1
}

pub fn spawn_new_piece(
    mut commands: Commands,
    mut status: ResMut<resources::Status>,
    mut piece: ResMut<resources::Piece>,
    grid: Res<resources::Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    active: Query<(&Active,)>,
) {
    if active.iter().next().is_none() && !status.game_over {
        status.next_movements.clear();
        piece.x = 4;
        piece.y = 0;
        piece.piece = constants::rand_tetromino();
        piece.rotation = 0;
        piece.blocked_bottom = false;
        for (idx, pos) in piece.piece.orientations[0].0.iter().enumerate() {
            let grid_pos = GridPos {
                x: piece.x + pos.0,
                y: piece.y + pos.1,
            };
            let mut shadow_color = piece.piece.color.clone();
            shadow_color.set_a(0.5);
            commands
                .spawn(SpriteComponents {
                    material: materials.add(shadow_color.into()),
                    sprite: Sprite::new(Vec2::new(grid.unit - 1.0, grid.unit - 1.0)),
                    transform: Transform::from_translation(
                        grid.as_translation(piece.x + pos.0, piece.y + pos.1),
                    ),
                    ..Default::default()
                })
                .with(BlocPosition(idx))
                .with(Shadow);
            commands
                .spawn(SpriteComponents {
                    material: materials.add(piece.piece.color.into()),
                    sprite: Sprite::new(Vec2::new(grid.unit - 1.0, grid.unit - 1.0)),
                    transform: Transform::from_translation(
                        grid.as_translation(grid_pos.x, grid_pos.y),
                    ),
                    ..Default::default()
                })
                .with(BlocPosition(idx))
                .with(grid_pos)
                .with(Active);
        }
    }
}

pub fn bloc_global_position(
    piece: Res<resources::Piece>,
    mut query: Query<With<Active, (&BlocPosition, &mut GridPos)>>,
) {
    for (position, mut grid_pos) in query.iter_mut() {
        let pos = piece.piece.orientations[piece.rotation].0[position.0];
        grid_pos.x = piece.x + pos.0;
        grid_pos.y = piece.y + pos.1;
    }
}

pub fn game_over(
    mut status: ResMut<resources::Status>,
    query: Query<Without<Active, (&GridPos,)>>,
) {
    if query.iter().any(|(grid_pos,)| grid_pos.y <= 0) {
        status.game_over = true;
    }
}

pub fn scoreboard(status: Res<resources::Status>, mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        text.value = format!(
            "Score: {}\nLevel: {}\nLines: {}{}",
            status.score,
            status.level,
            status.lines,
            if status.game_over { "\n Game Over" } else { "" }
        );
    }
}

pub fn remove_piece(
    mut commands: Commands,
    piece: Res<resources::Piece>,
    pieces: Query<With<Active, (Entity,)>>,
) {
    if piece.blocked_bottom {
        for (entity,) in pieces.iter() {
            commands.remove_one::<Active>(entity);
        }
    }
}

fn score(lines: usize, level: usize) -> usize {
    level
        * match lines {
            1 => 100,
            2 => 250,
            3 => 500,
            4 => 1000,
            _ => 0,
        }
}

pub fn completed_line(
    mut commands: Commands,
    grid: Res<resources::Grid>,
    mut status: ResMut<resources::Status>,
    piece: ResMut<resources::Piece>,
    mut blocks: Query<(Entity, &mut GridPos)>,
) {
    if piece.blocked_bottom {
        let counts = blocks
            .iter_mut()
            .map(|(_, grid_pos)| grid_pos.y)
            .fold(std::collections::HashMap::new(), |mut acc, y| {
                *acc.entry(y).or_insert(0) += 1;
                acc
            })
            .iter()
            .filter(|(_line, count)| *count == &grid.width)
            .map(|(line, _count)| *line)
            .collect::<Vec<_>>();
        status.lines += counts.len();
        status.score += score(counts.len(), status.level);
        status.level = status.level.max(1 + status.lines / 10);

        for line in counts {
            for (entity, _) in blocks.iter_mut().filter(|(_, pos)| (*pos).y == line) {
                commands.despawn(entity);
            }
            for (_, mut pos) in blocks.iter_mut().filter(|(_, pos)| (*pos).y < line) {
                pos.y += 1;
            }
        }
    }
}

pub fn update_speed(mut timer: ResMut<resources::SpeedTimer>, status: Res<resources::Status>) {
    timer.0.duration = (1.0 - status.level as f32 / 20.0).max(0.01)
}

pub fn compute_drop_height(
    grid: Res<resources::Grid>,
    mut piece: ResMut<resources::Piece>,
    blocks: Query<With<Active, (&GridPos,)>>,
    other: Query<Without<Active, (&GridPos,)>>,
) {
    piece.drop_height = (0..grid.height - 1)
        .filter(|depth| {
            blocks.iter().any(|(block,)| {
                block.y + depth == grid.height - 1
                    || other.iter().any(|(other,)| {
                        collides_bottom(
                            other,
                            &GridPos {
                                x: block.x,
                                y: block.y + depth,
                            },
                        )
                    })
            })
        })
        .next()
        .unwrap_or(0)
}

pub fn move_shadow(
    grid: Res<resources::Grid>,
    piece: Res<resources::Piece>,
    blocks: Query<With<Active, (&GridPos, &BlocPosition)>>,
    mut shadows: Query<With<Shadow, (&mut Transform, &BlocPosition)>>,
) {
    for (grid_pos, pos) in blocks.iter() {
        for (mut shadow, _) in shadows.iter_mut().filter(|(_, s_pos)| pos.0 == s_pos.0) {
            shadow.translation = grid.as_translation(grid_pos.x, grid_pos.y + piece.drop_height);
        }
    }
}
