mod components;
mod constants;
mod resources;
mod systems;
use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Add walls
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(
        constants::STEP * constants::WIDTH as f32,
        constants::STEP * constants::HEIGHT as f32,
    );
    commands
        // left
        .spawn(SpriteComponents {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                (bounds.x() + wall_thickness) / -2.0,
                0.0,
                0.0,
            )),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y() + wall_thickness)),
            ..Default::default()
        })
        .with(components::Collider)
        // right
        .spawn(SpriteComponents {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                (bounds.x() + wall_thickness) / 2.0,
                0.0,
                0.0,
            )),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y() + wall_thickness)),
            ..Default::default()
        })
        .with(components::Collider)
        // bottom
        .spawn(SpriteComponents {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                (bounds.y() + wall_thickness) / -2.0,
                0.0,
            )),
            sprite: Sprite::new(Vec2::new(bounds.x() + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .with(components::Collider);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(TextComponents {
            text: Text {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                value: "Score:".to_string(),
                style: TextStyle {
                    color: Color::rgb(0.5, 0.5, 1.0),
                    font_size: 40.0,
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
}

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(systems::scoreboard.system())
            .add_system(systems::input.system())
            .add_system(systems::input_movement.system())
            .add_system(systems::rotation.system())
            .add_system(systems::drop.system())
            .add_system(systems::collision.system())
            .add_system(systems::spawn.system())
            .add_system(systems::game_over.system())
            .add_system(systems::movement_to_pixels.system())
            .add_system(systems::block_grid_position.system())
            .add_system(systems::bottom_blocked.system());
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Tetris!".to_string(),
            width: 800,
            height: 1200,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_resource(resources::Scoreboard {
            score: 0,
            game_over: false,
        })
        .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_resource(resources::ControlTimer(Timer::from_seconds(0.20, true)))
        .add_resource(resources::SpeedTimer(Timer::from_seconds(0.80, true)))
        .add_plugins(DefaultPlugins)
        .add_plugin(InitPlugin)
        .run();
}
