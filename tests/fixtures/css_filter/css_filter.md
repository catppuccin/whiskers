# CSS Filter Test - Mocha

## Using css_filter as a function

Red filter:
```css
invert(55%) sepia(79%) saturate(233%) hue-rotate(292deg) brightness(101%) contrast(99%)
```

Green filter:
```css
invert(90%) sepia(86%) saturate(6874%) hue-rotate(83deg) brightness(101%) contrast(91%)
```

Blue filter:
```css
invert(68%) sepia(15%) saturate(1180%) hue-rotate(181deg) brightness(99%) contrast(99%)
```

## Using css_filter as a pipe filter

Red:
```css
invert(55%) sepia(79%) saturate(233%) hue-rotate(292deg) brightness(101%) contrast(99%)
```

Green:
```css
invert(90%) sepia(86%) saturate(6874%) hue-rotate(83deg) brightness(101%) contrast(91%)
```

Blue:
```css
invert(68%) sepia(15%) saturate(1180%) hue-rotate(181deg) brightness(99%) contrast(99%)
```

## Multiple colors to verify caching

Rosewater:
```css
invert(90%) sepia(0%) saturate(82%) hue-rotate(143deg) brightness(100%) contrast(100%)
```

Flamingo:
```css
invert(83%) sepia(6%) saturate(807%) hue-rotate(312deg) brightness(100%) contrast(100%)
```

Pink:
```css
invert(90%) sepia(79%) saturate(443%) hue-rotate(276deg) brightness(97%) contrast(97%)
```

Mauve:
```css
invert(86%) sepia(45%) saturate(3955%) hue-rotate(202deg) brightness(99%) contrast(98%)
```

## SVG Output

![CSS Filter Preview](css_filter.svg)
