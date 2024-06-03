use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use clap::Parser;
use clap_stdin::FileOrStdin;

type ValueMap = HashMap<String, serde_json::Value>;

#[derive(Parser, Debug)]
#[command(version, about)]
#[allow(clippy::struct_excessive_bools)] // not a problem for cli flags
pub struct Args {
    /// Path to the template file, or - for stdin
    #[arg(required_unless_present_any = ["list_functions", "list_flavors", "list_accents"])]
    pub template: Option<FileOrStdin>,

    /// Render a single flavor instead of all four
    #[arg(long, short)]
    pub flavor: Option<Flavor>,

    /// Set color overrides
    #[arg(long, value_parser = json_map::<ColorOverrides>)]
    pub color_overrides: Option<ColorOverrides>,

    /// Set frontmatter overrides
    #[arg(long, value_parser = json_map::<ValueMap>)]
    pub overrides: Option<ValueMap>,

    /// Instead of creating an output, check it against an example
    ///
    /// In single-output mode, a path to the example file must be provided.
    /// In multi-output mode, no path is required and, if one is provided, it
    /// will be ignored.
    #[arg(long, value_name = "EXAMPLE_PATH")]
    pub check: Option<Option<PathBuf>>,

    /// Dry run, don't write anything to disk
    #[arg(long)]
    pub dry_run: bool,

    /// List all Tera filters and functions
    #[arg(long)]
    pub list_functions: bool,

    /// List the Catppuccin flavors
    #[arg(long)]
    pub list_flavors: bool,

    /// List the Catppuccin accent colors
    #[arg(long)]
    pub list_accents: bool,

    /// Output format of --list-functions
    #[arg(short, long, default_value = "json")]
    pub output_format: OutputFormat,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Invalid JSON literal argument: {message}")]
    InvalidJsonLiteralArg { message: String },

    #[error("Invalid JSON file argument: {message}")]
    InvalidJsonFileArg { message: String },

    #[error("Failed to read file: {path}")]
    ReadFile {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum Flavor {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl From<Flavor> for catppuccin::FlavorName {
    fn from(val: Flavor) -> Self {
        match val {
            Flavor::Latte => Self::Latte,
            Flavor::Frappe => Self::Frappe,
            Flavor::Macchiato => Self::Macchiato,
            Flavor::Mocha => Self::Mocha,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ColorOverrides {
    #[serde(default)]
    pub all: HashMap<String, String>,
    #[serde(default)]
    pub latte: HashMap<String, String>,
    #[serde(default)]
    pub frappe: HashMap<String, String>,
    #[serde(default)]
    pub macchiato: HashMap<String, String>,
    #[serde(default)]
    pub mocha: HashMap<String, String>,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
    Markdown,
    Plain,

    /// Deprecated, now equivalent to `Markdown`
    #[clap(hide = true)]
    MarkdownTable,
}

fn json_map<T>(s: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    if Path::new(s).is_file() {
        let s = std::fs::read_to_string(s).map_err(|e| Error::ReadFile {
            path: s.to_string(),
            source: e,
        })?;
        serde_json::from_str(&s).map_err(|e| Error::InvalidJsonFileArg {
            message: e.to_string(),
        })
    } else {
        serde_json::from_str(s).map_err(|e| Error::InvalidJsonLiteralArg {
            message: e.to_string(),
        })
    }
}
