use termplot_rs::prelude::*;
use colored::Color;
use std::f64::consts::PI;
use std::thread;
use std::time::Duration;
use std::io::{self, Write as IoWrite};

fn main() {
    // 1. Limpiamos la pantalla inicial y ocultamos el cursor
    print!("\x1b[2J\x1b[H\x1b[?25l");
    io::stdout().flush().unwrap();
    
    let mut time: f64 = 0.0;

    loop {
        let mut out_buffer = String::with_capacity(16000);
        
        // --- CABECERA ---
        out_buffer.push_str("\x1b[1;36m=== 🌐 GLOBAL INFRASTRUCTURE COMMAND CENTER ===\x1b[0m\n\n");

        // --- PANEL 1: TELEMETRÍA DE RED ---
        let mut net_chart = ChartContext::new(70, 9);
        net_chart.set_scales(AxisScale::Linear, AxisScale::Log10);
        
        let mut net_data = Vec::new();
        for i in 0..70 {
            let x = i as f64;
            let base = 15.0 + (x * 0.2 + time * 2.0).sin() * 5.0;
            let spike = if (x - (time * 15.0) % 70.0).abs() < 1.5 { 2500.0 } else { 0.0 };
            let noise = (x * 7.0).cos() * 2.0;
            net_data.push((x, (base + spike + noise).max(1.0)));
        }

        net_chart.draw_grid(7, 3, Some(Color::BrightBlack));
        net_chart.draw_axes((0.0, 70.0), (1.0, 5000.0), Some(Color::BrightBlack));
        net_chart.line_chart(&net_data, Some(Color::BrightRed));
        net_chart.text("Gateway Latency (ms) [LOG10]", 0.02, 0.85, Some(Color::BrightYellow));
        
        let _ = net_chart.canvas.render_to(&mut out_buffer, true, Some("NETWORK TELEMETRY"));
        out_buffer.push('\n'); // Espacio extra

        // --- PANEL 2: COMPUTE CLUSTER ---
        let mut topo_chart = ChartContext::new(70, 9);
        
        let mut nodes = Vec::new();
        for i in 0..15 {
            let nx = (i as f64 * 13.7 + time).sin().abs();
            let ny = (i as f64 * 7.3 - time * 0.5).cos().abs();
            nodes.push((nx, ny));
        }
        
        topo_chart.polygon(&nodes[0..5], Some(Color::BrightBlue));
        topo_chart.scatter(&nodes, Some(Color::BrightCyan));
        topo_chart.text("Active Cluster Topology", 0.02, 0.85, Some(Color::White));

        let mem_usage = (50.0 + (time * 0.5).sin() * 40.0) as usize; 
        let max_w = topo_chart.canvas.pixel_width();
        let bar_w = (mem_usage * max_w) / 100;
        
        topo_chart.canvas.rect_filled(
            0, 
            topo_chart.canvas.pixel_height() as isize - 4, 
            bar_w, 
            4, 
            Some(Color::BrightMagenta)
        );
        topo_chart.text(&format!("MEMORIA GLOBAL: {}%", mem_usage), 0.7, 0.05, Some(Color::BrightMagenta));

        let _ = topo_chart.canvas.render_to(&mut out_buffer, true, Some("COMPUTE CLUSTER"));
        out_buffer.push('\n'); // Espacio extra

        // --- PANEL 3: RADAR DE SEGURIDAD ---
        let mut radar = ChartContext::new(70, 11);
        radar.canvas.blend_mode = ColorBlend::KeepFirst; 

        let cx = radar.canvas.pixel_width() as isize / 2;
        let cy = radar.canvas.pixel_height() as isize / 2;

        radar.canvas.circle(cx, cy, 10, Some(Color::Green));
        radar.canvas.circle(cx, cy, 20, Some(Color::Green));
        radar.canvas.circle(cx, cy, 30, Some(Color::Green));

        let sweep_x = cx + ((time * 2.0).cos() * 40.0) as isize;
        let sweep_y = cy + ((time * 2.0).sin() * 40.0) as isize;
        radar.canvas.line(cx, cy, sweep_x, sweep_y, Some(Color::BrightGreen));

        let mut threat_poly = Vec::new();
        for i in 0..3 {
            let angle = (i as f64 / 3.0) * 2.0 * PI - time;
            let radius = 0.2 + 0.05 * (time * 5.0).sin();
            threat_poly.push((0.5 + angle.cos() * radius, 0.5 + angle.sin() * radius));
        }
        radar.polygon(&threat_poly, Some(Color::BrightRed));
        radar.text("Intrusion Detection Sweep", 0.02, 0.9, Some(Color::White));

        let pulse_r = (20.0 + (time * 3.0).sin() * 5.0) as isize;
        radar.canvas.circle_filled(cx, cy, pulse_r, Some(Color::BrightBlack));

        let _ = radar.canvas.render_to(&mut out_buffer, true, Some("SECURITY RADAR"));
        out_buffer.push('\n');

        // 2. MAGIA DEL REBOBINADO: Contamos cuántas líneas exactas tiene nuestro frame
        let lines_to_rewind = out_buffer.lines().count();

        // 3. Imprimimos el frame entero de golpe
        print!("{}", out_buffer);
        io::stdout().flush().unwrap();

        // 4. Pausa de animación (20 FPS)
        thread::sleep(Duration::from_millis(50));
        
        // 5. Retrocedemos el cursor exactamente 'lines_to_rewind' posiciones hacia arriba
        // y lo colocamos en la columna 1 (\x1b[G) para el siguiente frame
        print!("\x1B[{}A\x1B[G", lines_to_rewind);
        io::stdout().flush().unwrap();
        
        time += 0.1;
    }
}
