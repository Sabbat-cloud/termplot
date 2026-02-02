# termplot

Gráficos en consola (TUI) usando **Unicode Braille** (2×4 “píxeles” por carácter) y color ANSI.
Permite dibujar **scatter plots, líneas, barras, polígonos, círculos, rejillas, ejes, texto** y animaciones
en terminal, sin depender de GUI.

> Ideal para logs, dashboards ligeros, debugging visual, CLI tools, demos y “plots” rápidos en SSH.

---

## ✨ Características

- Canvas de alta densidad con **Braille (2×4)** → más resolución que ASCII clásico.
- **Color por celda de carácter** (ANSI / TrueColor si tu terminal lo soporta).
- Primitivas: `set_pixel`, `line` (Bresenham), `circle` (punto medio), `set_char`.
- Charts:
  - `scatter()` nube de puntos
  - `line_chart()` serie conectada
  - `bar_chart()` barras
  - `polygon()` polígonos
  - `pie_chart()` “pie radar” (radios + contorno)
  - `draw_grid()` rejilla
  - `draw_axes()` ejes + etiquetas min/max
  - `plot_function()` plotea funciones `f(x)` directamente
  - `text()` capa de texto
- Animación por frames sobrescribiendo el cursor (sin limpiar pantalla completa).

---

## 📦 Instalación

### Cargo.toml

```toml
[dependencies]
termplot = { path = "." } # o desde crates.io cuando lo publiques
colored = "2"
````

> En tus fuentes: `use colored::Color;`

---

## 🚀 Quick start

Un scatter plot mínimo:

```rust
use termplot::charts::ChartContext;
use colored::Color;

fn main() {
    let mut chart = ChartContext::new(60, 15);

    let points = vec![
        (0.0, 0.0),
        (1.0, 2.0),
        (2.0, 3.0),
        (3.0, 5.0),
        (5.0, 8.0),
    ];

    chart.scatter(&points, Some(Color::Cyan));
    println!("{}", chart.canvas.render());
}
```

---

## 🧠 Conceptos básicos

### Resolución real (píxeles virtuales)

El canvas se define en **caracteres**: `(width, height)`.
Pero cada carácter Braille contiene **2×4 subpíxeles**, así que la resolución real es:

* `pixel_width = width * 2`
* `pixel_height = height * 4`

Cuando llamas a `set_pixel(px, py, color)` usas coordenadas de esos píxeles virtuales.

---

## 🧱 API principal

### `BrailleCanvas`

```rust
let mut c = BrailleCanvas::new(60, 15);
c.set_pixel(10, 5, Some(Color::Red));
c.line(0, 0, 50, 40, Some(Color::Green));
c.circle(30, 20, 10, Some(Color::Blue));
println!("{}", c.render());
```

### `ChartContext`

```rust
let mut chart = ChartContext::new(60, 15);

// charts dibujan sobre chart.canvas
chart.draw_grid(10, 4, Some(Color::TrueColor{ r:60, g:60, b:60 }));
chart.plot_function(|x| x.sin(), 0.0, 10.0, Some(Color::Cyan));
println!("{}", chart.canvas.render());
```

---

# ✅ Ejemplos (de básico a avanzado)

## 1) Barras (básico)

```rust
use termplot::charts::ChartContext;
use colored::Color;

fn main() {
    let mut chart = ChartContext::new(60, 10);

    let data = vec![
        (30.0, Some(Color::Red)),
        (55.0, Some(Color::Green)),
        (90.0, Some(Color::Blue)),
        (45.0, Some(Color::Yellow)),
        (70.0, Some(Color::Magenta)),
        (25.0, None),
    ];

    chart.bar_chart(&data);
    println!("{}", chart.canvas.render());
}
```

---

## 2) Scatter con dos series (básico/intermedio)

```rust
use termplot::charts::ChartContext;
use colored::Color;

fn main() {
    let mut chart = ChartContext::new(60, 15);

    let series_a = (0..120).map(|i| (i as f64 * 0.5, (i as f64).sin()*10.0 + 20.0)).collect::<Vec<_>>();
    let series_b = (0..120).map(|i| (i as f64 * 0.5, (i as f64).cos()*10.0 + 20.0)).collect::<Vec<_>>();

    chart.scatter(&series_a, Some(Color::Red));
    chart.scatter(&series_b, Some(Color::Cyan));
    chart.text("A=sin, B=cos", 0.35, 0.92, Some(Color::White));

    println!("{}", chart.canvas.render());
}
```

---

## 3) Geometría: círculo + triángulo (intermedio)

```rust
use termplot::charts::ChartContext;
use colored::Color;

fn main() {
    let mut chart = ChartContext::new(40, 20);

    chart.draw_circle((0.5, 0.5), 0.40, Some(Color::Green));
    let triangle = vec![(0.1, 0.1), (0.5, 0.9), (0.9, 0.1)];
    chart.polygon(&triangle, Some(Color::Magenta));
    chart.text("Geometria", 0.32, 0.95, Some(Color::White));

    println!("{}", chart.canvas.render());
}
```

---

## 4) “Pie chart” estilo radar (intermedio)

> Este gráfico dibuja radios proporcionales (no rellena sectores). Útil para “composición” visual rápida.

```rust
use termplot::charts::ChartContext;
use colored::Color;

fn main() {
    let mut chart = ChartContext::new(40, 20);

    let pie = vec![
        (30.0, Some(Color::Red)),
        (20.0, Some(Color::Blue)),
        (15.0, Some(Color::Green)),
        (25.0, Some(Color::Yellow)),
        (10.0, Some(Color::White)),
    ];

    chart.pie_chart(&pie);
    chart.text("Distribucion", 0.28, 0.95, Some(Color::White));

    println!("{}", chart.canvas.render());
}
```

---

## 5) Línea conectando puntos (intermedio)

```rust
use termplot::charts::ChartContext;
use colored::Color;

fn main() {
    let mut chart = ChartContext::new(60, 15);

    let points = (0..200)
        .map(|i| {
            let x = i as f64 / 20.0;
            let y = (x.sin() * 1.0) + (x.cos() * 0.3);
            (x, y)
        })
        .collect::<Vec<_>>();

    chart.line_chart(&points, Some(Color::Cyan));
    println!("{}", chart.canvas.render());
}
```

---

## 6) Función matemática + rejilla + ejes (avanzado)

```rust
use termplot::charts::ChartContext;
use colored::Color;

fn main() {
    let mut chart = ChartContext::new(60, 15);

    chart.draw_grid(10, 4, Some(Color::TrueColor{ r:80, g:80, b:80 }));
    chart.draw_axes((0.0, 10.0), (-1.5, 1.5), Some(Color::White));

    chart.plot_function(|x| x.sin(), 0.0, 10.0, Some(Color::Cyan));
    chart.plot_function(|x| (x*0.5).cos()*0.5, 0.0, 10.0, Some(Color::Magenta));

    chart.text("sin(x)", 0.75, 0.85, Some(Color::Cyan));
    chart.text("0.5*cos(0.5x)", 0.55, 0.10, Some(Color::Magenta));

    println!("{}", chart.canvas.render());
}
```

---

## 7) Animación (avanzado)

Animación de dos funciones con rejilla. Se renderiza frame a frame y se “rebobina” el cursor.

```rust
use termplot::charts::ChartContext;
use colored::Color;
use std::{thread, time};
use std::io::{self, Write};

fn main() {
    let width = 60;
    let height = 15;
    let mut chart = ChartContext::new(width, height);

    let lines_to_rewind = height + 2 + 1; // canvas + marco + línea extra
    let mut phase = 0.0;

    loop {
        chart.canvas.clear();

        chart.draw_grid(10, 4, Some(Color::TrueColor{ r:60, g:60, b:60 }));
        chart.draw_axes((0.0, 10.0), (-1.5, 1.5), Some(Color::White));

        chart.plot_function(|x| (x + phase).sin() * (x * 0.5).cos(), 0.0, 10.0, Some(Color::Cyan));
        chart.plot_function(|x| ((x - phase * 1.5).cos() * 0.5) - 0.5, 0.0, 10.0, Some(Color::Magenta));

        chart.text("Sistema Dual", 0.38, 0.90, Some(Color::Yellow));

        println!("{}", chart.canvas.render());
        println!("phase: {:.2}", phase);

        thread::sleep(time::Duration::from_millis(50));

        print!("\x1B[{}A", lines_to_rewind);
        io::stdout().flush().unwrap();

        phase += 0.1;
    }
}
```

---

## 🎛️ Consejos de uso

* Usa una **fuente monoespaciada** (obligatorio en terminal).
* Braille requiere **Unicode**.
* Para TrueColor: terminal moderno (Windows Terminal, iTerm2, kitty, Alacritty, etc.).

---

## 🗺️ Roadmap (ideas para v0.8+)

* Mejor manejo de límites en `text()` y en `bar_chart()` cuando hay muchas barras.
* “Auto-range” configurable: padding y clamping.
* Leyenda / labels verticales.
* Modo “no color” o feature flags.
* Export simple: render sin marco / con marco / con título.

---

## 📄 Licencia

MIT 
---
