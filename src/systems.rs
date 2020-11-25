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
        status.next_movement = resources::Movement::Down;
    }
}

pub fn input(mut status: ResMut<resources::Status>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Left) && !status.blocked_left {
        status.next_movement = resources::Movement::Left;
    }
    if keyboard_input.pressed(KeyCode::Right) && !status.blocked_right {
        status.next_movement = resources::Movement::Right;
    }
    if keyboard_input.pressed(KeyCode::Down) && !status.blocked_bottom {
        status.next_movement = resources::Movement::Down;
    }
    if keyboard_input.just_pressed(KeyCode::Up) {
        status.next_movement = resources::Movement::Rotation;
    }
}

pub fn input_movement(
    time: Res<Time>,
    mut status: ResMut<resources::Status>,
    mut timer: ResMut<resources::ControlTimer>,
) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        match status.next_movement {
            resources::Movement::Left => status.x -= 1,
            resources::Movement::Right => status.x += 1,
            resources::Movement::Down => status.y += 1,
            resources::Movement::Rotation => status.rotation = (status.rotation + 1) % 4,
            _ => (),
        }

        status.next_movement = resources::Movement::None;
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

pub fn collision(
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

pub fn spawn(
    mut commands: Commands,
    mut status: ResMut<resources::Status>,
    grid: Res<resources::Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    active: Query<(&Active,)>,
) {
    if active.iter().next().is_none() && !status.game_over {
        status.x = 4;
        status.y = 0;
        status.blocked_bottom = false;
        status.piece = constants::rand_tetromino();
        for (idx, pos) in status.piece.orientations[0].0.iter().enumerate() {
            let grid_pos = GridPos {
                x: status.x + pos.0,
                y: status.y + pos.1,
            };
            commands
                .spawn(SpriteComponents {
                    material: materials.add(status.piece.color.into()),
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
    status: Res<resources::Status>,
    mut query: Query<With<Active, (&BlocPosition, &mut GridPos)>>,
) {
    for (position, mut grid_pos) in query.iter_mut() {
        let pos = status.piece.orientations[status.rotation].0[position.0];
        grid_pos.x = status.x + pos.0;
        grid_pos.y = status.y + pos.1;
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

pub fn bottom_blocked(
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
