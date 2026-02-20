# 📈 termplot-rs

**High-performance terminal graphics engine (TUI).**

`termplot-rs` allows you to render mathematical plots, 3D visualizations, games, and complex interfaces directly in the console using **Unicode Braille characters** (2×4 dot matrix per character) and ANSI colors.

Unlike other TUI plotting libraries, `termplot-rs` is designed for **critical speed**: it uses flat memory buffers (`Vec<u8>`), bitwise operations, mathematical clipping, and a true *zero-allocation* rendering loop to achieve **thousands of FPS** in real-time applications.

> 🚀 **New in v0.9.0:** True Zero-Allocation rendering (`render_to`), Cohen-Sutherland Line Clipping, Filled Primitives (`rect_filled`, `circle_filled`), Pixel Erasing (`unset_pixel`), and Color Blending modes!

---

## ✨ Key Features

* **High Resolution:** 8 sub-pixels per character (Braille 2x4). A 100x50 terminal yields a 200x200 effective pixel canvas.
* **Extreme Performance:**
  * **Flat buffers** for maximum CPU cache locality.
  * **True Zero-Allocation Loop:** Render directly to `std::fmt::Write` or `stdout.lock()` without allocating a single `String` per frame.
  * **Cohen-Sutherland Clipping:** Mathematically discards off-screen geometry before rasterization, saving massive CPU cycles during zoom or out-of-bounds drawing. 
* **Advanced Pixel & Color Control:**
  * Erase and toggle individual Braille dots (`unset_pixel`, `toggle_pixel`).
  * **Color Blending Modes:** Control how sub-pixels sharing the same terminal cell interact (`Overwrite` vs `KeepFirst`).
* **Drawing Primitives:**
  * Lines (Bresenham), Circles, Polygons.
  * **Filled Shapes:** `rect_filled` and `circle_filled`.
  * Text Layer (overlay).
* **Ready-to-use Charts:**
  * `scatter()`, `line_chart()`, `bar_chart()`, `pie_chart()`, `plot_function()`.
* **Auto-Range & Smart Axes:** Automatic axis scaling and tick generation based on your dataset.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
termplot-rs = "0.9.0"
colored = "2.0"
# Optional, for generating test data
rand = "0.8"   

```

---

## 🚀 Quick Start

```rust
use termplot_rs::ChartContext;
use colored::Color;

fn main() {
    // 1. Create context (Width, Height in characters)
    let mut chart = ChartContext::new(60, 15);

    // 2. Generate data (e.g., Sine wave)
    let points: Vec<(f64, f64)> = (0..100)
        .map(|x| (x as f64 / 10.0, (x as f64 / 5.0).sin()))
        .collect();

    // 3. Draw
    // Auto-range calculates min/max automatically
    let (range_x, range_y) = ChartContext::get_auto_range(&points, 0.1);

    chart.draw_axes(range_x, range_y, Some(Color::White));
    chart.line_chart(&points, Some(Color::Cyan));
    chart.text("Sine Wave", 0.5, 0.9, Some(Color::Yellow));

    // 4. Render and print (Standard way)
    println!("{}", chart.canvas.render());
}

```

### 🏎️ Zero-Allocation Render Loop (For Games/Animations)

If you are building a real-time app at 60 FPS, avoid `render()` (which creates a new `String` every frame) and use `render_to()`:

```rust
use std::fmt::Write;

// Inside your game loop:
let mut buffer = String::with_capacity(8000); 
chart.canvas.render_to(&mut buffer, true, Some("60 FPS UI"))?;
print!("{}", buffer);
buffer.clear(); // Reuse memory!

```

---

## 📐 Coordinate System & Pixel API

To avoid mathematical confusion, `termplot-rs` offers two coordinate modes and multiple pixel operators:

| Coordinate Mode | Origin (0,0) | Y Direction | Best For |
| --- | --- | --- | --- |
| **Cartesian** | **Bottom-Left** | Grows **Up** | Math plots, functions, charts. |
| **Screen** | **Top-Left** | Grows **Down** | UI, Games, Sprites, 3D Projections. |

**Pixel Manipulation Methods:**

* `set_pixel / set_pixel_screen`: Turns a dot ON.
* `unset_pixel / unset_pixel_screen`: Turns a dot OFF (Erases).
* `toggle_pixel_screen`: Flips the current state of a dot.

---

## 🧪 Examples & Demos

The repository includes advanced examples to showcase the library's power.

### 1. Primitive Shapes & Blending (NEW)

Interactive screensaver showcasing Cohen-Sutherland clipping, filled shapes, pixel erasing (dynamic holes), and real-time color blending mode switching.

```bash
cargo run --release --example primitives_demo

```

### 2. Solar System Kepler 3D

Full physics simulation of the Solar System using true orbital mechanics, 3D rotations, and a custom software Z-Buffer.

```bash
cargo run --release --example solarsystem_kepler

```

### 3. Sprite Engine

A retro space-invaders style demo showcasing how to load and render custom ASCII art as fast Braille sprites.

```bash
cargo run --release --example sprite_demo

```

### 4. Interactive Fractals

Mandelbrot and Julia explorer with infinite Zoom and rotation.

```bash
cargo run --release --example fractalmove

```

### 5. Chart Gallery

Shows all available static chart types (Bars, Scatter, Pie, Auto-Ticks).

```bash
cargo run --example demo

```

---

## ⚡ Performance

`termplot-rs` is rigorously optimized.
In a benchmark with a **236x104 sub-pixel** canvas (full fill with trigonometric noise and particles), on a modern machine:

* **Debug Mode:** ~60 FPS
* **Release Mode:** ~1600+ FPS

This makes it viable for audio visualization, high-frequency server monitoring, retro terminal games, or lightweight physics simulations.

---

## 🗺️ Roadmap

* [x] Memory optimization (Flat `Vec<u8>` buffers).
* [x] Explicit coordinate APIs (`screen` vs `cartesian`).
* [x] Mathematical Cohen-Sutherland Line Clipping.
* [x] True zero-allocation rendering (`render_to`).
* [x] Filled Primitives (`rect_filled`, `circle_filled`) & Erasers.
* [x] Color Blending Policies (`Overwrite`, `KeepFirst`).
* [ ] Logarithmic scaling support.
* [ ] Automatic Legend Box.
* [ ] Trait-based pluggable terminal renderers (`CellRenderer` for HalfBlocks/Quadrants).

---

## 📄 License

This project is licensed under the **MIT** license. Feel free to use it in your CLI tools, dashboards, or graphical experiments.

```
