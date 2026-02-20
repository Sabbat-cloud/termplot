# 📈 termplot-rs

**Motor de gráficos de terminal de alto rendimiento (TUI).**

`termplot-rs` te permite renderizar gráficos matemáticos, visualizaciones en 3D, juegos e interfaces complejas directamente en la consola utilizando **caracteres Braille Unicode** (matriz de puntos 2×4 por carácter) y colores ANSI.

A diferencia de otras bibliotecas de gráficos para TUI, `termplot-rs` está diseñado para una **velocidad crítica**: utiliza búferes de memoria planos (`Vec<u8>`), operaciones a nivel de bits, recorte matemático (clipping) y un bucle de renderizado con *cero asignaciones* de memoria (zero-allocation) para alcanzar **miles de FPS** en aplicaciones en tiempo real.

> 🚀 **Novedades en v0.9.0:** Bucle de renderizado real de cero asignaciones (`render_to`), recorte de líneas (clipping) de Cohen-Sutherland, primitivas rellenas (`rect_filled`, `circle_filled`), borrado de píxeles (`unset_pixel`) y modos de mezcla de color (Color Blending).

---

## ✨ Características Principales

* **Alta Resolución:** 8 sub-píxeles por carácter (Braille 2x4). Una terminal de 100x50 produce un lienzo efectivo de 200x200 píxeles.
* **Rendimiento Extremo:**
  * **Búferes planos** para máxima localidad en la caché de la CPU.
  * **Bucle de Cero Asignaciones (True Zero-Allocation):** Renderiza directamente a un `std::fmt::Write` o `stdout.lock()` sin crear ni un solo `String` por fotograma.
  * **Recorte Cohen-Sutherland:** Descarta matemáticamente la geometría fuera de la pantalla antes de la rasterización, ahorrando ciclos masivos de CPU al hacer zoom o dibujar fuera de los límites. 
* **Control Avanzado de Píxeles y Color:**
  * Borra o alterna puntos Braille individuales (`unset_pixel`, `toggle_pixel`).
  * **Modos de Mezcla (Color Blending):** Controla cómo interactúan los sub-píxeles que comparten la misma celda de la terminal (`Overwrite` vs `KeepFirst`).
* **Primitivas de Dibujo:**
  * Líneas (Bresenham), Círculos, Polígonos.
  * **Formas Rellenas:** `rect_filled` y `circle_filled`.
  * Capa de Texto (superpuesta).
* **Gráficos listos para usar:**
  * `scatter()`, `line_chart()`, `bar_chart()`, `pie_chart()`, `plot_function()`.
* **Rango Automático y Ejes Inteligentes:** Escalado automático de ejes y generación de marcas (ticks) basados en tu conjunto de datos.

---

## 📦 Instalación

Añade esto a tu `Cargo.toml`:

```toml
[dependencies]
termplot-rs = "0.9.0"
colored = "2.0"
# Opcional, para generar datos de prueba
rand = "0.8"   

```

---

## 🚀 Inicio Rápido

```rust
use termplot_rs::ChartContext;
use colored::Color;

fn main() {
    // 1. Crear el contexto (Ancho, Alto en caracteres)
    let mut chart = ChartContext::new(60, 15);

    // 2. Generar datos (ej. Onda senoidal)
    let points: Vec<(f64, f64)> = (0..100)
        .map(|x| (x as f64 / 10.0, (x as f64 / 5.0).sin()))
        .collect();

    // 3. Dibujar
    // Auto-range calcula el min/max automáticamente
    let (range_x, range_y) = ChartContext::get_auto_range(&points, 0.1);

    chart.draw_axes(range_x, range_y, Some(Color::White));
    chart.line_chart(&points, Some(Color::Cyan));
    chart.text("Onda Senoidal", 0.5, 0.9, Some(Color::Yellow));

    // 4. Renderizar e imprimir (Método estándar)
    println!("{}", chart.canvas.render());
}

```

### 🏎️ Bucle de Renderizado de Cero Asignaciones (Para Juegos/Animaciones)

Si estás construyendo una app en tiempo real a 60 FPS, evita usar `render()` (que crea un nuevo `String` en cada frame) y usa `render_to()`:

```rust
use std::fmt::Write;

// Dentro de tu bucle de juego:
let mut buffer = String::with_capacity(8000); 
chart.canvas.render_to(&mut buffer, true, Some("UI a 60 FPS"))?;
print!("{}", buffer);
buffer.clear(); // ¡Reutiliza la memoria!

```

---

## 📐 Sistema de Coordenadas y API de Píxeles

Para evitar confusiones matemáticas, `termplot-rs` ofrece dos modos de coordenadas y múltiples operadores de píxeles:

| Modo de Coordenadas | Origen (0,0) | Dirección Y | Ideal Para |
| --- | --- | --- | --- |
| **Cartesian (Cartesiano)** | **Abajo-Izquierda** | Crece hacia **Arriba** | Gráficos matemáticos, funciones, charts. |
| **Screen (Pantalla)** | **Arriba-Izquierda** | Crece hacia **Abajo** | UI, Juegos, Sprites, Proyecciones 3D. |

**Métodos de Manipulación de Píxeles:**

* `set_pixel / set_pixel_screen`: ENCIENDE un punto.
* `unset_pixel / unset_pixel_screen`: APAGA un punto (Borra).
* `toggle_pixel_screen`: Invierte el estado actual de un punto.

---

## 🧪 Ejemplos y Demos

El repositorio incluye ejemplos avanzados para mostrar el poder de la biblioteca.

### 1. Primitivas y Modos de Mezcla (NUEVO)

Un salvapantallas interactivo que muestra el recorte (clipping) de Cohen-Sutherland, formas rellenas, borrado de píxeles (agujeros dinámicos) y cambio de modos de mezcla de color en tiempo real.

```bash
cargo run --release --example primitives_demo

```

### 2. Sistema Solar Kepler 3D

Simulación física completa del Sistema Solar utilizando mecánica orbital real, rotaciones 3D y un Z-Buffer por software personalizado.

```bash
cargo run --release --example solarsystem_kepler

```

### 3. Motor de Sprites

Una demo de estilo Space Invaders retro que muestra cómo cargar y renderizar arte ASCII personalizado como sprites Braille ultrarrápidos.

```bash
cargo run --release --example sprite_demo

```

### 4. Fractales Interactivos

Explorador de Mandelbrot y Julia con Zoom infinito y rotación.

```bash
cargo run --release --example fractalmove

```

### 5. Galería de Gráficos

Muestra todos los tipos de gráficos estáticos disponibles (Barras, Dispersión, Pastel, Auto-Ticks).

```bash
cargo run --example demo

```

---

## ⚡ Rendimiento

`termplot-rs` está rigurosamente optimizado.
En un benchmark con un lienzo de **236x104 sub-píxeles** (llenado completo con ruido trigonométrico y partículas), en una máquina moderna:

* **Modo Debug:** ~60 FPS
* **Modo Release:** ~1600+ FPS

Esto lo hace viable para visualización de audio, monitorización de servidores de alta frecuencia, juegos de terminal retro, o simulaciones físicas ligeras.

---

## 🗺️ Hoja de Ruta (Roadmap)

* [x] Optimización de memoria (Búferes `Vec<u8>` planos).
* [x] APIs de coordenadas explícitas (`screen` vs `cartesian`).
* [x] Recorte matemático de líneas (Cohen-Sutherland).
* [x] Renderizado real con cero asignaciones (`render_to`).
* [x] Primitivas Rellenas (`rect_filled`, `circle_filled`) y Borradores.
* [x] Políticas de Mezcla de Color (`Overwrite`, `KeepFirst`).
* [ ] Soporte para escala logarítmica.
* [ ] Caja de Leyenda automática.
* [ ] Renderizadores de terminal conectables basados en *Traits* (`CellRenderer` para HalfBlocks/Quadrants).

---

## 📄 Licencia

Este proyecto está licenciado bajo la licencia **MIT**. Siéntete libre de usarlo en tus herramientas CLI, paneles de control (dashboards) o experimentos gráficos.

```
