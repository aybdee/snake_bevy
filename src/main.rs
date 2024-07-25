use bevy::{
    color::palettes::css::{BLACK, GREEN, RED, YELLOW},
    input::keyboard::KeyboardInput,
    math::NormedVectorSpace,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::Rng;

use bevy_prototype_lyon::prelude::*;

const PLAYER_SIZE: f32 = 10.0;
const DEFAULT_DIRECTION: Direction = Direction::LEFT;

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
struct Object;

#[derive(Component)]
struct Player {
    direction: Direction,
    size: usize,
}

#[derive(Component)]
struct Food;

#[derive(Component)]
struct PlayerBody {
    direction: Direction,
    index: usize,
}

#[derive(Component)]
struct Score;

#[derive(Copy, Clone)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

fn generate_random_vec2(x_bounds: (f32, f32), y_bounds: (f32, f32)) -> Vec2 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(x_bounds.0..=x_bounds.1);
    let y = rng.gen_range(y_bounds.0..=y_bounds.1);
    Vec2::new(x, y)
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    b: Res<Boundary>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            format!("Score\n {}", 0),
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 30.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            right: Val::Px(100.0),
            ..default()
        }),
        Score,
    ));

    //draw boundary
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::new(b.width, b.height),
                ..shapes::Rectangle::default()
            }),
            spatial: SpatialBundle {
                transform: Transform::from_translation(Vec3::new(b.x, b.y, 0.0)),
                ..default()
            },
            ..default()
        },
        Stroke::new(b.color, b.thickness),
    ));

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle {
                radius: 10.0,
                ..default()
            }),
            spatial: SpatialBundle {
                transform: Transform::from_translation(Vec3::new(100.0, 100.0, 0.0)),
                ..default()
            },
            ..default()
        },
        Fill::color(RED),
        Food,
    ));

    commands.spawn((
        Object,
        Player {
            direction: DEFAULT_DIRECTION,
            size: 1,
        },
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::splat(PLAYER_SIZE),
                ..shapes::Rectangle::default()
            }),
            spatial: SpatialBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..default()
            },
            ..default()
        },
        Fill::color(YELLOW),
    ));

    commands.spawn((
        PlayerBody {
            direction: DEFAULT_DIRECTION,
            index: 0,
        },
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::splat(PLAYER_SIZE),
                ..shapes::Rectangle::default()
            }),
            spatial: SpatialBundle {
                transform: Transform::from_translation(Vec3::new(-PLAYER_SIZE, 0.0, 1.0)),
                ..default()
            },
            ..default()
        },
        Fill::color(GREEN),
    ));
}

fn update_score(player_query: Query<&Player>, mut score_query: Query<&mut Text, With<Score>>) {
    let player = player_query.iter().next().unwrap();
    let mut score_text = score_query.iter_mut().next().unwrap();
    score_text.sections[0].value = format!("Score \n {}", player.size - 1)
}

fn animate_food(time: Res<Time>, mut query: Query<&mut Path, With<Food>>) {
    let mut path = query.iter_mut().next().unwrap();
    let new_radius = 5.0 + (time.elapsed_seconds() * 5.0).sin() * 2.5;
    *path = GeometryBuilder::build_as(&shapes::Circle {
        radius: new_radius,
        ..default()
    });
}

fn handle_keyboard(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Player>) {
    for mut player in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            player.direction = Direction::UP;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            player.direction = Direction::DOWN;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            player.direction = Direction::LEFT;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            player.direction = Direction::RIGHT;
        }
    }
}

fn spawn_body(commands: &mut Commands, x: f32, y: f32, index: usize) {
    commands.spawn((
        PlayerBody {
            direction: DEFAULT_DIRECTION,
            index,
        },
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::splat(PLAYER_SIZE),
                ..shapes::Rectangle::default()
            }),
            spatial: SpatialBundle {
                transform: Transform::from_translation(Vec3::new(x, y, 1.0)),
                ..default()
            },
            ..default()
        },
        Fill::color(GREEN),
    ));
}

fn move_player(
    time: Res<Time>,
    mut commands: Commands,
    boundary: Res<Boundary>,
    mut player_query: Query<(&mut Player, &mut Transform), Without<PlayerBody>>,
    mut player_body_query: Query<(&mut PlayerBody, &mut Transform)>,
) {
    let offset = 10.0;
    let (mut player, mut transform) = player_query.iter_mut().next().unwrap();
    let mut last_position = transform.translation;
    let mut last_direction = player.direction;
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

    //handle body movement
    let segments: Vec<_> = player_body_query.iter_mut().collect();

    if segments.len() != player.size {
        let (last_segment, transform) = segments.last().unwrap();

        match last_segment.direction {
            Direction::LEFT => spawn_body(
                &mut commands,
                transform.translation.x + 1.0,
                transform.translation.y,
                last_segment.index + 1,
            ),

            Direction::RIGHT => spawn_body(
                &mut commands,
                transform.translation.x - 1.0,
                transform.translation.y,
                last_segment.index + 1,
            ),

            Direction::UP => spawn_body(
                &mut commands,
                transform.translation.x,
                transform.translation.y - 1.0,
                last_segment.index + 1,
            ),

            Direction::DOWN => spawn_body(
                &mut commands,
                transform.translation.x,
                transform.translation.y,
                last_segment.index + 1,
            ),
        }
    }

    for (mut segment, mut transform) in segments {
        std::mem::swap(&mut transform.translation, &mut last_position);
        std::mem::swap(&mut segment.direction, &mut last_direction);
    }
}

fn eat_food(
    boundary: Res<Boundary>,
    mut player_query: Query<(&mut Player, &mut Transform), Without<Food>>,
    mut food_query: Query<(&mut Food, &mut Transform)>,
) {
    let (mut player, player_transform) = player_query.iter_mut().next().unwrap();
    let (food, mut food_transform) = food_query.iter_mut().next().unwrap();

    let food_distance = player_transform
        .translation
        .xy()
        .distance(food_transform.translation.xy());
    if food_distance < 10.0 {
        let new_position = generate_random_vec2(
            (
                -(boundary.width / 2.0) + boundary.x,
                (boundary.width / 2.0) + boundary.x,
            ),
            (
                -(boundary.height / 2.0) + boundary.y,
                (boundary.height / 2.0) + boundary.y,
            ),
        );

        food_transform.translation = Vec3::new(new_position.x, new_position.y, 0.0);
        player.size += 1;
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .insert_resource(Boundary {
            width: 400.0,
            height: 400.0,
            x: 0.0,
            y: 0.0,
            color: Color::Srgba(BLACK),
            thickness: 5.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_food, eat_food, update_score))
        .add_systems(Update, (handle_keyboard, move_player).chain())
        .run();
}
