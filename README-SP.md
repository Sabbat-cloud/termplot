# 📈 termplot-rs

Motor de gráficos de terminal de alto rendimiento (TUI).

`termplot-rs` te permite renderizar gráficos matemáticos, visualizaciones 3D, juegos e interfaces complejas directamente en la consola utilizando caracteres Braille Unicode (matriz de puntos 2x4 por carácter) y colores ANSI.

A diferencia de otras bibliotecas de gráficos TUI, `termplot-rs` está diseñado teniendo en cuenta la **velocidad crítica**: utiliza búferes de memoria planos (`Vec<u8>`), operaciones a nivel de bits, recorte matemático (clipping) y un bucle de renderizado real sin asignaciones de memoria (*zero-allocation*) para lograr miles de FPS en aplicaciones en tiempo real.

**🚀 Novedades en la v0.4.0:** Escala Logarítmica (`AxisScale::Log10`), renderizado verdaderamente sin asignaciones de memoria (`render_to`), recorte de líneas (Clipping) de Cohen-Sutherland, primitivas con relleno (`rect_filled`, `circle_filled`), borrado de píxeles (`unset_pixel`) y modos de mezcla de color (*Color Blending*)!

---

## ✨ Características Principales

*   **Alta Resolución:** 8 subpíxeles por carácter (Braille 2x4). Una terminal de 100x50 produce un lienzo de píxeles efectivos de 200x200.
*   **Rendimiento Extremo:**
    *   Búferes planos para máxima localidad de caché de la CPU.
    *   **Bucle Verdadero sin Asignaciones (Zero-Allocation):** Renderiza directamente a `std::fmt::Write` o `stdout.lock()` sin asignar un solo `String` por fotograma.
    *   **Recorte de Cohen-Sutherland:** Descarta matemáticamente la geometría fuera de la pantalla antes de la rasterización, ahorrando ciclos masivos de CPU durante el zoom o dibujos fuera de los límites.
*   **Control Avanzado de Píxeles y Color:**
    *   Borra y alterna puntos Braille individuales (`unset_pixel`, `toggle_pixel`).
    *   **Modos de Mezcla de Color:** Controla cómo interactúan los subpíxeles que comparten la misma celda de la terminal (`Overwrite` vs `KeepFirst`).
*   **Primitivas de Dibujo:**
    *   Líneas (Bresenham), Círculos, Polígonos.
    *   Formas Rellenas: `rect_filled` y `circle_filled`.
    *   Capa de Texto (superposición).
*   **Gráficos Listos para Usar:**
    *   `scatter()`, `line_chart()`, `bar_chart()`, `pie_chart()`, `plot_function()`.
    *   **Rango Automático y Ejes Inteligentes:** Escalado automático de ejes y generación de marcas (soporta escalas Lineales y **Log10**) basándose en tu conjunto de datos.

---

## 📦 Instalación

Añade esto a tu archivo `Cargo.toml`:

```toml
[dependencies]
termplot-rs = "0.4.0"
colored = "2.0"

# Opcional, para generar datos de prueba en los ejemplos
rand = "0.8"   

```

---

## 🚀 Inicio Rápido

```rust
use termplot_rs::ChartContext;
use colored::Color;

fn main() {
    // 1. Crear contexto (Ancho, Alto en caracteres)
    let mut chart = ChartContext::new(60, 15);

    // 2. Generar datos (ej. Onda senoidal)
    let points: Vec<(f64, f64)> = (0..100)
        .map(|x| (x as f64 / 10.0, (x as f64 / 5.0).sin()))
        .collect();

    // 3. Dibujar
    // El auto-rango calcula el mín/máx automáticamente
    let (range_x, range_y) = ChartContext::get_auto_range(&points, 0.1);

    chart.draw_axes(range_x, range_y, Some(Color::White));
    chart.line_chart(&points, Some(Color::Cyan));
    chart.text("Onda Senoidal", 0.5, 0.9, Some(Color::Yellow));

    // 4. Renderizar e imprimir (Método estándar)
    println!("{}", chart.canvas.render());
}

```

## 📊 Escalas Logarítmicas (Nuevo)

Perfecto para visualizar datos con picos masivos, como la latencia de un servidor, el tráfico de red o un crecimiento exponencial.

```rust
use termplot_rs::{ChartContext, AxisScale};
use colored::Color;

fn main() {
    let mut chart = ChartContext::new(60, 15);
    
    // Establecer el eje Y en escala logarítmica
    chart.set_y_scale(AxisScale::Log10);

    // Datos con un pico masivo (ej. latencia de servidor en ms)
    let latency_data: Vec<(f64, f64)> = vec![
        (1.0, 45.0), (2.0, 50.0), (3.0, 48.0), 
        (4.0, 1500.0), // ¡Pico masivo repentino!
        (5.0, 55.0), (6.0, 49.0),
    ];

    // Usar get_auto_range_scaled para tener en cuenta la transformación Log10
    let (range_x, range_y) = ChartContext::get_auto_range_scaled(
        &latency_data, 0.1, chart.x_scale(), chart.y_scale()
    );

    chart.draw_axes(range_x, range_y, Some(Color::White));
    chart.line_chart(&latency_data, Some(Color::Red));
    chart.text("Latencia (ms)", 0.5, 0.9, Some(Color::Yellow));

    println!("{}", chart.canvas.render());
}

```

---

## 🏎️ Bucle de Renderizado sin Asignaciones (Para Juegos/Animaciones)

Si estás construyendo una aplicación en tiempo real a 60 FPS, evita `render()` (que crea un nuevo `String` en cada fotograma) y usa `render_to()`:

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
| **Cartesiano** | Abajo-Izquierda | Crece hacia Arriba | Gráficos matemáticos, funciones, diagramas. |
| **Pantalla** | Arriba-Izquierda | Crece hacia Abajo | UI, Juegos, Sprites, Proyecciones 3D. |

**Métodos de Manipulación de Píxeles:**

* `set_pixel` / `set_pixel_screen`: Enciende un punto.
* `unset_pixel` / `unset_pixel_screen`: Apaga un punto (Borra).
* `toggle_pixel_screen`: Invierte el estado actual de un punto.

---

## 🧪 Ejemplos y Demos

El repositorio incluye muchos ejemplos para mostrar la potencia y versatilidad de la biblioteca.

**1. 3dengine.rs**
`cargo run --release --example 3dengine`

**2. barcentertext.rs**
`cargo run --release --example barcentertext`

**3. demo.rs**
`cargo run --release --example demo.rs`

**4. fireworksinterac.rs**
`cargo run --release --example fireworksinterac`

**5. fireworks.rs**
`cargo run --example fireworks`

**6. fractalmove.rs**
`cargo run --release --example fractalmove`

**7. infrastructure_monitor.rs**
`cargo run --release --example infrastructure_monitor`

**8. logaritmica.rs**
`cargo run --release --example logaritmica`

**9. math_stress.rs**
`cargo run --release --example math_stress`

**10. plasma.rs**
`cargo run --example plasma`

**11. primitives_demo.rs**
`cargo run --release --example primitives_demo`

**12. showcase.rs**
`cargo run --release --example showcase`

**13. solarsystem_kepler.rs**
`cargo run --release --example solarsystem_kepler`

**14. solarsystemnobody.rs**
`cargo run --release --example solarsystemnobody`

**15. sprite_demo.rs**
`cargo run --example sprite_demo`

**16. system_monitor.rs**
`cargo run --release --example system_monitor`

**17. waves.rs**
`cargo run --example waves`
---

## ⚡ Rendimiento

`termplot-rs` está rigurosamente optimizado. En un banco de pruebas con un lienzo de subpíxeles de 236x104 (rellenado completo con ruido trigonométrico y partículas), en una máquina moderna:

* **Modo Debug:** ~60 FPS
* **Modo Release:** ~1600+ FPS

Esto lo hace viable para visualización de audio, monitorización de servidores de alta frecuencia, juegos de terminal retro o simulaciones físicas ligeras.

---

## 🗺️ Hoja de Ruta

* [x] Optimización de memoria (Búferes planos `Vec<u8>`).
* [x] APIs explícitas de coordenadas (pantalla vs cartesiano).
* [x] Recorte de líneas matemático (Cohen-Sutherland).
* [x] Renderizado verdadero sin asignaciones (`render_to`).
* [x] Primitivas con relleno (`rect_filled`, `circle_filled`) y Borradores.
* [x] Políticas de mezcla de color (`Overwrite`, `KeepFirst`).
* [x] Soporte para escala logarítmica.
* [ ] Caja de leyenda automática.
* [ ] Renderizadores de terminal conectables basados en Traits (`CellRenderer` para Medios Bloques/Cuadrantes).

---

## 📄 Licencia

Este proyecto está bajo la licencia MIT. Siéntete libre de usarlo en tus herramientas CLI, paneles de control o experimentos gráficos.

