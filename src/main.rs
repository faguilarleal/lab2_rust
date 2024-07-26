use minifb::{Key, Window, WindowOptions};
mod framebuffer;
mod bm;
mod color;

use std::time::Duration;
use framebuffer::Framebuffer;

fn render(framebuffer: &mut Framebuffer) {
    framebuffer.set_background_color(0x333355);
    framebuffer.clear();
    framebuffer.set_current_color(0x00FF1F);
    framebuffer.point(20, 40, 0xFB00FF);
}

fn create_glider(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_cell(x + 1, y, true);
    framebuffer.set_cell(x + 2, y + 1, true);
    framebuffer.set_cell(x, y + 2, true);
    framebuffer.set_cell(x + 1, y + 2, true);
    framebuffer.set_cell(x + 2, y + 2, true);
}

fn create_blinker(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_cell(x, y, true);
    framebuffer.set_cell(x + 1, y, true);
    framebuffer.set_cell(x + 2, y, true);
}

fn create_pulsar(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let pulsar_offsets = [
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),

        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12)
    ];

    for (dx, dy) in pulsar_offsets.iter() {
        framebuffer.set_cell(x + dx, y + dy, true);
    }
}

fn create_pentadecathlon(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let offsets = [
        (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0),
        (2, x-1), (5,x- 1),
        (2, 1), (5, 1)
    ];

    for (dx, dy) in offsets.iter() {
        framebuffer.set_cell(x + dx, y + dy, true);
    }
}

fn next_generation(framebuffer: &mut Framebuffer) {
    let width = framebuffer.width();
    let height = framebuffer.height();
    let mut new_cells = vec![false; width * height];

    for y in 0..height {
        for x in 0..width {
            let mut live_neighbors = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx != 0 || dy != 0 {
                        let nx = (x as isize + dx).rem_euclid(width as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(height as isize) as usize;

                        if framebuffer.is_alive(nx, ny) {
                            live_neighbors += 1;
                        }
                    }
                }
            }

            let cell_alive = framebuffer.is_alive(x, y);
            new_cells[y * width + x] = match (cell_alive, live_neighbors) {
                (true, 2) | (true, 3) => true,
                (true, _) => false,
                (false, 3) => true,
                (false, _) => false,
            };
        }
    }

    for y in 0..height {
        for x in 0..width {
            framebuffer.set_cell(x, y, new_cells[y * width + x]);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 80;
    let framebuffer_height = 60;

    let frame_delay = Duration::from_millis(50);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    render(&mut framebuffer);

    create_pulsar(&mut framebuffer, 55, 10);
    create_pulsar(&mut framebuffer, 15, 10);
    

    create_blinker(&mut framebuffer, 5, 5);
    create_blinker(&mut framebuffer, 75, 5);
    create_blinker(&mut framebuffer, 5, 55);
    create_blinker(&mut framebuffer, 75, 55);
    create_blinker(&mut framebuffer, 65, 45);
    create_blinker(&mut framebuffer, 25, 55);


    create_glider(&mut framebuffer, 55,10);
    create_glider(&mut framebuffer, 15,10);
    create_glider(&mut framebuffer, 55,40);
    create_glider(&mut framebuffer, 15,40);

    let x = framebuffer_width / 2 - 4;
    let y = framebuffer_height / 2 - 1;
    create_pentadecathlon(&mut framebuffer, x, y);

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        next_generation(&mut framebuffer);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
