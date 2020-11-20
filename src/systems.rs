use crate::components::*;
use crate::constants;
use crate::resources;
use bevy::prelude::*;

pub fn drop(
    time: Res<Time>,
    mut timer: ResMut<resources::SpeedTimer>,
    mut query: Query<With<Piece, (&mut Movement,)>>,
) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        for (mut movement,) in query.iter_mut() {
            *movement = Movement::Down;
        }
    }
}

pub fn input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<With<Piece, (&mut Rotation, &mut Movement, &Blocked)>>,
) {
    for (mut rotation, mut movement, blocked) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) && !blocked.left {
            *movement = Movement::Left;
        }
        if keyboard_input.pressed(KeyCode::Right) && !blocked.right {
            *movement = Movement::Right;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            *movement = Movement::Down;
        }
        if keyboard_input.just_pressed(KeyCode::Up) {
            *movement = Movement::Rotation;
            rotation.0 = (rotation.0 + 1) % 4;
        }
    }
}

pub fn input_movement(
    time: Res<Time>,
    mut timer: ResMut<resources::ControlTimer>,
    mut query: Query<With<Piece, (&mut Blocked, &mut Movement, &mut GridPos)>>,
) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        for (mut blocked, mut movement, mut grid_pos) in query.iter_mut() {
            match *movement {
                Movement::Left => grid_pos.x -= 1,
                Movement::Right => grid_pos.x += 1,
                Movement::Down => grid_pos.y += 1,
                _ => (),
            }

            *movement = Movement::None;
            blocked.left = false;
            blocked.right = false;
            blocked.bottom = false;
        }
    }
}

pub fn movement_to_pixels(
    grid: Res<resources::Grid>,
    mut query: Query<With<Piece, (&mut Transform, &GridPos)>>,
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
    bloc: Query<With<Active, (&BlocPosition, &GridPos)>>,
    other: Query<Without<Active, (&Collider, &GridPos)>>,
    mut blocked: Query<(&Piece, &mut Blocked)>,
) {
    for (_global, mut b) in blocked.iter_mut() {
        for (_bloc, grid_pos) in bloc.iter() {
            b.left = b.left || grid_pos.x == 0;
            b.right = b.right || grid_pos.x == grid.width - 1;
            b.bottom = b.bottom || grid_pos.y == grid.height;

            for (_other, other_grid_pos) in other.iter() {
                b.left = b.left || collides_left(other_grid_pos, grid_pos);
                b.right = b.right || collides_right(other_grid_pos, grid_pos);
                b.bottom = b.bottom || collides_bottom(other_grid_pos, grid_pos);
            }
        }
    }
}

pub fn spawn(
    mut commands: Commands,
    grid: Res<resources::Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    piece: Query<(&Piece,)>,
) {
    if piece.iter().next().is_none() {
        let grid_pos = GridPos { x: 4, y: 0 };
        commands
            .spawn((Piece, Rotation(0), Movement::None))
            .with(Transform::from_translation(
                grid.as_translation(grid_pos.x, grid_pos.y),
            ))
            .with(GlobalTransform::default())
            .with(Blocked {
                left: false,
                right: false,
                bottom: false,
            })
            .with(grid_pos)
            .with_children(|parent| {
                for (idx, pos) in constants::T.orientations[0].0.iter().enumerate() {
                    let x = grid.unit * pos.0 as f32;
                    let y = grid.unit * pos.1 as f32;
                    parent
                        .spawn(SpriteComponents {
                            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
                            sprite: Sprite::new(Vec2::new(grid.unit, grid.unit)),
                            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                            ..Default::default()
                        })
                        .with(BlocPosition(idx))
                        .with(GridPos {
                            x: 4 + pos.0,
                            y: pos.1,
                        })
                        .with(Active)
                        .with(Collider);
                }
            });
    }
}

pub fn rotation(
    grid: Res<resources::Grid>,
    mut query: Query<With<Piece, (&Children, &Rotation)>>,
    mut q: Query<(&BlocPosition, &mut Transform)>,
) {
    for (children, rotation) in query.iter_mut() {
        for child in children.iter() {
            if let Ok((position, mut transform)) = q.get_mut(*child) {
                let pos = constants::T.orientations[rotation.0].0[position.0];
                let x = grid.unit * pos.0 as f32;
                let y = grid.unit * pos.1 as f32;
                *transform = Transform::from_translation(Vec3::new(x, y, 0.0));
            }
        }
    }
}

pub fn game_over(
    mut score: ResMut<resources::Scoreboard>,
    query: Query<Without<Active, (&BlocPosition, &GridPos)>>,
) {
    for _ in query.iter().filter(|(_, grid_pos)| grid_pos.y <= 0) {
        score.game_over = true;
    }
}

pub fn scoreboard(scoreboard: Res<resources::Scoreboard>, mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        if scoreboard.game_over {
            text.value = format!("Score: {}. Game Over", scoreboard.score);
        } else {
            text.value = format!("Score: {}", scoreboard.score);
        };
    }
}

pub fn block_grid_position(
    mut query: Query<With<Piece, (&Children, &GridPos, &Rotation)>>,
    mut q: Query<(&BlocPosition, &mut GridPos)>,
) {
    for (children, parent_grid_pos, rotation) in query.iter_mut() {
        for child in children.iter() {
            if let Ok((position, mut grid_pos)) = q.get_mut(*child) {
                let pos = constants::T.orientations[rotation.0].0[position.0];
                grid_pos.x = parent_grid_pos.x + pos.0;
                grid_pos.y = parent_grid_pos.y + pos.1;
            }
        }
    }
}

pub fn bottom_blocked(mut commands: Commands, pieces: Query<(Entity, &Children, &Blocked)>) {
    for (entity, children, blocked) in pieces.iter() {
        if blocked.bottom {
            for child in children.iter() {
                commands.remove_one::<Active>(*child);
            }
            commands.despawn(entity);
        }
    }
}
