use raylib::prelude::*;
use raylib::consts::KeyboardKey;

mod maze;

// This implements http://www.migapro.com/depth-first-search/

fn main() {
    let (mut rl, ref thread) = raylib::init()
        .size(750, 650)
        .title("Maze using Depth-First Search")
        .build();

    let mut maze = maze::Maze::new(71, 71);
    let w = rl.get_screen_width() / maze.width as i32;
    let h = rl.get_screen_height() / maze.height as i32;

    while !rl.window_should_close() {        
        maze.generate_next();

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            maze = maze::Maze::new(71, 71);
        }

        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);

        for cell in maze.iter() {
            d.draw_rectangle(
                cell.x * w,
                cell.y * h,
                w,
                h,
                match cell.cell_type {
                    maze::CellType::Wall => Color::BLACK,
                    maze::CellType::Path => Color::GREEN,
                },
            );
        }

        d.draw_rectangle(
            maze.active_position.0 * w,
            maze.active_position.1 * h,
            w,
            h,
            Color::BLUE,
        );
    }
}