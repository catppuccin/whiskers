<h3 align="center">
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/logos/exports/1544x1544_circle.png" width="100" alt="Logo"/><br/>
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
  Catppuccin Whiskers
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
</h3>

<p align="center">
  <a href="https://github.com/catppuccin/whiskers/stargazers"><img src="https://img.shields.io/github/stars/catppuccin/whiskers?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
  <a href="https://github.com/catppuccin/whiskers/issues"><img src="https://img.shields.io/github/issues/catppuccin/whiskers?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
  <a href="https://github.com/catppuccin/whiskers/contributors"><img src="https://img.shields.io/github/contributors/catppuccin/whiskers?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

&nbsp;

A templating tool to simplify the creation of Catppuccin ports.

## Installation

You can install Whiskers using one of the methods below:

| Installation Method                   | Instructions                                                                                                        |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| crates.io                             | `cargo install catppuccin-whiskers`                                                                                 |
| Source                                | `cargo install --git https://github.com/catppuccin/whiskers catppuccin-whiskers`                                     |
| Homebrew                              | `brew install catppuccin/tap/whiskers`                                                                              |
| Nix                                   | `nix profile install github:catppuccin/whiskers`<br/>`nix run github:catppuccin/whiskers -- <args>` |
| Binaries<br/>(Windows, MacOS & Linux) | Available from the [latest GitHub release](https://github.com/catppuccin/whiskers/releases?q=whiskers).              |

## Usage

Whiskers is a port creation helper tool that is custom-built for Catppuccin,
allowing developers to define template files which the palette can be injected
into.

```console
$ whiskers --help
Soothing port creation tool for the high-spirited!

Usage: whiskers [OPTIONS] [TEMPLATE]

Arguments:
  [TEMPLATE]
          Path to the template file, or - for stdin

Options:
  -f, --flavor <FLAVOR>
          Render a single flavor instead of all four
          
          [possible values: latte, frappe, macchiato, mocha]

      --color-overrides <COLOR_OVERRIDES>
          Set color overrides

      --overrides <OVERRIDES>
          Set frontmatter overrides

      --check [<EXAMPLE_PATH>]
          Instead of creating an output, check it against an example
          
          In single-output mode, a path to the example file must be provided. In multi-output mode, no path is required and, if one is provided, it will be ignored.

      --dry-run
          Dry run, don't write anything to disk

      --list-functions
          List all Tera filters and functions

      --list-flavors
          List the Catppuccin flavors

      --list-accents
          List the Catppuccin accent colors

  -o, --output-format <OUTPUT_FORMAT>
          Output format of --list-functions
          
          [default: json]
          [possible values: json, yaml, markdown, markdown-table, plain]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Template

Please familiarize yourself with [Tera](https://keats.github.io/tera/),
which is the templating engine used in Whiskers.

### Naming Convention

Whiskers imposes no restrictions on template names. However, we recommend you use one the following options:

- `<port name>.tera` in the repo root for ports that only need one template.
  - For example the [lazygit](https://github.com/catppuccin/lazygit) port uses [`lazygit.tera`](https://github.com/catppuccin/lazygit/blob/main/lazygit.tera)
- `templates/<file name>.tera` especially for ports that have multiple templates.
  - For example, a port that generates files called `ui.cfg` and `palette.cfg` could use `templates/ui.tera` and `templates/palette.tera` respectively.

These conventions exist to make it easier for contributors to find templates and to give code editors a hint about the correct file type.

### Context Variables

The following variables are available for use in your templates:

#### Single-Flavor Mode

| Variable | Description |
| - | - |
| `flavor` ([`Flavor`](#flavor)) | The flavor being templated. |
| `rosewater`, `flamingo`, `pink`, [etc.](https://github.com/catppuccin/catppuccin#-palette) ([`Color`](#color)) | All colors of the flavor being templated. |
| Any Frontmatter | All frontmatter variables as described in the [Frontmatter](#Frontmatter) section. |

#### Multi-Flavor Mode

| Variable | Description |
| - | - |
| `flavors` (Map\<String, [`Flavor`](#flavor)>) | An array containing all of the named flavors, with every other context variable. |
| Any Frontmatter | All frontmatter variables as described in the [Frontmatter](#Frontmatter) section. |

#### Types

These types are designed to closely match the [palette.json](https://github.com/catppuccin/palette/blob/main/palette.json).

##### Flavor

| Field | Type | Description | Examples |
| - | - | - | - |
| `name` | `String` | The name of the flavor. | `"Latte"`, `"Frappé"`, `"Macchiato"`, `"Mocha"` |
| `identifier` | `String` | The identifier of the flavor. | `"latte"`, `"frappe"`, `"macchiato"`, `"mocha"` |
| `emoji` | `char` | Emoji associated with the flavor. | `'🌻'`, `'🪴'`, `'🌺'`, `'🌿'` |
| `order` | `u32` | Order of the flavor in the palette spec. | `0` to `3` |
| `dark` | `bool` | Whether the flavor is dark. | `false` for Latte, `true` for others |
| `light` | `bool` | Whether the flavor is light. | `true` for Latte, `false` for others |
| `colors` | `Map<String, Color>` | A map of color identifiers to their respective values. | |

##### Color

| Field | Type | Description | Examples |
| - | - | - | - |
| `name` | `String` | The name of the color. | `"Rosewater"`, `"Surface 0"`, `"Base"` |
| `identifier` | `String` | The identifier of the color. | `"rosewater"`, `"surface0"`, `"base"` |
| `order` | `u32` | Order of the color in the palette spec. | `0` to `25` |
| `accent` | `bool` | Whether the color is an accent color. | |
| `hex` | `String` | The color in hexadecimal format. | `"1e1e2e"` |
| `rgb` | `RGB` | The color in RGB format. | |
| `hsl` | `HSL` | The color in HSL format. | |
| `opacity` | `u8` | The opacity of the color. | `0` to `255` |

##### RGB

| Field | Type | Description |
| - | - | - |
| `r` | `u8` | The red channel of the color. |
| `g` | `u8` | The green channel of the color. |
| `b` | `u8` | The blue channel of the color. |

##### HSL

| Field | Type | Description |
| - | - | - |
| `h` | `u16` | The hue of the color. |
| `s` | `u8` | The saturation of the color. |
| `l` | `u8` | The lightness of the color. |

### Functions

| Name | Description | Examples |
|------|-------------|----------|
| `if` | Return one value if a condition is true, and another if it's false | `if(cond=true, t=1, f=0)` => `1` |
| `object` | Create an object from the input | `object(a=1, b=2)` => `{a: 1, b: 2}` |
| `css_rgb` | Convert a color to an RGB CSS string | `css_rgb(color=red)` => `rgb(210, 15, 57)` |
| `css_rgba` | Convert a color to an RGBA CSS string | `css_rgba(color=red)` => `rgba(210, 15, 57, 1.00)` |
| `css_hsl` | Convert a color to an HSL CSS string | `css_hsl(color=red)` => `hsl(347, 87%, 44%)` |
| `css_hsla` | Convert a color to an HSLA CSS string | `css_hsla(color=red)` => `hsla(347, 87%, 44%, 1.00)` |
| `read_file` | Read and include the contents of a file, path is relative to the template file | `read_file(path="abc.txt")` => `abc` |

### Filters

| Name | Description | Examples |
|------|-------------|----------|
| `add` | Add a value to a color | `red \| add(hue=30)` => `#ff6666` |
| `sub` | Subtract a value from a color | `red \| sub(hue=30)` => `#d30f9b` |
| `mod` | Modify a color | `red \| mod(lightness=80)` => `#f8a0b3` |
| `mix` | Mix two colors together | `red \| mix(color=base, amount=0.5)` => `#e08097` |
| `urlencode_lzma` | Serialize an object into a URL-safe string with LZMA compression | `red \| urlencode_lzma` => `#ff6666` |
| `trunc` | Truncate a number to a certain number of places | `1.123456 \| trunc(places=3)` => `1.123` |
| `css_rgb` | Convert a color to an RGB CSS string | `red \| css_rgb` => `rgb(210, 15, 57)` |
| `css_rgba` | Convert a color to an RGBA CSS string | `red \| css_rgba` => `rgba(210, 15, 57, 1.00)` |
| `css_hsl` | Convert a color to an HSL CSS string | `red \| css_hsl` => `hsl(347, 87%, 44%)` |
| `css_hsla` | Convert a color to an HSLA CSS string | `red \| css_hsla` => `hsla(347, 87%, 44%, 1.00)` |

> [!NOTE]
> You also have access to all of Tera's own built-in filters and functions.
> See [the Tera documentation](https://keats.github.io/tera/docs/#built-ins) for
> more information.

## Frontmatter

Whiskers templates may include a frontmatter section at the top of the file.

The frontmatter is a YAML block that contains metadata about the template. If
present, the frontmatter section must be the first thing in the file and must
take the form of valid YAML set between triple-dashed lines.

### Template Version

The most important frontmatter key is the Whiskers version. This key allows
Whiskers to ensure that it is rendering a template that it can understand.

Example:

```yaml
---
whiskers:
  version: "2.0.0"
---
... standard template content goes here ...
```

If the version key is not present, Whiskers will display a warning and attempt
to render the template anyway. However, it is recommended to always include the
version key to ensure compatibility with future versions of Whiskers.

### Frontmatter Variables

You can also include additional context variables in the templating process by
adding them to your template's frontmatter.

As a simple example, given the following template (`example.tera`):

```yaml
---
app: 'Pepperjack'
author: 'winston'
---
# Catppuccin for {{app}}
# by {{author}}
bg = '{{base.hex}}'
fg = '{{text.hex}}'
```

Running `whiskers example.tera -f mocha` produces the following output:

```yaml
# Catppuccin for Pepperjack
# by winston
bg = '1e1e2e'
fg = 'cdd6f4'
```

A common use of frontmatter is setting an accent color for the theme:

```
---
accent: "mauve"
---
{% set darkGreen = green | sub(lightness=30) %}
bg = "#{{base.hex}}"
fg = "#{{text.hex}}"
border = "#{{flavor.colors[accent].hex}}"
diffAddFg = "#{{green.hex}}"
diffAddBg = "#{{darkGreen.hex}}"
```

Rendering the above template produces the following output:

```ini
bg = "#1e1e2e"
fg = "#cdd6f4"
border = "#cba6f7"
diffaddfg = "#a6e3a1"
diffaddbg = "#40b436"
```

## Overrides

Frontmatter overrides can also be specified through the cli via the
`--overrides` flag, taking in a JSON string resembling the frontmatter. This is
particularly useful with build scripts to automatically generate files for each
accent:

`example.tera`

```yaml
---
accent: "mauve"
---
theme:
  accent: "{{flavor.colors[accent].hex}}"
```

When running `whiskers example.tera -f latte --overrides '{"accent": "pink"}'`,
the `accent` will be overridden to pink.

## Color Overrides

Color overrides can be specified through the cli via the `--color-overrides`
flag. This flag takes a JSON string like the following:

```json
{
  "all": {
    "text": "ff0000"
  },
  "mocha": {
    "base": "000000",
    "mantle": "010101",
    "crust": "020202",
  }
}
```

Passing these overrides would set the `text` color to bright red for all
flavors, and the `base`, `mantle`, and `crust` colors to black/near-black for
Mocha.

## Single-Flavor Mode

Running Whiskers with the `--flavor/-f` flag causes it to run in single-flavor mode.
This means the chosen flavor is placed into the template context as `flavor` and,
for convenience, all of its colors are also placed into the context as their respective
identifiers (`red`, `surface0`, et cetera.)

## Multi-Flavor Mode

Running Whiskers without the `--flavor/-f` flag causes it to run in multi-flavor mode.
In this mode, all flavors are placed into the template context as a map of flavor identifiers
to their respective [`Flavor`](#flavor) objects.

This map can be iterated like so:

```
{% for id, flavor in flavors %}
{{id}} is one of "latte", "frappe", "macchiato", or "mocha".
{{flavor}} is an object containing the flavor's properties and colors.
{% endfor %}
```

Please see the [examples/single-file](examples/single-file) directory for more
concrete examples on how it can be used.

## Template Matrix

Whiskers can render multiple outputs from a single template using a matrix set
in the frontmatter. This can be useful for generating one output per flavor per
accent color, for example.

In this mode Whiskers will render directly into a set of files as specified by
the `filename` key in the frontmatter. This can be disabled with the `--dry-run`
flag, in which case Whiskers will render the templates but not actually write
them anywhere.

The matrix is defined as a list of iterables. Whiskers will generate a file for
each combination of the iterables in the matrix.

Some of the iterables in the matrix can be strings without any values provided.
In this case, Whiskers will treat it as a "magic iterable", which is an iterable
that Whiskers can automatically generate values for before rendering the
template.

The following magic iterables are supported:

- `flavor`: latte, frappe, macchiato, mocha
- `accent`: rosewater, flamingo, pink, mauve, red, maroon, peach, yellow, green, teal, sky, sapphire, blue, lavender

Example:

```
---
whiskers:
  version: 2.0.0
  matrix:
    - variant: ["normal", "no-italics"]
    - flavor
    - accent
  filename: "catppuccin-{{flavor.identifier}}-{{accent}}-{{variant}}.ini"
---
# Catppuccin {{flavor.name}}{% if variant == "no-italics" %} (no italics){% endif %}
[theme]
{{accent}}: #{{flavor.colors[accent].hex}}
```

Running `whiskers template.tera` will generate the following files:

```
catppuccin-latte-rosewater-normal.ini
catppuccin-latte-rosewater-no-italics.ini
catppuccin-latte-flamingo-normal.ini
catppuccin-latte-flamingo-no-italics.ini
...
catppuccin-frappe-rosewater-normal.ini
catppuccin-frappe-rosewater-no-italics.ini
```

... and so on for every combination of flavor, accent, and variant. Notice that
the filenames are generated by rendering the `filename` key in the frontmatter
for each combination of the matrix iterables.

## Check Mode

You can use Whiskers as a linter with *check mode*. To do so, set the `--check`
option. Whiskers will render your template as per usual, but then instead of
printing the result it will check it against the expected output and fail with
exit code 1 if they differ.

This is especially useful in CI pipelines to ensure that the generated files are
not changed without a corresponding change to the templates.

In single-flavor mode, you must provide the path to the expected output file as
an argument to the `--check` option. In multi-flavor mode, the path is
unnecessary and will be ignored.

Whiskers will diff the output against the check file using the program set in
the `DIFFTOOL` environment variable, falling back to `diff` if it's not set. The
command will be invoked as `$DIFFTOOL <actual> <expected>`.

```console
$ whiskers theme.tera latte --check themes/latte.cfg
(no output, exit code 0)

$ whiskers theme.tera latte --check themes/latte.cfg
Templating would result in changes.
4c4
< accent is #ea76cb
---
> accent is #40a02b

(exit code 1)
```

## Editor Support

Tera's syntax is not natively supported by most editors. Some editors have
extensions available that provide syntax highlighting and other features for
Tera templates. In the case that your editor does not have a viable extension
available, you can try using a Jinja extension instead. While not an exact
match, Tera's syntax is similar enough to Jinja's that it can be used quite
well in most cases.

For Visual Studio Code users we recommend the [Better Jinja](https://marketplace.visualstudio.com/items?itemName=samuelcolvin.jinjahtml) extension.

## Further Reading

- See the [examples](examples) directory which further showcase the utilities
  and power of whiskers.
- See the RFC,
  [CAT-0003-Whiskers](https://github.com/catppuccin/community/blob/main/rfc/CAT-0003-Whiskers.md),
  to understand the motivation behind creating whiskers.

&nbsp;

<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2023-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
<p align="center"><a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=302d41&colorB=b7bdf8"/></a></p>
