use crate::bm::write_bmp_file;
use crate::color::Color;

pub struct Framebuffer {
    width: usize,
    height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
    line_color: u32,
    cells: Vec<bool>, // Nuevo campo para almacenar el estado de las células

}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let buffer = vec![0; width * height];
        let cells = vec![false; width * height]; // Inicializar las células como muertas
        Framebuffer {
            width,
            height,
            buffer,
            background_color: 0x000000, // Color de fondo predeterminado (negro)
            current_color: 0xFFFFFF, // Color de dibujo predeterminado (blanco)
            line_color: 0xFFFFFF, // Color de línea predeterminado (blanco)
            cells,
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, alive: bool) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = alive;
            let color = if alive { self.current_color } else { self.background_color };
            self.point(x, y, color);
        }
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x]
        } else {
            false
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    
    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
        for cell in self.cells.iter_mut() {
            *cell = false;
        }
    }


    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn set_line_color(&mut self, color: u32) {
        self.line_color = color;
    }
    
    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let inverted_y = self.height - 1 - y;
            self.buffer[inverted_y * self.width + x] = color;
        }
    }

    pub fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        let buffer: Vec<Color> = self.buffer.iter()
            .map(|&color_value| {
                let red = ((color_value >> 16) & 0xFF) as u8;
                let green = ((color_value >> 8) & 0xFF) as u8;
                let blue = (color_value & 0xFF) as u8;
                Color::new(red, green, blue)
            })
            .collect();
        write_bmp_file(file_path, &buffer, self.width, self.height)
    }
}
