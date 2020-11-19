use crate::collision;
use crate::components::*;
use crate::constants;
use crate::resources;
use bevy::prelude::*;

pub fn drop(
    time: Res<Time>,
    mut timer: ResMut<resources::SpeedTimer>,
    mut query: Query<With<Active, (&Piece, &mut Velocity)>>,
) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        for (_piece, mut velocity) in query.iter_mut() {
            velocity.0.set_y(-constants::STEP);
        }
    }
}

pub fn input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<With<Active, (&Piece, &mut Rotation, &mut Velocity, &Blocked)>>,
) {
    for (_piece, mut rotation, mut velocity, blocked) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) && !blocked.left {
            velocity.0.set_x(-constants::STEP);
        }
        if keyboard_input.pressed(KeyCode::Right) && !blocked.right {
            velocity.0.set_x(constants::STEP);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            velocity.0.set_y(-constants::STEP);
        }
        if keyboard_input.just_pressed(KeyCode::Up) {
            rotation.0 = (rotation.0 + 1) % 4;
        }
    }
}

pub fn input_movement(
    time: Res<Time>,
    mut timer: ResMut<resources::ControlTimer>,
    mut query: Query<With<Active, (&Piece, &mut Velocity, &mut Transform, &mut Blocked)>>,
) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        for (_piece, mut velocity, mut transform, mut blocked) in query.iter_mut() {
            let translation = &mut transform.translation;
            *translation.x_mut() += velocity.0.x();
            *translation.y_mut() += velocity.0.y();
            velocity.0 = Vec2::zero();
            blocked.left = false;
            blocked.right = false;
        }
    }
}

pub fn collision(
    mut commands: Commands,
    bloc: Query<With<Active, (&BlocPosition, &GlobalTransform, &Sprite)>>,
    other: Query<Without<Active, (&Collider, &GlobalTransform, &Sprite)>>,
    active: Query<With<Active, (Entity,)>>,
    mut blocked: Query<(&Piece, &mut Blocked)>,
) {
    for (_bloc, transform, sprite) in bloc.iter() {
        let bounds = collision::Bounds::from_pos_size(transform.translation, sprite.size);
        for (_other, other_transform, other_sprite) in other.iter() {
            let other_bounds =
                collision::Bounds::from_pos_size(other_transform.translation, other_sprite.size);
            for (_global, mut b) in blocked.iter_mut() {
                b.left = b.left || bounds.left(&other_bounds);
                b.right = b.right || bounds.right(&other_bounds);
            }
            if bounds.bottom(&other_bounds) {
                for (entity,) in active.iter() {
                    commands.remove_one::<Active>(entity);
                }
            }
        }
    }
}

pub fn spawn(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    active: Query<With<Active, (Entity,)>>,
) {
    if active.iter().next() == None {
        commands
            .spawn((Piece,))
            .with(Transform::from_translation(Vec3::new(
                0.0,
                constants::STEP * 10.0,
                0.0,
            )))
            .with(GlobalTransform::default())
            .with(Active)
            .with(Rotation(0))
            .with(Velocity(Vec2::zero()))
            .with(Blocked {
                left: false,
                right: false,
            })
            .with_children(|parent| {
                for (idx, pos) in constants::T.orientations[0].0.iter().enumerate() {
                    let x = constants::STEP * pos.0 as f32;
                    let y = constants::STEP * pos.1 as f32;
                    parent
                        .spawn(SpriteComponents {
                            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
                            sprite: Sprite::new(Vec2::new(constants::STEP, constants::STEP)),
                            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                            ..Default::default()
                        })
                        .with(BlocPosition(idx))
                        .with(Active)
                        .with(Collider);
                }
            });
    }
}

pub fn rotation(
    mut query: Query<With<Active, (&Children, &Rotation)>>,
    mut q: Query<(&BlocPosition, &mut Transform)>,
) {
    for (children, rotation) in query.iter_mut() {
        for child in children.iter() {
            if let Ok((position, mut transform)) = q.get_mut(*child) {
                let pos = constants::T.orientations[rotation.0].0[position.0];
                let x = constants::STEP * pos.0 as f32;
                let y = constants::STEP * pos.1 as f32;
                *transform = Transform::from_translation(Vec3::new(x, y, 0.0));
            }
        }
    }
}

pub fn game_over(
    mut score: ResMut<resources::Scoreboard>,
    query: Query<Without<Active, (&BlocPosition, &GlobalTransform)>>,
) {
    for (_block, transform) in query.iter() {
        if transform.translation.y() > constants::HEIGHT as f32 / 2.0 * constants::STEP {
            score.game_over = true;
        }
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
