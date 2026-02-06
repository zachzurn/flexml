# Flexml {🎨}

A lightweight, human-friendly document rendering language with flexible layout capabilities.

Flexml combines the simplicity of Markdown with the power of CSS, letting you create nicely formatted documents using a bracket based syntax.

Think of Flexml as the sweet spot between Markdown and LaTeX.

**Note**: Flexml is in early development. The syntax and features are subject to change.

## Features

- Easy to read and write, minimal boilerplate
- Combine and compose styles like building blocks
- Create reusable components with customizable parameters
- Full flexbox support for complex layouts

## Quick Start

### Basic Element

Elements are defined with square brackets and can contain text and nested elements:

```flexml
[This is a simple element]

[This is an element with [nested content]]
```

### Style Definitions

Define reusable styles with curly braces. 
Style definitions are always defined before any element is defined.
```flexml
{styledBox =
    bgColor: #FF0000
    color: #FFFFFF
    padding: 20px
    borderRadius: 10px
}

[styledBox
    This text will have a red background with white text
]
```

### Style Composition

Combine multiple styles using the `+` operator:
```flexml
{redBox = bgColor: #FF0000 + color: #FFFFFF}
{boldText = fontWeight: bold}
{largeText = fontSize: 24px}

[redBox + boldText + largeText
    This combines all three styles
]
```

### Style Forwarding (Parameters)

Create reusable components with customizable properties using the `>` forward operator:
```flexml
{colorBox =
    >color              // Forward parameter (no default)
    >bgColor: #F0F0F0   // Forward parameter with default
    padding: 20px
    borderRadius: 5px
}

[colorBox: #FF0000>#00FF00
    Red text on green background
]

[colorBox: #0000FF
    Blue text on default gray background
]
```

The syntax `[colorBox: #FF0000>#00FF00 ...]` passes values to forwarded parameters in order:
- First value (`#FF0000`) → `color`
- Second value (`#00FF00`) → `bgColor`

### Font Families

Load font families using file patterns:
```flexml
{Mona = fontFamily: "../assets/MonaSans*.otf"}
{Hubot = fontFamily: "../assets/HubotSans*.otf"}

[Mona This text uses the Mona Sans font family]
```

Flexml automatically discovers all font weights and styles matching the pattern.

## Complete Example

Here's a full example demonstrating various features.

The image below is a render from the current codebase.

![Example Render](resources/test/out/rendertest.png)
```flexml
{Mona = fontFamily: "../assets/MonaSans*.otf"}
{Hubot = fontFamily: "../assets/HubotSans*.otf"}

{flexml =
    Mona
    pageWidth: 5in
    pageHeight: 5in
    pixelsPerInch: 350
}

{lightBlueBox =
    >color
    >bgColor: #0000FF0A
    padding: 20px
    width: 250px
    borderRadius: 10px
}

{bold = fontWeight: bold + color: #9a50ba}
{italic = fontStyle: italic + color: #2fcc4e}
{inline = display: inline-block + fontSize: 1.1em + color: #68c2e3 + bgColor: #fce99a}
{box = display: block + bgColor: #fce99a + color: #000000 + padding: 5px + borderRadius: 5px + marginTop: 15px}

[lightBlueBox
    We have 😄 some [bold Bold Purple] text that should wrap onto a new line
    [inline this is inline block]
    with some inline content here [italic which is italic and green]
    with some more text followed by a
    [box + Hubot breaking block element that shows up on its own line]
]

[lightBlueBox: #000000>#00FF000A + Hubot
    [display: block 
        A variant of the light blue box, which should be green. 
        This uses a style forward >bgColor from the lightBlueBox definition.
    ]
    [display: block 
        The lightBlueBox: #00FF000A syntax forwards to the bgColor style value.
    ]
]
```

## Available Style Properties

### Layout
- `display` - `block`, `inline`, `inline-block`, `flex`
- `flexDirection` - `row`, `column`, `row-reverse`, `column-reverse`
- `justifyContent` - `flex-start`, `flex-end`, `center`, `space-between`, `space-around`
- `alignItems` - `flex-start`, `flex-end`, `center`, `stretch`, `baseline`
- `width`, `height` - Size values (px, %, em, in, cm, mm, pt)
- `padding`, `margin` - Spacing values
- `gap` - Flexbox gap

### Typography
- `fontFamily` - Font family path pattern
- `fontSize` - Font size
- `fontWeight` - `normal`, `bold`, `100`-`900`
- `fontStyle` - `normal`, `italic`, `oblique`
- `color` - Text color (hex, rgba)
- `lineHeight` - Line height
- `textAlign` - `left`, `right`, `center`, `justify`

### Visual
- `bgColor` - Background color (hex, rgba)
- `borderRadius` - Corner rounding
- `opacity` - Transparency (0.0 - 1.0)

### Page Setup (root only)
Height can be auto for single page docs, otherwise pages are created
- `pageWidth`, `pageHeight` - Page dimensions
- `pixelsPerInch` - Resolution (DPI)

## Project Status

This is an active work in progress. Current priorities:

- [x] Core syntax and parser
- [x] Style system and composition
- [x] Style forwarding/parameters
- [x] Flexbox layout engine
- [x] Text rendering and wrapping
- [ ] Font family discovery and loading
- [ ] Image embedding
- [ ] Real world test documents (Invoices, Receipts)
- [ ] Additional output formats (PDF, HTML)
- [ ] Standard library of common styles


## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Contributions are welcome! This project follows a "never crash" philosophy - the renderer should gracefully handle errors and provide helpful warnings rather than panicking.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.