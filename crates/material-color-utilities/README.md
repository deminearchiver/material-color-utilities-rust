# Material Color Utilities

Algorithms and utilities that power the Material Design 3 (M3) color system,
including choosing theme colors from images and creating tones of colors;
all in a new color space.

See the shared repository
[deminearchiver/material-color-utilities](https://github.com/deminearchiver/material-color-utilities)
for additional information.

## Design

Color is a powerful design tool and part of the Material system along with styles like typography and shape.
In products, colors and the way they are used can be vast and varied.
An app’s color scheme can express brand and style.
Semantic colors can communicate meaning.
And color contrast control supports visual accessibility.

In many design systems of the past,
designers manually picked app colors to support the necessary range of color applications and use cases.
Material 3 introduces a dynamic color system,
which does not rely on hand-picked colors.
Instead, it uses color algorithms to generate beautiful,
accessible color schemes based on dynamic inputs like a user’s wallpaper.
This enables greater flexibility, personalization, and expression,
all while streamlining work for designers and teams.

Material Color Ultilities (MCU) powers dynamic color
with a set of color libraries containing algorithms and utilities
that make it easier for you to develop color themes and schemes in your app.


## Features

The library consists of various components,
each having its own folder and tests,
designed to be as self-contained as possible.
This enables seamless integration of subsets into other libraries,
like Material Design Components and Android System UI.
Some consumers do not require all components,
for example, MDC doesn’t need quantization, scoring, image extraction.

## Learn about color science
[The Science of Color & Design - Material Design](https://m3.material.io/blog/science-of-color-design)

## Usage

### Installation

```toml
[dependencies]
material-color-utilities = "1.0"
```

<!-- ### Generating a color scheme 

```rust
use material_color_utilities::dynamiccolor::DynamicScheme;

fn main() {
  let scheme = DynamicScheme::from();
}
``` -->


### Serde compatibility

```toml
[dependencies]
material-color-utilities = { version = "1.0", features = ["serde"] }
serde = "1"
serde_json = "1"
```


