use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::PathBuf,
};

use crate::models::Color;

pub fn if_fn(args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let cond = args
        .get("cond")
        .ok_or_else(|| tera::Error::msg("cond is required"))?
        .as_bool()
        .ok_or_else(|| tera::Error::msg("cond must be a boolean"))?;
    let t = args
        .get("t")
        .ok_or_else(|| tera::Error::msg("t is required"))?
        .clone();
    let f = args
        .get("f")
        .ok_or_else(|| tera::Error::msg("f is required"))?
        .clone();

    Ok(if cond { t } else { f })
}

pub fn object(args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    // sorting the args gives us stable output
    let args: BTreeMap<_, _> = args.iter().collect();
    Ok(tera::to_value(args)?)
}

pub fn css_rgb(args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let color: Color = tera::from_value(
        args.get("color")
            .ok_or_else(|| tera::Error::msg("color is required"))?
            .clone(),
    )?;

    let color: css_colors::RGB = (&color).into();
    Ok(tera::to_value(color.to_string())?)
}

pub fn css_rgba(args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let color: Color = tera::from_value(
        args.get("color")
            .ok_or_else(|| tera::Error::msg("color is required"))?
            .clone(),
    )?;
    let color: css_colors::RGBA = (&color).into();
    Ok(tera::to_value(color.to_string())?)
}

pub fn css_hsl(args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let color: Color = tera::from_value(
        args.get("color")
            .ok_or_else(|| tera::Error::msg("color is required"))?
            .clone(),
    )?;

    let color: css_colors::HSL = (&color).into();
    Ok(tera::to_value(color.to_string())?)
}

pub fn css_hsla(args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let color: Color = tera::from_value(
        args.get("color")
            .ok_or_else(|| tera::Error::msg("color is required"))?
            .clone(),
    )?;
    let color: css_colors::HSLA = (&color).into();
    Ok(tera::to_value(color.to_string())?)
}

pub fn read_file_handler(
    template_directory: PathBuf,
) -> impl Fn(&HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    move |args| -> Result<tera::Value, tera::Error> {
        let path: String = tera::from_value(
            args.get("path")
                .ok_or_else(|| tera::Error::msg("path is required"))?
                .clone(),
        )?;
        let path = template_directory.join(path);
        let contents = fs::read_to_string(&path)
            .map_err(|_| format!("Failed to open file {}", path.display()))?;
        Ok(tera::to_value(contents)?)
    }
}
