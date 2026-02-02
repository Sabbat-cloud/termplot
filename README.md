---

```markdown
# termplot

Gráficos en consola (TUI) usando **Unicode Braille** (2×4 “píxeles” por carácter) y color ANSI.
Permite dibujar **scatter plots, líneas, barras, polígonos, círculos, rejillas, ejes, texto** y animaciones
en terminal, sin depender de GUI.

> Ideal para logs, dashboards ligeros, debugging visual, CLI tools, demos y “plots” rápidos en SSH.

---

## ✨ Características

- Canvas de alta densidad con **Braille (2×4)** → más resolución que ASCII clásico.
- **Color por celda de carácter** (ANSI / TrueColor si tu terminal lo soporta).
- **Auto-Range (v0.8+)**: Cálculo automático de escalas para tus datos.
- **Modo No-Color (v0.8+)**: Renderizado compatible con logs planos y terminales antiguos.
- Primitivas: `set_pixel`, `line` (Bresenham), `circle` (punto medio), `set_char`, `set_char_vertical`.
- Charts:
  - `scatter()` nube de puntos
  - `line_chart()` serie conectada
  - `bar_chart()` barras (con auto-ancho inteligente)
  - `polygon()` polígonos
  - `pie_chart()` “pie radar” (radios + contorno)
  - `draw_grid()` rejilla
  - `draw_axes()` ejes + etiquetas min/max (ahora en ambos lados)
  - `plot_function()` plotea funciones `f(x)` directamente
  - `text()` capa de texto horizontal y vertical
- Animación por frames sobrescribiendo el cursor (sin limpiar pantalla completa).

---

## 📦 Instalación

### Cargo.toml

```toml
[dependencies]
termplot = { path = "." } 
# Opcional: Desactivar colores si no se necesitan
# termplot = { path = ".", default-features = false }

```

---

## 🚀 Quick start: Auto-Scaling (v0.8+)

Ahora no necesitas calcular los rangos manualmente. `termplot` lo hace por ti:

```rust
use termplot::charts::ChartContext;
use colored::Color;

fn main() {
    let mut chart = ChartContext::new(60, 15);
    let points = vec![(0.0, 0.0), (10.0, 50.0), (20.0, 20.0), (40.0, 80.0)];

    // Calculamos rango automáticamente con 10% de margen
    let (range_x, range_y) = ChartContext::get_auto_range(&points, 0.1);

    chart.draw_axes(range_x, range_y, Some(Color::White));
    chart.line_chart(&points, Some(Color::Cyan));
    
    println!("{}", chart.canvas.render());
}

```

---

## 🧱 Opciones de Renderizado (v0.8+)

### 1. Renderizado Estándar (con bordes y color)

```rust
println!("{}", chart.canvas.render());

```

### 2. Renderizado con Título y sin Bordes

Ideal para dashboards minimalistas.

```rust
println!("{}", chart.canvas.render_with_options(false, Some("RENDIMIENTO CPU")));

```

### 3. Modo No-Color (Plain Text)

Para cuando necesitas guardar el output en un archivo de texto plano sin códigos ANSI.

```rust
println!("{}", chart.canvas.render_no_color());

```

---

## 🧠 Conceptos básicos

### Resolución real (píxeles virtuales)

El canvas se define en **caracteres**: `(width, height)`. Pero cada carácter Braille contiene **2×4 subpíxeles**, por lo que la resolución real es:

* `pixel_width = width * 2`
* `pixel_height = height * 4`

---

# ✅ Ejemplos Destacados

## 1) Ajuste Automático y Etiquetas Duales

El nuevo `draw_axes` ahora imprime etiquetas en ambos lados del gráfico para mejorar la lectura en pantallas anchas.

## 2) Barras con protección de límites

El método `bar_chart` ahora ajusta automáticamente el ancho de las barras para que siempre ocupen al menos 1 píxel y nunca desborden el canvas lateralmente.

---

## 🎛️ Consejos de uso

* **Fuentes**: Usa una fuente monoespaciada con buen soporte Unicode.
* **Feature Flags**: Si usas `default-features = false`, el método `render()` funcionará pero ignorará las llamadas a `Color`.
* **Rendimiento**: Para animaciones fluidas, mantén el tamaño del canvas por debajo de 80x40 caracteres.

---

## 🗺️ Roadmap Actualizado

* [x] Mejor manejo de límites en `text()` y `bar_chart()`.
* [x] Cálculo de "Auto-range" con padding configurable.
* [x] Etiquetas de eje Y duplicadas (izq/der).
* [x] Soporte para renderizado sin color ANSI.
* [x] Títulos centrados opcionales.
* [ ] **Próximamente**: Soporte para leyendas automáticas (Series A, B, C).
* [ ] **Próximamente**: Histogramas de frecuencia.

---

## 📄 Licencia

MIT

```

```
