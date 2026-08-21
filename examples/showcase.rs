use colored::Color;
use std::f64::consts::PI;
use termplot_rs::prelude::*;

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                  TERMPLOT SHOWCASE                         ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    demo_canvas();
    demo_functions();
    demo_multiplot();
    demo_scatter_polygon();
    demo_bars();
    demo_pie();
    demo_logarithmic();
    demo_truecolor();
}

// ============================================================================
// 1. BRAILLE CANVAS / PRIMITIVAS
// ============================================================================

fn demo_canvas() {
    println!("1. BRAILLE CANVAS — primitivas gráficas\n");

    let mut canvas = BrailleCanvas::new(70, 20);

    let w = canvas.pixel_width() as isize;
    let h = canvas.pixel_height() as isize;

    // Líneas diagonales con clipping
    canvas.line_screen(
        -20,
        -10,
        w + 20,
        h + 10,
        Some(Color::BrightCyan),
    );

    canvas.line_screen(
        -20,
        h + 10,
        w + 20,
        -10,
        Some(Color::BrightMagenta),
    );

    // Rectángulos
    canvas.rect(
        6,
        6,
        35,
        18,
        Some(Color::BrightGreen),
    );

    canvas.rect_filled(
        50,
        10,
        15,
        8,
        Some(Color::BrightBlue),
    );

    // Círculos
    canvas.circle(
        95,
        40,
        18,
        Some(Color::BrightYellow),
    );

    canvas.circle_filled(
        125,
        55,
        10,
        Some(Color::Red),
    );

    // Texto
    canvas.set_char(
        3,
        2,
        'T',
        Some(Color::BrightWhite),
    );

    println!(
        "{}\n",
        canvas.render_with_options(true, Some("BrailleCanvas / primitives"))
    );
}

// ============================================================================
// 2. FUNCIONES + EJES + GRID
// ============================================================================

fn demo_functions() {
    println!("2. FUNCIONES MATEMÁTICAS\n");

    let mut chart = ChartContext::new(80, 22);

    chart.draw_grid(
        8,
        4,
        Some(Color::BrightBlack),
    );

    chart.draw_axes(
        (-2.0 * PI, 2.0 * PI),
        (-1.5, 1.5),
        Some(Color::White),
    );

    chart.plot_function(
        |x| x.sin(),
        -2.0 * PI,
        2.0 * PI,
        Some(Color::BrightCyan),
    );

    chart.text(
        "sin(x)",
        0.75,
        0.90,
        Some(Color::BrightCyan),
    );

    println!(
        "{}\n",
        chart
            .canvas
            .render_with_options(true, Some("f(x) = sin(x)"))
    );
}

// ============================================================================
// 3. MÚLTIPLES FUNCIONES / OVERLAYS
// ============================================================================

fn demo_multiplot() {
    println!("3. MÚLTIPLES CURVAS Y OVERLAYS\n");

    let mut chart = ChartContext::new(80, 24);

    chart.draw_grid(
        8,
        6,
        Some(Color::BrightBlack),
    );

    chart.draw_axes(
        (-PI, PI),
        (-1.5, 1.5),
        Some(Color::White),
    );

    chart.plot_function(
        |x| x.sin(),
        -PI,
        PI,
        Some(Color::BrightCyan),
    );

    chart.plot_function(
        |x| x.cos(),
        -PI,
        PI,
        Some(Color::BrightYellow),
    );

    chart.plot_function(
        |x| 0.5 * (2.0 * x).sin(),
        -PI,
        PI,
        Some(Color::BrightMagenta),
    );

    chart.text(
        "sin(x)",
        0.68,
        0.92,
        Some(Color::BrightCyan),
    );

    chart.text(
        "cos(x)",
        0.68,
        0.84,
        Some(Color::BrightYellow),
    );

    chart.text(
        "0.5 sin(2x)",
        0.68,
        0.76,
        Some(Color::BrightMagenta),
    );

    println!(
        "{}\n",
        chart
            .canvas
            .render_with_options(true, Some("Multiple functions"))
    );
}

// ============================================================================
// 4. SCATTER + LINE CHART + POLYGON
// ============================================================================

fn demo_scatter_polygon() {
    println!("4. SCATTER + LINE CHART + POLYGON\n");

    let points: Vec<(f64, f64)> = (0..25)
        .map(|i| {
            let x = i as f64 * 0.4;
            let noise = ((i * 37 % 11) as f64 - 5.0) * 0.08;
            let y = 0.35 * x + noise + 1.0;

            (x, y)
        })
        .collect();

    let mut chart = ChartContext::new(80, 22);

    chart.draw_grid(
        8,
        4,
        Some(Color::BrightBlack),
    );

    chart.scatter(
        &points,
        Some(Color::BrightYellow),
    );

    chart.line_chart(
        &points,
        Some(Color::BrightCyan),
    );

    // Coordenadas normalizadas: aprovecha el caso especial de polygon()
    let polygon = [
        (0.10, 0.20),
        (0.30, 0.80),
        (0.55, 0.90),
        (0.85, 0.55),
        (0.70, 0.15),
    ];

    chart.polygon(
        &polygon,
        Some(Color::BrightMagenta),
    );

    chart.draw_circle(
        (0.50, 0.50),
        0.12,
        Some(Color::BrightGreen),
    );

    println!(
        "{}\n",
        chart
            .canvas
            .render_with_options(true, Some("Scatter / line / polygon"))
    );
}

// ============================================================================
// 5. BAR CHART
// ============================================================================

fn demo_bars() {
    println!("5. BAR CHART\n");

    let data = [
        (12.0, Some(Color::BrightBlue)),
        (25.0, Some(Color::BrightGreen)),
        (17.0, Some(Color::BrightYellow)),
        (41.0, Some(Color::BrightMagenta)),
        (32.0, Some(Color::BrightCyan)),
        (48.0, Some(Color::BrightRed)),
        (21.0, Some(Color::White)),
    ];

    let mut chart = ChartContext::new(70, 18);

    chart.bar_chart(&data);

    println!(
        "{}\n",
        chart
            .canvas
            .render_with_options(true, Some("Bar chart"))
    );
}

// ============================================================================
// 6. PIE CHART
// ============================================================================

fn demo_pie() {
    println!("6. PIE CHART\n");

    let slices = [
        (35.0, Some(Color::BrightCyan)),
        (25.0, Some(Color::BrightMagenta)),
        (20.0, Some(Color::BrightYellow)),
        (12.0, Some(Color::BrightGreen)),
        (8.0, Some(Color::BrightRed)),
    ];

    let mut chart = ChartContext::new(50, 20);

    chart.pie_chart(&slices);

    chart.draw_circle(
        (0.5, 0.5),
        0.48,
        Some(Color::White),
    );

    chart.text(
        "35%",
        0.67,
        0.63,
        Some(Color::BrightCyan),
    );

    println!(
        "{}\n",
        chart
            .canvas
            .render_with_options(true, Some("Pie chart"))
    );
}

// ============================================================================
// 7. ESCALA LOGARÍTMICA
// ============================================================================

fn demo_logarithmic() {
    println!("7. ESCALA LOGARÍTMICA\n");

    let data = [
        (1.0, 2.0),
        (10.0, 18.0),
        (100.0, 210.0),
        (1_000.0, 1_800.0),
        (10_000.0, 22_000.0),
        (100_000.0, 170_000.0),
        (1_000_000.0, 2_100_000.0),
    ];

    let mut chart = ChartContext::new(80, 22);

    chart.set_scales(
        AxisScale::Log10,
        AxisScale::Log10,
    );

    chart.draw_grid(
        6,
        4,
        Some(Color::BrightBlack),
    );

    chart.draw_axes(
        (1.0, 1_000_000.0),
        (1.0, 10_000_000.0),
        Some(Color::White),
    );

    chart.line_chart(
        &data,
        Some(Color::BrightCyan),
    );

    chart.scatter(
        &data,
        Some(Color::BrightYellow),
    );

    println!(
        "{}\n",
        chart
            .canvas
            .render_with_options(true, Some("Log10 / Log10"))
    );
}

// ============================================================================
// 8. TRUE COLOR + BLENDING
// ============================================================================

fn demo_truecolor() {
    println!("8. TRUECOLOR + BLEND MODES\n");

    let mut canvas = BrailleCanvas::new(80, 16);

    canvas.blend_mode = ColorBlend::KeepFirst;

    let width = canvas.pixel_width();
    let height = canvas.pixel_height();

    for x in 0..width {
        let t = x as f64 / width as f64;

        let r = (255.0 * t) as u8;
        let g = (255.0 * (1.0 - t)) as u8;
        let b = (128.0 + 127.0 * (t * PI).sin()) as u8;

        let color = Color::TrueColor { r, g, b };

        let y = (
            height as f64 / 2.0
                + (height as f64 * 0.35)
                    * (t * PI * 6.0).sin()
        ) as usize;

        if y < height {
            canvas.set_pixel(
                x,
                y,
                Some(color),
            );
        }
    }

    println!(
        "{}\n",
        canvas.render_with_options(
            true,
            Some("TrueColor gradient")
        )
    );
}
