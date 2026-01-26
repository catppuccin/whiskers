# CSS Filter Test - Mocha

## Using css_filter as a function

Red filter:
```css
invert(61%) sepia(33%) saturate(540%) hue-rotate(293deg) brightness(101%) contrast(90%)
```

Green filter:
```css
invert(86%) sepia(22%) saturate(538%) hue-rotate(63deg) brightness(97%) contrast(88%)
```

Blue filter:
```css
invert(57%) sepia(66%) saturate(1020%) hue-rotate(195deg) brightness(115%) contrast(96%)
```

## Using css_filter as a pipe filter

Red:
```css
invert(61%) sepia(33%) saturate(540%) hue-rotate(293deg) brightness(101%) contrast(90%)
```

Green:
```css
invert(86%) sepia(22%) saturate(538%) hue-rotate(63deg) brightness(97%) contrast(88%)
```

Blue:
```css
invert(57%) sepia(66%) saturate(1020%) hue-rotate(195deg) brightness(115%) contrast(96%)
```

## Multiple colors to verify caching

Rosewater:
```css
invert(84%) sepia(22%) saturate(508%) hue-rotate(312deg) brightness(113%) contrast(91%)
```

Flamingo:
```css
invert(79%) sepia(23%) saturate(510%) hue-rotate(311deg) brightness(110%) contrast(90%)
```

Pink:
```css
invert(76%) sepia(28%) saturate(508%) hue-rotate(290deg) brightness(112%) contrast(92%)
```

Mauve:
```css
invert(63%) sepia(71%) saturate(1021%) hue-rotate(208deg) brightness(110%) contrast(94%)
```

## SVG Output

![CSS Filter Preview](css_filter.svg)
