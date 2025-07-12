# Flexml

A simple human friendly document rendering language with support for flexible layout.

# Warning

Project is in EXTREMELY early state, but code and RFC docs will be pushed to the main branch regularly.

# Flexml Spec

See the RFC here -> 

```
[redBox This box will be red ]

This is some text [ This is in a box ] and [ Some more ]

This is some text <aTag> and some more text and some |= Raw \|= text [this_is_not_a_box it's just text] \=| =|

[bold + italic
  A box with text in it
]

[small + 
 large +
 italic
  This is a box with text [ And another ] 
]
```