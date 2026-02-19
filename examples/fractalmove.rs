use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};
use std::io::{self, Write};
use std::time::Duration;
use termplot_rs::ChartContext;

fn main() -> io::Result<()> {
    loop {
        // Limpiamos pantalla para el menú
        execute!(io::stdout(), terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        println!("{}", "=== FRACTAL EXPLORER ===".bright_cyan().bold());
        println!("1. {}", "Mandelbrot (Zoom/Pan/Rotate)".green());
        println!("2. {}", "Julia (Zoom/Pan/Rotate)".green());
        println!("3. {}", "Lorenz 3D (Rotate/Zoom)".yellow());
        println!("q. Exit");
        println!("ESC. To close fractal");
        print!("\nOption > ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => run_interactive_mandelbrot(false)?,
            "2" => run_interactive_mandelbrot(true)?,
            "3" => run_interactive_lorenz()?,
            "q" => break,
            _ => continue,
        }
    }
    // Restaurar cursor al salir
    execute!(io::stdout(), cursor::Show)?;
    Ok(())
}

fn run_interactive_mandelbrot(is_julia: bool) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    let mut center_x: f64 = -0.75;
    let mut center_y: f64 = 0.0;
    let mut zoom: f64 = 1.0;
    let mut rotation: f64 = 0.0;

    // Parámetros para Julia
    let c_re = -0.7;
    let c_im = 0.27015;

    let mut running = true;

    // OPTIMIZACIÓN: Inicializamos el chart fuera del loop para reutilizar buffers
    let (cols, rows) = terminal::size()?;
    let mut width = (cols as usize).saturating_sub(4);
    let mut height = (rows as usize).saturating_sub(4);
    let mut chart = ChartContext::new(width, height);

    while running {
        // 1. Detección de tamaño (Responsive)
        let (cols, rows) = terminal::size()?;
        let new_w = (cols as usize).saturating_sub(4);
        let new_h = (rows as usize).saturating_sub(4);

        // Solo recreamos memoria si la terminal cambia de tamaño
        if new_w != width || new_h != height {
            width = new_w;
            height = new_h;
            if width < 10 || height < 5 {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
            chart = ChartContext::new(width, height);
        } else {
            // Si no cambia, limpiamos el buffer existente (muy rápido)
            chart.canvas.clear();
        }

        // 2. Input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Esc => running = false,
                    // Zoom
                    KeyCode::Char('z') => zoom *= 1.2,
                    KeyCode::Char('x') => zoom /= 1.2,
                    // Movimiento
                    KeyCode::Right => center_x += 0.15 / zoom,
                    KeyCode::Left => center_x -= 0.15 / zoom,
                    KeyCode::Up => center_y += 0.15 / zoom,
                    KeyCode::Down => center_y -= 0.15 / zoom,
                    // Rotación
                    KeyCode::Char('e') => rotation += 0.1,
                    KeyCode::Char('q') => rotation -= 0.1,
                    _ => {}
                }
            }
        }

        // 3. Renderizado Matemático
        // Usamos los getters optimizados del canvas
        let w_px = chart.canvas.pixel_width();
        let h_px = chart.canvas.pixel_height();
        let w_px_f = w_px as f64;
        let h_px_f = h_px as f64;
        let aspect_ratio = 0.5;

        let max_iter = (50.0 + 15.0 * zoom.ln()).min(300.0) as usize;

        // Iteramos píxeles de pantalla
        for py in 0..h_px {
            for px in 0..w_px {
                // Mapeo Píxel -> Coordenada Matemática
                let u = (px as f64 - w_px_f / 2.0) / (w_px_f / 2.0);
                let v = (py as f64 - h_px_f / 2.0) / (h_px_f / 2.0);

                // Rotación
                let u_rot = u * rotation.cos() - v * rotation.sin();
                let v_rot = u * rotation.sin() + v * rotation.cos();

                let x0 = center_x + (u_rot * aspect_ratio * 2.5 / zoom);
                let y0 = center_y + (v_rot * 2.5 / zoom);

                let mut iter = 0;

                if !is_julia {
                    // Mandelbrot
                    let mut x = 0.0;
                    let mut y = 0.0;
                    let mut x2 = 0.0;
                    let mut y2 = 0.0;
                    while x2 + y2 <= 4.0 && iter < max_iter {
                        y = 2.0 * x * y + y0;
                        x = x2 - y2 + x0;
                        x2 = x * x;
                        y2 = y * y;
                        iter += 1;
                    }
                } else {
                    // Julia
                    let mut new_re = x0;
                    let mut new_im = y0;
                    while iter < max_iter {
                        let old_re = new_re;
                        let old_im = new_im;
                        new_re = old_re * old_re - old_im * old_im + c_re;
                        new_im = 2.0 * old_re * old_im + c_im;
                        if (new_re * new_re + new_im * new_im) > 4.0 { break; }
                        iter += 1;
                    }
                }

                if iter < max_iter {
                    let color = match iter % 8 {
                        0..=1 => Color::Blue,
                        2..=3 => Color::Cyan,
                        4..=5 => Color::Magenta,
                        6 => Color::Red,
                        _ => Color::White,
                    };
                    // IMPORTANTE: Usamos set_pixel_screen porque estamos barriendo el raster de pantalla (py crece hacia abajo)
                    // Si usáramos set_pixel, invertiría Y y el fractal saldría espejo vertical.
                    chart.canvas.set_pixel_screen(px, py, Some(color));
                }
            }
        }

        execute!(stdout, cursor::MoveTo(0, 0))?;
        let title = if is_julia { "JULIA" } else { "MANDELBROT" };
        let info = format!("{} (Zoom: {:.1}x | Iter: {})", title, zoom, max_iter);
        
        // Renderizado optimizado
        let output = chart.canvas.render_with_options(true, Some(&info));
        
        // Escribimos directamente, evitando replaces costosos si es posible. 
        // En Unix \n suele bastar, pero mantenemos replace para compatibilidad máxima si es necesario,
        // aunque crossterm suele manejar esto bien en raw mode.
        print!("{}", output.replace('\n', "\r\n"));
        io::stdout().flush()?;
    }

    disable_raw_mode()?;
    Ok(())
}

fn run_interactive_lorenz() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    // Pre-cálculo de Lorenz (igual que antes)
    let mut points_3d = Vec::new();
    let mut x = 0.1; let mut y = 0.0; let mut z = 0.0;
    let dt = 0.01;
    for _ in 0..1500 {
        let dx = 10.0 * (y - x);
        let dy = x * (28.0 - z) - y;
        let dz = x * y - (8.0/3.0) * z;
        x += dx * dt; y += dy * dt; z += dz * dt;
        points_3d.push((x, y, z));
    }

    let mut angle_y: f64 = 0.0;
    let mut angle_x: f64 = 0.0;
    let mut zoom: f64 = 1.0;
    let mut running = true;

    // Inicialización fuera del loop
    let (cols, rows) = terminal::size()?;
    let mut width = (cols as usize).saturating_sub(4);
    let mut height = (rows as usize).saturating_sub(4);
    let mut chart = ChartContext::new(width, height);

    while running {
        // 1. Resize Check
        let (cols, rows) = terminal::size()?;
        let new_w = (cols as usize).saturating_sub(4);
        let new_h = (rows as usize).saturating_sub(4);

        if new_w != width || new_h != height {
            width = new_w;
            height = new_h;
             if width < 10 || height < 5 { continue; }
            chart = ChartContext::new(width, height);
        } else {
            chart.canvas.clear();
        }

        // 2. Input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Esc => running = false,
                    KeyCode::Right => angle_y += 0.1,
                    KeyCode::Left => angle_y -= 0.1,
                    KeyCode::Up => angle_x -= 0.1,
                    KeyCode::Down => angle_x += 0.1,
                    KeyCode::Char('z') => zoom += 0.1,
                    KeyCode::Char('x') => zoom = (zoom - 0.1).max(0.1),
                    _ => {}
                }
            }
        }

        // 3. Renderizado 3D
        let canvas_w = chart.canvas.pixel_width() as f64;
        let canvas_h = chart.canvas.pixel_height() as f64;

        // Proyección y Dibujo Manual
        // NO usamos chart.line_chart() aquí porque line_chart intenta calcular auto-range.
        // Como nosotros ya estamos calculando las coordenadas exactas de pantalla (proyección),
        // queremos dibujar líneas directas entre píxeles.

        let mut prev_point: Option<(isize, isize)> = None;

        for &(px, py, pz) in &points_3d {
            let mut rx = px;
            let mut ry = py;
            let mut rz = pz - 25.0;

            // Rotación Y
            let tmp_x = rx * angle_y.cos() - rz * angle_y.sin();
            let tmp_z = rx * angle_y.sin() + rz * angle_y.cos();
            rx = tmp_x; rz = tmp_z;

            // Rotación X
            let tmp_y = ry * angle_x.cos() - rz * angle_x.sin();
            // Actualmente no se usa rz en la rotación final, solo se proyecta X e Y en la pantalla
            //rz = ry * angle_x.sin() + rz * angle_x.cos();
            ry = tmp_y;

            // Proyección
            let screen_x = (rx * zoom * 2.0) + (canvas_w / 2.0);
            let screen_y = (canvas_h / 2.0) - (ry * zoom); // Y crece abajo en pantalla

            let current_point = (screen_x as isize, screen_y as isize);

            if let Some(prev) = prev_point {
                // Usamos line_screen porque ya tenemos coordenadas de pantalla calculadas manualmente
                chart.canvas.line_screen(prev.0, prev.1, current_point.0, current_point.1, Some(Color::BrightGreen));
            }
            prev_point = Some(current_point);
        }

        execute!(stdout, cursor::MoveTo(0, 0))?;
        let output = chart.canvas.render_with_options(true, Some("LORENZ 3D (Arrows/Zoom)"));
        print!("{}", output.replace('\n', "\r\n"));
        io::stdout().flush()?;

        std::thread::sleep(Duration::from_millis(30));
    }

    disable_raw_mode()?;
    Ok(())
}

fn enable_raw_mode() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(io::stdout(), cursor::Hide, terminal::Clear(ClearType::All))
}

fn disable_raw_mode() -> io::Result<()> {
    execute!(io::stdout(), cursor::Show)?;
    terminal::disable_raw_mode()
}
