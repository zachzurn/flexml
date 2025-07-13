# Flexml

A simple human friendly document rendering language with support for flexible layout.


Goals:
* Geared toward receipts, invoices and reports that typically use html or a headless browser to render
* Simple style definitions
* Every unit based on base font size (padding, spacing, line height etc)
* Simple styling that is defined at the start of the document (Reducing complexity and memory use)
* Cascading styles 
* Everything is a style and styles are simple names with possible single arguments (no padding: 10 10 10 10 type of thing)
* Support images (Bitmap and SVG)
* Human friendly markup that is easy to use for basic features but can have more advanced markup
* Pages (Width/Height/Density) provided outside of the markup (render multiple versions at different sizes)
* Simple tag elements like <CurrentPage> <TotalPages> <NewPage> and flex grow container <->
* Nested boxes with box model, flex, table and inline
* Repeatable headers and footers that can be changed i.e. after a <NewPage> tag
* Zero interactivity (With the exception of maybe links for certain formats)
* Image and Html rendering initially (Html rendering for easier early renders)
* Possibly add barcode and qr rendering in the future
  
# Warning

Project is in EXTREMELY early state, but code and RFC docs will be pushed to the main branch regularly.

# Flexml Spec

See the RFC in the wiki for more details

```
{ redBox = 
    colorText: red
    pad: 2
}

{ header = bold + sizeText: 2  }

[redBox This box will be red with some padding ]

This is some text [ This is in a box ] and [ Some more ]

[flex
    [ Some text on the left ] <-> [Some text on the right ]
]

[table
    [header [ a column ] [ a column ] ]
    [ [ a column ] [ a column ] ]
    [ [ a column ] [ a column ] ]
]

This is some raw |= text that can have [ literal text in it ] =|

[bold + italic
  A box with text in it
]

[small + 
 large +
 italic
  This is a box with text [ And another ] 
]
```
