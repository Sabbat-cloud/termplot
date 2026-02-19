use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};
//use std::f64::consts::PI; No usamos angle de momento
use std::io::{self, Write};
use std::time::{Duration, Instant};
use termplot_rs::ChartContext;

// Estructura simple para partículas
// Quito angle para evitar un warning ya que no se usa en esta version
struct Particle {
    x: f64,
    y: f64,
    speed: f64,
    //angle: f64,
}

fn main() -> io::Result<()> {
    // Configuración de terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, cursor::Hide, terminal::Clear(ClearType::All))?;

    // Detección inicial
    let (term_cols, term_rows) = terminal::size()?;
    let mut width = (term_cols as usize).saturating_sub(2);
    let mut height = (term_rows as usize).saturating_sub(4);

    // Creamos el contexto UNA sola vez (reutilización de memoria)
    let mut chart = ChartContext::new(width, height);

    let mut t: f64 = 0.0;
    let mut running = true;

    // FPS
    let mut frames = 0;
    let mut last_time = Instant::now();
    let mut fps = 0;

    // Generamos 1000 partículas para estrés adicional
    let mut particles: Vec<Particle> = (0..1000).map(|_| {
        Particle {
            x: rand::random::<f64>() * (width * 2) as f64,
            y: rand::random::<f64>() * (height * 4) as f64,
            speed: 1.0 + rand::random::<f64>() * 3.0,
            //angle: rand::random::<f64>() * PI * 2.0, Por ahora no se usa
        }
    }).collect();

    while running {
        let frame_start = Instant::now();

        // 1. Resize Handling
        let (nc, nr) = terminal::size()?;
        let nw = (nc as usize).saturating_sub(2);
        let nh = (nr as usize).saturating_sub(4);
        if nw != width || nh != height {
            width = nw;
            height = nh;
            chart = ChartContext::new(width, height);
            // Re-spawn particles to fit screen
            particles.iter_mut().for_each(|p| {
                p.x = rand::random::<f64>() * (width * 2) as f64;
                p.y = rand::random::<f64>() * (height * 4) as f64;
            });
        } else {
            chart.canvas.clear();
        }

        // 2. Input Handling
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                if code == KeyCode::Char('q') || code == KeyCode::Esc {
                    running = false;
                }
            }
        }

        // 3. RENDERIZADO DEL PLASMA (Intensivo por píxel)
        // Usamos los getters optimizados
        let w_px = chart.canvas.pixel_width();
        let h_px = chart.canvas.pixel_height();
        let w_float = w_px as f64;
        let h_float = h_px as f64;

        // Iteramos sobre CADA PÍXEL de la pantalla
        // Nota: Rayon (paralelismo) haría esto volar, pero queremos probar
        // la velocidad "single-threaded" de tu librería.
        for py in (0..h_px).step_by(2) { // Step 2 para optimizar un poco la demo visual, o 1 para full estrés
            for px in (0..w_px).step_by(2) {
                let x = px as f64;
                let y = py as f64;

                // Matemáticas complejas para generar patrones de interferencia
                // Valor 1: Seno moviéndose horizontalmente
                let v1 = (x * 0.02 + t).sin();
                
                // Valor 2: Seno moviéndose en diagonal
                let v2 = ((x * 0.5 + y * 0.5) * 0.02 + t * 1.5).sin(); // [Image of sine wave interference]

                // Valor 3: Distancia circular desde un punto móvil
                let cx = w_float / 2.0 + (t * 1.2).cos() * (w_float / 3.0);
                let cy = h_float / 2.0 + (t * 2.3).sin() * (h_float / 3.0);
                let dist = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
                let v3 = (dist * 0.04 - t * 2.0).sin();

                // Suma y normalización (-1 a 1 -> 0 a 1)
                let v = (v1 + v2 + v3) / 3.0;
                
                // Mapeo a colores ANSI básicos según la intensidad
                let color = if v > 0.8 { Some(Color::White) }
                else if v > 0.4 { Some(Color::Cyan) }
                else if v > 0.0 { Some(Color::Blue) }
                else if v > -0.4 { Some(Color::Magenta) }
                else { Some(Color::Red) }; // Fondo "oscuro"

                // Dibujamos un bloque de 2x2 para rellenar rápido (optimización visual)
                if let Some(c) = color {
                    chart.canvas.set_pixel_screen(px, py, Some(c));
                    chart.canvas.set_pixel_screen(px + 1, py, Some(c));
                    chart.canvas.set_pixel_screen(px, py + 1, Some(c));
                    chart.canvas.set_pixel_screen(px + 1, py + 1, Some(c));
                }
            }
        }

        // 4. RENDERIZADO DE PARTÍCULAS (Sobre el plasma)
        for p in &mut particles {
            // Física simple
            p.y += p.speed;
            p.x += (t * 2.0).sin(); // Viento

            // Reset si sale de pantalla
            if p.y >= h_float {
                p.y = 0.0;
                p.x = rand::random::<f64>() * w_float;
            }
            if p.x >= w_float { p.x = 0.0; }
            if p.x < 0.0 { p.x = w_float; }

            // Dibujar partícula (Punto brillante)
            // Usamos set_pixel_screen directo
            chart.canvas.set_pixel_screen(p.x as usize, p.y as usize, Some(Color::BrightYellow));
        }

        // 5. SALIDA A PANTALLA
        execute!(stdout, cursor::MoveTo(0, 0))?;
        
        // Creamos el string del frame
        let output = chart.canvas.render_with_options(true, Some("PLASMA STRESS TEST (Math + Particles)"));
        
        // Imprimimos de golpe
        // Usamos write_all para evitar formateos extra de print!
        stdout.write_all(output.replace('\n', "\r\n").as_bytes())?;
        
        // Info debug
        let debug_info = format!("\r\nFPS: {} | Res: {}x{} px | Pts: {}", fps, w_px, h_px, w_px * h_px);
        stdout.write_all(debug_info.as_bytes())?;
        stdout.flush()?;

        // 6. CÁLCULO DE TIEMPO
        t += 0.05;
        frames += 1;
        if last_time.elapsed() >= Duration::from_secs(1) {
            fps = frames;
            frames = 0;
            last_time = Instant::now();
        }

        // Limitar a ~60 FPS si va muy rápido (en release mode debería volar)
        let elapsed = frame_start.elapsed();
        if elapsed < Duration::from_millis(16) {
            std::thread::sleep(Duration::from_millis(16) - elapsed);
            // Comentado para dejar que corra al MAXIMO posible y ver los FPS reales
        }
    }

    execute!(stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
