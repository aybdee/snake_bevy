use bevy::prelude::*;

#[derive(Component)]
struct Wall;

#[derive(Resource)]
struct Boundary {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    color: Color,
    thickness: f32,
}

#[derive(Component)]
struct Player {
    direction: Direction,
}

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player {
            direction: Direction::RIGHT,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(100.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(15., 15.)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        },
    ));

    create_boundary(
        &mut commands,
        Boundary {
            width: 500.0,
            height: 500.0,
            x: 0.0,
            y: 0.0,
            color: Color::srgb(0.0, 0.0, 0.0),
            thickness: 5.0,
        },
    )
}

fn create_boundary(commands: &mut Commands, boundary: Boundary) {
    //destructure boundary
    let Boundary {
        width,
        height,
        x,
        y,
        color,
        thickness,
    } = boundary;
    commands.insert_resource(boundary);

    let dimensions = [
        //size, position
        //left wall
        (
            Vec2::new(thickness, height),
            Vec3::new((-(width / 2 as f32)) + x, y, 0.0),
        ),
        //top wall
        (
            Vec2::new(width + thickness, thickness),
            Vec3::new(x, (height / 2 as f32) + y, 0.0),
        ),
        //right wall
        (
            Vec2::new(thickness, height),
            Vec3::new(((width / 2 as f32) + x), y, 0.0),
        ),
        //bottom wall
        (
            Vec2::new(width + thickness, thickness),
            Vec3::new(x, (-(height / 2 as f32)) + y, 0.0),
        ),
    ];

    for (size, position) in dimensions.iter() {
        commands.spawn((
            Wall,
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(*size),
                    ..Default::default()
                },
                transform: Transform::from_translation(*position),
                ..Default::default()
            },
        ));
    }
}

fn move_player(
    time: Res<Time>,
    boundary: Res<Boundary>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let offset = 15 as f32;
    for (mut player, mut transform) in query.iter_mut() {
        match player.direction {
            Direction::LEFT => {
                if transform.translation.x - offset <= -(boundary.width / 2.0) {
                    player.direction = Direction::DOWN;
                } else {
                    transform.translation.x -= time.delta_seconds() * 300.0;
                }
            }
            Direction::RIGHT => {
                if transform.translation.x + offset >= boundary.width / 2.0 {
                    player.direction = Direction::UP;
                } else {
                    transform.translation.x += time.delta_seconds() * 300.0;
                }
            }
            Direction::UP => {
                if transform.translation.y + offset >= boundary.height / 2.0 {
                    player.direction = Direction::LEFT;
                } else {
                    transform.translation.y += time.delta_seconds() * 300.0;
                }
            }
            Direction::DOWN => {
                if transform.translation.y - offset <= -(boundary.height / 2.0) {
                    player.direction = Direction::RIGHT;
                } else {
                    transform.translation.y -= time.delta_seconds() * 300.0;
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}
