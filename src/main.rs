use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
struct FieldSize {
    size: i32,
}

#[derive(Component)]
struct Food {
    position: Position,
}

struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Snake {
    position: Vec<Position>,
    length: i32,
    direction: KeyCode,
}

fn initialize_snake(mut commands: Commands) {
    commands.spawn(Snake {
        position: vec![Position { x: 0, y: 0 }],
        length: 1,
        direction: KeyCode::Comma,
    });
}

fn initialize_food(mut commands: Commands, field: Res<FieldSize>) {
    let mut rng = rand::rng();
    let half_size = field.size / 2;
    loop {
        let random_x = rng.random_range(-half_size..half_size + 1);
        let random_y = rng.random_range(-half_size..half_size + 1);
        if random_x != 0 || random_y != 0 {
            commands.spawn(Food {
                position: Position {
                    x: random_x,
                    y: random_y,
                },
            });
            break;
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    field: Res<FieldSize>,
    window: Query<&Window>,
) {
    let window = window.single();
    let font_size = window.resolution.height() / field.size as f32;
    // Spawn an entity with the new text-related components
    commands.spawn((
        // 1) This node defines UI layout, anchoring, sizing, etc.
        Node {
            margin: UiRect::all(Val::Auto),
            ..Default::default()
        },
        Text("".to_string()),
        // 3) Layout options for how text is wrapped / aligned
        TextLayout::new_with_justify(JustifyText::Center),
        // 4) The font to use
        TextFont {
            // Load a font from your assets folder
            font: asset_server.load("LiberationMono-Regular.ttf"),
            font_size,
            font_smoothing: bevy::text::FontSmoothing::None,
        },
        // 5) Text color
        TextColor(Color::WHITE),
        // 6) Background color if you want the Node to have a background
        //    (optional)
        BackgroundColor(Color::NONE),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(FieldSize { size: 30 })
        .add_systems(
            Startup,
            (initialize_snake, initialize_food, setup_ui, setup_camera),
        )
        .add_systems(Update, snake_movement_system)
        .add_systems(Update, render_system)
        .run();
}

fn snake_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Snake>,
    field: Res<FieldSize>,
) {
    if let Ok(mut snake) = query.get_single_mut() {
        // Move the snake based on keyboard input
        if keyboard_input.just_pressed(KeyCode::KeyW) {
            let legitimacy = direction_check(snake.direction, KeyCode::KeyW);
            println!("{:?}", legitimacy);
            println!("Moving up!");
            if legitimacy {
                snake.direction = KeyCode::KeyW
            };
        }
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            let legitimacy = direction_check(snake.direction, KeyCode::KeyS);
            println!("{:?}", legitimacy);
            println!("Moving down!");
            if legitimacy {
                snake.direction = KeyCode::KeyS
            };
        }
        if keyboard_input.just_pressed(KeyCode::KeyA) {
            let legitimacy = direction_check(snake.direction, KeyCode::KeyA);
            println!("{:?}", legitimacy);
            println!("Moving left!");
            if legitimacy {
                snake.direction = KeyCode::KeyA
            };
        }
        if keyboard_input.just_pressed(KeyCode::KeyD) {
            let legitimacy = direction_check(snake.direction, KeyCode::KeyD);
            println!("{:?}", legitimacy);
            println!("Moving right!");
            if legitimacy {
                snake.direction = KeyCode::KeyD
            };
        }

        let head = &snake.position[0];
        let new_head = match snake.direction {
            KeyCode::KeyW => Position {
                x: head.x,
                y: head.y + 1,
            },
            KeyCode::KeyS => Position {
                x: head.x,
                y: head.y - 1,
            },
            KeyCode::KeyA => Position {
                x: head.x - 1,
                y: head.y,
            },
            KeyCode::KeyD => Position {
                x: head.x + 1,
                y: head.y,
            },
            _ => Position {
                x: head.x,
                y: head.y,
            },
        };
        std::thread::sleep(std::time::Duration::from_millis(250));

        snake.position.insert(0, new_head);
        if snake.position.len() > snake.length as usize {
            snake.position.pop();
        }
    }
}

fn direction_check(previous_direction: KeyCode, new_direction: KeyCode) -> bool {
    match (previous_direction, new_direction) {
        (KeyCode::KeyW, KeyCode::KeyS) => false,
        (KeyCode::KeyS, KeyCode::KeyW) => false,
        (KeyCode::KeyA, KeyCode::KeyD) => false,
        (KeyCode::KeyD, KeyCode::KeyA) => false,
        _ => true,
    }
}

fn render_system(
    snake_query: Query<&Snake>,
    food_query: Query<&Food>,
    field: Res<FieldSize>,
    mut text_query: Query<&mut Text>,
) {
    let size = field.size as usize;
    // We'll make a 2D array for the ASCII: ' ' for empty,
    // '~' for snake, '*' for food
    let mut grid = vec![vec![' '; size]; size];

    // Mark snake positions
    if let Ok(snake) = snake_query.get_single() {
        for pos in &snake.position {
            let half = (field.size / 2) as i32;
            let x = (pos.x + half) as usize;
            let y = (pos.y + half) as usize;
            // Make sure it's in bounds
            if x < size && y < size {
                grid[y][x] = '~';
            }
        }
    }

    // Mark food
    for food in food_query.iter() {
        let half = (field.size / 2) as i32;
        let x = (food.position.x + half) as usize;
        let y = (food.position.y + half) as usize;
        if x < size && y < size {
            grid[y][x] = '*';
        }
    }

    // Build a string (top row to bottom row)
    let mut grid_string = String::new();
    for row in grid.iter().rev() {
        for cell in row {
            grid_string.push(*cell);
        }
        grid_string.push('\n');
    }

    // Update the text
    if let Ok(mut text) = text_query.get_single_mut() {
        text.0 = grid_string;
    }
}
