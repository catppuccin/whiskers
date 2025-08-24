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

Whiskers is a port creation helper tool that is custom-built for Catppuccin,
allowing developers to define template files which the palette can be injected
into.

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

&nbsp;

<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2023-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
<p align="center"><a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=302d41&colorB=b7bdf8"/></a></p>
