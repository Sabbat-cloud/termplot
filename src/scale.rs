#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisScale {
    Linear,
    Log10,
}

impl AxisScale {
    pub fn transform(self, value: f64) -> Option<f64> {
        if !value.is_finite() {
            return None;
        }

        match self {
            Self::Linear => Some(value),
            Self::Log10 if value > 0.0 => Some(value.log10()),
            Self::Log10 => None,
        }
    }

    pub fn inverse_transform(self, value: f64) -> f64 {
        match self {
            Self::Linear => value,
            Self::Log10 => 10f64.powf(value),
        }
    }

    pub fn transformed_range(self, range: (f64, f64)) -> Option<(f64, f64)> {
        let min = self.transform(range.0)?;
        let max = self.transform(range.1)?;
        Some(if min <= max { (min, max) } else { (max, min) })
    }

    pub fn axis_ticks(self, range: (f64, f64)) -> Vec<f64> {
        match self {
            Self::Linear => {
                let (min, max) = range;
                let step = (max - min) / 3.0;
                vec![min, min + step, min + step * 2.0, max]
            }
            Self::Log10 => Self::log_ticks(range),
        }
    }

    fn log_ticks(range: (f64, f64)) -> Vec<f64> {
        let (min, max) = match Self::Log10.transformed_range(range) {
            Some((min, max)) => (10f64.powf(min), 10f64.powf(max)),
            None => return Vec::new(),
        };

        let min_exp = min.log10().floor() as i32;
        let max_exp = max.log10().ceil() as i32;
        let powers: Vec<f64> = (min_exp..=max_exp)
            .map(|exp| 10f64.powi(exp))
            .filter(|value| *value >= min && *value <= max)
            .collect();

        if powers.len() >= 2 {
            return Self::downsample_ticks(&powers, 5);
        }

        let min_t = min.log10();
        let max_t = max.log10();
        let step = (max_t - min_t) / 3.0;
        let ticks: Vec<f64> = (0..=3)
            .map(|i| 10f64.powf(min_t + step * i as f64))
            .collect();

        Self::dedup_ticks(ticks)
    }

    fn downsample_ticks(ticks: &[f64], max_ticks: usize) -> Vec<f64> {
        if ticks.len() <= max_ticks {
            return ticks.to_vec();
        }

        let last_index = ticks.len() - 1;
        let sampled: Vec<f64> = (0..max_ticks)
            .map(|i| {
                let ratio = i as f64 / (max_ticks - 1) as f64;
                let index = (ratio * last_index as f64).round() as usize;
                ticks[index]
            })
            .collect();

        Self::dedup_ticks(sampled)
    }

    fn dedup_ticks(ticks: Vec<f64>) -> Vec<f64> {
        let mut deduped = Vec::with_capacity(ticks.len());
        for tick in ticks {
            let is_duplicate = deduped
                .last()
                .map(|last| (last - tick).abs() < 1e-9)
                .unwrap_or(false);
            if !is_duplicate {
                deduped.push(tick);
            }
        }
        deduped
    }

    pub fn format_tick(self, value: f64) -> String {
        match self {
            Self::Linear => format!("{:.1}", value),
            Self::Log10 => Self::format_log_tick(value),
        }
    }

    fn format_log_tick(value: f64) -> String {
        if !value.is_finite() {
            return "NaN".to_string();
        }

        if value <= 0.0 {
            return format!("{:.1}", value);
        }

        let exp = value.log10().round() as i32;
        let exact_power = 10f64.powi(exp);

        if (value - exact_power).abs() / exact_power.max(1.0) < 1e-9 {
            return match exp {
                -2 => "0.01".to_string(),
                -1 => "0.1".to_string(),
                0 => "1".to_string(),
                1 => "10".to_string(),
                2 => "100".to_string(),
                _ => format!("1e{}", exp),
            };
        }

        Self::format_compact(value)
    }

    fn format_compact(value: f64) -> String {
        let abs = value.abs();
        let raw = if abs >= 1000.0 || (abs > 0.0 && abs < 0.1) {
            format!("{:.1e}", value)
                .replace("e+0", "e")
                .replace("e+", "e")
                .replace("e-0", "e-")
        } else if abs >= 10.0 {
            format!("{:.1}", value)
        } else {
            format!("{:.2}", value)
        };

        Self::trim_trailing_zeros(raw)
    }

    fn trim_trailing_zeros(mut value: String) -> String {
        if let Some(exp_index) = value.find('e') {
            let exponent = value.split_off(exp_index);
            let trimmed = Self::trim_decimal(value);
            return format!("{trimmed}{exponent}");
        }

        Self::trim_decimal(value)
    }

    fn trim_decimal(mut value: String) -> String {
        if value.contains('.') {
            while value.ends_with('0') {
                value.pop();
            }
            if value.ends_with('.') {
                value.pop();
            }
        }
        value
    }
}
