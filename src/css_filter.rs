//! CSS Filter generation using COBYLA (Constrained Optimization BY Linear Approximation)
//!
//! This module implements an algorithm to derive CSS filter properties that transform
//! black (#000000) into a target color. This is useful for applying color transformations
//! to SVG icons and other elements that cannot be colored with fill, using CSS filters instead.
//!
//! The loss function uses the Oklab perceptually uniform color space for more accurate
//! color matching.
//!
//! This implementation uses fixed-point arithmetic for deterministic cross-platform results.
//!
//! Reference implementations:
//! - <https://stackoverflow.com/questions/42966641>
//! - <https://codepen.io/sosuke/pen/Pjoqqp>
//! - <https://github.com/angel-rs/css-color-filter-generator>

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use cobyla::{minimize, RhoBeg, StopTols};
use fixed::types::{I16F16, I3F29, I8F24};

/// Type alias for the filter cache (RGB)
type FilterCache = Mutex<HashMap<(u8, u8, u8), String>>;

/// In-memory cache for filter results
static FILTER_CACHE: OnceLock<FilterCache> = OnceLock::new();

/// Fixed-point type for RGB color channels (0.0-1.0 range, 24 fractional bits)
type Fx = I8F24;

/// Fixed-point type for saturate parameter (can be up to 75.0, 16 integer bits)
type FxParam = I16F16;

// Rec. 709 luma coefficients and their complements
const R_LUMA: f64 = 0.2126;
const G_LUMA: f64 = 0.7152;
const B_LUMA: f64 = 0.0722;
const R_COMP: f64 = 1.0 - R_LUMA; // 0.7874
const G_COMP: f64 = 1.0 - G_LUMA; // 0.2848
const B_COMP: f64 = 1.0 - B_LUMA; // 0.9278

// Hue rotation auxiliary coefficients (derived from luma)
const HUE_A: f64 = 0.14303; // sqrt(1/3) * R_COMP
const HUE_B: f64 = 0.14014; // sqrt(1/3) * (1 - G_LUMA - R_LUMA)
const HUE_C: f64 = 0.28302; // sqrt(1/3) * (G_LUMA + R_LUMA)

/// Internal color representation for filter computation using fixed-point arithmetic
#[derive(Debug, Clone, Copy)]
struct FilterColor {
    r: Fx,
    g: Fx,
    b: Fx,
}

impl FilterColor {
    fn new(r: u8, g: u8, b: u8) -> Self {
        // Convert to 0.0-1.0 range (divide in f32 first since u8 255 > I8F24 max ~127)
        Self {
            r: Fx::from_num(f32::from(r) / 255.0),
            g: Fx::from_num(f32::from(g) / 255.0),
            b: Fx::from_num(f32::from(b) / 255.0),
        }
    }

    const fn black() -> Self {
        Self {
            r: Fx::ZERO,
            g: Fx::ZERO,
            b: Fx::ZERO,
        }
    }

    fn set_rgb(&mut self, r: Fx, g: Fx, b: Fx) {
        self.r = r.clamp(Fx::ZERO, Fx::ONE);
        self.g = g.clamp(Fx::ZERO, Fx::ONE);
        self.b = b.clamp(Fx::ZERO, Fx::ONE);
    }

    fn invert(&mut self, value: Fx) {
        // r * (1 - 2*value) + value
        let factor = Fx::ONE.saturating_sub(Fx::from_num(2).saturating_mul(value));
        self.set_rgb(
            self.r.saturating_mul(factor).saturating_add(value),
            self.g.saturating_mul(factor).saturating_add(value),
            self.b.saturating_mul(factor).saturating_add(value),
        );
    }

    fn sepia(&mut self, value: Fx) {
        let inv = Fx::ONE.saturating_sub(value);
        // Sepia matrix coefficients (standard sepia tone values)
        let f = |a: f64, b: f64| {
            Fx::from_num(a)
                .saturating_mul(inv)
                .saturating_add(Fx::from_num(b))
        };
        let g = |a: f64| Fx::from_num(a).saturating_sub(Fx::from_num(a).saturating_mul(inv));
        self.multiply(&[
            f(0.607, 0.393),
            g(0.769),
            g(0.189),
            g(0.349),
            f(0.314, 0.686),
            g(0.168),
            g(0.272),
            g(0.534),
            f(0.869, 0.131),
        ]);
    }

    fn saturate(&mut self, value: FxParam) {
        // Saturate can be up to 75.0, so we use I16F16 then convert to I8F24
        let (r, g, b) = (
            FxParam::from_num(R_LUMA),
            FxParam::from_num(G_LUMA),
            FxParam::from_num(B_LUMA),
        );
        let (rc, gc, bc) = (
            FxParam::from_num(R_COMP),
            FxParam::from_num(G_COMP),
            FxParam::from_num(B_COMP),
        );
        let sat = |base: FxParam, mult: FxParam, add: bool| {
            let result = if add {
                base.saturating_add(mult.saturating_mul(value))
            } else {
                base.saturating_sub(mult.saturating_mul(value))
            };
            Fx::saturating_from_num(result)
        };
        self.multiply(&[
            sat(r, rc, true),
            sat(g, g, false),
            sat(b, b, false),
            sat(r, r, false),
            sat(g, gc, true),
            sat(b, b, false),
            sat(r, r, false),
            sat(g, g, false),
            sat(b, bc, true),
        ]);
    }

    fn hue_rotate(&mut self, angle_deg: f64) {
        // Normalize to [-π, π] for I3F29 compatibility with cordic
        let pi = std::f64::consts::PI;
        let angle_rad = angle_deg.to_radians();
        let normalized = ((angle_rad + pi).rem_euclid(2.0 * pi)) - pi;
        let (sin, cos) = cordic::sin_cos(I3F29::from_num(normalized));
        let (sin, cos) = (Fx::from_num(sin), Fx::from_num(cos));

        // Luma coefficients
        let (r, g, b) = (
            Fx::from_num(R_LUMA),
            Fx::from_num(G_LUMA),
            Fx::from_num(B_LUMA),
        );
        let (rc, gc, bc) = (
            Fx::from_num(R_COMP),
            Fx::from_num(G_COMP),
            Fx::from_num(B_COMP),
        );
        let (ha, hb, hc) = (
            Fx::from_num(HUE_A),
            Fx::from_num(HUE_B),
            Fx::from_num(HUE_C),
        );

        self.multiply(&[
            r.saturating_add(cos.saturating_mul(rc))
                .saturating_sub(sin.saturating_mul(r)),
            g.saturating_sub(cos.saturating_mul(g))
                .saturating_sub(sin.saturating_mul(g)),
            b.saturating_sub(cos.saturating_mul(b))
                .saturating_add(sin.saturating_mul(bc)),
            r.saturating_sub(cos.saturating_mul(r))
                .saturating_add(sin.saturating_mul(ha)),
            g.saturating_add(cos.saturating_mul(gc))
                .saturating_add(sin.saturating_mul(hb)),
            b.saturating_sub(cos.saturating_mul(b))
                .saturating_sub(sin.saturating_mul(hc)),
            r.saturating_sub(cos.saturating_mul(r))
                .saturating_sub(sin.saturating_mul(rc)),
            g.saturating_sub(cos.saturating_mul(g))
                .saturating_add(sin.saturating_mul(g)),
            b.saturating_add(cos.saturating_mul(bc))
                .saturating_add(sin.saturating_mul(b)),
        ]);
    }

    fn brightness(&mut self, value: Fx) {
        self.linear(value, Fx::ZERO);
    }

    fn contrast(&mut self, value: Fx) {
        let half = Fx::from_num(0.5);
        self.linear(value, half.saturating_sub(half.saturating_mul(value)));
    }

    fn linear(&mut self, slope: Fx, intercept: Fx) {
        self.set_rgb(
            self.r.saturating_mul(slope).saturating_add(intercept),
            self.g.saturating_mul(slope).saturating_add(intercept),
            self.b.saturating_mul(slope).saturating_add(intercept),
        );
    }

    fn multiply(&mut self, m: &[Fx; 9]) {
        let (r, g, b) = (self.r, self.g, self.b);
        self.set_rgb(
            r.saturating_mul(m[0])
                .saturating_add(g.saturating_mul(m[1]))
                .saturating_add(b.saturating_mul(m[2])),
            r.saturating_mul(m[3])
                .saturating_add(g.saturating_mul(m[4]))
                .saturating_add(b.saturating_mul(m[5])),
            r.saturating_mul(m[6])
                .saturating_add(g.saturating_mul(m[7]))
                .saturating_add(b.saturating_mul(m[8])),
        );
    }

    fn to_oklab(self) -> oklab::Oklab {
        oklab::srgb_f32_to_oklab(oklab::Rgb {
            r: self.r.to_num(),
            g: self.g.to_num(),
            b: self.b.to_num(),
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

/// Quantize to 0.1 precision to ensure deterministic optimizer convergence across platforms
#[inline]
fn quantize(x: f64) -> f64 {
    (x * 10.0).round() / 10.0
}

/// Compute the loss (distance in Oklab color space) for given filter values
fn compute_loss(filters: &[f64], target_oklab: oklab::Oklab) -> f64 {
    let mut color = FilterColor::black();

    // Quantize params to ensure deterministic convergence across platforms
    // (COBYLA uses f64 internally which can vary slightly by platform)
    let f: [f64; 6] = std::array::from_fn(|i| quantize(filters[i]));

    // Convert quantized f64 params to fixed-point
    color.invert(Fx::from_num(f[0] / 100.0));
    color.sepia(Fx::from_num(f[1] / 100.0));
    color.saturate(FxParam::from_num(f[2] / 100.0));
    color.hue_rotate(f[3] * 3.6);
    color.brightness(Fx::from_num(f[4] / 100.0));
    color.contrast(Fx::from_num(f[5] / 100.0));

    // Euclidean distance in Oklab color space (perceptually uniform)
    let ok = color.to_oklab();
    let (dl, da, db) = (ok.l - target_oklab.l, ok.a - target_oklab.a, ok.b - target_oklab.b);
    f64::from(dl.mul_add(dl, da.mul_add(da, db * db))) * 5000.0
}

/// Solve for CSS filter values using COBYLA optimization
fn solve(target: FilterColor) -> FilterResult {
    let target_oklab = target.to_oklab();

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
        let oklab = color.to_oklab();
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
