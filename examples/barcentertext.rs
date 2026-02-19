//use colored::{Color, Colorize};
use colored::Color;
use termplot_rs::ChartContext;

fn main() {
    let width = 60;
    let mut chart = ChartContext::new(width, 10);
    let data_bars = vec![
        (10.0, Some(Color::Red)),
        (20.0, Some(Color::Green)),
        (30.0, Some(Color::Blue)),
        (40.0, Some(Color::Yellow)),
        (50.0, Some(Color::Magenta)),
        (100.0, Some(Color::BrightMagenta)),
    ];

    chart.bar_chart(&data_bars);

    let labels = vec!["Ene", "Feb", "Mar", "Abr", "May", "Jun"];
    let num_bars = data_bars.len();
    
    // Ancho de cada barra en caracteres
    let bar_char_width = width / num_bars;

    for i in 0..num_bars {
        let label = labels[i];
        
        // Calculamos la columna de carácter donde debe empezar para estar centrado
        let start_col = i * bar_char_width;
        let center_offset = (bar_char_width as i32 - label.len() as i32) / 2;
        let target_col = (start_col as i32 + center_offset).max(0) as f64;

        // Convertimos a coordenada normalizada (0.0 - 1.0)
        // El método .text usa (x_norm * (width - 1)) internamente
        let x_norm = target_col / (width as f64 - 1.0);

        // y_norm = 0.0 es la base
        chart.text(label, x_norm, 0.0, Some(Color::White));
    }

    println!("{}", chart.canvas.render());
}
