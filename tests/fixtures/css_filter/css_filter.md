# CSS Filter Test - Mocha

## Using css_filter as a function

Red filter:
```css
invert(70%) sepia(70%) saturate(1022%) hue-rotate(296deg) brightness(105%) contrast(91%)
```

Green filter:
```css
invert(88%) sepia(25%) saturate(546%) hue-rotate(58deg) brightness(100%) contrast(79%)
```

Blue filter:
```css
invert(59%) sepia(73%) saturate(1023%) hue-rotate(194deg) brightness(112%) contrast(96%)
```

## Using css_filter as a pipe filter

Red:
```css
invert(70%) sepia(70%) saturate(1022%) hue-rotate(296deg) brightness(105%) contrast(91%)
```

Green:
```css
invert(88%) sepia(25%) saturate(546%) hue-rotate(58deg) brightness(100%) contrast(79%)
```

Blue:
```css
invert(59%) sepia(73%) saturate(1023%) hue-rotate(194deg) brightness(112%) contrast(96%)
```

## Multiple colors to verify caching

Rosewater:
```css
invert(83%) sepia(18%) saturate(512%) hue-rotate(314deg) brightness(113%) contrast(92%)
```

Flamingo:
```css
invert(80%) sepia(24%) saturate(510%) hue-rotate(311deg) brightness(110%) contrast(90%)
```

Pink:
```css
invert(82%) sepia(15%) saturate(535%) hue-rotate(268deg) brightness(100%) contrast(92%)
```

Mauve:
```css
invert(68%) sepia(27%) saturate(533%) hue-rotate(225deg) brightness(100%) contrast(93%)
```

## SVG Output

![CSS Filter Preview](css_filter.svg)
