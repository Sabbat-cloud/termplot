# 📈 termplot-rs

**Motor gráfico de alto rendimiento para terminal (TUI).**

`termplot-rs` permite renderizar gráficos matemáticos, visualizaciones 3D y interfaces complejas en la consola utilizando caracteres **Unicode Braille** (matriz de 2×4 puntos por carácter) y colores ANSI.

A diferencia de otras librerías, `termplot-rs` está diseñada para **velocidad crítica**: utiliza buffers de memoria planos (`Vec<u8>`), operaciones a nivel de bit y renderizado *zero-allocation* para alcanzar **miles de FPS** en aplicaciones en tiempo real.

> 🚀 **Nuevo en v0.8:** Renderizado optimizado (1600+ FPS en stress tests), sistema de coordenadas dual (Cartesiano/Pantalla) y robustez contra `NaN`/infinitos.

---

## ✨ Características Principales

* **Alta Resolución:** 8 sub-píxeles por carácter (Braille 2x4). Una terminal de 100x50 ofrece un canvas de 200x200 píxeles reales.
* **Rendimiento Extremo:**
* Uso de **buffers planos** para máxima localidad de caché.
* Minimización de asignaciones de memoria en el bucle de renderizado.
* Salida ANSI optimizada (no repite códigos de color redundantes).


* **Robusto:** Manejo seguro de datos (ignora `NaN`, evita divisiones por cero, clamps automáticos).
* **Primitivas Gráficas:**
* Líneas (Bresenham), Círculos, Polígonos.
* Texto sobreimpreso (Text Layer).


* **Gráficos Listos para Usar:**
* `scatter()` (Nube de puntos).
* `line_chart()` (Series temporales).
* `bar_chart()` (Barras con auto-ancho).
* `pie_chart()` (Gráfico de pastel/radar).
* `plot_function()` (Ploteo directo de funciones matemáticas `y = f(x)`).


* **Auto-Range:** Cálculo automático de escalas y ejes basado en tus datos.

---

## 📦 Instalación

Añade esto a tu `Cargo.toml`:

```toml
[dependencies]
termplot-rs = "0.1.1" 
rand = "0.8" # Opcional, para generar datos de prueba
colored = "2.0"

```

---

## 🚀 Quick Start

```rust
use termplot_rs::{ChartContext, ChartOptions};
use colored::Color;

fn main() {
    // 1. Crear contexto (Ancho, Alto en caracteres)
    let mut chart = ChartContext::new(60, 15);

    // 2. Generar datos (ej. función Seno)
    let points: Vec<(f64, f64)> = (0..100)
        .map(|x| (x as f64 / 10.0, (x as f64 / 5.0).sin()))
        .collect();

    // 3. Dibujar
    // Auto-range calcula los límites automáticamente
    let (range_x, range_y) = ChartContext::get_auto_range(&points, 0.1);
    
    chart.draw_axes(range_x, range_y, Some(Color::White));
    chart.line_chart(&points, Some(Color::Cyan));
    chart.text("Onda Senoidal", 0.5, 0.9, Some(Color::Yellow));

    // 4. Renderizar a String e imprimir
    println!("{}", chart.canvas.render());
}

```

---

## 📐 Sistema de Coordenadas

Para evitar confusiones matemáticas, `termplot-rs` ofrece dos modos de dibujar píxeles:

| Método | Origen (0,0) | Dirección Y | Uso Recomendado |
| --- | --- | --- | --- |
| `set_pixel(x, y)` | **Abajo-Izquierda** | Crece hacia **Arriba** | Gráficos matemáticos, funciones, charts. |
| `set_pixel_screen(x, y)` | **Arriba-Izquierda** | Crece hacia **Abajo** | UI, Imágenes, Renderizado 3D proyectado, Video. |

> **Nota:** Las funciones de alto nivel (`scatter`, `line_chart`) usan internamente coordenadas matemáticas cartesianas.

---

## 🧪 Ejemplos y Demos

El repositorio incluye ejemplos avanzados para demostrar la potencia de la librería.

### 1. Stress Test "Plasma" (+1000 FPS)

Calcula trigonometría compleja por píxel y partículas en tiempo real. **Ejecutar en modo release para ver la velocidad real.**

```bash
cargo run --release --example plasma

```

### 2. Fractales Interactivos

Explorador de Mandelbrot y Julia con Zoom infinito y rotación.

```bash
cargo run --release --example fractalmove

```

### 3. Cubo 3D

Renderizado de wireframe 3D con proyección y rotación.

```bash
cargo run --example cube2

```

### 4. Galería de Charts

Muestra todos los tipos de gráficos estáticos disponibles.

```bash
cargo run --example demo

```

---

## ⚡ Rendimiento

`termplot-rs` está optimizado para evitar "allocations" innecesarias.
En un benchmark con un canvas de **236x104 sub-píxeles** (relleno completo con ruido Perlin y partículas), en una máquina moderna:

* **Debug Mode:** ~60 FPS
* **Release Mode:** ~1600+ FPS

Esto lo hace viable para visualización de audio, monitoreo de servidores en alta frecuencia o simulaciones físicas ligeras directamente en terminal.

---

## 🗺️ Roadmap

* [x] Optimización de memoria (Buffers planos `Vec<u8>`).
* [x] APIs de coordenadas explícitas (`screen` vs `cartesian`).
* [x] Robustez en `bar_chart` y `auto_range` (fix division by zero).
* [x] Métodos `plot_function` y `draw_circle`.
* [ ] Soporte para escalas logarítmicas.
* [ ] Leyendas automáticas (Legend Box).
* [ ] Soporte opcional para `serde` en estructuras de configuración.

---

## 📄 Licencia

Este proyecto está bajo la licencia **MIT**. Siéntete libre de usarlo en tus herramientas CLI, dashboards o experimentos gráficos.
