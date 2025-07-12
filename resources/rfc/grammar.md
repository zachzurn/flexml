# Flexml Spec

## Lexical elements:

* TEXT = `Any text separated by newlines`
* AN = `Alpha numeric`
* NAME = `alphanumeric characters, -, _`
* WS = `Any contiguous whitespace`
* PLUS = `+`
* COLON = `:`
* OB = `[`
* CB = `]`
* OS = `{`
* CS = `}`
* OT = `<`
* CT = `>`
* OR = `|=` can't be preceded with a backslash
* CR = `=|` can't be preceded with a backslash
* STYLE = `NAME|(COLON|NAME)?`
* STYLE_SEP = `WS?|PLUS|WS?`
* STYLE_LIST = `STYLE+|WS`

## Components:

We will use pipe (|) to separate components. Parenthesis are groups. ? means optional. + means can match multiple times in a row.

TEXT_CONTAINER: Any text, separated by newlines

STYLE_CONTAINER:
Format = `OS|NAME|WS|STYLE_LIST|CS`
Example = `{customStyle bold+italic+fontSize:3}`

RAW_CONTAINER:
Format = `RAW|ANY|RAW`
Example = `|= Anything goes in here and is \|= \=| not parsed. =|`

BOX_CONTAINER:
Format = `OB|STYLE_LIST|WS|(RAW_CONTAINER, BOX_CONTAINER, TAG_CONTAINER, STYLE_CONTAINER, TEXT_CONTAINER)+|WS|CB`
Example = `[bold+italic This is some text [ This is a box ] ]`

TAG_CONTAINER:
Format = `OT|NAME|CT`
Example = `<this_is_a-tag>`


Document Example:

```flexml
{redBox
    colorFg: red +
    colorBorder: red +
    pad: 3
}


[redBox This text will be in a red padded box ]

[redBox + fontSize:4 This text will be in a red padded box and it will be large ]

This is some text [ This is in a box ] and [ Some more ]

This is some text <aTag> and some more text and some |= Raw \|= text [this_is_not_a_box it's just text] \=| =|

[bold + italic
    A box with text in it
]

[small+ large + italic
    This is a box with text [ And another ]
]
```
