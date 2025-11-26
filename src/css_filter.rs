//! CSS Filter generation using COBYLA (Constrained Optimization BY Linear Approximation)
//!
//! This module implements an algorithm to derive CSS filter properties that transform
//! black (#000000) into a target color. This is useful for applying color transformations
//! to SVG icons and other elements that cannot be colored with fill, using CSS filters instead.
//!
//! The loss function uses the Oklab perceptually uniform color space for more accurate
//! color matching.
//!
//! Reference implementations:
//! - <https://stackoverflow.com/questions/42966641>
//! - <https://codepen.io/sosuke/pen/Pjoqqp>
//! - <https://github.com/angel-rs/css-color-filter-generator>

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use cobyla::{minimize, RhoBeg, StopTols};

/// Type alias for the filter cache (RGB)
type FilterCache = Mutex<HashMap<(u8, u8, u8), String>>;

/// In-memory cache for filter results
static FILTER_CACHE: OnceLock<FilterCache> = OnceLock::new();

/// Internal color representation for filter computation
#[derive(Debug, Clone, Copy)]
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

    fn oklab(&self) -> oklab::Oklab {
        oklab::srgb_f32_to_oklab(oklab::Rgb {
            r: self.0,
            g: self.1,
            b: self.2,
        })
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

/// Compute the loss (distance in Oklab color space) for given filter values
fn compute_loss(filters: &[f64], target_oklab: oklab::Oklab) -> f64 {
    let mut color = FilterColor::new(0, 0, 0);

    color.invert((filters[0] / 100.0) as f32);
    color.sepia((filters[1] / 100.0) as f32);
    color.saturate((filters[2] / 100.0) as f32);
    color.hue_rotate((filters[3] * 3.6) as f32);
    color.brightness((filters[4] / 100.0) as f32);
    color.contrast((filters[5] / 100.0) as f32);

    // Compute Euclidean distance in Oklab color space (perceptually uniform)
    let current_oklab = color.oklab();
    let dl = current_oklab.l - target_oklab.l;
    let da = current_oklab.a - target_oklab.a;
    let db = current_oklab.b - target_oklab.b;
    f64::from(dl * dl + da * da + db * db) * 5000.0
}

/// Solve for CSS filter values using COBYLA optimization
fn solve(target: FilterColor) -> FilterResult {
    let target_oklab = target.oklab();

    // Objective function for COBYLA
    let objective = |x: &[f64], (): &mut ()| compute_loss(x, target_oklab);

    // Bounds for each parameter:
    // [invert, sepia, saturate, hue_rotate, brightness, contrast]
    let bounds = [
        (0.0, 100.0),  // invert: 0-100%
        (0.0, 100.0),  // sepia: 0-100%
        (0.0, 7500.0), // saturate: 0-7500%
        (0.0, 100.0),  // hue_rotate: 0-100 (×3.6 = 0-360deg)
        (0.0, 200.0),  // brightness: 0-200%
        (0.0, 200.0),  // contrast: 0-200%
    ];

    // No additional constraints beyond bounds
    let cons: Vec<&dyn cobyla::Func<()>> = vec![];

    let stop_tols = StopTols {
        ftol_rel: 1e-10,
        ftol_abs: 1e-10,
        xtol_rel: 1e-8,
        ..StopTols::default()
    };

    // Try multiple starting points and keep the best result
    // Parameters: [invert, sepia, saturate, hue_rotate, brightness, contrast]
    // Starting points derived from known good solutions for Catppuccin accent colors
    let starting_points = [
        // Rosewater-like
        [100.0, 0.0, 519.0, 63.0, 103.0, 80.0],
        // Flamingo-like
        [100.0, 0.0, 532.0, 37.0, 94.0, 77.0],
        // Pink-like
        [69.0, 15.0, 3742.0, 57.5, 122.0, 118.0],
        // Mauve-like
        [63.0, 71.0, 1010.0, 57.7, 110.0, 94.0],
        // Red-like
        [66.0, 83.0, 4997.0, 90.0, 157.0, 149.0],
        // Maroon-like
        [70.0, 24.0, 510.0, 83.3, 100.0, 82.0],
        // Peach-like
        [48.0, 89.0, 2010.0, 4.7, 124.0, 100.0],
        // Yellow-like
        [100.0, 0.0, 520.0, 63.6, 103.0, 79.0],
        // Green-like
        [87.0, 24.0, 529.0, 16.7, 97.0, 86.0],
        // Teal-like
        [90.0, 24.0, 521.0, 29.2, 96.0, 85.0],
        // Sky-like
        [81.0, 27.0, 525.0, 39.4, 100.0, 84.0],
        // Sapphire-like
        [76.0, 28.0, 508.0, 42.2, 95.0, 86.0],
        // Blue-like
        [66.0, 32.0, 518.0, 49.4, 100.0, 94.0],
        // Lavender-like
        [66.0, 19.0, 3749.0, 55.8, 117.0, 113.0],
    ];

    let mut best_result = FilterResult {
        values: [0.0; 6],
        loss: f64::INFINITY,
    };

    for x0 in &starting_points {
        let (x_opt, loss) = match minimize(
            objective,
            x0,
            &bounds,
            &cons,
            (),
            3000, // max iterations
            RhoBeg::All(10.0),
            Some(stop_tols.clone()),
        ) {
            Ok((_, x_opt, loss)) | Err((_, x_opt, loss)) => (x_opt, loss),
        };
        let result = FilterResult {
            values: [x_opt[0], x_opt[1], x_opt[2], x_opt[3], x_opt[4], x_opt[5]],
            loss,
        };

        if result.loss < best_result.loss {
            best_result = result;
        }

        // Early exit if we found a good solution
        if best_result.loss < 0.01 {
            break;
        }
    }

    best_result
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
    let cache_key = (color.rgb.r, color.rgb.g, color.rgb.b);

    // Initialize cache if needed
    let cache = FILTER_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    // Check cache for color
    if let Some(cached) = cache.lock().expect("lock good").get(&cache_key) {
        return cached.clone();
    }

    // Compute if not cached
    let target = FilterColor::new(color.rgb.r, color.rgb.g, color.rgb.b);
    let result = solve(target);
    let filter_string = result.css();

    // Store in cache
    cache
        .lock()
        .expect("lock good")
        .insert(cache_key, filter_string.clone());

    filter_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oklab_conversion() {
        let color = FilterColor::new(255, 0, 0);
        let oklab = color.oklab();
        // Red in Oklab should have positive a (towards red) and positive b (towards yellow)
        assert!(oklab.l > 0.5); // Should have decent lightness
        assert!(oklab.a > 0.1); // Should be strongly towards red
        assert!(oklab.b > 0.0); // Red has positive b
    }

    #[test]
    fn test_repeated_output() {
        // Same color should produce identical results across multiple runs
        let color1 = FilterColor::new(210, 15, 57);
        let result1 = solve(color1);

        let color2 = FilterColor::new(210, 15, 57);
        let result2 = solve(color2);

        assert_eq!(result1.css(), result2.css());
        assert_eq!(result1.values, result2.values);
    }

    #[test]
    fn test_low_loss() {
        // The solver should achieve a low loss for typical colors
        let color = FilterColor::new(210, 15, 57); // Catppuccin red
        let result = solve(color);

        // Loss should be reasonably low (good color match)
        assert!(result.loss < 1.0, "Loss {} is too high", result.loss);
    }
}
