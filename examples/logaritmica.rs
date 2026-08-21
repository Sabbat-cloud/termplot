use termplot_rs::{ChartContext, AxisScale};
use colored::Color;

fn main() {
    let mut chart = ChartContext::new(60, 15);
    
    // Establecer el eje Y en escala logarítmica
    chart.set_y_scale(AxisScale::Log10);

    // Datos con un pico masivo (ej. latencia de servidor en ms)
    let latency_data: Vec<(f64, f64)> = vec![
        (1.0, 45.0), (2.0, 50.0), (3.0, 48.0), 
        (4.0, 1500.0), // ¡Pico masivo repentino!
        (5.0, 55.0), (6.0, 49.0),
    ];

    // Usar get_auto_range_scaled para tener en cuenta la transformación Log10
    let (range_x, range_y) = ChartContext::get_auto_range_scaled(
        &latency_data, 0.1, chart.x_scale(), chart.y_scale()
    );

    chart.draw_axes(range_x, range_y, Some(Color::White));
    chart.line_chart(&latency_data, Some(Color::Red));
    chart.text("Latencia (ms)", 0.5, 0.9, Some(Color::Yellow));

    println!("{}", chart.canvas.render());
}
