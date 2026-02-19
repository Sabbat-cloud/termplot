use colored::{Color, Colorize};
use rand::Rng;
use std::io::{self, Write};
use std::{thread, time};
// Importamos PI y TAU explícitamente
use std::f64::consts::{PI, TAU};
use termplot_rs::ChartContext;

fn main() {
    println!(
        "{}",
        "--- TERMPLOT v0.8: ROADMAP UPDATE ---\n".bold().cyan()
    );

    // --- EJEMPLOS ESTÁTICOS (1 al 7) ---

    // 1. Gráfico de Barras Coloreado
    println!("{}", "1. Color bars".yellow());
    let mut chart = ChartContext::new(60, 10);
    let data_bars = vec![
        (30.0, Some(Color::Red)),
        (55.0, Some(Color::Green)),
        (90.0, Some(Color::Blue)),
        (45.0, Some(Color::Yellow)),
        (70.0, Some(Color::Magenta)),
        (25.0, None),
    ];
    chart.bar_chart(&data_bars);
    println!("{}", chart.canvas.render());

    // 2. Scatter Plot
    println!("\n{}", "2. Scatter Plot".yellow());
    let mut chart = ChartContext::new(60, 15);
    let mut rng = rand::thread_rng();
    let mut series_a = Vec::new();
    let mut series_b = Vec::new();
    for _ in 0..150 {
        series_a.push((rng.gen_range(0.0..60.0), rng.gen_range(0.0..60.0)));
        series_b.push((rng.gen_range(40.0..100.0), rng.gen_range(40.0..100.0)));
    }
    chart.scatter(&series_a, Some(Color::Red));
    chart.scatter(&series_b, Some(Color::Cyan));
    println!("{}", chart.canvas.render());

    // 3. Geometría
    println!("\n{}", "3. Geometry".yellow());
    let mut chart = ChartContext::new(40, 20);
    chart.draw_circle((0.5, 0.5), 0.4, Some(Color::Green));
    let triangle = vec![(0.1, 0.1), (0.5, 0.9), (0.9, 0.1)];
    chart.polygon(&triangle, Some(Color::Magenta));
    println!("{}", chart.canvas.render());

    // 4. Pastel
    println!("\n{}", "4. Pie chart".yellow());
    let mut chart = ChartContext::new(40, 20);
    let pie_data = vec![
        (30.0, Some(Color::Red)),
        (20.0, Some(Color::Blue)),
        (15.0, Some(Color::Green)),
        (25.0, Some(Color::Yellow)),
        (10.0, Some(Color::White)),
    ];
    chart.pie_chart(&pie_data);
    println!("{}", chart.canvas.render());

    // 5. Espiral Logarítmica (Actualizada v0.8 con Auto-Range)
    println!("\n{}", "5. Logarithmic Spiral (Auto-Scaled)".yellow());
    let mut chart = ChartContext::new(60, 20);
    let mut spiral_points = Vec::new();
    
    // CORRECCIÓN: Tipado explícito para que .sin()/.cos() sepan que es f64
    let mut t: f64 = 0.0;

    // Generamos los puntos de la espiral
    while t < 8.0 * PI {
        let a = 0.1;
        let b = 0.2;
        let r = a * (b * t).exp();
        let x = r * t.cos();
        let y = r * t.sin();
        spiral_points.push((x, y));
        t += 0.05;
    }

    // Ajuste automático de ejes para la geometría de la espiral
    let (range_x, range_y) = ChartContext::get_auto_range(&spiral_points, 0.1);

    chart.draw_grid(
        8,
        4,
        Some(Color::TrueColor {
            r: 50,
            g: 50,
            b: 50,
        }),
    );
    chart.draw_axes(range_x, range_y, Some(Color::White));
    chart.scatter(&spiral_points, Some(Color::BrightBlue));

    chart.text("Spiral Analysis", 0.35, 0.9, Some(Color::White));
    println!("{}", chart.canvas.render());

    // 6. Serie Temporal Estática (con ejes)
    println!("\n{}", "6. Static Time Series".yellow());
    let mut chart = ChartContext::new(60, 15);
    chart.draw_grid(
        10,
        4,
        Some(Color::TrueColor {
            r: 100,
            g: 100,
            b: 100,
        }),
    ); // Rejilla suave
    chart.draw_axes((0.0, 10.0), (-1.0, 1.0), Some(Color::White));
    chart.plot_function(|x: f64| x.sin(), 0.0, 10.0, Some(Color::Cyan));
    chart.text("Sine Wave", 0.4, 0.9, Some(Color::White));
    println!("{}", chart.canvas.render());

    // 7 - Función matemática + rejilla + ejes
    println!("\n{}", "7.  Function + grid + axes".yellow());
    let mut chart = ChartContext::new(60, 15);
    chart.draw_grid(
        10,
        4,
        Some(Color::TrueColor {
            r: 80,
            g: 80,
            b: 80,
        }),
    );
    chart.draw_axes((0.0, 10.0), (-1.5, 1.5), Some(Color::White));
    chart.plot_function(|x: f64| x.sin(), 0.0, 10.0, Some(Color::Cyan));
    chart.plot_function(
        |x: f64| (x * 0.5).cos() * 0.5,
        0.0,
        10.0,
        Some(Color::Magenta),
    );
    chart.text("sin(x)", 0.75, 0.85, Some(Color::Cyan));
    chart.text("0.5*cos(0.5x)", 0.55, 0.10, Some(Color::Magenta));
    println!("{}", chart.canvas.render());

    // --- 8. NUEVAS OPCIONES v0.8+ ---
    println!("\n{}", "8. Options v0.8+ (No color / Títles)".yellow());
    let mut v8_chart = ChartContext::new(40, 10);
    v8_chart.draw_grid(5, 2, Some(Color::White));
    v8_chart.plot_function(|x: f64| x.cos(), 0.0, TAU, Some(Color::Green));
    v8_chart.text("Cosine", 0.4, 0.8, None);

    // Sub-ejemplo A: Sin color (ASCII/Braille puro)
    println!(
        "{}",
        "\nA) Render (render_no_color):".bright_black()
    );
    println!("{}", v8_chart.canvas.render_no_color());

    // Sub-ejemplo B: Con título centrado y sin bordes
    println!(
        "{}",
        "\nB) With title and without borders (render_with_options):".bright_black()
    );
    println!(
        "{}",
        v8_chart
            .canvas
            .render_with_options(false, Some("MY CUSTOM CHART"))
    );

    // 9 AUTO RANGE
    println!("\n{}", "9.Auto-Range".yellow());
    let mut chart = ChartContext::new(60, 15);

    // Datos aleatorios en un rango desconocido
    let points: Vec<(f64, f64)> = (0..50)
        .map(|i| (i as f64, (i as f64 * 0.2).sin() * 50.0 + 20.0))
        .collect();

    // Calculamos el rango automáticamente con un 10% de margen
    let (range_x, range_y) = ChartContext::get_auto_range(&points, 0.1);

    chart.draw_grid(
        10,
        4,
        Some(Color::TrueColor {
            r: 40,
            g: 40,
            b: 40,
        }),
    );
    chart.draw_axes(range_x, range_y, Some(Color::White));
    chart.line_chart(&points, Some(Color::Yellow));
    chart.text("Auto-Scaled", 0.4, 0.9, Some(Color::Yellow));

    println!("{}", chart.canvas.render());

    // --- 10. ANIMACIÓN PRO ---
    println!(
        "\n{}",
        "10. Anim PRO"
            .on_red()
            .white()
            .bold()
    );
    println!("Rendering... (Ctrl+C exit)");
    thread::sleep(time::Duration::from_secs(1));

    let width = 60;
    let height = 15;
    let mut chart = ChartContext::new(width, height);
    let mut phase = 0.0;
    let lines_to_rewind = height + 2 + 1;

    loop {
        chart.canvas.clear();
        chart.draw_grid(
            10,
            4,
            Some(Color::TrueColor {
                r: 60,
                g: 60,
                b: 60,
            }),
        );
        chart.draw_axes((0.0, 10.0), (-1.5, 1.5), Some(Color::White));
        chart.plot_function(
            |x: f64| (x + phase).sin() * (x * 0.5).cos(),
            0.0,
            10.0,
            Some(Color::Cyan),
        );
        chart.plot_function(
            |x: f64| ((x - phase * 1.5).cos() * 0.5) - 0.5,
            0.0,
            10.0,
            Some(Color::Magenta),
        );
        chart.text("Dual System", 0.40, 0.9, Some(Color::Yellow));

        let output = chart.canvas.render();
        println!("{}", output);
        println!("Phase: {:.2} | Grid: ON | Funcs: 2", phase);

        thread::sleep(time::Duration::from_millis(50));
        print!("\x1B[{}A", lines_to_rewind);
        io::stdout().flush().unwrap();
        phase += 0.1;
    }
}
