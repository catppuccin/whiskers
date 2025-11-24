# CSS Filter Test - Mocha

## Using css_filter as a function

Red filter:
```css
invert(71%) sepia(9%) saturate(2897%) hue-rotate(300deg) brightness(101%) contrast(91%)
```

Green filter:
```css
invert(84%) sepia(44%) saturate(277%) hue-rotate(61deg) brightness(93%) contrast(95%)
```

Blue filter:
```css
invert(60%) sepia(32%) saturate(494%) hue-rotate(179deg) brightness(106%) contrast(103%)
```

## Using css_filter as a pipe filter

Red:
```css
invert(71%) sepia(9%) saturate(2897%) hue-rotate(300deg) brightness(101%) contrast(91%)
```

Green:
```css
invert(84%) sepia(44%) saturate(277%) hue-rotate(61deg) brightness(93%) contrast(95%)
```

Blue:
```css
invert(60%) sepia(32%) saturate(494%) hue-rotate(179deg) brightness(106%) contrast(103%)
```

## Multiple colors to verify caching

Rosewater:
```css
invert(93%) sepia(10%) saturate(500%) hue-rotate(313deg) brightness(96%) contrast(99%)
```

Flamingo:
```css
invert(84%) sepia(11%) saturate(647%) hue-rotate(311deg) brightness(103%) contrast(90%)
```

Pink:
```css
invert(89%) sepia(9%) saturate(1467%) hue-rotate(279deg) brightness(100%) contrast(91%)
```

Mauve:
```css
invert(64%) sepia(20%) saturate(814%) hue-rotate(221deg) brightness(109%) contrast(94%)
```

## SVG Output

![CSS Filter Preview](css_filter.svg)
