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

    let color: farver::RGB = (&color).into();
    Ok(tera::to_value(color.to_string())?)
}

pub fn css_rgba(args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let color: Color = tera::from_value(
        args.get("color")
            .ok_or_else(|| tera::Error::msg("color is required"))?
            .clone(),
    )?;
    let color: farver::RGBA = (&color).into();
    Ok(tera::to_value(color.to_string())?)
}

pub fn css_hsl(args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let color: Color = tera::from_value(
        args.get("color")
            .ok_or_else(|| tera::Error::msg("color is required"))?
            .clone(),
    )?;

    let color: farver::HSL = (&color).into();
    Ok(tera::to_value(color.to_string())?)
}

pub fn css_hsla(args: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let color: Color = tera::from_value(
        args.get("color")
            .ok_or_else(|| tera::Error::msg("color is required"))?
            .clone(),
    )?;
    let color: farver::HSLA = (&color).into();
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
        let start_line: usize = args
            .get("start_line")
            .unwrap_or(&tera::to_value(1)?)
            .as_u64()
            .ok_or_else(|| tera::Error::msg("start_line must be a usize"))?
            .try_into()
            .map_err(|_| tera::Error::msg("start_line must be a usize"))?;
        let path = template_directory.join(path);
        let contents = fs::read_to_string(&path)
            .map_err(|_| format!("Failed to open file {}", path.display()))?;
        let mut content_lines: Vec<&str> = contents.lines().collect();
        if contents.ends_with('\n') {
            content_lines.push("");
        } else {
            return Err(tera::Error::msg("couldn't get file ending of file"));
        }
        let end_line: usize = args
            .get("end_line")
            .unwrap_or(&tera::to_value(content_lines.len())?)
            .as_u64()
            .ok_or_else(|| tera::Error::msg("end_line must be a usize"))?
            .try_into()
            .map_err(|_| tera::Error::msg("end_line must be a usize"))?;
        if start_line > end_line {
            return Err(tera::Error::msg("start_line is greater than end_line"));
        }
        if end_line > content_lines.len() {
            return Err(tera::Error::msg(
                "end_line is greater than the number of lines in the file",
            ));
        }
        let line_ending = if contents.ends_with("\r\n") {
            "\r\n"
        } else if contents.ends_with('\n') {
            "\n"
        } else {
            return Err(tera::Error::msg("couldn't get file ending of file"));
        };
        let mut lines = content_lines[start_line - 1..end_line].to_vec();
        lines
            .last_mut()
            .ok_or_else(|| tera::Error::msg("could not read last value"))?
            .to_string()
            .push_str(line_ending);
        Ok(tera::to_value(lines.join(line_ending))?)
    }
}
