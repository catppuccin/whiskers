use std::sync::OnceLock;

use css_colors::Color as _;
use indexmap::IndexMap;
use serde_json::json;
use tera::Tera;

use crate::cli::ColorOverrides;

// a frankenstein mix of Catppuccin & css_colors types to get all the
// functionality we want.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Palette {
    pub flavors: IndexMap<String, Flavor>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Flavor {
    pub name: String,
    pub identifier: String,
    pub emoji: char,
    pub order: u32,
    pub dark: bool,
    pub light: bool,
    pub colors: IndexMap<String, Color>,
    pub ansi_colors: IndexMap<String, AnsiColor>,
    pub ansi_color_pairs: IndexMap<String, AnsiColorPair>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Color {
    pub name: String,
    pub identifier: String,
    pub order: u32,
    pub accent: bool,
    pub hex: String,
    pub int24: u32,
    pub uint32: u32,
    pub sint32: i32,
    pub rgb: RGB,
    pub hsl: HSL,
    pub opacity: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AnsiColor {
    pub name: String,
    pub identifier: String,
    pub code: u8,
    pub hex: String,
    pub int24: u32,
    pub uint32: u32,
    pub sint32: i32,
    pub rgb: RGB,
    pub hsl: HSL,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AnsiColorPair {
    pub name: String,
    pub identifier: String,
    pub order: u32,
    pub normal: AnsiColor,
    pub bright: AnsiColor,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct HSL {
    pub h: u16,
    pub s: f32,
    pub l: f32,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Hex formatting failed: {0}")]
    HexFormat(#[from] tera::Error),
    #[error("Failed to parse hex color: {0}")]
    ParseHex(#[from] std::num::ParseIntError),
}

// we have many functions that need to know how to format hex colors.
// they can't know this at build time, as the format may be provided by the template.
// we have little to no available state in many of these functions to store this information.
// these possible solutions were evaluated:
// 1. pass the format string to every function that needs it. this is cumbersome and error-prone.
// 2. store the format string in the `Colour` struct, thus duplicating it for every color. this is wasteful.
// 3. store it in a global static, and initialize it when the template frontmatter is read.
// we opted for the third option, with a convenience macro for accessing it.
pub static HEX_FORMAT: OnceLock<String> = OnceLock::new();
macro_rules! format_hex {
    ($r:expr, $g:expr, $b: expr, $a: expr) => {
        format_hex(
            $r,
            $g,
            $b,
            $a,
            &*HEX_FORMAT.get().expect("HEX_FORMAT was never set"),
        )
    };
}

/// attempt to canonicalize a hex string, using the provided format string.
fn format_hex(r: u8, g: u8, b: u8, a: u8, hex_format: &str) -> tera::Result<String> {
    Tera::one_off(
        hex_format,
        &tera::Context::from_serialize(json!({
            "r": format!("{r:02x}"),
            "g": format!("{g:02x}"),
            "b": format!("{b:02x}"),
            "a": format!("{a:02x}"),
            "z": if a == 0xFF { String::new() } else { format!("{a:02x}") },
            "R": format!("{r:02X}"),
            "G": format!("{g:02X}"),
            "B": format!("{b:02X}"),
            "A": format!("{a:02X}"),
            "Z": if a == 0xFF { String::new() } else { format!("{a:02X}") },
        }))
        .expect("hardcoded context is always valid"),
        true,
    )
}

/// produce three values from a given rgb value and opacity:
/// 1. a 24-bit unsigned integer with the format `0xRRGGBB`
/// 2. a 32-bit unsigned integer with the format `0xAARRGGBB`
/// 3. a 32-bit signed integer with the format `0xAARRGGBB`
///
/// opacity is optional, and defaults to `0xFF`.
fn rgb_to_ints(rgb: &RGB, opacity: Option<u8>) -> (u32, u32, i32) {
    let opacity = opacity.unwrap_or(0xFF);
    let uint24 = u32::from_be_bytes([0x00, rgb.r, rgb.g, rgb.b]);
    let uint32 = u32::from_be_bytes([opacity, rgb.r, rgb.g, rgb.b]);
    #[allow(clippy::cast_possible_wrap)]
    (uint24, uint32, uint32 as i32)
}

fn color_from_hex_override(hex: &str, blueprint: &catppuccin::Color) -> Result<Color, Error> {
    let i = u32::from_str_radix(hex, 16)?;
    let rgb = RGB {
        r: ((i >> 16) & 0xFF) as u8,
        g: ((i >> 8) & 0xFF) as u8,
        b: (i & 0xFF) as u8,
    };
    let hsl = css_colors::rgb(rgb.r, rgb.g, rgb.b).to_hsl();
    let hex = format_hex!(rgb.r, rgb.g, rgb.b, 0xFF)?;
    let (int24, uint32, sint32) = rgb_to_ints(&rgb, None);
    Ok(Color {
        name: blueprint.name.to_string(),
        identifier: blueprint.name.identifier().to_string(),
        order: blueprint.order,
        accent: blueprint.accent,
        hex,
        int24,
        uint32,
        sint32,
        rgb,
        hsl: HSL {
            h: hsl.h.degrees(),
            s: hsl.s.as_f32(),
            l: hsl.l.as_f32(),
        },
        opacity: 0xFF,
    })
}

fn color_from_catppuccin(color: &catppuccin::Color) -> tera::Result<Color> {
    let hex = format_hex!(color.rgb.r, color.rgb.g, color.rgb.b, 0xFF)?;
    let rgb: RGB = color.rgb.into();
    let (int24, uint32, sint32) = rgb_to_ints(&rgb, None);
    Ok(Color {
        name: color.name.to_string(),
        identifier: color.name.identifier().to_string(),
        order: color.order,
        accent: color.accent,
        hex,
        int24,
        uint32,
        sint32,
        rgb,
        hsl: HSL {
            h: color.hsl.h.round() as u16,
            s: color.hsl.s as f32,
            l: color.hsl.l as f32,
        },
        opacity: 255,
    })
}

fn ansi_color_from_catppuccin(ansi_color: &catppuccin::AnsiColor) -> tera::Result<AnsiColor> {
    let hex = format_hex!(ansi_color.rgb.r, ansi_color.rgb.g, ansi_color.rgb.b, 0xFF)?;
    let rgb: RGB = ansi_color.rgb.into();
    let (int24, uint32, sint32) = rgb_to_ints(&rgb, None);
    Ok(AnsiColor {
        name: ansi_color.name.to_string(),
        identifier: ansi_color.name.identifier().to_string(),
        code: ansi_color.code,
        hex,
        int24,
        uint32,
        sint32,
        rgb,
        hsl: HSL {
            h: ansi_color.hsl.h.round() as u16,
            s: ansi_color.hsl.s as f32,
            l: ansi_color.hsl.l as f32,
        },
    })
}

fn ansi_color_pair_from_catppuccin(
    ansi_color_pair: &catppuccin::AnsiColorPair,
) -> tera::Result<AnsiColorPair> {
    Ok(AnsiColorPair {
        name: ansi_color_pair.name.to_string(),
        identifier: ansi_color_pair.name.identifier().to_string(),
        order: ansi_color_pair.order,
        normal: ansi_color_from_catppuccin(&ansi_color_pair.normal)?,
        bright: ansi_color_from_catppuccin(&ansi_color_pair.bright)?,
    })
}

/// Build a [`Palette`] from [`catppuccin::PALETTE`], optionally applying color overrides.
pub fn build_palette(color_overrides: Option<&ColorOverrides>) -> Result<Palette, Error> {
    // make a `Color` from a `catppuccin::Color`, taking into account `color_overrides`.
    // overrides apply in this order:
    // 1. base color
    // 2. "all" override
    // 3. flavor override
    let make_color =
        |color: &catppuccin::Color, flavor_name: catppuccin::FlavorName| -> Result<Color, Error> {
            let flavor_override = color_overrides
                .map(|co| match flavor_name {
                    catppuccin::FlavorName::Latte => &co.latte,
                    catppuccin::FlavorName::Frappe => &co.frappe,
                    catppuccin::FlavorName::Macchiato => &co.macchiato,
                    catppuccin::FlavorName::Mocha => &co.mocha,
                })
                .and_then(|o| o.get(color.name.identifier()).cloned())
                .map(|s| color_from_hex_override(&s, color))
                .transpose()?;

            let all_override = color_overrides
                .and_then(|co| co.all.get(color.name.identifier()).cloned())
                .map(|s| color_from_hex_override(&s, color))
                .transpose()?;

            let base_color = color_from_catppuccin(color)?;

            Ok(flavor_override.or(all_override).unwrap_or(base_color))
        };

    let mut flavors = IndexMap::new();
    for flavor in &catppuccin::PALETTE {
        let mut colors = IndexMap::new();
        for color in &flavor.colors {
            colors.insert(
                color.name.identifier().to_string(),
                make_color(color, flavor.name)?,
            );
        }

        let mut ansi_colors = IndexMap::new();
        for ansi_color in &flavor.ansi_colors {
            ansi_colors.insert(
                ansi_color.name.identifier().to_string(),
                ansi_color_from_catppuccin(ansi_color)?,
            );
        }

        let mut ansi_color_pairs = IndexMap::new();
        for ansi_color_pair in &flavor.ansi_colors.all_pairs() {
            ansi_color_pairs.insert(
                ansi_color_pair.name.identifier().to_string(),
                ansi_color_pair_from_catppuccin(ansi_color_pair)?,
            );
        }

        flavors.insert(
            flavor.name.identifier().to_string(),
            Flavor {
                name: flavor.name.to_string(),
                identifier: flavor.name.identifier().to_string(),
                emoji: flavor.emoji,
                order: flavor.order,
                dark: flavor.dark,
                light: !flavor.dark,
                colors,
                ansi_colors,
                ansi_color_pairs,
            },
        );
    }
    Ok(Palette { flavors })
}

impl Palette {
    #[must_use]
    pub fn iter(&self) -> indexmap::map::Iter<String, Flavor> {
        self.flavors.iter()
    }
}

impl<'a> IntoIterator for &'a Palette {
    type Item = (&'a String, &'a Flavor);
    type IntoIter = indexmap::map::Iter<'a, String, Flavor>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Flavor {
    #[must_use]
    pub fn iter(&self) -> indexmap::map::Iter<String, Color> {
        self.colors.iter()
    }
}

impl<'a> IntoIterator for &'a Flavor {
    type Item = (&'a String, &'a Color);
    type IntoIter = indexmap::map::Iter<'a, String, Color>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn rgb_to_hex(rgb: &RGB, opacity: u8) -> tera::Result<String> {
    format_hex!(rgb.r, rgb.g, rgb.b, opacity)
}

impl Color {
    fn from_hsla(hsla: css_colors::HSLA, blueprint: &Self) -> tera::Result<Self> {
        let rgb = hsla.to_rgb();
        let rgb = RGB {
            r: rgb.r.as_u8(),
            g: rgb.g.as_u8(),
            b: rgb.b.as_u8(),
        };
        let hsl = HSL {
            h: hsla.h.degrees(),
            s: hsla.s.as_f32(),
            l: hsla.l.as_f32(),
        };
        let opacity = hsla.a.as_u8();
        let (int24, uint32, sint32) = rgb_to_ints(&rgb, Some(opacity));
        Ok(Self {
            name: blueprint.name.clone(),
            identifier: blueprint.identifier.clone(),
            order: blueprint.order,
            accent: blueprint.accent,
            hex: rgb_to_hex(&rgb, opacity)?,
            int24,
            uint32,
            sint32,
            rgb,
            hsl,
            opacity,
        })
    }

    fn from_rgba(rgba: css_colors::RGBA, blueprint: &Self) -> tera::Result<Self> {
        let hsl = rgba.to_hsl();
        let rgb = RGB {
            r: rgba.r.as_u8(),
            g: rgba.g.as_u8(),
            b: rgba.b.as_u8(),
        };
        let hsl = HSL {
            h: hsl.h.degrees(),
            s: hsl.s.as_f32(),
            l: hsl.l.as_f32(),
        };
        let opacity = rgba.a.as_u8();
        let (int24, uint32, sint32) = rgb_to_ints(&rgb, Some(opacity));
        Ok(Self {
            name: blueprint.name.clone(),
            identifier: blueprint.identifier.clone(),
            order: blueprint.order,
            accent: blueprint.accent,
            hex: rgb_to_hex(&rgb, opacity)?,
            int24,
            uint32,
            sint32,
            rgb,
            hsl,
            opacity,
        })
    }

    pub fn mix(base: &Self, blend: &Self, amount: f64) -> tera::Result<Self> {
        let amount = (amount * 100.0).clamp(0.0, 100.0).round() as u8;
        let blueprint = base;
        let base: css_colors::RGBA = base.into();
        let base = base.to_rgba();
        let blend: css_colors::RGBA = blend.into();
        let result = base.mix(blend, css_colors::percent(amount));
        Self::from_rgba(result, blueprint)
    }

    pub fn mod_hue(&self, hue: i32) -> tera::Result<Self> {
        let mut hsl: css_colors::HSL = self.into();
        hsl.h = css_colors::deg(hue);
        Self::from_hsla(hsl.to_hsla(), self)
    }

    pub fn add_hue(&self, hue: i32) -> tera::Result<Self> {
        let hsl: css_colors::HSL = self.into();
        let hsl = hsl.spin(css_colors::deg(hue));
        Self::from_hsla(hsl.to_hsla(), self)
    }

    pub fn sub_hue(&self, hue: i32) -> tera::Result<Self> {
        let hsl: css_colors::HSL = self.into();
        let hsl = hsl.spin(-css_colors::deg(hue));
        Self::from_hsla(hsl.to_hsla(), self)
    }

    pub fn mod_saturation(&self, saturation: u8) -> tera::Result<Self> {
        let mut hsl: css_colors::HSL = self.into();
        hsl.s = css_colors::percent(saturation);
        Self::from_hsla(hsl.to_hsla(), self)
    }

    pub fn add_saturation(&self, saturation: u8) -> tera::Result<Self> {
        let hsl: css_colors::HSL = self.into();
        let hsl = hsl.saturate(css_colors::percent(saturation));
        Self::from_hsla(hsl.to_hsla(), self)
    }

    pub fn sub_saturation(&self, saturation: u8) -> tera::Result<Self> {
        let hsl: css_colors::HSL = self.into();
        let hsl = hsl.desaturate(css_colors::percent(saturation));
        Self::from_hsla(hsl.to_hsla(), self)
    }

    pub fn mod_lightness(&self, lightness: u8) -> tera::Result<Self> {
        let mut hsl: css_colors::HSL = self.into();
        hsl.l = css_colors::percent(lightness);
        Self::from_hsla(hsl.to_hsla(), self)
    }

    pub fn add_lightness(&self, lightness: u8) -> tera::Result<Self> {
        let hsl: css_colors::HSL = self.into();
        let hsl = hsl.lighten(css_colors::percent(lightness));
        Self::from_hsla(hsl.to_hsla(), self)
    }

    pub fn sub_lightness(&self, lightness: u8) -> tera::Result<Self> {
        let hsl: css_colors::HSL = self.into();
        let hsl = hsl.darken(css_colors::percent(lightness));
        Self::from_hsla(hsl.to_hsla(), self)
    }

    pub fn mod_opacity(&self, opacity: f32) -> tera::Result<Self> {
        let opacity = (opacity * 255.0).round() as u8;
        let (int24, uint32, sint32) = rgb_to_ints(&self.rgb, Some(opacity));
        Ok(Self {
            opacity,
            hex: rgb_to_hex(&self.rgb, opacity)?,
            int24,
            uint32,
            sint32,
            ..self.clone()
        })
    }

    pub fn add_opacity(&self, opacity: f32) -> tera::Result<Self> {
        let opacity = (opacity * 255.0).round() as u8;
        let opacity = self.opacity.saturating_add(opacity);
        let (int24, uint32, sint32) = rgb_to_ints(&self.rgb, Some(opacity));
        Ok(Self {
            opacity,
            hex: rgb_to_hex(&self.rgb, opacity)?,
            int24,
            uint32,
            sint32,
            ..self.clone()
        })
    }

    pub fn sub_opacity(&self, opacity: f32) -> tera::Result<Self> {
        let opacity = (opacity * 255.0).round() as u8;
        let opacity = self.opacity.saturating_sub(opacity);
        let (int24, uint32, sint32) = rgb_to_ints(&self.rgb, Some(opacity));
        Ok(Self {
            opacity,
            hex: rgb_to_hex(&self.rgb, opacity)?,
            int24,
            uint32,
            sint32,
            ..self.clone()
        })
    }
}

impl From<&Color> for css_colors::RGB {
    fn from(c: &Color) -> Self {
        Self {
            r: css_colors::Ratio::from_u8(c.rgb.r),
            g: css_colors::Ratio::from_u8(c.rgb.g),
            b: css_colors::Ratio::from_u8(c.rgb.b),
        }
    }
}

impl From<&Color> for css_colors::RGBA {
    fn from(c: &Color) -> Self {
        Self {
            r: css_colors::Ratio::from_u8(c.rgb.r),
            g: css_colors::Ratio::from_u8(c.rgb.g),
            b: css_colors::Ratio::from_u8(c.rgb.b),
            a: css_colors::Ratio::from_u8(c.opacity),
        }
    }
}

impl From<&Color> for css_colors::HSL {
    fn from(c: &Color) -> Self {
        Self {
            h: css_colors::Angle::new(c.hsl.h),
            s: css_colors::Ratio::from_f32(c.hsl.s),
            l: css_colors::Ratio::from_f32(c.hsl.l),
        }
    }
}

impl From<&Color> for css_colors::HSLA {
    fn from(c: &Color) -> Self {
        Self {
            h: css_colors::Angle::new(c.hsl.h),
            s: css_colors::Ratio::from_f32(c.hsl.s),
            l: css_colors::Ratio::from_f32(c.hsl.l),
            a: css_colors::Ratio::from_u8(c.opacity),
        }
    }
}

impl From<catppuccin::Rgb> for RGB {
    fn from(rgb: catppuccin::Rgb) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
        }
    }
}
