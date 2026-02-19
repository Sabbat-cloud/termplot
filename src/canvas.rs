use colored::{Color, Colorize};
use std::fmt::Write;

pub struct BrailleCanvas {
    pub width: usize,
    pub height: usize,
    // Buffer plano: width * height. Mucho más rápido que Vec<Vec<u8>>
    buffer: Vec<u8>,
    // Buffer plano de colores.
    colors: Vec<Option<Color>>,
    // Capa de texto plana
    text_layer: Vec<Option<char>>,
    // Buffer interno reutilizable (opcional, aquí creamos uno nuevo por frame para thread-safety simple)
}

impl BrailleCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            buffer: vec![0u8; size],
            colors: vec![None; size],
            text_layer: vec![None; size],
        }
    }

    /// Ancho en "sub-píxeles" (puntos braille)
    #[inline]
    pub fn pixel_width(&self) -> usize {
        self.width * 2
    }

    /// Alto en "sub-píxeles" (puntos braille)
    #[inline]
    pub fn pixel_height(&self) -> usize {
        self.height * 4
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
        self.colors.fill(None);
        self.text_layer.fill(None);
    }

    // --- Helpers de Coordenadas ---

    #[inline]
    fn idx(&self, col: usize, row: usize) -> usize {
        row * self.width + col
    }

    fn set_pixel_impl(&mut self, px: usize, py: usize, color: Option<Color>) {
        if px >= self.pixel_width() || py >= self.pixel_height() {
            return;
        }

        let col = px / 2;
        let row = py / 4;

        let sub_x = px % 2;
        let sub_y = py % 4;

        let mask = match (sub_x, sub_y) {
            (0, 0) => 0x01, (1, 0) => 0x08,
            (0, 1) => 0x02, (1, 1) => 0x10,
            (0, 2) => 0x04, (1, 2) => 0x20,
            (0, 3) => 0x40, (1, 3) => 0x80,
            _ => 0,
        };

        let index = self.idx(col, row);
        self.buffer[index] |= mask;

        if let Some(c) = color {
            self.colors[index] = Some(c);
        }
    }

    // --- API Pública de Dibujo ---

    /// Coordenadas CARTESIANAS: (0,0) abajo-izquierda
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Option<Color>) {
        let inverted_y = self.pixel_height().saturating_sub(1).saturating_sub(y);
        self.set_pixel_impl(x, inverted_y, color);
    }

    /// Coordenadas DE PANTALLA: (0,0) arriba-izquierda
    pub fn set_pixel_screen(&mut self, x: usize, y: usize, color: Option<Color>) {
        self.set_pixel_impl(x, y, color);
    }

    /// Línea Cartesiana
    pub fn line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, color: Option<Color>) {
        self.bresenham(x0, y0, x1, y1, color, true);
    }

    /// Línea de Pantalla
    pub fn line_screen(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, color: Option<Color>) {
        self.bresenham(x0, y0, x1, y1, color, false);
    }

    fn bresenham(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, color: Option<Color>, cartesian: bool) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        loop {
            if x >= 0 && y >= 0 {
                if cartesian {
                    self.set_pixel(x as usize, y as usize, color);
                } else {
                    self.set_pixel_screen(x as usize, y as usize, color);
                }
            }
            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x += sx; }
            if e2 <= dx { err += dx; y += sy; }
        }
    }
    
    pub fn circle(&mut self, xc: isize, yc: isize, r: isize, color: Option<Color>) {
        let mut x = 0;
        let mut y = r;
        let mut d = 3 - 2 * r;

        let mut draw_octants = |cx: isize, cy: isize, x: isize, y: isize| {
            let points = [
                (cx + x, cy + y), (cx - x, cy + y), (cx + x, cy - y), (cx - x, cy - y),
                (cx + y, cy + x), (cx - y, cy + x), (cx + y, cy - x), (cx - y, cy - x),
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

    pub fn set_char(&mut self, col: usize, row: usize, c: char, color: Option<Color>) {
        // Mantenemos la lógica invertida para compatibilidad con charts actuales
        let inverted_row = self.height.saturating_sub(1).saturating_sub(row);
        
        if col < self.width && inverted_row < self.height {
            let idx = self.idx(col, inverted_row);
            self.text_layer[idx] = Some(c);
            if let Some(col_val) = color {
                self.colors[idx] = Some(col_val);
            }
        }
    }
    
    // --- Renderizado Optimizado ---

    pub fn render(&self) -> String {
        self.render_with_options(true, None)
    }
    
    pub fn render_no_color(&self) -> String {
        let mut output = String::with_capacity(self.width * self.height + self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.idx(col, row);
                let mask = self.buffer[idx];
                let ch = std::char::from_u32(0x2800 + mask as u32).unwrap_or(' ');
                output.push(ch);
            }
            output.push('\n');
        }
        output
    }

    pub fn render_with_options(&self, show_border: bool, title: Option<&str>) -> String {
        let mut out = String::with_capacity(self.width * self.height * 15);

        if let Some(t) = title {
            let _ = writeln!(out, "{:^width$}", t, width = self.width + 2);
        }

        if show_border {
            out.push('┌');
            for _ in 0..self.width { out.push('─'); }
            out.push('┐');
            out.push('\n');
        }

        let mut last_color: Option<Color> = None;

        for row in 0..self.height {
            if show_border { out.push('│'); }

            for col in 0..self.width {
                let idx = self.idx(col, row);
                
                let char_to_print = if let Some(c) = self.text_layer[idx] {
                    c
                } else {
                    let mask = self.buffer[idx];
                    std::char::from_u32(0x2800 + mask as u32).unwrap_or(' ')
                };

                let current_color = self.colors[idx];

                // Optimización: Solo cambiar el código ANSI si el color es diferente al anterior
                if current_color != last_color {
                    match current_color {
                        Some(c) => {
                            // TRUCO: Creamos un string vacío coloreado "\x1b[31m\x1b[0m"
                            // y reemplazamos el reset final para obtener solo el prefijo "\x1b[31m".
                            // Esto funciona para cualquier Color (Standard o TrueColor).
                            let ansi_full = format!("{}", "".color(c));
                            let ansi_prefix = ansi_full.replace("\x1b[0m", "");
                            out.push_str(&ansi_prefix);
                        },
                        None => {
                             out.push_str("\x1b[0m"); 
                        }
                    }
                    last_color = current_color;
                }

                out.push(char_to_print);
            }
            
            // Reset al final de línea para seguridad
            if last_color.is_some() {
                out.push_str("\x1b[0m");
                last_color = None;
            }

            if show_border { out.push('│'); }
            out.push('\n');
        }

        if show_border {
            out.push('└');
            for _ in 0..self.width { out.push('─'); }
            out.push('┘');
        }

        out
    }
}
// --- TESTS UNITARIOS ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_mapping() {
        let canvas = BrailleCanvas::new(10, 5);
        // La celda (0,0) es el índice 0
        assert_eq!(canvas.idx(0, 0), 0);
        // La celda (0,1) es el índice ancho (10)
        assert_eq!(canvas.idx(0, 1), 10);
    }

    #[test]
    fn test_braille_bitmask() {
        // Creamos un canvas de 1x1 caracteres (2x4 píxeles)
        let mut canvas = BrailleCanvas::new(1, 1);

        // Píxel superior izquierda (0,0) -> Máscara 0x01
        canvas.set_pixel_screen(0, 0, None);
        assert_eq!(canvas.buffer[0], 0x01);

        // Píxel a su derecha (1,0) -> Máscara 0x08
        canvas.set_pixel_screen(1, 0, None);
        assert_eq!(canvas.buffer[0], 0x01 | 0x08); // 0x09

        // Píxel abajo del primero (0,1) -> Máscara 0x02
        canvas.set_pixel_screen(0, 1, None);
        assert_eq!(canvas.buffer[0], 0x09 | 0x02); // 0x0B
    }

    #[test]
    fn test_clear() {
        let mut canvas = BrailleCanvas::new(2, 2);
        canvas.set_pixel(0, 0, None);
        assert!(canvas.buffer.iter().any(|&x| x != 0));

        canvas.clear();
        assert!(canvas.buffer.iter().all(|&x| x == 0));
        assert!(canvas.colors.iter().all(|x| x.is_none()));
    }
}
