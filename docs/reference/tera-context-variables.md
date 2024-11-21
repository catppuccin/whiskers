<div align="center">
  <h2>⌨️ Tera Context Variables</h2>
  <p>View the full list of functions and variables available in the Tera templates.</p>
</div>

The following functions and variables are available for use in your templates:

> [!IMPORTANT]
> You also have access to all of Tera's own built-in filters and functions.
> See [the Tera documentation](https://keats.github.io/tera/docs/#built-ins) for
> more information.

### Single-Flavor Mode

| Variable                                                                                                       | Description                                                                        |
| -------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| `flavor` ([`Flavor`](#flavor))                                                                                 | The flavor being templated.                                                        |
| `rosewater`, `flamingo`, `pink`, [etc.](https://github.com/catppuccin/catppuccin#-palette) ([`Color`](#color)) | All colors of the flavor being templated.                                          |
| Any Frontmatter                                                                                                | All frontmatter variables as described in the [Frontmatter](#Frontmatter) section. |

### Multi-Flavor Mode

| Variable                                      | Description                                                                        |
| --------------------------------------------- | ---------------------------------------------------------------------------------- |
| `flavors` (Map\<String, [`Flavor`](#flavor)>) | An array containing all of the named flavors, with every other context variable.   |
| Any Frontmatter                               | All frontmatter variables as described in the [Frontmatter](#Frontmatter) section. |

### Types

These types are designed to closely match the [palette.json](https://github.com/catppuccin/palette/blob/main/palette.json).

#### Flavor

| Field        | Type                 | Description                                            | Examples                                        |
| ------------ | -------------------- | ------------------------------------------------------ | ----------------------------------------------- |
| `name`       | `String`             | The name of the flavor.                                | `"Latte"`, `"Frappé"`, `"Macchiato"`, `"Mocha"` |
| `identifier` | `String`             | The identifier of the flavor.                          | `"latte"`, `"frappe"`, `"macchiato"`, `"mocha"` |
| `emoji`      | `char`               | Emoji associated with the flavor.                      | `'🌻'`, `'🪴'`, `'🌺'`, `'🌿'`                  |
| `order`      | `u32`                | Order of the flavor in the palette spec.               | `0` to `3`                                      |
| `dark`       | `bool`               | Whether the flavor is dark.                            | `false` for Latte, `true` for others            |
| `light`      | `bool`               | Whether the flavor is light.                           | `true` for Latte, `false` for others            |
| `colors`     | `Map<String, Color>` | A map of color identifiers to their respective values. |                                                 |

#### Color

| Field        | Type     | Description                                     | Examples                               |
| ------------ | -------- | ----------------------------------------------- | -------------------------------------- |
| `name`       | `String` | The name of the color.                          | `"Rosewater"`, `"Surface 0"`, `"Base"` |
| `identifier` | `String` | The identifier of the color.                    | `"rosewater"`, `"surface0"`, `"base"`  |
| `order`      | `u32`    | Order of the color in the palette spec.         | `0` to `25`                            |
| `accent`     | `bool`   | Whether the color is an accent color.           |                                        |
| `hex`        | `String` | The color in hexadecimal format.                | `"1e1e2e"`                             |
| `int24`      | `u32`    | Big-endian 24-bit color in RGB order.           | `1973806`                              |
| `uint32`     | `u32`    | Big-endian unsigned 32-bit color in ARGB order. | `4280163886`                           |
| `sint32`     | `i32`    | Big-endian signed 32-bit color in ARGB order.   | `-14803410`                            |
| `rgb`        | `RGB`    | The color in RGB format.                        |                                        |
| `hsl`        | `HSL`    | The color in HSL format.                        |                                        |
| `opacity`    | `u8`     | The opacity of the color.                       | `0` to `255`                           |

#### RGB

| Field | Type | Description                     |
| ----- | ---- | ------------------------------- |
| `r`   | `u8` | The red channel of the color.   |
| `g`   | `u8` | The green channel of the color. |
| `b`   | `u8` | The blue channel of the color.  |

#### HSL

| Field | Type  | Description                  |
| ----- | ----- | ---------------------------- |
| `h`   | `u16` | The hue of the color.        |
| `s`   | `u8`  | The saturation of the color. |
| `l`   | `u8`  | The lightness of the color.  |

### Functions

| Name        | Description                                                                    | Examples                                            |
| ----------- | ------------------------------------------------------------------------------ | --------------------------------------------------- |
| `if`        | Return one value if a condition is true, and another if it's false             | `if(cond=true, t=1, f=0)` ⇒ `1`                     |
| `object`    | Create an object from the input                                                | `object(a=1, b=2)` ⇒ `{a: 1, b: 2}`                 |
| `css_rgb`   | Convert a color to an RGB CSS string                                           | `css_rgb(color=red)` ⇒ `rgb(210, 15, 57)`           |
| `css_rgba`  | Convert a color to an RGBA CSS string                                          | `css_rgba(color=red)` ⇒ `rgba(210, 15, 57, 1.00)`   |
| `css_hsl`   | Convert a color to an HSL CSS string                                           | `css_hsl(color=red)` ⇒ `hsl(347, 87%, 44%)`         |
| `css_hsla`  | Convert a color to an HSLA CSS string                                          | `css_hsla(color=red)` ⇒ `hsla(347, 87%, 44%, 1.00)` |
| `read_file` | Read and include the contents of a file, path is relative to the template file | `read_file(path="abc.txt")` ⇒ `abc`                 |

### Filters

| Name             | Description                                                      | Examples                                         |
| ---------------- | ---------------------------------------------------------------- | ------------------------------------------------ |
| `add`            | Add a value to a color                                           | `red \| add(hue=30)` ⇒ `#ff6666`                 |
| `sub`            | Subtract a value from a color                                    | `red \| sub(hue=30)` ⇒ `#d30f9b`                 |
| `mod`            | Modify a color                                                   | `red \| mod(lightness=80)` ⇒ `#f8a0b3`           |
| `mix`            | Mix two colors together                                          | `red \| mix(color=base, amount=0.5)` ⇒ `#e08097` |
| `urlencode_lzma` | Serialize an object into a URL-safe string with LZMA compression | `red \| urlencode_lzma` ⇒ `#ff6666`              |
| `trunc`          | Truncate a number to a certain number of places                  | `1.123456 \| trunc(places=3)` ⇒ `1.123`          |
| `css_rgb`        | Convert a color to an RGB CSS string                             | `red \| css_rgb` ⇒ `rgb(210, 15, 57)`            |
| `css_rgba`       | Convert a color to an RGBA CSS string                            | `red \| css_rgba` ⇒ `rgba(210, 15, 57, 1.00)`    |
| `css_hsl`        | Convert a color to an HSL CSS string                             | `red \| css_hsl` ⇒ `hsl(347, 87%, 44%)`          |
| `css_hsla`       | Convert a color to an HSLA CSS string                            | `red \| css_hsla` ⇒ `hsla(347, 87%, 44%, 1.00)`  |
