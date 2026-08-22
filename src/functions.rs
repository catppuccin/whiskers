use std::{collections::BTreeMap, fs, path::PathBuf};

use serde::Deserialize;
use tera::{Kwargs, State};

use crate::models::Color;

pub fn if_fn(kwargs: Kwargs, _: &State) -> Result<tera::Value, tera::Error> {
    let cond = kwargs
        .get::<&tera::Value>("cond")?
        .ok_or_else(|| tera::Error::message("cond is required"))?
        .as_bool()
        .ok_or_else(|| tera::Error::message("cond must be a boolean"))?;
    let t = kwargs
        .get::<&tera::Value>("t")?
        .ok_or_else(|| tera::Error::message("t is required"))?
        .clone();
    let f = kwargs
        .get::<&tera::Value>("f")?
        .ok_or_else(|| tera::Error::message("f is required"))?
        .clone();

    Ok(if cond { t } else { f })
}

pub fn object(kwargs: Kwargs, _: &State) -> Result<tera::Value, tera::Error> {
    // sorting the args gives us stable output
    let args: BTreeMap<_, _> = kwargs.iter().collect();
    Ok(tera::value::Value::try_from_serializable(&args)?)
}

pub fn css_rgb(kwargs: Kwargs, _: &State) -> Result<tera::Value, tera::Error> {
    let color: Color = Deserialize::deserialize(
        kwargs
            .get::<&tera::Value>("color")?
            .ok_or_else(|| tera::Error::message("color is required"))?
            .clone(),
    )
    .map_err(|e| tera::Error::message(e.to_string()))?;

    let color: farver::RGB = (&color).into();
    Ok(tera::value::Value::try_from_serializable(
        &color.to_string(),
    )?)
}

pub fn css_rgba(kwargs: Kwargs, _: &State) -> Result<tera::Value, tera::Error> {
    let color: Color = Deserialize::deserialize(
        kwargs
            .get::<&tera::Value>("color")?
            .ok_or_else(|| tera::Error::message("color is required"))?
            .clone(),
    )
    .map_err(|e| tera::Error::message(e.to_string()))?;
    let color: farver::RGBA = (&color).into();
    Ok(tera::value::Value::try_from_serializable(
        &color.to_string(),
    )?)
}

pub fn css_hsl(kwargs: Kwargs, _: &State) -> Result<tera::Value, tera::Error> {
    let color: Color = Deserialize::deserialize(
        kwargs
            .get::<&tera::Value>("color")?
            .ok_or_else(|| tera::Error::message("color is required"))?
            .clone(),
    )
    .map_err(|e| tera::Error::message(e.to_string()))?;

    let color: farver::HSL = (&color).into();
    Ok(tera::value::Value::try_from_serializable(
        &color.to_string(),
    )?)
}

pub fn css_hsla(kwargs: Kwargs, _: &State) -> Result<tera::Value, tera::Error> {
    let color: Color = Deserialize::deserialize(
        kwargs
            .get::<&tera::Value>("color")?
            .ok_or_else(|| tera::Error::message("color is required"))?
            .clone(),
    )
    .map_err(|e| tera::Error::message(e.to_string()))?;
    let color: farver::HSLA = (&color).into();
    Ok(tera::value::Value::try_from_serializable(
        &color.to_string(),
    )?)
}

pub fn read_file_handler(
    template_directory: PathBuf,
) -> impl Fn(Kwargs, &tera::State) -> Result<tera::Value, tera::Error> {
    move |kwargs, _: &tera::State| -> Result<tera::Value, tera::Error> {
        let path: String = Deserialize::deserialize(
            kwargs
                .get::<&tera::Value>("path")?
                .ok_or_else(|| tera::Error::message("path is required"))?
                .clone(),
        )
        .map_err(|e| tera::Error::message(e.to_string()))?;
        let path = template_directory.join(path);
        let contents = fs::read_to_string(&path)
            .map_err(|_| format!("Failed to open file {}", path.display()))
            .map_err(tera::Error::message)?;
        Ok(tera::value::Value::try_from_serializable(&contents)?)
    }
}
