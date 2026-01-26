# CSS Filter Test - Mocha

## Using css_filter as a function

Red filter:
```css
invert(70%) sepia(69%) saturate(1024%) hue-rotate(296deg) brightness(105%) contrast(91%)
```

Green filter:
```css
invert(89%) sepia(25%) saturate(545%) hue-rotate(57deg) brightness(100%) contrast(78%)
```

Blue filter:
```css
invert(59%) sepia(73%) saturate(1023%) hue-rotate(193deg) brightness(111%) contrast(96%)
```

## Using css_filter as a pipe filter

Red:
```css
invert(70%) sepia(69%) saturate(1024%) hue-rotate(296deg) brightness(105%) contrast(91%)
```

Green:
```css
invert(89%) sepia(25%) saturate(545%) hue-rotate(57deg) brightness(100%) contrast(78%)
```

Blue:
```css
invert(59%) sepia(73%) saturate(1023%) hue-rotate(193deg) brightness(111%) contrast(96%)
```

## Multiple colors to verify caching

Rosewater:
```css
invert(83%) sepia(17%) saturate(512%) hue-rotate(314deg) brightness(114%) contrast(92%)
```

Flamingo:
```css
invert(80%) sepia(25%) saturate(510%) hue-rotate(311deg) brightness(110%) contrast(90%)
```

Pink:
```css
invert(81%) sepia(16%) saturate(536%) hue-rotate(271deg) brightness(101%) contrast(92%)
```

Mauve:
```css
invert(63%) sepia(71%) saturate(1021%) hue-rotate(208deg) brightness(110%) contrast(94%)
```

## SVG Output

![CSS Filter Preview](css_filter.svg)
