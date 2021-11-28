use raylib::prelude::*;
use raylib::consts::KeyboardKey;

mod maze;

// This implements http://www.migapro.com/depth-first-search/

fn main() {
    let (mut rl, ref thread) = raylib::init()
        .size(800, 600)
        .title("Maze using Depth-First Search")
        .build();

    let mut maze = maze::Maze::new(20, 20);
    let w = rl.get_screen_width() / maze.width as i32;
    let h = rl.get_screen_height() / maze.height as i32;

    while !rl.window_should_close() {    
        maze.generate_next();

        if maze.active_position == (0, 0) {
            rl.set_target_fps(10);
        }
        else {
            rl.set_target_fps(500);
        }
        
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            maze = maze::Maze::new(20, 20);
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
                    Color::WHITE,
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
                    1.0,
                    Color::BLACK,
                );
            }
        }

        for (pos, is_wall) in maze.horizontal_walls.iter() {
            if *is_wall {
                d.draw_line_ex(
                    Vector2::new(pos.0 as f32 * w as f32, pos.1 as f32 * h as f32),
                    Vector2::new(pos.0 as f32 * w as f32 + w as f32, pos.1 as f32 * h as f32),
                    1.0,
                    Color::BLACK,
                );
            }
        }
    }
}