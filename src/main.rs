use raylib::prelude::*;
use raylib::consts::KeyboardKey;

mod maze;

// This implements http://www.migapro.com/depth-first-search/

const MAZE_WIDTH: i32 = 20;
const MAZE_HEIGHT: i32 = 20;

fn main() {
    let (mut rl, ref thread) = raylib::init()
        .size(800, 600)
        .title("Maze using Depth-First Search")
        .vsync()
        .build();

    let mut maze = maze::Maze::new(MAZE_WIDTH, MAZE_HEIGHT);
    let w = rl.get_screen_width() / maze.width as i32;
    let h = rl.get_screen_height() / maze.height as i32;

    while !rl.window_should_close() {
        let mut fast_mode = false;

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            maze = maze::Maze::new(MAZE_WIDTH, MAZE_HEIGHT);
        }
        else if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            fast_mode = true;
        }
        else if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            break;
        }

        if !maze.completed {
            if fast_mode {
                while !maze.completed {
                    maze.generate_next();
                }
            } else {
                maze.generate_next();
            }
        }

        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);

        for cell in maze.iter() {
            if cell.visited {
                d.draw_rectangle(
                    cell.x * w,
                    cell.y * h,
                    w,
                    h,
                    Color::new(50, 50, 50, 255),
                );
            }
        }

        d.draw_rectangle(
            maze.active_position.0 * w,
            maze.active_position.1 * h,
            w,
            h,
            Color::RED,
        );

        d.draw_rectangle(
            (maze.width - 1) * w,
            (maze.height - 1) * h,
            w,
            h,
            Color::BLUE,
        );

        for (pos, is_wall) in maze.vertical_walls.iter() {
            if *is_wall {
                d.draw_line_ex(
                    Vector2::new(pos.0 as f32 * w as f32, pos.1 as f32 * h as f32),
                    Vector2::new(pos.0 as f32 * w as f32, pos.1 as f32 * h as f32 + h as f32),
                    3.0,
                    Color::GREEN,
                );
            }
        }

        for (pos, is_wall) in maze.horizontal_walls.iter() {
            if *is_wall {
                d.draw_line_ex(
                    Vector2::new(pos.0 as f32 * w as f32, pos.1 as f32 * h as f32),
                    Vector2::new(pos.0 as f32 * w as f32 + w as f32, pos.1 as f32 * h as f32),
                    3.0,
                    Color::GREEN,
                );
            }
        }

        if !maze.completed {
            d.draw_text(
                "Generating...",
                10,
                10,
                20,
                Color::WHITE,
            );

            d.draw_text(
                "Press ENTER to skip",
                10,
                30,
                5,
                Color::WHITE,
            );
        } else {
            d.draw_text(
                "Completed!",
                10,
                10,
                20,
                Color::WHITE,
            );

            d.draw_text(
                "Press Space to regenerate",
                10,
                30,
                5,
                Color::WHITE,
            );
        }
    }
}