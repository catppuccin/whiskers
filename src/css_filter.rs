#![allow(clippy::many_single_char_names)]

//! CSS Filter generation using SPSA (Simultaneous Perturbation Stochastic Approximation)
//!
//! This module implements an algorithm to derive CSS filter properties that transform
//! black (#000000) into a target color. This is useful for applying color transformations
//! to SVG icons and other elements that cannot be colored with fill, using CSS filters instead.
//!
//! Reference implementations:
//! - <https://stackoverflow.com/questions/42966641>
//! - <https://codepen.io/sosuke/pen/Pjoqqp>
//! - <https://github.com/angel-rs/css-color-filter-generator>

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use css_colors::Color as _;
use rand::{Rng, SeedableRng};

/// Type alias for the filter cache
type FilterCache = Mutex<HashMap<(u8, u8, u8), String>>;

/// In-memory cache for filter results
static FILTER_CACHE: OnceLock<FilterCache> = OnceLock::new();

/// Internal color representation for filter computation
#[derive(Debug, Clone)]
struct FilterColor(f32, f32, f32);

impl FilterColor {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self(
            f32::from(r) / 255.0,
            f32::from(g) / 255.0,
            f32::from(b) / 255.0,
        )
    }

    const fn r(&self) -> f32 {
        self.0
    }

    const fn g(&self) -> f32 {
        self.1
    }

    const fn b(&self) -> f32 {
        self.2
    }

    const fn set_rgb(&mut self, r: f32, g: f32, b: f32) {
        // Handle NaN and ensure valid ranges
        let r = if r.is_finite() { r } else { 0.0 };
        let g = if g.is_finite() { g } else { 0.0 };
        let b = if b.is_finite() { b } else { 0.0 };
        self.0 = r.clamp(0.0, 1.0);
        self.1 = g.clamp(0.0, 1.0);
        self.2 = b.clamp(0.0, 1.0);
    }

    fn invert(&mut self, value: f32) {
        let r = self
            .r()
            .mul_add(2.0f32.mul_add(-value, 1.0), value)
            .clamp(0.0, 1.0);
        let g = self
            .g()
            .mul_add(2.0f32.mul_add(-value, 1.0), value)
            .clamp(0.0, 1.0);
        let b = self
            .b()
            .mul_add(2.0f32.mul_add(-value, 1.0), value)
            .clamp(0.0, 1.0);
        self.set_rgb(r, g, b);
    }

    fn sepia(&mut self, value: f32) {
        let matrix = [
            0.607f32.mul_add(1.0 - value, 0.393),
            0.769f32.mul_add(-(1.0 - value), 0.769),
            0.189f32.mul_add(-(1.0 - value), 0.189),
            0.349f32.mul_add(-(1.0 - value), 0.349),
            0.314f32.mul_add(1.0 - value, 0.686),
            0.168f32.mul_add(-(1.0 - value), 0.168),
            0.272f32.mul_add(-(1.0 - value), 0.272),
            0.534f32.mul_add(-(1.0 - value), 0.534),
            0.869f32.mul_add(1.0 - value, 0.131),
        ];
        self.multiply(&matrix);
    }

    fn saturate(&mut self, value: f32) {
        let matrix = [
            0.787f32.mul_add(value, 0.213),
            0.715f32.mul_add(-value, 0.715),
            0.072f32.mul_add(-value, 0.072),
            0.213f32.mul_add(-value, 0.213),
            0.285f32.mul_add(value, 0.715),
            0.072f32.mul_add(-value, 0.072),
            0.213f32.mul_add(-value, 0.213),
            0.715f32.mul_add(-value, 0.715),
            0.928f32.mul_add(value, 0.072),
        ];
        self.multiply(&matrix);
    }

    fn hue_rotate(&mut self, angle: f32) {
        let angle = angle / 180.0 * std::f32::consts::PI;
        let sin = angle.sin();
        let cos = angle.cos();

        let matrix = [
            0.213 + cos * 0.787 - sin * 0.213,
            0.715 - cos * 0.715 - sin * 0.715,
            0.072 - cos * 0.072 + sin * 0.928,
            0.213 - cos * 0.213 + sin * 0.143,
            0.715 + cos * 0.285 + sin * 0.140,
            0.072 - cos * 0.072 - sin * 0.283,
            0.213 - cos * 0.213 - sin * 0.787,
            0.715 - cos * 0.715 + sin * 0.715,
            0.072 + cos * 0.928 + sin * 0.072,
        ];
        self.multiply(&matrix);
    }

    fn brightness(&mut self, value: f32) {
        self.linear(value, 0.0);
    }

    fn contrast(&mut self, value: f32) {
        self.linear(value, -(0.5 * value) + 0.5);
    }

    fn linear(&mut self, slope: f32, intercept: f32) {
        let r = self.r().mul_add(slope, intercept).clamp(0.0, 1.0);
        let g = self.g().mul_add(slope, intercept).clamp(0.0, 1.0);
        let b = self.b().mul_add(slope, intercept).clamp(0.0, 1.0);
        self.set_rgb(r, g, b);
    }

    fn multiply(&mut self, matrix: &[f32; 9]) {
        let r = self.r();
        let g = self.g();
        let b = self.b();
        let new_r = b
            .mul_add(matrix[2], r.mul_add(matrix[0], g * matrix[1]))
            .clamp(0.0, 1.0);
        let new_g = b
            .mul_add(matrix[5], r.mul_add(matrix[3], g * matrix[4]))
            .clamp(0.0, 1.0);
        let new_b = b
            .mul_add(matrix[8], r.mul_add(matrix[6], g * matrix[7]))
            .clamp(0.0, 1.0);
        self.set_rgb(new_r, new_g, new_b);
    }

    fn hsl(&self) -> css_colors::HSL {
        css_colors::RGB {
            r: css_colors::Ratio::from_f32(self.0),
            g: css_colors::Ratio::from_f32(self.1),
            b: css_colors::Ratio::from_f32(self.2),
        }
        .to_hsl()
    }

    // Fallback HSL conversion for intermediate filter calculations
    // css_colors can panic on edge case RGB values during SPSA optimization
    fn hsl_lossy(&self) -> (f32, f32, f32) {
        let r = self.r();
        let g = self.g();
        let b = self.b();
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let h;
        let s;
        let l = f32::midpoint(max, min);

        if (max - min).abs() < f32::EPSILON {
            h = 0.0;
            s = 0.0;
        } else {
            let d = max - min;
            s = if l > 0.5 {
                d / (2.0 - max - min)
            } else {
                d / (max + min)
            };

            h = if (max - r).abs() < f32::EPSILON {
                (g - b) / d + if g < b { 6.0 } else { 0.0 }
            } else if (max - g).abs() < f32::EPSILON {
                (b - r) / d + 2.0
            } else {
                (r - g) / d + 4.0
            } / 6.0;
        }

        (h * 360.0, s, l)
    }
}

/// Result of the CSS filter solver
#[derive(Debug, Clone)]
pub struct FilterResult {
    /// The filter values: [invert, sepia, saturate, `hue_rotate`, brightness, contrast]
    pub values: [f64; 6],
    /// Loss metric indicating how close the result is to the target
    pub loss: f64,
}

impl FilterResult {
    fn css(&self) -> String {
        format!(
            "invert({}%) sepia({}%) saturate({}%) hue-rotate({}deg) brightness({}%) contrast({}%)",
            self.values[0].round() as i32,
            self.values[1].round() as i32,
            self.values[2].round() as i32,
            (self.values[3] * 3.6).round() as i32,
            self.values[4].round() as i32,
            self.values[5].round() as i32,
        )
    }
}

struct Solver {
    target: FilterColor,
    target_hsl: css_colors::HSL,
    rng_seed: u64,
}

impl Solver {
    fn new(target: FilterColor) -> Self {
        let target_hsl = target.hsl();
        // Deterministic seed based on RGB values for reproducible builds
        let r = (target.r() * 255.0).round() as u32;
        let g = (target.g() * 255.0).round() as u32;
        let b = (target.b() * 255.0).round() as u32;
        let rng_seed = u64::from(r) | (u64::from(g) << 16) | (u64::from(b) << 32);
        Self {
            target,
            target_hsl,
            rng_seed,
        }
    }

    fn solve(&self) -> FilterResult {
        self.solve_narrow(&self.solve_wide())
    }

    fn solve_wide(&self) -> FilterResult {
        let a_val = 5.0;
        let c = 15.0;
        let a = [60.0, 180.0, 18000.0, 600.0, 1.2, 1.2];

        let mut best = FilterResult {
            loss: f64::INFINITY,
            values: [0.0; 6],
        };

        for _ in 0..10 {
            if best.loss <= 5.0 {
                break;
            }
            let initial = [50.0, 20.0, 3750.0, 50.0, 100.0, 100.0];
            let result = self.spsa(a_val, &a, c, &initial, 3000);
            if result.loss < best.loss {
                best = result;
            }
        }
        best
    }

    fn solve_narrow(&self, wide: &FilterResult) -> FilterResult {
        let a_val = wide.loss;
        let c = 2.0;
        let a1 = a_val + 1.0;
        let a = [0.25 * a1, 0.25 * a1, a1, 0.25 * a1, 0.2 * a1, 0.2 * a1];
        self.spsa(a_val, &a, c, &wide.values, 1000)
    }

    fn spsa(
        &self,
        big_a: f64,
        a: &[f64; 6],
        c: f64,
        initial_values: &[f64; 6],
        iters: usize,
    ) -> FilterResult {
        let alpha = 1.0;
        let gamma = 1.0 / 6.0;

        let mut best: Option<FilterResult> = None;
        let mut best_loss = f64::INFINITY;
        let mut values = *initial_values;
        // Use deterministic RNG seeded by target color for reproducible builds
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.rng_seed);

        for k in 0..iters {
            let ck = c / f64::from(k as u32 + 1).powf(gamma);
            let mut deltas = [0.0; 6];
            let mut high_args = [0.0; 6];
            let mut low_args = [0.0; 6];

            for i in 0..6 {
                deltas[i] = if rng.gen::<f64>() > 0.5 { 1.0 } else { -1.0 };
                high_args[i] = values[i] + ck * deltas[i];
                low_args[i] = values[i] - ck * deltas[i];
            }

            let loss_diff = self.loss(&high_args) - self.loss(&low_args);
            for i in 0..6 {
                let g = loss_diff / (2.0 * ck) * deltas[i];
                let ak = a[i] / f64::from((big_a as u32) + k as u32 + 1).powf(alpha);
                values[i] = fix(values[i] - ak * g, i);
            }

            let loss = self.loss(&values);
            if loss < best_loss {
                best = Some(FilterResult { values, loss });
                best_loss = loss;
            }
        }

        best.unwrap_or(FilterResult {
            values,
            loss: best_loss,
        })
    }

    fn loss(&self, filters: &[f64; 6]) -> f64 {
        let mut color = FilterColor::new(0, 0, 0);

        color.invert((filters[0] / 100.0) as f32);
        color.sepia((filters[1] / 100.0) as f32);
        color.saturate((filters[2] / 100.0) as f32);
        color.hue_rotate((filters[3] * 3.6) as f32);
        color.brightness((filters[4] / 100.0) as f32);
        color.contrast((filters[5] / 100.0) as f32);

        // Use lossy HSL for intermediate calculations to avoid css_colors panics
        let (h, s, l) = color.hsl_lossy();
        let target_h = f32::from(self.target_hsl.h.degrees());
        let target_s = self.target_hsl.s.as_f32();
        let target_l = self.target_hsl.l.as_f32();

        f64::from((color.r() - self.target.r()).abs() * 255.0)
            + f64::from((color.g() - self.target.g()).abs() * 255.0)
            + f64::from((color.b() - self.target.b()).abs() * 255.0)
            + f64::from((h - target_h).abs())
            + f64::from((s - target_s).abs() * 100.0)
            + f64::from((l - target_l).abs() * 100.0)
    }
}

fn fix(value: f64, idx: usize) -> f64 {
    let max = match idx {
        2 => 7500.0,    // saturate
        4 | 5 => 200.0, // brightness, contrast
        _ => 100.0,
    };

    if idx == 3 {
        // hue-rotate
        if value > max {
            value % max
        } else if value < 0.0 {
            max + value % max
        } else {
            value
        }
    } else if value < 0.0 {
        0.0
    } else if value > max {
        max
    } else {
        value
    }
}

/// Generate a CSS filter string that transforms black to the given color
///
/// Results are cached in memory to avoid redundant expensive computations.
///
/// # Arguments
/// * `color` - A Color object containing RGB values
///
/// # Returns
/// A CSS filter string
///
/// # Panics
/// Panics if the cache mutex is poisoned (i.e., a thread panicked while holding the lock)
pub fn css_filter(color: &crate::models::Color) -> String {
    let rgb_key = (color.rgb.r, color.rgb.g, color.rgb.b);

    // Initialize cache if needed
    let cache = FILTER_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    // Check cache for color
    if let Some(cached) = cache.lock().expect("lock good").get(&rgb_key) {
        return cached.clone();
    }

    // Compute if not cached
    let target = FilterColor::new(color.rgb.r, color.rgb.g, color.rgb.b);
    let solver = Solver::new(target);
    let result = solver.solve();
    let filter_string = result.css();

    // Store in cache
    cache
        .lock()
        .expect("lock good")
        .insert(rgb_key, filter_string.clone());

    filter_string
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]
    use super::*;

    #[test]
    fn test_fix_saturate() {
        assert_eq!(fix(8000.0, 2), 7500.0);
        assert_eq!(fix(-10.0, 2), 0.0);
        assert_eq!(fix(5000.0, 2), 5000.0);
    }

    #[test]
    fn test_fix_hue_rotate() {
        assert!(fix(150.0, 3) > 0.0);
        assert!(fix(-50.0, 3) > 0.0);
    }

    #[test]
    fn test_hsl_conversion() {
        let color = FilterColor::new(255, 0, 0);
        let hsl = color.hsl();
        assert!(hsl.h.degrees() < 1); // Red should be near 0 degrees
        assert!(hsl.s.as_f32() > 0.9); // Should be highly saturated
        assert!((hsl.l.as_f32() - 0.5).abs() < 0.05); // Should be around 50% lightness
    }

    #[test]
    fn test_repeated_output() {
        // Same color should produce identical results across multiple runs
        let color1 = FilterColor::new(210, 15, 57);
        let solver1 = Solver::new(color1);
        let result1 = solver1.solve();

        let color2 = FilterColor::new(210, 15, 57);
        let solver2 = Solver::new(color2);
        let result2 = solver2.solve();

        assert_eq!(result1.css(), result2.css());
        assert_eq!(result1.values, result2.values);
    }
}
