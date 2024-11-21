<div align="center">
  <h2>🌱 Getting Started</h2>
  <p>Understand the fundamentals and basics on using Whiskers and creating Whiskers templates.</p>
</div>

Whiskers is a port creation helper tool that is custom-built for Catppuccin,
allowing developers to define template files which the palette can be injected
into. By the end of this tutorial, you will be able to create a Whiskers
template that generates a file containing some of Catppuccin's colors.

Let's get started!

### 1. Install Whiskers

Install whiskers by choosing one of the installation methods listed in our
[README](https://github.com/catppuccin/whiskers?tab=readme-ov-file#installation).

Confirm that you are able to invoke `whiskers` on the command line after
installation by running `whiskers --version`.

### 2. Invoke the Whiskers CLI

Whiskers is primarily used with one or multiple template files on disk, but the
terminals standard input can also be passed in. This is extremely useful in
situations where you'd like to confirm or experiment some output of whiskers but
do not want to create a temporary test file.

Let's get the hex code for `Catppuccin Latte Mauve`, run the following command:

```
whiskers --flavor latte -
```

The terminal will now be waiting for some input, type/paste in the following text:

```
Catppuccin Latte Mauve is: {{mauve.hex}}
```

Then give the terminal an EOF sequence. On Windows, this is usually `ctrl+z` and
for Linux/macOS, it's usually `ctrl+d`. You should now have the following output
in your terminal:

```shell
$ whiskers -f latte -
Catppuccin Latte Mauve is: {{mauve.hex}}
Catppuccin Latte Mauve is: 8839ef
```

You can see that the `{{mauve.hex}}` has been parsed by Whiskers and has been
converted into the hex code for Catppuccin Latte Mauve!

Whiskers uses the [Tera](https://keats.github.io/tera/) templating engine which
is why the `{{}}` braces are special. Understanding Tera is crucial to designing
and implementing Whiskers template files.

### 3. Create a template file

TODO

### Next Steps

TODO

### Further Reading

- See the RFC,
  [CAT-0003-Whiskers](https://github.com/catppuccin/community/blob/main/rfc/CAT-0003-Whiskers.md),
  to understand the motivation behind creating Whiskers.
