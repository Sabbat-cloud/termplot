use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};
use std::collections::VecDeque;
use std::f64::consts::PI;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use termplot_rs::ChartContext;

fn main() -> io::Result<()> {
    // Configuración inicial
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, cursor::Hide, terminal::Clear(ClearType::All))?;

    let (mut cols, mut rows) = terminal::size()?;
    let width = (cols as usize).saturating_sub(4);
    let height = (rows as usize).saturating_sub(4);

    let mut chart = ChartContext::new(width, height);
    let mut running = true;

    // --- ESTADO DE LA SIMULACIÓN ---
    let mut phase: f64 = 0.0;
    
    // Buffer para la onda (Osciloscopio)
    // Guardamos suficientes puntos para llenar el ancho en píxeles
    let max_points = chart.canvas.pixel_width();
    let mut wave_buffer: VecDeque<f64> = VecDeque::with_capacity(max_points);
    // Llenar con ceros iniciales
    for _ in 0..max_points { wave_buffer.push_back(0.0); }

    // Buffer para el espectro (Barras)
    let num_bars = 30;
    let mut spectrum: Vec<f64> = vec![0.0; num_bars];

    let mut frame_count = 0;
    let start_time = Instant::now();

    while running {
        // 1. Resize Check
        let (nc, nr) = terminal::size()?;
        if nc != cols || nr != rows {
            cols = nc; rows = nr;
            let w = (cols as usize).saturating_sub(4);
            let h = (rows as usize).saturating_sub(4);
            chart = ChartContext::new(w, h);
            // Ajustar buffer si el ancho cambia
            let new_max = chart.canvas.pixel_width();
            if new_max > wave_buffer.len() {
                wave_buffer.resize(new_max, 0.0);
            } else {
                wave_buffer.truncate(new_max);
            }
        } else {
            chart.canvas.clear();
        }

        // 2. Input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                if code == KeyCode::Char('q') || code == KeyCode::Esc {
                    running = false;
                }
            }
        }

        // 3. ACTUALIZAR DATOS (Simular señal de audio compleja)
        phase += 0.15;
        
        // Componer una señal con 3 ondas senoidales
        let signal = (phase).sin() * 0.5        // Baja frecuencia (Bass)
                   + (phase * 3.0).sin() * 0.3  // Medios
                   + (phase * 7.0).sin() * 0.1; // Agudos (ruido)
        
        // Scroll: Sacar el viejo, meter el nuevo
        wave_buffer.pop_front();
        wave_buffer.push_back(signal);

        // Simular espectro de frecuencias (Animación procedural)
        for i in 0..num_bars {
            // Un ruido Perlin simulado con senos
            let freq_val = ((phase * 0.5 + i as f64 * 0.5).sin() + 1.0) / 2.0;
            // Suavizado (Lerp) para que las barras no salten locamente
            spectrum[i] = spectrum[i] * 0.8 + freq_val * 0.2; 
        }

        // 4. RENDERIZADO
        let w_px = chart.canvas.pixel_width();
        let h_px = chart.canvas.pixel_height();
        let h_half = h_px / 2; // Dividir pantalla en dos

        // --- A) PARTE SUPERIOR: OSCILOSCOPIO (Línea) ---
        let mut points = Vec::with_capacity(w_px);
        for (i, &val) in wave_buffer.iter().enumerate() {
            // Mapear X al ancho total
            let x = i as f64;
            // Mapear Y a la mitad superior (0 a h_half)
            // Centrado en h_half / 2
            let center_y = (h_px as f64) * 0.75; 
            let amplitude = (h_px as f64) * 0.2;
            let y = center_y + val * amplitude;
            
            points.push((x, y));
        }
        
        // Dibujar rejilla superior
        chart.canvas.line_screen(0, h_half as isize, w_px as isize, h_half as isize, Some(Color::White)); // Separador
        chart.text("CH-A: WAVEFORM", 0.02, 0.55, Some(Color::Cyan));
        
        // Dibujar la onda manualmente (más rápido que chart.line_chart para datos ya en px)
        for w in points.windows(2) {
            chart.canvas.line_screen(
                w[0].0 as isize, w[0].1 as isize, 
                w[1].0 as isize, w[1].1 as isize, 
                Some(Color::BrightCyan)
            );
        }

        // --- B) PARTE INFERIOR: ANALIZADOR DE ESPECTRO (Barras) ---
        let bar_w = w_px / num_bars;
        let floor_y = (h_px as f64 * 0.45) as isize; // Base de las barras
        
        chart.text("CH-B: SPECTRUM", 0.02, 0.95, Some(Color::Magenta));

        for (i, &val) in spectrum.iter().enumerate() {
            let bar_h = (val * (h_px as f64 * 0.4)) as isize;
            let x_start = (i * bar_w) as isize;
            let x_end = x_start + (bar_w as isize).max(1) - 1; // Espacio entre barras

            // Color gradiente según altura
            let color = if val > 0.8 { Color::Red } 
                       else if val > 0.5 { Color::Yellow } 
                       else { Color::Magenta };

            for x in x_start..x_end {
                // Dibujar línea vertical desde el suelo hacia arriba
                // En coordenadas de pantalla Y crece hacia abajo, así que:
                // floor_y es abajo, floor_y - bar_h es arriba
                chart.canvas.line_screen(x, floor_y, x, floor_y - bar_h, Some(color));
            }
        }

        // --- C) OVERLAY DE INFORMACIÓN ---
        let uptime = start_time.elapsed().as_secs();
        let info = format!("REC [●] | T: {:02}:{:02} | FPS: High", uptime / 60, uptime % 60);
        chart.text(&info, 0.7, 0.05, Some(Color::Red));

        // Output
        execute!(stdout, cursor::MoveTo(0, 0))?;
        let output = chart.canvas.render_with_options(true, Some("AUDIO VISUALIZER SIMULATION"));
        print!("{}", output.replace('\n', "\r\n"));
        io::stdout().flush()?;

        frame_count += 1;
        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS cap
    }

    execute!(stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
