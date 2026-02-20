use crate::canvas::BrailleCanvas;
use colored::Color;
use std::f64::consts::PI;

pub struct ChartContext {
    pub canvas: BrailleCanvas,
}

impl ChartContext {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            canvas: BrailleCanvas::new(width, height),
        }
    }

    pub fn get_auto_range(points: &[(f64, f64)], padding: f64) -> ((f64, f64), (f64, f64)) {
        let valid_points: Vec<&(f64, f64)> = points
            .iter()
            .filter(|(x, y)| x.is_finite() && y.is_finite())
            .collect();

        if valid_points.is_empty() {
            return ((0.0, 1.0), (0.0, 1.0));
        }

        let (min_x, max_x) = valid_points.iter().fold(
            (f64::INFINITY, f64::NEG_INFINITY),
            |(min, max), p| (min.min(p.0), max.max(p.0)),
        );

        let (min_y, max_y) = valid_points.iter().fold(
            (f64::INFINITY, f64::NEG_INFINITY),
            |(min, max), p| (min.min(p.1), max.max(p.1)),
        );

        let rx = if (max_x - min_x).abs() < 1e-9 { 1.0 } else { max_x - min_x };
        let ry = if (max_y - min_y).abs() < 1e-9 { 1.0 } else { max_y - min_y };

        (
            (min_x - rx * padding, max_x + rx * padding),
            (min_y - ry * padding, max_y + ry * padding),
        )
    }

    fn map_coords(&self, x: f64, y: f64, x_range: (f64, f64), y_range: (f64, f64)) -> (isize, isize) {
        let (min_x, max_x) = x_range;
        let (min_y, max_y) = y_range;
        let width = self.canvas.pixel_width() as f64;
        let height = self.canvas.pixel_height() as f64;
        let range_x = (max_x - min_x).max(1e-9);
        let range_y = (max_y - min_y).max(1e-9);

        let px = ((x - min_x) / range_x * (width - 1.0)).round();
        let py = ((y - min_y) / range_y * (height - 1.0)).round();

        (px as isize, py as isize)
    }

    // --- GRÁFICOS ---

    pub fn scatter(&mut self, points: &[(f64, f64)], color: Option<Color>) {
        if points.is_empty() { return; }
        let (x_range, y_range) = Self::get_auto_range(points, 0.05);
        let w_px = self.canvas.pixel_width();
        let h_px = self.canvas.pixel_height();

        for &(x, y) in points {
            if !x.is_finite() || !y.is_finite() { continue; }
            let (px, py) = self.map_coords(x, y, x_range, y_range);
            if px >= 0 && py >= 0 && (px as usize) < w_px && (py as usize) < h_px {
                self.canvas.set_pixel(px as usize, py as usize, color);
            }
        }
    }

    pub fn line_chart(&mut self, points: &[(f64, f64)], color: Option<Color>) {
        if points.len() < 2 { return; }
        let (x_range, y_range) = Self::get_auto_range(points, 0.05);

        for window in points.windows(2) {
            let (x0, y0) = window[0];
            let (x1, y1) = window[1];
            if !x0.is_finite() || !y0.is_finite() || !x1.is_finite() || !y1.is_finite() { continue; }

            let p0 = self.map_coords(x0, y0, x_range, y_range);
            let p1 = self.map_coords(x1, y1, x_range, y_range);
            self.canvas.line(p0.0, p0.1, p1.0, p1.1, color);
        }
    }

    pub fn bar_chart(&mut self, values: &[(f64, Option<Color>)]) {
        if values.is_empty() { return; }
        let max_val = values.iter()
            .filter_map(|(v, _)| if v.is_finite() { Some(*v) } else { None })
            .fold(0.0f64, f64::max);

        if max_val <= 1e-9 { return; }

        let w_px = self.canvas.pixel_width();
        let h_px = self.canvas.pixel_height();
        let bar_width = (w_px / values.len()).max(1);

        for (i, &(val, color)) in values.iter().enumerate() {
            if !val.is_finite() || val <= 0.0 { continue; }
            let normalized_h = (val / max_val * (h_px as f64)).round();
            let bar_height = (normalized_h as usize).min(h_px);
            let x_start = i * bar_width;
            let x_end = (x_start + bar_width).min(w_px);
            if x_start >= w_px { break; }

            for x in x_start..x_end {
                self.canvas.line(x as isize, 0, x as isize, bar_height as isize, color);
            }
        }
    }

    pub fn polygon(&mut self, vertices: &[(f64, f64)], color: Option<Color>) {
        if vertices.len() < 2 { return; }
        let (x_range, y_range) = Self::get_auto_range(vertices, 0.05);

        for i in 0..vertices.len() {
            let (x0, y0) = vertices[i];
            let (x1, y1) = vertices[(i + 1) % vertices.len()];
            if !x0.is_finite() || !y0.is_finite() || !x1.is_finite() || !y1.is_finite() { continue; }
            let p0 = self.map_coords(x0, y0, x_range, y_range);
            let p1 = self.map_coords(x1, y1, x_range, y_range);
            self.canvas.line(p0.0, p0.1, p1.0, p1.1, color);
        }
    }

    pub fn pie_chart(&mut self, slices: &[(f64, Option<Color>)]) {
        let total: f64 = slices.iter()
            .filter_map(|(v, _)| if v.is_finite() && *v > 0.0 { Some(*v) } else { None })
            .sum();
        if total <= 1e-9 { return; }

        let w_px = self.canvas.pixel_width() as isize;
        let h_px = self.canvas.pixel_height() as isize;
        let cx = w_px / 2;
        let cy = h_px / 2;
        let radius = (w_px.min(h_px) as f64 / 2.0 * 0.95) as isize;
        let mut current_angle = 0.0;

        for (value, color) in slices {
            if !value.is_finite() || *value <= 0.0 { continue; }
            let slice_angle = (value / total) * 2.0 * PI;
            let end_angle = current_angle + slice_angle;

            let end_x = cx + (radius as f64 * end_angle.cos()) as isize;
            let end_y = cy + (radius as f64 * end_angle.sin()) as isize;

            self.canvas.line(cx, cy, end_x, end_y, *color);
            current_angle = end_angle;
        }
    }

    pub fn draw_circle(&mut self, center: (f64, f64), radius_norm: f64, color: Option<Color>) {
        let w_px = self.canvas.pixel_width() as f64;
        let h_px = self.canvas.pixel_height() as f64;
        let min_dim = w_px.min(h_px);

        let r_px = (radius_norm * min_dim) as isize;
        let cx_px = (center.0 * (w_px - 1.0)) as isize;
        let cy_px = (center.1 * (h_px - 1.0)) as isize;

        self.canvas.circle(cx_px, cy_px, r_px, color);
    }

    pub fn plot_function<F>(&mut self, func: F, min_x: f64, max_x: f64, color: Option<Color>)
    where
        F: Fn(f64) -> f64,
    {
        let steps = self.canvas.pixel_width();
        let mut points = Vec::with_capacity(steps);

        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            let x = min_x + t * (max_x - min_x);
            let y = func(x);
            if y.is_finite() {
                points.push((x, y));
            }
        }
        self.line_chart(&points, color);
    }

    // --- UTILIDADES ---

    pub fn text(&mut self, text: &str, x_norm: f64, y_norm: f64, color: Option<Color>) {
        let w = self.canvas.width;
        let h = self.canvas.height;
        let cx = (x_norm * (w.saturating_sub(1)) as f64).round() as usize;
        let cy = (y_norm * (h.saturating_sub(1)) as f64).round() as usize;

        for (i, ch) in text.chars().enumerate() {
            if cx + i >= w { break; }
            self.canvas.set_char(cx + i, cy, ch, color);
        }
    }

    /// Dibuja los ejes calculando "ticks" intermedios de forma automática.
    pub fn draw_axes(&mut self, x_range: (f64, f64), y_range: (f64, f64), color: Option<Color>) {
        let w_px = self.canvas.pixel_width() as isize;
        let h_px = self.canvas.pixel_height() as isize;

        self.canvas.line(0, 0, 0, h_px - 1, color);
        self.canvas.line(0, 0, w_px - 1, 0, color);

        // helper para generar ~4 marcas equidistantes
        let draw_ticks = |min: f64, max: f64| -> Vec<f64> {
            let step = (max - min) / 3.0; // 4 marcas incluyendo bordes
            vec![min, min + step, min + step * 2.0, max]
        };

        // Y Ticks
        let y_ticks = draw_ticks(y_range.0, y_range.1);
        for (i, &val) in y_ticks.iter().enumerate() {
            let norm_y = i as f64 / (y_ticks.len() - 1) as f64;
            self.text(&format!("{:.1}", val), 0.0, norm_y, color);
        }

        // X Ticks
        let x_ticks = draw_ticks(x_range.0, x_range.1);
        for (i, &val) in x_ticks.iter().enumerate() {
            let norm_x = i as f64 / (x_ticks.len() - 1) as f64;
            // Desplazamos un poco el primero y último para que no se corten
            let safe_x = norm_x.clamp(0.05, 0.90);
            self.text(&format!("{:.1}", val), safe_x, 0.0, color);
        }
    }

    pub fn draw_grid(&mut self, divs_x: usize, divs_y: usize, color: Option<Color>) {
         let w_px = self.canvas.pixel_width() as isize;
         let h_px = self.canvas.pixel_height() as isize;

         for i in 1..divs_x {
             let x = (i as f64 / divs_x as f64 * (w_px as f64)).round() as isize;
             self.canvas.line(x, 0, x, h_px, color);
         }

         for i in 1..divs_y {
             let y = (i as f64 / divs_y as f64 * (h_px as f64)).round() as isize;
             self.canvas.line(0, y, w_px, y, color);
         }
    }
}
