# Flexbox + css based document rendering (FLEX:ML)

Project is currently a toy project.

## Goals

Make a fast and low memory renderer for invoices, reports, receipt type documents as well as any design type templates. No interactivity is planned.

The current option is to use html for something it is not designed for.

* Somewhat low memory use / somewhat fast
* Ability to create very large documents
* Minimal markup
* Simple markup


## Tasks

- [x] Parser
- [ ] Rendering Abstraction
- [ ] Image Renderer
- [ ] PDF Renderer

## Markup

Everything is based on elements with classes. Elements must start with a list of classnames separated by periods and then an open curly brace.
Elements can also use block syntax which stars with 3 uppercase characters and followed by a colon.

```
class.anotherclass{

    Some text

    RAW:class.anotherclass{
    
        A RAW block that can have special behavior.
        This block preserves whitespace and ignores any element markup.
        
    }
    
}
```

## Css

Css can be defined using block syntax (currently called macro in code and will be renamed).

```
CSS:{
    .red {
        color: red;
    }
}
```

## Base Styles

Base styles will be defined for common html parallels.

hflex, vflex, box, h1, h2, h3, h4, h5, b, i, u, table, row, cell, span
