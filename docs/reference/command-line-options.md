<div align="center">
  <h2>⌨️ CLI Reference</h2>
  <p>View the options and arguments offered by the whiskers CLI.</p>
</div>

Run `whiskers --help`:

```shell
😾 Soothing port creation tool for the high-spirited!

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
          [possible values: json, yaml, markdown, plain]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Listed below are a few practical examples to demonstrate the usage of the CLI:

### Render a single flavor from a file

```shell
whiskers -f latte template.tera
```

### Render a single flavor from standard input

```shell
whiskers -f latte - <<< "This is mauve: #{{mauve.hex}}"
```

### List flavors and accents for iteration

The list of flavors and accents can be retrieved in a "plain" format separated
by newlines with the following commands:

```shell
whiskers --list-flavors -o plain
whiskers --list-accents -o plain
```

As a result, it is easy to iterate over the Catppuccin flavors and accents in,
for example, a bash script:

```shell
#!/usr/bin/env bash
whiskers --list-flavors -o plain | while read -r flavor; do
  whiskers --list-accents -o plain | while read -r accent; do
    echo "${flavor} - ${accent}"
  done
done
```
