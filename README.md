# termplot

[![Crates.io](https://img.shields.io/crates/v/termplot.svg)](https://crates.io/crates/termplot)
[![Docs.rs](https://img.shields.io/docsrs/termplot)](https://docs.rs/termplot)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![CI](https://img.shields.io/github/actions/workflow/status/Sabbat-cloud/termplot/ci.yml?branch=main)](https://github.com/Sabbat-cloud/termplot/actions)

Gráficos en consola (TUI) usando **Unicode Braille** (2×4 “píxeles” por carácter) y color ANSI.
Permite dibujar **scatter plots, líneas, barras, polígonos, círculos, rejillas, ejes, texto** y animaciones
en terminal, sin depender de GUI.

> Ideal para logs, dashboards ligeros, debugging visual, CLI tools, demos y “plots” rápidos en SSH.

---

## ✨ Características

- Canvas de alta densidad con **Braille (2×4)** → más resolución que ASCII clásico.
- **Color por celda de carácter** (ANSI / TrueColor si tu terminal lo soporta).
- **Auto-Range (v0.8+)**: cálculo automático de escalas para tus datos.
- **Modo No-Color (v0.8+)**: renderizado sin códigos ANSI para logs planos y terminales antiguos.
- Primitivas de dibujo:
  - `set_pixel`
  - `line` (Bresenham)
  - `circle` (punto medio)
  - `set_char`, `set_char_vertical`
- Charts:
  - `scatter()` nube de puntos
  - `line_chart()` serie conectada
  - `bar_chart()` barras (con auto-ancho y protección de límites)
  - `polygon()` polígonos
  - `pie_chart()` “pie radar” (radios + contorno)
  - `draw_grid()` rejilla
  - `draw_axes()` ejes + etiquetas min/max
  - `plot_function()` plotea funciones `f(x)` directamente
  - `text()` capa de texto
- Animación por frames sobrescribiendo el cursor (sin limpiar pantalla completa).

---

## 📦 Instalación

### Crates.io

```toml
[dependencies]
termplot-rs = "0.1.0"
````

### Desde repo (path local)

```toml
[dependencies]
termplot-rs = { path = "../termplot" }
```

---

## 🚀 Quick start

```rust
use termplot_rs::ChartContext;
use colored::Color;

fn main() {
    let mut chart = ChartContext::new(60, 15);

    let points = vec![
        (0.0, 0.0),
        (10.0, 50.0),
        (20.0, 20.0),
        (40.0, 80.0),
    ];

    let (range_x, range_y) = ChartContext::get_auto_range(&points, 0.1);

    chart.draw_axes(range_x, range_y, Some(Color::White));
    chart.line_chart(&points, Some(Color::Cyan));

    println!("{}", chart.canvas.render());
}
```

---

## 🧪 Demo / Examples

El proyecto incluye una demo completa en `examples/demo.rs`.

```bash
cargo run --example demo
```

> Tip: si quieres comprobar solo compilación del ejemplo:
>
> ```bash
> cargo build --example demo
> ```

---

## 🧱 Renderizado

### Estándar (con bordes)

```rust
println!("{}", chart.canvas.render());
```

### Con título y sin bordes (dashboards minimalistas)

```rust
println!("{}", chart.canvas.render_with_options(false, Some("RENDIMIENTO CPU")));
```

### No-Color (plain text / logs)

```rust
println!("{}", chart.canvas.render_no_color());
```

---

## 🧠 Conceptos básicos

El canvas se define en **caracteres**: `(width, height)`.
Cada carácter Braille contiene **2×4 subpíxeles**, así que la resolución real es:

* `pixel_width = width * 2`
* `pixel_height = height * 4`

---

## ⚙️ Features

Actualmente:

* `render_no_color()` produce salida sin ANSI (ideal para logs).
* El crate usa `colored` para colorear el render estándar.

> Nota: aunque `colored` puede estar definido como `optional` en `Cargo.toml`, el código
> todavía no está “feature-gated” por completo. En roadmap: soportar `--no-default-features`
> de forma limpia con `cfg(feature="color")`.

---

## ✅ Compatibilidad (MSRV)

* Rust **edition 2024** (necesitas una toolchain que soporte 2024).
* Terminal con buen soporte Unicode recomendado.

---

## 🧰 Desarrollo

```bash
cargo fmt
cargo clippy
cargo test
cargo run --example demo
```

Generar docs:

```bash
cargo doc --open
```

---

## 📦 Publicación (checklist rápida)

Antes de publicar en crates.io:

1. Asegúrate de tener `LICENSE` (MIT) y `README.md` en el repo.
2. Revisa `Cargo.toml`:

   * `description`, `license`, `repository`, `homepage` (si aplica)
   * `keywords`, `categories` (opcional pero recomendable)
3. Ejecuta:

```bash
cargo fmt
cargo clippy
cargo test
cargo package
```

Y publica:

```bash
cargo publish
```

---

## 🗺️ Roadmap

* [x] Mejor manejo de límites en `text()` y `bar_chart()`.
* [x] Cálculo de auto-range con padding configurable.
* [x] Renderizado sin color ANSI (`render_no_color()`).
* [x] Títulos centrados opcionales (`render_with_options`).
* [ ] Hacer `colored` realmente opcional con `cfg(feature="color")`.
* [ ] Leyendas automáticas (Series A, B, C).
* [ ] Histogramas de frecuencia.

---

## 📄 Licencia

MIT

