# Flexml

A simple human friendly document rendering language with support for flexible layout.


Some goals:
* Stream event based parsing where top level elements are emitted.
* Limited nesting depth (x levels)
* Width/Height based pages
* possibly Repeatable headers/footers

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