use minifb::{Key, Window, WindowOptions};
mod framebuffer;
mod bm;
mod color;

use std::time::Duration;
use framebuffer::Framebuffer;

fn render(framebuffer: &mut Framebuffer) {
    // Clear the framebuffer
    framebuffer.set_background_color(0x333355);
    framebuffer.clear();

    // Draw some points
    framebuffer.set_current_color(0xFFDDDD);
    framebuffer.point(20, 40, 0xFFDDDD);
}

fn create_initial_pattern(width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; width]; height];

    // Ejemplo: patrón Glider
    grid[1][2] = true;
    grid[2][3] = true;
    grid[3][1] = true;
    grid[3][2] = true;
    grid[3][3] = true;

    grid
}

fn next_generation(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let width = grid[0].len();
    let height = grid.len();
    let mut new_grid = vec![vec![false; width]; height];

    for y in 0..height {
        for x in 0..width {
            let mut live_neighbors = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx != 0 || dy != 0 {
                        let nx = (x as isize + dx).rem_euclid(width as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(height as isize) as usize;

                        if grid[ny][nx] {
                            live_neighbors += 1;
                        }
                    }
                }
            }

            new_grid[y][x] = match (grid[y][x], live_neighbors) {
                (true, 2) | (true, 3) => true,
                (true, _) => false,
                (false, 3) => true,
                (false, _) => false,
            };
        }
    }

    new_grid
}
fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 80;
    let framebuffer_height = 60;

    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut grid = create_initial_pattern(framebuffer_width, framebuffer_height);

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        framebuffer.set_background_color(0x000000);
        framebuffer.clear();

        for y in 0..framebuffer_height {
            for x in 0..framebuffer_width {
                framebuffer.set_cell(x, y, grid[y][x]);
            }
        }

        grid = next_generation(&grid);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
