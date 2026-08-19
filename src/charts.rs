use crate::canvas::BrailleCanvas;
use crate::scale::AxisScale;
use colored::Color;
use std::f64::consts::PI;

pub struct ChartContext {
    pub canvas: BrailleCanvas,
    background_mask: Vec<u8>,
    x_scale: AxisScale,
    y_scale: AxisScale,
}

impl ChartContext {
    pub fn new(width: usize, height: usize) -> Self {
        let canvas = BrailleCanvas::new(width, height);
        Self {
            background_mask: vec![0; width * height],
            canvas,
            x_scale: AxisScale::Linear,
            y_scale: AxisScale::Linear,
        }
    }

    pub fn set_x_scale(&mut self, scale: AxisScale) -> &mut Self {
        self.x_scale = scale;
        self
    }

    pub fn set_y_scale(&mut self, scale: AxisScale) -> &mut Self {
        self.y_scale = scale;
        self
    }

    pub fn set_scales(&mut self, x_scale: AxisScale, y_scale: AxisScale) -> &mut Self {
        self.x_scale = x_scale;
        self.y_scale = y_scale;
        self
    }

    pub fn x_scale(&self) -> AxisScale {
        self.x_scale
    }

    pub fn y_scale(&self) -> AxisScale {
        self.y_scale
    }

    pub fn get_auto_range(points: &[(f64, f64)], padding: f64) -> ((f64, f64), (f64, f64)) {
        Self::get_auto_range_scaled(points, padding, AxisScale::Linear, AxisScale::Linear)
    }

    pub fn get_auto_range_scaled(
        points: &[(f64, f64)],
        padding: f64,
        x_scale: AxisScale,
        y_scale: AxisScale,
    ) -> ((f64, f64), (f64, f64)) {
        let valid_points: Vec<(f64, f64)> = points
            .iter()
            .filter_map(|&(x, y)| Some((x_scale.transform(x)?, y_scale.transform(y)?)))
            .collect();

        if valid_points.is_empty() {
            return (Self::default_range(x_scale), Self::default_range(y_scale));
        }

        let (min_x, max_x) = valid_points
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), (x, _)| {
                (min.min(*x), max.max(*x))
            });

        let (min_y, max_y) = valid_points
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), (_, y)| {
                (min.min(*y), max.max(*y))
            });

        (
            Self::expand_range(min_x, max_x, padding, x_scale),
            Self::expand_range(min_y, max_y, padding, y_scale),
        )
    }

    fn default_range(scale: AxisScale) -> (f64, f64) {
        match scale {
            AxisScale::Linear => (0.0, 1.0),
            AxisScale::Log10 => (1.0, 10.0),
        }
    }

    fn expand_range(min: f64, max: f64, padding: f64, scale: AxisScale) -> (f64, f64) {
        let range = if (max - min).abs() < 1e-9 {
            1.0
        } else {
            max - min
        };
        let min_exp = scale.inverse_transform(min - range * padding);
        let max_exp = scale.inverse_transform(max + range * padding);
        (min_exp, max_exp)
    }

    fn normalized_axis_position(scale: AxisScale, value: f64, range: (f64, f64)) -> Option<f64> {
        let value_t = scale.transform(value)?;
        let (min, max) = scale.transformed_range(range)?;
        let span = (max - min).max(1e-9);
        Some(((value_t - min) / span).clamp(0.0, 1.0))
    }

    // Convertido en helper estático para evitar conflictos de prestamos (Borrow Checker) con los overlays
    #[allow(clippy::too_many_arguments)]
    fn map_coords(
        x: f64,
        y: f64,
        x_range: (f64, f64),
        y_range: (f64, f64),
        x_scale: AxisScale,
        y_scale: AxisScale,
        width_px: usize,
        height_px: usize,
        left_inset_px: usize,
        bottom_inset_px: usize,
    ) -> Option<(isize, isize)> {
        let x_t = x_scale.transform(x)?;
        let y_t = y_scale.transform(y)?;
        let (min_x, max_x) = x_scale.transformed_range(x_range)?;
        let (min_y, max_y) = y_scale.transformed_range(y_range)?;

        let range_x = (max_x - min_x).max(1e-9);
        let range_y = (max_y - min_y).max(1e-9);
        let drawable_width = (width_px.saturating_sub(1 + left_inset_px)).max(1) as f64;
        let drawable_height = (height_px.saturating_sub(1 + bottom_inset_px)).max(1) as f64;

        let px = left_inset_px as f64 + ((x_t - min_x) / range_x * drawable_width).round();
        let py = bottom_inset_px as f64 + ((y_t - min_y) / range_y * drawable_height).round();

        Some((px as isize, py as isize))
    }

    fn draw_foreground_overlay<F>(&mut self, draw: F)
    where
        F: FnOnce(&mut BrailleCanvas),
    {
        let mut overlay = BrailleCanvas::new(self.canvas.width, self.canvas.height);
        overlay.blend_mode = self.canvas.blend_mode;
        draw(&mut overlay);
        self.canvas
            .overlay_without_background(&overlay, &self.background_mask);
    }

    fn draw_background_overlay<F>(&mut self, draw: F)
    where
        F: FnOnce(&mut BrailleCanvas),
    {
        let mut overlay = BrailleCanvas::new(self.canvas.width, self.canvas.height);
        overlay.blend_mode = self.canvas.blend_mode;
        draw(&mut overlay);
        self.canvas.merge(&overlay);
        for (mask, cell) in self.background_mask.iter_mut().zip(overlay.cell_masks()) {
            *mask |= *cell;
        }
    }

    // --- GRÁFICOS ---

    fn line_chart_with_ranges(
        &mut self,
        points: &[(f64, f64)],
        x_range: (f64, f64),
        y_range: (f64, f64),
        color: Option<Color>,
    ) {
        let w_px = self.canvas.pixel_width();
        let h_px = self.canvas.pixel_height();
        let (left_inset_px, bottom_inset_px) = self.canvas.plot_insets();
        let x_scale = self.x_scale;
        let y_scale = self.y_scale;

        self.draw_foreground_overlay(|overlay| {
            for window in points.windows(2) {
                let (x0, y0) = window[0];
                let (x1, y1) = window[1];
                if !x0.is_finite() || !y0.is_finite() || !x1.is_finite() || !y1.is_finite() {
                    continue;
                }

                let Some(p0) = Self::map_coords(
                    x0,
                    y0,
                    x_range,
                    y_range,
                    x_scale,
                    y_scale,
                    w_px,
                    h_px,
                    left_inset_px,
                    bottom_inset_px,
                ) else {
                    continue;
                };
                let Some(p1) = Self::map_coords(
                    x1,
                    y1,
                    x_range,
                    y_range,
                    x_scale,
                    y_scale,
                    w_px,
                    h_px,
                    left_inset_px,
                    bottom_inset_px,
                ) else {
                    continue;
                };
                overlay.line(p0.0, p0.1, p1.0, p1.1, color);
            }
        });
    }

    pub fn scatter(&mut self, points: &[(f64, f64)], color: Option<Color>) {
        if points.is_empty() {
            return;
        }
        let (x_range, y_range) =
            Self::get_auto_range_scaled(points, 0.05, self.x_scale, self.y_scale);
        let w_px = self.canvas.pixel_width();
        let h_px = self.canvas.pixel_height();
        let (left_inset_px, bottom_inset_px) = self.canvas.plot_insets();
        let x_scale = self.x_scale;
        let y_scale = self.y_scale;

        self.draw_foreground_overlay(|overlay| {
            for &(x, y) in points {
                if !x.is_finite() || !y.is_finite() {
                    continue;
                }
                let Some((px, py)) = Self::map_coords(
                    x,
                    y,
                    x_range,
                    y_range,
                    x_scale,
                    y_scale,
                    w_px,
                    h_px,
                    left_inset_px,
                    bottom_inset_px,
                ) else {
                    continue;
                };

                if px >= 0 && py >= 0 && (px as usize) < w_px && (py as usize) < h_px {
                    overlay.set_pixel(px as usize, py as usize, color);
                }
            }
        });
    }

    pub fn line_chart(&mut self, points: &[(f64, f64)], color: Option<Color>) {
        if points.len() < 2 {
            return;
        }
        let (x_range, y_range) =
            Self::get_auto_range_scaled(points, 0.05, self.x_scale, self.y_scale);
        self.line_chart_with_ranges(points, x_range, y_range, color);
    }

    pub fn bar_chart(&mut self, values: &[(f64, Option<Color>)]) {
        if values.is_empty() {
            return;
        }

        let w_px = self.canvas.pixel_width();
        let h_px = self.canvas.pixel_height();
        let bar_width = (w_px / values.len()).max(1);

        match self.y_scale {
            AxisScale::Linear => {
                let max_val = values
                    .iter()
                    .filter_map(|(v, _)| if v.is_finite() { Some(*v) } else { None })
                    .fold(0.0f64, f64::max);

                if max_val <= 1e-9 {
                    return;
                }

                for (i, &(val, color)) in values.iter().enumerate() {
                    if !val.is_finite() || val <= 0.0 {
                        continue;
                    }
                    let normalized_h = (val / max_val * (h_px as f64)).round();
                    let bar_height = (normalized_h as usize).min(h_px);
                    let x_start = i * bar_width;
                    let x_end = (x_start + bar_width).min(w_px);
                    if x_start >= w_px {
                        break;
                    }

                    for x in x_start..x_end {
                        self.canvas
                            .line(x as isize, 0, x as isize, bar_height as isize, color);
                    }
                }
            }
            AxisScale::Log10 => {
                let transformed: Vec<(f64, Option<Color>)> = values
                    .iter()
                    .filter_map(|(value, color)| {
                        Some((AxisScale::Log10.transform(*value)?, *color))
                    })
                    .collect();

                if transformed.is_empty() {
                    return;
                }

                let min_val = transformed
                    .iter()
                    .map(|(value, _)| *value)
                    .fold(f64::INFINITY, f64::min);
                let max_val = transformed
                    .iter()
                    .map(|(value, _)| *value)
                    .fold(f64::NEG_INFINITY, f64::max);
                let range = (max_val - min_val).max(1e-9);

                for (i, &(raw_value, color)) in values.iter().enumerate() {
                    let Some(value) = AxisScale::Log10.transform(raw_value) else {
                        continue;
                    };

                    let normalized_h = (value - min_val) / range;
                    let bar_height =
                        (((normalized_h * (h_px.saturating_sub(1)) as f64).round() as usize) + 1)
                            .min(h_px);
                    let x_start = i * bar_width;
                    let x_end = (x_start + bar_width).min(w_px);
                    if x_start >= w_px {
                        break;
                    }

                    for x in x_start..x_end {
                        self.canvas
                            .line(x as isize, 0, x as isize, bar_height as isize, color);
                    }
                }
            }
        }
    }

    pub fn polygon(&mut self, vertices: &[(f64, f64)], color: Option<Color>) {
        if vertices.len() < 2 {
            return;
        }

        let normalized_polygon = vertices.iter().all(|&(x, y)| {
            x.is_finite() && y.is_finite() && (0.0..=1.0).contains(&x) && (0.0..=1.0).contains(&y)
        });

        let (x_range, y_range) = if normalized_polygon
            && self.x_scale == AxisScale::Linear
            && self.y_scale == AxisScale::Linear
        {
            ((0.0, 1.0), (0.0, 1.0))
        } else {
            Self::get_auto_range_scaled(vertices, 0.05, self.x_scale, self.y_scale)
        };

        let w_px = self.canvas.pixel_width();
        let h_px = self.canvas.pixel_height();
        let (left_inset_px, bottom_inset_px) = self.canvas.plot_insets();
        let x_scale = self.x_scale;
        let y_scale = self.y_scale;

        self.draw_foreground_overlay(|overlay| {
            for i in 0..vertices.len() {
                let (x0, y0) = vertices[i];
                let (x1, y1) = vertices[(i + 1) % vertices.len()];
                if !x0.is_finite() || !y0.is_finite() || !x1.is_finite() || !y1.is_finite() {
                    continue;
                }

                let Some(p0) = Self::map_coords(
                    x0,
                    y0,
                    x_range,
                    y_range,
                    x_scale,
                    y_scale,
                    w_px,
                    h_px,
                    left_inset_px,
                    bottom_inset_px,
                ) else {
                    continue;
                };
                let Some(p1) = Self::map_coords(
                    x1,
                    y1,
                    x_range,
                    y_range,
                    x_scale,
                    y_scale,
                    w_px,
                    h_px,
                    left_inset_px,
                    bottom_inset_px,
                ) else {
                    continue;
                };
                overlay.line(p0.0, p0.1, p1.0, p1.1, color);
            }
        });
    }

    pub fn pie_chart(&mut self, slices: &[(f64, Option<Color>)]) {
        let total: f64 = slices
            .iter()
            .filter_map(|(v, _)| {
                if v.is_finite() && *v > 0.0 {
                    Some(*v)
                } else {
                    None
                }
            })
            .sum();
        if total <= 1e-9 {
            return;
        }

        let w_px = self.canvas.pixel_width() as isize;
        let h_px = self.canvas.pixel_height() as isize;
        let cx = w_px / 2;
        let cy = h_px / 2;
        let radius = ((w_px.min(h_px).saturating_sub(1)) / 2).max(1);
        let mut current_angle = 0.0;

        for (value, color) in slices {
            if !value.is_finite() || *value <= 0.0 {
                continue;
            }
            let slice_angle = (value / total) * 2.0 * PI;
            let end_angle = current_angle + slice_angle;

            let end_x = cx + (radius as f64 * end_angle.cos()) as isize;
            let end_y = cy + (radius as f64 * end_angle.sin()) as isize;

            self.draw_foreground_overlay(|overlay| {
                overlay.line(cx, cy, end_x, end_y, *color);
            });
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

        self.draw_foreground_overlay(|overlay| {
            overlay.circle(cx_px, cy_px, r_px, color);
        });
    }

    pub fn plot_function<F>(&mut self, func: F, min_x: f64, max_x: f64, color: Option<Color>)
    where
        F: Fn(f64) -> f64,
    {
        let steps = self.canvas.pixel_width().saturating_sub(1).max(1);
        let Some(min_x_t) = self.x_scale.transform(min_x) else {
            return;
        };
        let Some(max_x_t) = self.x_scale.transform(max_x) else {
            return;
        };
        let mut points = Vec::with_capacity(steps + 1);

        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            let x = self
                .x_scale
                .inverse_transform(min_x_t + t * (max_x_t - min_x_t));
            let y = func(x);
            if self.y_scale.transform(y).is_some() {
                points.push((x, y));
            }
        }
        if points.len() < 2 {
            return;
        }
        let (_, y_range) = Self::get_auto_range_scaled(&points, 0.05, self.x_scale, self.y_scale);
        self.line_chart_with_ranges(&points, (min_x, max_x), y_range, color);
    }

    // --- UTILIDADES ---

    pub fn text(&mut self, text: &str, x_norm: f64, y_norm: f64, color: Option<Color>) {
        let w = self.canvas.width;
        let h = self.canvas.height;
        let cx = (x_norm * (w.saturating_sub(1)) as f64).round() as usize;
        let cy = (y_norm * (h.saturating_sub(1)) as f64).round() as usize;

        for (i, ch) in text.chars().enumerate() {
            if cx + i >= w {
                break;
            }
            self.canvas.set_char(cx + i, cy, ch, color);
        }
    }

    pub fn draw_axes(&mut self, x_range: (f64, f64), y_range: (f64, f64), color: Option<Color>) {
        let w_px = self.canvas.pixel_width() as isize;
        let h_px = self.canvas.pixel_height() as isize;
        self.canvas.set_plot_insets(1, 1);
        let (left_inset_px, bottom_inset_px) = self.canvas.plot_insets();

        self.draw_background_overlay(|overlay| {
            overlay.line(
                left_inset_px as isize,
                bottom_inset_px as isize,
                left_inset_px as isize,
                h_px - 1,
                color,
            );
            overlay.line(
                left_inset_px as isize,
                bottom_inset_px as isize,
                w_px - 1,
                bottom_inset_px as isize,
                color,
            );
        });

        let y_ticks = self.y_scale.axis_ticks(y_range);
        for val in y_ticks {
            let Some(norm_y) = Self::normalized_axis_position(self.y_scale, val, y_range) else {
                continue;
            };
            self.text(&self.y_scale.format_tick(val), 0.0, norm_y, color);
        }

        let x_ticks = self.x_scale.axis_ticks(x_range);
        for val in x_ticks {
            let Some(norm_x) = Self::normalized_axis_position(self.x_scale, val, x_range) else {
                continue;
            };
            let label = self.x_scale.format_tick(val);
            let margin = if self.canvas.width > 1 {
                (label.len().saturating_sub(1) as f64 / (self.canvas.width - 1) as f64).min(0.45)
            } else {
                0.0
            };
            let safe_x = norm_x.clamp(margin, 1.0 - margin);
            self.text(&label, safe_x, 0.0, color);
        }
    }

    pub fn draw_grid(&mut self, divs_x: usize, divs_y: usize, color: Option<Color>) {
        let w_px = self.canvas.pixel_width() as isize;
        let h_px = self.canvas.pixel_height() as isize;

        self.draw_background_overlay(|overlay| {
            for i in 1..divs_x {
                let x = (i as f64 / divs_x as f64 * (w_px as f64)).round() as isize;
                overlay.line(x, 0, x, h_px, color);
            }

            for i in 1..divs_y {
                let y = (i as f64 / divs_y as f64 * (h_px as f64)).round() as isize;
                overlay.line(0, y, w_px, y, color);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::ChartContext;
    use crate::scale::AxisScale;

    fn visible_render(chart: &ChartContext) -> String {
        chart
            .canvas
            .render_with_options(false, None)
            .replace('\u{2800}', " ")
    }

    #[test]
    fn log_scatter_renders_even_spacing_across_decades() {
        let mut chart = ChartContext::new(12, 6);
        chart.set_scales(AxisScale::Log10, AxisScale::Log10);
        chart.scatter(
            &[(1.0, 1.0), (10.0, 10.0), (100.0, 100.0), (1000.0, 1000.0)],
            None,
        );

        assert_eq!(
            chart.canvas.render_no_color(),
            concat!(
                "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂\n",
                "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
                "⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀\n",
                "⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀\n",
                "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
                "⠠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n",
            )
        );
    }

    #[test]
    fn plot_function_renders_over_grid_without_cell_artifacts() {
        let mut chart = ChartContext::new(12, 6);
        chart.draw_grid(4, 2, None);
        chart.draw_axes((0.0, 6.0), (-1.0, 1.0), None);
        chart.plot_function(|x: f64| x.sin(), 0.0, 6.0, None);

        assert_eq!(
            chart.canvas.render_no_color(),
            concat!(
                "⢸⠀⢠⠒⠢⡀⡇⠀⠀⡇⠀⠀\n",
                "⢸⢠⠃⡇⠀⠱⡀⠀⠀⡇⠀⠀\n",
                "⢠⠃⣀⣇⣀⣀⡇⣀⣀⣇⣀⣀\n",
                "⢸⠀⠀⡇⠀⠀⠸⡀⠀⡇⠀⢠\n",
                "⢸⠀⠀⡇⠀⠀⡇⠱⡀⡇⢠⠃\n",
                "⠸⠤⠤⡧⠤⠤⡧⠤⠑⠒⠁⠤\n",
            ),
        );
    }

    #[test]
    fn multiple_foreground_plots_keep_crossings() {
        let mut chart = ChartContext::new(10, 5);
        chart.draw_grid(2, 2, None);
        chart.draw_axes((0.0, 6.0), (-1.0, 1.0), None);
        chart.plot_function(|x: f64| x.sin(), 0.0, 6.0, None);
        chart.plot_function(|x: f64| (x * 0.5).cos() * 0.5, 0.0, 6.0, None);

        assert_eq!(
            chart.canvas.render_no_color(),
            concat!(
                "⠐⠒⡴⡒⢄⡇⠀⠀⠀⠀\n",
                "⢸⡜⠀⠈⢺⡄⠀⠀⠀⠀\n",
                "⠘⠒⠒⠒⠒⢣⡀⠒⠒⢀\n",
                "⢸⠀⠀⠀⠀⠈⢗⢄⢀⠎\n",
                "⠸⠤⠤⠤⠤⡧⠈⠒⠋⠒\n",
            ),
        );
    }

    #[test]
    fn log_axes_render_power_of_ten_labels() {
        let mut chart = ChartContext::new(18, 6);
        chart.set_scales(AxisScale::Log10, AxisScale::Log10);
        chart.draw_axes((1.0, 1000.0), (1.0, 1000.0), None);

        assert_eq!(
            visible_render(&chart),
            "1e3               \n⢸                 \n100               \n10                \n⢸                 \n1⠤⠤⠤⠤⠤10⠤⠤⠤100⠤1e3\n"
        );
    }

    #[test]
    fn line_chart_uses_full_x_span() {
        let mut chart = ChartContext::new(6, 3);
        chart.line_chart(&[(0.0, 0.0), (1.0, 1.0)], None);

        let rendered = chart.canvas.render_no_color();
        let rows: Vec<_> = rendered.lines().collect();
        let blank = '\u{2800}';

        assert!(rows
            .iter()
            .any(|row| row.chars().next().unwrap_or(blank) != blank));
        assert!(rows
            .iter()
            .any(|row| row.chars().last().unwrap_or(blank) != blank));
    }
}
