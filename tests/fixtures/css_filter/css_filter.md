# CSS Filter Test - Mocha

## Using css_filter as a function

Red filter:
```css
invert(76%) sepia(5%) saturate(5510%) hue-rotate(299deg) brightness(91%) contrast(110%)
```

Green filter:
```css
invert(91%) sepia(17%) saturate(720%) hue-rotate(57deg) brightness(90%) contrast(97%)
```

Blue filter:
```css
invert(76%) sepia(19%) saturate(4666%) hue-rotate(187deg) brightness(102%) contrast(96%)
```

## Using css_filter as a pipe filter

Red:
```css
invert(76%) sepia(5%) saturate(5510%) hue-rotate(299deg) brightness(91%) contrast(110%)
```

Green:
```css
invert(91%) sepia(17%) saturate(720%) hue-rotate(57deg) brightness(90%) contrast(97%)
```

Blue:
```css
invert(76%) sepia(19%) saturate(4666%) hue-rotate(187deg) brightness(102%) contrast(96%)
```

## Multiple colors to verify caching

Rosewater:
```css
invert(100%) sepia(97%) saturate(410%) hue-rotate(20deg) brightness(108%) contrast(108%)
```

Flamingo:
```css
invert(100%) sepia(83%) saturate(6887%) hue-rotate(304deg) brightness(92%) contrast(104%)
```

Pink:
```css
invert(95%) sepia(99%) saturate(4139%) hue-rotate(200deg) brightness(102%) contrast(92%)
```

Mauve:
```css
invert(73%) sepia(9%) saturate(1597%) hue-rotate(222deg) brightness(94%) contrast(106%)
```
