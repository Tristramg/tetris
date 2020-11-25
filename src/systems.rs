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
    if keyboard_input.pressed(KeyCode::Left) && !status.blocked_left {
        status.next_movements.insert(resources::Movement::Left);
    }
    if keyboard_input.pressed(KeyCode::Right) && !status.blocked_right {
        status.next_movements.insert(resources::Movement::Right);
    }
    if keyboard_input.pressed(KeyCode::Down) && !status.blocked_bottom {
        status.next_movements.insert(resources::Movement::Down);
    }
    if keyboard_input.just_pressed(KeyCode::Up) {
        status.next_movements.insert(resources::Movement::Rotation);
    }
}

pub fn apply_movement(
    time: Res<Time>,
    mut status: ResMut<resources::Status>,
    mut piece: ResMut<resources::Piece>,
    mut timer: ResMut<resources::ControlTimer>,
) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        for movement in status.next_movements.drain() {
            match movement {
                resources::Movement::Left => piece.x -= 1,
                resources::Movement::Right => piece.x += 1,
                resources::Movement::Down => piece.y += 1,
                resources::Movement::Rotation => {
                    piece.rotation = (piece.rotation + 1) % 4;
                }
            }
        }

        status.blocked_left = false;
        status.blocked_right = false;
        status.blocked_bottom = false;
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

pub fn test_collisions(
    grid: Res<resources::Grid>,
    mut status: ResMut<resources::Status>,
    bloc: Query<With<Active, (&BlocPosition, &GridPos)>>,
    other: Query<Without<Active, (&Collider, &GridPos)>>,
) {
    for (_bloc, grid_pos) in bloc.iter() {
        status.blocked_left = status.blocked_left || grid_pos.x == 0;
        status.blocked_right = status.blocked_right || grid_pos.x == grid.width - 1;
        status.blocked_bottom = status.blocked_bottom || grid_pos.y == grid.height - 1;

        for (_other, other_grid_pos) in other.iter() {
            status.blocked_left = status.blocked_left || collides_left(other_grid_pos, grid_pos);
            status.blocked_right = status.blocked_right || collides_right(other_grid_pos, grid_pos);
            status.blocked_bottom =
                status.blocked_bottom || collides_bottom(other_grid_pos, grid_pos);
        }
    }
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
        piece.x = 4;
        piece.y = 0;
        piece.piece = constants::rand_tetromino();
        status.blocked_bottom = false;
        for (idx, pos) in piece.piece.orientations[0].0.iter().enumerate() {
            let grid_pos = GridPos {
                x: piece.x + pos.0,
                y: piece.y + pos.1,
            };
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
                .with(Active)
                .with(Collider);
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
    query: Query<Without<Active, (&BlocPosition, &GridPos)>>,
) {
    if query.iter().any(|(_, grid_pos)| grid_pos.y <= 0) {
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
    status: Res<resources::Status>,
    pieces: Query<With<Active, (Entity,)>>,
) {
    if status.blocked_bottom {
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
    mut blocks: Query<With<BlocPosition, (Entity, &mut GridPos)>>,
) {
    if status.blocked_bottom {
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
