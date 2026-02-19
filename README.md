# 📈 termplot-rs

**High-performance terminal graphics engine (TUI).**

`termplot-rs` allows you to render mathematical plots, 3D visualizations, and complex interfaces directly in the console using **Unicode Braille characters** (2×4 dot matrix per character) and ANSI colors.

Unlike other TUI plotting libraries, `termplot-rs` is designed for **critical speed**: it uses flat memory buffers (`Vec<u8>`), bitwise operations, and a *zero-allocation* rendering loop to achieve **thousands of FPS** in real-time applications.

> 🚀 **New in v0.8:** Optimized rendering engine (1600+ FPS in stress tests), Dual Coordinate System (Cartesian/Screen), and robust handling of `NaN`/Infinity values.

---

## ✨ Key Features

* **High Resolution:** 8 sub-pixels per character (Braille 2x4). A 100x50 terminal yields a 200x200 effective pixel canvas.
* **Extreme Performance:**
* **Flat buffers** for maximum CPU cache locality.
* Minimized memory allocations during the render loop.
* Optimized ANSI output (redundant color codes are stripped).


* **Robust:** Safe data handling (ignores `NaN`, prevents division by zero, auto-clamping).
* **Drawing Primitives:**
* Lines (Bresenham), Circles, Polygons.
* Text Layer (overlay).


* **Ready-to-use Charts:**
* `scatter()` (Scatter plots).
* `line_chart()` (Time series / connected points).
* `bar_chart()` (Bar graphs with auto-width).
* `pie_chart()` (Radar/Pie style).
* `plot_function()` (Direct plotting of `y = f(x)` functions).


* **Auto-Range:** Automatic axis scaling based on your dataset.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
termplot-rs = "0.1.1"
rand = "0.8"   # Optional, for generating test data
colored = "2.0"

```

---

## 🚀 Quick Start

```rust
use termplot_rs::{ChartContext, ChartOptions};
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

    // 4. Render to String and print
    println!("{}", chart.canvas.render());
}

```

---

## 📐 Coordinate System

To avoid mathematical confusion, `termplot-rs` offers two modes for pixel manipulation:

| Method | Origin (0,0) | Y Direction | Best For |
| --- | --- | --- | --- |
| `set_pixel(x, y)` | **Bottom-Left** | Grows **Up** | Math plots, functions, charts. |
| `set_pixel_screen(x, y)` | **Top-Left** | Grows **Down** | UI, Images, 3D Projections, Video. |

> **Note:** High-level functions (`scatter`, `line_chart`) internally use Cartesian (Math) coordinates.

---

## 🧪 Examples & Demos

The repository includes advanced examples to showcase the library's power.

### 1. "Plasma" Stress Test (+1000 FPS)

Calculates complex trigonometry per sub-pixel plus particle physics in real-time. **Run in release mode to see real speed.**

```bash
cargo run --release --example plasma

```

### 2. Interactive Fractals

Mandelbrot and Julia explorer with infinite Zoom and rotation.

```bash
cargo run --release --example fractalmove

```

### 3. 3D Cube

Wireframe 3D rendering with projection matrix and rotation.

```bash
cargo run --example cube2

```

### 4. Chart Gallery

Shows all available static chart types.

```bash
cargo run --example demo

```

---

## ⚡ Performance

`termplot-rs` is optimized to avoid unnecessary allocations.
In a benchmark with a **236x104 sub-pixel** canvas (full fill with trigonometric noise and particles), on a modern machine:

* **Debug Mode:** ~60 FPS
* **Release Mode:** ~1600+ FPS

This makes it viable for audio visualization, high-frequency server monitoring, or lightweight physics simulations directly in the terminal.

---

## 🗺️ Roadmap

* [x] Memory optimization (Flat `Vec<u8>` buffers).
* [x] Explicit coordinate APIs (`screen` vs `cartesian`).
* [x] Robustness in `bar_chart` and `auto_range` (fix division by zero).
* [x] `plot_function` and `draw_circle` methods.
* [ ] Logarithmic scaling support.
* [ ] Automatic Legend Box.
* [ ] Optional `serde` support for configuration structs.

---

## 📄 License

This project is licensed under the **MIT** license. Feel free to use it in your CLI tools, dashboards, or graphical experiments.
