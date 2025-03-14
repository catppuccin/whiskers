<p align="center">
  <h2 align="center">📖 The Whiskers Guide</h2>
</p>

<p align="center">
	Beginner-friendly guide for creating a Whiskers port!
</p>

&nbsp;

The top section of each Whiskers template, beginning and ending with `---`, is referred to as "frontmatter".
This section conveys important information to Whiskers through the `whiskers` key, though you can also add your own variables that will be added to the context.

Let's consider the following frontmatter:

```jinja
---
whiskers:
  version: "2.3.0"
---
{%- for _, flavor in flavors %}
{{ flavor.emoji}} {{ flavor.name }}
{%- endfor %}
```

This is as simple as a Whiskers template can be. The frontmatter defines `whiskers.version` and nothing more. When compiled, Whiskers prints the following to your console:

```
🌻 Latte
🪴 Frappé
🌺 Macchiato
🌿 Mocha
```

Perfect! But we want this in a file. You can either [redirect the output](https://www.gnu.org/software/bash/manual/html_node/Redirections.html) with `>` (e.g. `whiskers <filename> > output.md`), or preferably define the `whiskers.filename` property in the frontmatter:

```diff
---
whiskers:
  version: "2.3.0"
+ filename: "output.md"
---
{%- for _, flavor in flavors %}
[{{ flavor.emoji}} {{ flavor.name }}]
{%- endfor %}
```

> [!TIP]
> The `whiskers.filename` property also allows you to use Tera expressions within it, like a miniature template. This is helpful for more complex and dynamic output locations - see the below section on `whiskers.matrix`.

This instructs Whiskers to write the file for you, and helpfully it will also create any missing directories in the path.

In most cases, a single template will need to result in multiple files (one for each flavor, or occasionally one for each flavor and accent combination). This behavior is built-in to Whiskers through the `whiskers.matrix` property. The most basic usage of it is as follows:

```jinja
---
whiskers:
  version: "2.3.0"
  matrix:
    - flavor
  filename: "{{ flavor.identifier }}.md"
---
{{ flavor.emoji }} {{ flavor.name }}
```

This template instructs Whiskers to execute the template once for each flavor through `whiskers.matrix`, which involves injecting the `flavor` variable of the current flavor of the matrix (equivalent to `flavors[flavor]` under the hood) in each iteration. The `flavor` variable is used twice, once in `whiskers.filename` to dynamically generate a filename based on the name of the flavor -- executing this template will output the following files: `latte.md`, `frappe.md`, `macchiato.md`, and `mocha.md` -- and secondly to display the `emoji` and `name` properties of each flavor (see https://github.com/catppuccin/palette/blob/main/palette.json).

In addition to the mandatory `whiskers` object, you may define arbitrary keys in your frontmatter as an easier way of defining variables within the template's context. For example, consider the following template.

```jinja
---
accent: mauve
my_variable: xyz
whiskers:
  version: "2.3.0"
---
accent: {{ accent }} -> #{{ flavors.mocha.colors[accent].hex }}
my_variable: {{ my_variable }}
```

This outputs the following:

```
accent: mauve -> #cba6f7
my_variable: xyz
```

The `accent` key becomes the `accent` variable, which can be used with `{{ accent }}`, and the same with `my_variable` as `{{ my_variable }}`. A common use case is defining an accent color at the top of the template to use throughout. This is demonstrated in the above template, where we get the `mauve` key (from our `accent` variable) of the `flavors.mocha.colors` object to print the associated hex code.

All flavors are accessible in Whiskers templates through the `flavors` variable. If a template is using a flavor matrix, or the `--flavor` option, the `flavor` variable will contain the data for the current flavor, and additionally each color of the current flavor is available in the root context. This means you can use simply use a color by its identifier: red with `{{ red }}`, and blue with `{{ blue }}`.

Some ports simply need each color to be defined at the top, which can be acheieved by looping over the `flavor.colors` object.

```jinja
| Name | Identifier |
| ---- | ---------- |
{%- for _, color in flavor.colors %}
| {{ color.name }} | `{{ color.identifier }}` |
{%- endfor %}
```

## Filters

Tera has many [built-in filters](https://keats.github.io/tera/docs/#built-in-filters), though the color modification and representation filters
are provided by Whiskers itself, such as `css_rgb` and `css_hsl`.

Colors may need to be modified from their original form - often, this is in the form of adjusting the opacity.
The Whiskers-provided `mod` filter lets us do this; red is normally `{{ red | css_rgba }}`, though we can set the opacity to 50% with `{{ red | mod(opacity=0.5) | css_rgba }}`.
