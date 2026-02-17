use std::{
    collections::{hash_map::Entry, HashMap},
    env,
    io::{Read, Write as _},
    path::{Path, PathBuf},
    process::{self, exit},
};

use anyhow::{anyhow, Context as _};
use catppuccin::FlavorName;
use clap::Parser as _;
use encoding_rs_io::DecodeReaderBytes;
use itertools::Itertools;
use whiskers::{
    cli::{Args, OutputFormat},
    context::merge_values,
    frontmatter, markdown,
    matrix::{self, Matrix},
    models::{self, HEX_FORMAT},
    templating,
};

const FRONTMATTER_OPTIONS_SECTION: &str = "whiskers";

fn default_hex_format() -> String {
    "{{r}}{{g}}{{b}}{{z}}".to_string()
}

#[derive(Default, Debug, serde::Deserialize)]
struct TemplateOptions {
    version: Option<(semver::VersionReq, String)>,
    matrix: Option<Matrix>,
    filename: Option<String>,
    hex_format: String,
    skip_if: Option<String>,
}

impl TemplateOptions {
    fn from_frontmatter(
        frontmatter: &HashMap<String, tera::Value>,
        only_flavor: Option<FlavorName>,
    ) -> anyhow::Result<Self> {
        // a `TemplateOptions` object before matrix transformation
        #[derive(serde::Deserialize)]
        struct RawTemplateOptions {
            version: Option<semver::VersionReq>,
            matrix: Option<Vec<tera::Value>>,
            filename: Option<String>,
            hex_format: Option<String>,
            hex_prefix: Option<String>,
            #[serde(default)]
            capitalize_hex: bool,
            skip_if: Option<String>,
        }

        if let Some(opts_section) = frontmatter.get(FRONTMATTER_OPTIONS_SECTION) {
            let raw_opts: RawTemplateOptions = tera::from_value(opts_section.clone())
                .context("Frontmatter `whiskers` section is invalid")?;

            let matrix = raw_opts
                .matrix
                .map(|m| matrix::from_values(m, only_flavor))
                .transpose()
                .context("Frontmatter matrix is invalid")?;

            // if there's no hex_format but there is hex_prefix and/or capitalize_hex,
            // we can construct a hex_format from those.
            let hex_format = if let Some(hex_format) = raw_opts.hex_format {
                hex_format
            } else {
                // throw a deprecation warning for hex_prefix and capitalize_hex
                if raw_opts.hex_prefix.is_some() {
                    eprintln!("warning: `hex_prefix` is deprecated and will be removed in a future version. Use `hex_format` instead.");
                }

                if raw_opts.capitalize_hex {
                    eprintln!("warning: `capitalize_hex` is deprecated and will be removed in a future version. Use `hex_format` instead.");
                }

                let prefix = raw_opts.hex_prefix.unwrap_or_default();
                let components = default_hex_format();
                if raw_opts.capitalize_hex {
                    format!("{prefix}{}", components.to_uppercase())
                } else {
                    format!("{prefix}{components}")
                }
            };

            Ok(Self {
                version: raw_opts.version.map(|version| {
                    (
                        version,
                        opts_section["version"].as_str().map(String::from).expect(
                            "version string is guaranteed to be Some if `raw_opts.version` is Some",
                        ),
                    )
                }),
                matrix,
                filename: raw_opts.filename,
                hex_format,
                skip_if: raw_opts.skip_if,
            })
        } else {
            Ok(Self {
                hex_format: default_hex_format(),
                ..Default::default()
            })
        }
    }
}

fn main() -> anyhow::Result<()> {
    // parse command-line arguments & template frontmatter
    let args = Args::parse();
    handle_list_flags(&args)?;

    let template_arg = args
        .template
        .clone()
        .expect("args.template is guaranteed by clap to be set");
    let template_from_stdin = template_arg.is_stdin();
    let template_name = template_name(&template_arg);
    let template_directory =
        template_directory(&template_arg).context("Template file does not exist")?;

    let mut decoder = DecodeReaderBytes::new(
        template_arg
            .into_reader()
            .context("Failed to open template file")?,
    );
    let mut template = String::new();
    decoder
        .read_to_string(&mut template)
        .context("Template could not be read")?;

    let doc = frontmatter::parse(&template).context("Frontmatter is invalid")?;
    let mut template_opts =
        TemplateOptions::from_frontmatter(&doc.frontmatter, args.flavor.map(Into::into))
            .context("Could not get template options from frontmatter")?;

    if !template_from_stdin && !template_is_compatible(&template_opts) {
        std::process::exit(1);
    }

    // merge frontmatter with command-line overrides and add to Tera context
    let mut frontmatter = doc.frontmatter;
    if let Some(ref overrides) = args.overrides {
        for (key, value) in overrides {
            frontmatter
                .entry(key.clone())
                .and_modify(|v| {
                    *v = merge_values(v, value);
                })
                .or_insert(
                    tera::to_value(value)
                        .with_context(|| format!("Value of {key} override is invalid"))?,
                );

            // overrides also work on matrix iterables
            if let Some(ref mut matrix) = template_opts.matrix {
                override_matrix(matrix, value, key)?;
            }
        }
    }
    let mut ctx = tera::Context::new();
    for (key, value) in &frontmatter {
        ctx.insert(key, &value);
    }

    HEX_FORMAT
        .set(template_opts.hex_format)
        .expect("can always set HEX_FORMAT");

    // build the palette and add it to the templating context
    let palette = models::build_palette(args.color_overrides.as_ref())
        .context("Palette context cannot be built")?;

    ctx.insert("flavors", &palette.flavors);
    if let Some(flavor) = args.flavor {
        let flavor: catppuccin::FlavorName = flavor.into();
        let flavor = &palette.flavors[flavor.identifier()];
        ctx.insert("flavor", flavor);

        // also throw in the flavor's colors for convenience
        for (_, color) in flavor {
            ctx.insert(&color.identifier, &color);
        }
    }

    // build the Tera engine
    let mut tera = templating::make_engine(&template_directory);
    tera.add_raw_template(&template_name, &doc.body)
        .context("Template is invalid")?;

    if let Some(matrix) = template_opts.matrix {
        let Some(filename_template) = template_opts.filename else {
            anyhow::bail!("Filename template is required for multi-output render");
        };

        render_multi_output(
            matrix,
            &filename_template,
            template_opts.skip_if.as_deref(),
            &ctx,
            &palette,
            &tera,
            &template_name,
            &args,
        )
        .context("Multi-output render failed")?;
    } else {
        let check = args
            .check
            .map(|c| {
                c.ok_or_else(|| anyhow!("--check requires a file argument in single-output mode"))
            })
            .transpose()?;

        render_single_output(
            &ctx,
            &tera,
            &template_name,
            check,
            template_opts.filename,
            template_opts.skip_if.as_deref(),
            args.dry_run,
        )
        .context("Single-output render failed")?;
    }

    Ok(())
}

fn handle_list_flags(args: &Args) -> anyhow::Result<()> {
    if args.list_functions {
        list_functions(args.output_format)?;
        exit(0);
    }

    if args.list_flavors {
        list_flavors(args.output_format)?;
        exit(0);
    }

    if args.list_accents {
        list_accents(args.output_format);
        exit(0);
    }

    Ok(())
}

fn override_matrix(
    matrix: &mut Matrix,
    value: &tera::Value,
    key: &str,
) -> Result<(), anyhow::Error> {
    let Entry::Occupied(e) = matrix.entry(key.to_string()) else {
        return Ok(());
    };

    // if the override is a list, we can just replace the iterable.
    if let Some(value_list) = value.as_array() {
        let value_list = value_list
            .iter()
            .map(|v| v.as_str().map(ToString::to_string))
            .collect::<Option<Vec<_>>>()
            .context("Override value is not a list of strings")?;
        *e.into_mut() = value_list;
    }
    // if the override is a string, we instead replace the iterable with a
    // single-element list containing the string.
    else if let Some(value_string) = value.as_str() {
        *e.into_mut() = vec![value_string.to_string()];
    }

    Ok(())
}

fn list_functions(format: OutputFormat) -> anyhow::Result<()> {
    let functions = templating::all_functions();
    let filters = templating::all_filters();
    println!(
        "{}",
        match format {
            OutputFormat::Json | OutputFormat::Yaml => {
                let output = serde_json::json!({
                    "functions": functions,
                    "filters": filters,
                });

                if matches!(format, OutputFormat::Json) {
                    serde_json::to_string_pretty(&output).expect("output is guaranteed to be valid")
                } else {
                    serde_yaml::to_string(&output).expect("output is guaranteed to be valid")
                }
            }
            OutputFormat::Markdown | OutputFormat::MarkdownTable => {
                format!(
                    "{}\n\n{}",
                    markdown::display_as_table(&functions, "Functions")?,
                    markdown::display_as_table(&filters, "Filters")?
                )
            }
            OutputFormat::Plain => {
                let mut list = filters
                    .iter()
                    .map(|f| f.name.clone())
                    .collect::<Vec<String>>();

                list.extend(functions.iter().map(|f| f.name.clone()));

                list.join("\n")
            }
        }
    );
    Ok(())
}

fn list_flavors(format: OutputFormat) -> anyhow::Result<()> {
    // we want all the flavor info minus the colors
    #[derive(serde::Serialize)]
    struct FlavorInfo {
        identifier: String,
        name: String,
        emoji: char,
        order: u32,
        dark: bool,
    }

    impl markdown::TableDisplay for FlavorInfo {
        fn table_headings() -> Box<[String]> {
            vec![
                "Identifier".to_string(),
                "Name".to_string(),
                "Dark".to_string(),
                "Emoji".to_string(),
            ]
            .into_boxed_slice()
        }

        fn table_row(&self) -> Box<[String]> {
            vec![
                self.identifier.clone(),
                self.name.clone(),
                self.dark.to_string(),
                self.emoji.to_string(),
            ]
            .into_boxed_slice()
        }
    }

    let flavors = catppuccin::PALETTE
        .all_flavors()
        .into_iter()
        .map(|f| FlavorInfo {
            identifier: f.identifier().to_string(),
            name: f.name.to_string(),
            emoji: f.emoji,
            order: f.order,
            dark: f.dark,
        })
        .collect::<Vec<_>>();

    println!(
        "{}",
        match format {
            // for structured data, we output the full flavor info objects
            OutputFormat::Json | OutputFormat::Yaml => {
                if matches!(format, OutputFormat::Json) {
                    serde_json::to_string_pretty(&flavors)
                        .expect("flavors are guaranteed to be valid json")
                } else {
                    serde_yaml::to_string(&flavors)
                        .expect("flavors are guaranteed to be valid yaml")
                }
            }
            // for plain output, we just list the flavor identifiers
            OutputFormat::Plain => {
                flavors.iter().map(|f| &f.identifier).join("\n")
            }
            // and finally for human-readable markdown, we list the flavor names
            OutputFormat::Markdown | OutputFormat::MarkdownTable => {
                markdown::display_as_table(&flavors, "Flavors")?
            }
        }
    );

    Ok(())
}

fn list_accents(format: OutputFormat) {
    let accents = catppuccin::PALETTE
        .latte
        .colors
        .all_colors()
        .into_iter()
        .filter(|c| c.accent)
        .collect::<Vec<_>>();

    println!(
        "{}",
        match format {
            // for structured data, we can include both name and identifier of each color
            OutputFormat::Json | OutputFormat::Yaml => {
                let accents = accents
                    .into_iter()
                    .map(|c| {
                        serde_json::json!({
                            "name": c.name,
                            "identifier": c.identifier(),
                        })
                    })
                    .collect::<Vec<_>>();
                if matches!(format, OutputFormat::Json) {
                    serde_json::to_string_pretty(&accents)
                        .expect("accents are guaranteed to be valid json")
                } else {
                    serde_yaml::to_string(&accents)
                        .expect("accents are guaranteed to be valid yaml")
                }
            }
            // for plain output, we just list the identifiers
            OutputFormat::Plain => {
                accents
                    .into_iter()
                    .map(catppuccin::Color::identifier)
                    .join("\n")
            }
            // and finally for human-readable markdown, we list the names
            OutputFormat::Markdown | OutputFormat::MarkdownTable => {
                markdown::display_as_list(
                    &accents.into_iter().map(|c| c.name).collect::<Vec<_>>(),
                    "Accents",
                )
            }
        }
    );
}

fn template_name(template: &clap_stdin::FileOrStdin) -> String {
    if template.is_stdin() {
        "template".to_string()
    } else {
        Path::new(template.filename()).file_name().map_or_else(
            || "template".to_string(),
            |name| name.to_string_lossy().to_string(),
        )
    }
}

fn template_directory(template: &clap_stdin::FileOrStdin) -> anyhow::Result<PathBuf> {
    if template.is_stdin() {
        Ok(std::env::current_dir()?)
    } else {
        Ok(Path::new(template.filename())
            .canonicalize()?
            .parent()
            .expect("file path must have a parent")
            .to_owned())
    }
}

fn template_is_compatible(template_opts: &TemplateOptions) -> bool {
    let whiskers_version = semver::Version::parse(env!("CARGO_PKG_VERSION"))
        .expect("CARGO_PKG_VERSION is always valid");
    if let Some((template_version, template_version_raw)) = &template_opts.version {
        // warn if the template is using an implicit constraint instead of an explicit one
        // i.e. `version: "2.5.1"` instead of `version: "^2.5.1"`
        if let &[comp] = &template_version.comparators.as_slice() {
            if comp.op == semver::Op::Caret && !template_version_raw.starts_with('^') {
                eprintln!("warning: Template specifies an implicit constraint of {template_version_raw}, consider explicitly specifying ^{template_version_raw} instead");
            }
        }

        if !template_version.matches(&whiskers_version) {
            eprintln!(
                "error: This template requires a version of Whiskers compatible with \
                \"{template_version}\", but you are running Whiskers \
                {whiskers_version} which is not compatible with this \
                requirement."
            );
            return false;
        }
    } else {
        eprintln!("warning: No Whiskers version requirement specified in template.");
        eprintln!("This template may not be compatible with this version of Whiskers.");
        eprintln!();
        eprintln!("To fix this, specify a Whiskers version requirement in the template frontmatter as follows:");
        eprintln!();
        eprintln!("---");
        eprintln!("whiskers:");
        eprintln!("    version: \"^{whiskers_version}\"");
        eprintln!("---");
        eprintln!();
    }

    true
}

fn write_template(dry_run: bool, filename: &str, result: String) -> Result<(), anyhow::Error> {
    let filename = Path::new(&filename);

    if dry_run || cfg!(test) {
        println!(
            "Would write {} bytes into {}",
            result.len(),
            filename.display()
        );
    } else {
        maybe_create_parents(filename)?;
        std::fs::write(filename, result)
            .with_context(|| format!("Couldn't write to {}", filename.display()))?;
    }

    Ok(())
}

fn render_single_output(
    ctx: &tera::Context,
    tera: &tera::Tera,
    template_name: &str,
    check: Option<PathBuf>,
    filename: Option<String>,
    skip_if: Option<&str>,
    dry_run: bool,
) -> Result<(), anyhow::Error> {
    if should_skip(skip_if, ctx)? {
        return Ok(());
    }

    let result = tera
        .render(template_name, ctx)
        .context("Template render failed")?;

    if let Some(path) = check {
        if matches!(
            check_result_with_file(&path, &result).context("Check mode failed")?,
            CheckResult::Fail
        ) {
            std::process::exit(1);
        }
    } else if let Some(filename) = filename {
        write_template(dry_run, &filename, result)?;
    } else {
        print!("{result}");
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn render_multi_output(
    matrix: HashMap<String, Vec<String>>,
    filename_template: &str,
    skip_if: Option<&str>,
    ctx: &tera::Context,
    palette: &models::Palette,
    tera: &tera::Tera,
    template_name: &str,
    args: &Args,
) -> Result<(), anyhow::Error> {
    let iterables = matrix
        .into_iter()
        .map(|(key, iterable)| iterable.into_iter().map(move |v| (key.clone(), v)))
        .multi_cartesian_product()
        .collect::<Vec<_>>();
    let mut check_results: Vec<CheckResult> = Vec::with_capacity(iterables.len());

    for iterable in iterables {
        let mut ctx = ctx.clone();
        for (key, value) in iterable {
            // expand flavor automatically to prevent requiring:
            // `{% set flavor = flavors[flavor] %}`
            // at the top of every template.
            if key == "flavor" {
                let flavor: catppuccin::FlavorName = value.parse()?;
                let flavor = &palette.flavors[flavor.identifier()];
                ctx.insert("flavor", flavor);

                // also throw in the flavor's colors for convenience
                for (_, color) in flavor {
                    ctx.insert(&color.identifier, &color);
                }
            } else {
                ctx.insert(key, &value);
            }
        }

        if should_skip(skip_if, &ctx)? {
            continue;
        }

        let result = tera
            .render(template_name, &ctx)
            .context("Main template render failed")?;
        let filename = tera::Tera::one_off(filename_template, &ctx, false)
            .context("Filename template render failed")?;

        if args.check.is_some() {
            check_results
                .push(check_result_with_file(&filename, &result).context("Check mode failed")?);
        } else {
            write_template(args.dry_run, &filename, result)?;
        }
    }

    if check_results.iter().any(|r| matches!(r, CheckResult::Fail)) {
        std::process::exit(1);
    }

    Ok(())
}

fn should_skip(skip_if: Option<&str>, ctx: &tera::Context) -> Result<bool, anyhow::Error> {
    Ok(skip_if
        .map(|cond| tera::Tera::one_off(cond, ctx, false))
        .transpose()?
        .is_some_and(|s| s.trim().to_lowercase() == "true"))
}

fn maybe_create_parents(filename: &Path) -> anyhow::Result<()> {
    if let Some(parent) = filename.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!(
                "Couldn't create parent directories for {}",
                filename.display()
            )
        })?;
    }
    Ok(())
}

#[must_use]
enum CheckResult {
    Pass,
    Fail,
}

fn check_result_with_file<P>(path: &P, result: &str) -> anyhow::Result<CheckResult>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let expected = std::fs::read_to_string(path).with_context(|| {
        format!(
            "error: Couldn't read {} for comparison against result",
            path.display()
        )
    })?;
    if *result == expected {
        Ok(CheckResult::Pass)
    } else {
        eprintln!("error: Output does not match {}", path.display());
        invoke_difftool(result, path)?;
        Ok(CheckResult::Fail)
    }
}

fn invoke_difftool<P>(actual: &str, expected_path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let expected_path = expected_path.as_ref();
    let tool = env::var("DIFFTOOL").unwrap_or_else(|_| "diff".to_string());

    let mut actual_file = tempfile::NamedTempFile::new()?;
    write!(&mut actual_file, "{actual}")?;
    if let Ok(mut child) = process::Command::new(tool)
        .args([actual_file.path(), expected_path])
        .spawn()
    {
        child.wait()?;
    } else {
        eprintln!("warning: Can't display diff, try setting $DIFFTOOL.");
    }

    Ok(())
}
