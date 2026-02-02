use colored::{Color, Colorize};

pub struct BrailleCanvas {
    pub width: usize,
    pub height: usize,
    // Matriz de máscaras de bits para la forma
    grid: Vec<Vec<u8>>,
    // Matriz paralela para almacenar el color de cada celda de carácter
    color_grid: Vec<Vec<Option<Color>>>,
    // Capa de texto
    text_layer: Vec<Vec<Option<char>>>,
}

impl BrailleCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![0u8; width]; height],
            // Inicialmente sin color (None se renderizará como blanco/por defecto)
            color_grid: vec![vec![None; width]; height],
            text_layer: vec![vec![None; width]; height],
        }
    }

    pub fn clear(&mut self) {
        for row in self.grid.iter_mut() { row.fill(0); }
        for row in self.color_grid.iter_mut() { row.fill(None); }
        for row in self.text_layer.iter_mut() { row.fill(None); }
    }

    /// Enciende un píxel virtual. Acepta un color opcional.
    pub fn set_pixel(&mut self, px: usize, py: usize, color: Option<Color>) {
        let pixel_width = self.width * 2;
        let pixel_height = self.height * 4;

        if px >= pixel_width || py >= pixel_height { return; }

        // Coordenadas de la celda de carácter
        let col_char = px / 2;
        // Invertir Y para sistema cartesiano (0,0 abajo-izquierda)
        let row_char = (pixel_height - 1 - py) / 4;

        if row_char >= self.height || col_char >= self.width { return; }

        // Coordenadas dentro del carácter Braille
        let dx = px % 2;
        let dy = (pixel_height - 1 - py) % 4;

        // Máscaras Unicode Braille estándar
        let mask = match (dx, dy) {
            (0, 0) => 0x01, (1, 0) => 0x08,
            (0, 1) => 0x02, (1, 1) => 0x10,
            (0, 2) => 0x04, (1, 2) => 0x20,
            (0, 3) => 0x40, (1, 3) => 0x80,
            _ => 0,
        };

        self.grid[row_char][col_char] |= mask;
        
        // Si se especifica un color, actualizamos el color de toda la celda
        if let Some(c) = color {
            self.color_grid[row_char][col_char] = Some(c);
        }
    }

    /// Dibuja una línea (Bresenham) con color opcional
    pub fn line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, color: Option<Color>) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        loop {
            if x >= 0 && y >= 0 {
                self.set_pixel(x as usize, y as usize, color);
            }
            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x += sx; }
            if e2 <= dx { err += dx; y += sy; }
        }
    }

    /// Dibuja un círculo (Algoritmo de punto medio)
    pub fn circle(&mut self, xc: isize, yc: isize, r: isize, color: Option<Color>) {
        let mut x = 0;
        let mut y = r;
        let mut d = 3 - 2 * r;

        // Función auxiliar para dibujar los 8 octantes simétricos
        let mut draw_octants = |cx: isize, cy: isize, x: isize, y: isize| {
            let points = [
                (cx + x, cy + y), (cx - x, cy + y), (cx + x, cy - y), (cx - x, cy - y),
                (cx + y, cy + x), (cx - y, cy + x), (cx + y, cy - x), (cx - y, cy - x)
            ];
            for (px, py) in points {
                if px >= 0 && py >= 0 {
                     self.set_pixel(px as usize, py as usize, color);
                }
            }
        };

        draw_octants(xc, yc, x, y);
        while y >= x {
            x += 1;
            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            } else {
                d = d + 4 * x + 6;
            }
            draw_octants(xc, yc, x, y);
        }
    }

    pub fn set_char(&mut self, char_x: usize, char_y: usize, c: char, color: Option<Color>) {
        // Invertimos Y como siempre
        let row = self.height.saturating_sub(1).saturating_sub(char_y);

        if row < self.height && char_x < self.width {
            self.text_layer[row][char_x] = Some(c);
            if let Some(col) = color {
                self.color_grid[row][char_x] = Some(col);
            }
        }
    }

    pub fn set_char_vertical(&mut self, char_x: usize, char_y_start: usize, text: &str, color: Option<Color>) {
        for (i, ch) in text.chars().enumerate() {
            let y = char_y_start.saturating_sub(i);
            self.set_char(char_x, y, ch, color); // Reutiliza lógica de inversión de Y
            }   
    }   
    pub fn render(&self) -> String {
        self.render_with_options(true, None)
    }

    /// RENDER NO COLOR
    pub fn render_no_color(&self) -> String {
        let mut output = String::new();
        for row_masks in &self.grid {
            for &mask in row_masks {
                let ch = std::char::from_u32(0x2800 + mask as u32).unwrap_or(' ');
                output.push(ch);
            }
            output.push('\n');
        }
        output
    }

    /// Renderiza el canvas combinando forma y color con opciones extendidas
    pub fn render_with_options(&self, show_border: bool, title: Option<&str>) -> String {
        let mut output = String::new();
        
        // Título opcional centrado
        if let Some(t) = title {
            output.push_str(&format!("{:^width$}\n", t, width = self.width + 2));
        }

        // Borde superior opcional
        if show_border {
            output.push_str(&format!("┌{}┐\n", "─".repeat(self.width)));
        }

        for (r_idx, row_masks) in self.grid.iter().enumerate() {
            if show_border { output.push('│'); }
            
            for (c_idx, &mask) in row_masks.iter().enumerate() {
                let char_to_print = if let Some(c) = self.text_layer[r_idx][c_idx] {
                    c.to_string()
                } else {
                    std::char::from_u32(0x2800 + mask as u32).unwrap_or(' ').to_string()
                };

                if let Some(color) = self.color_grid[r_idx][c_idx] {
                    output.push_str(&char_to_print.color(color).to_string());
                } else {
                    output.push_str(&char_to_print);
                }
            }
            
            if show_border { output.push('│'); }
            output.push('\n');
        }

        // Borde inferior opcional
        if show_border {
            output.push_str(&format!("└{}┘", "─".repeat(self.width)));
        }
        
        output
    }
}
