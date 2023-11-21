# WIP * RFC
### flexml

Project is currently a toy project. Do not attempt to use anything in this repo.

# Goals

Make a fast and low memory renderer for invoice, simple reports, receipt type documents. No interactivity

# Markup

Simple block format with minimal markup.

Elements can start with and @ symbol and then a-z A-Z 1-9 characters with dashes and periods.

Elements that start with @ are macro elements that can set context, style or generate more elements.

Container with flex and test class. Child container with box and test class. Text Content node inside.

```
flex.test {
    box.test {
        Text content
    }
}
```

Config macro json. Define page width and height in inches (0 means infinite).
Pixels per inch. Pages can be infinitely wide or tall to allow for paging.


```
@cnf{
    "ppi" : 96,
    "page_width" : 8.5,
    "page_height" : 0,
}
```

Css macro with blue and bold class.

```
@css{
    .main {
        font-size: 30px;
    }
}
```

Markdown macro with class main. Markdown macro uses first line as base spaces.

```
@mkd.main {
    ## Try CommonMark
    
    You can try CommonMark here.  This dingus is powered by
    [commonmark.js](https://github.com/commonmark/commonmark.js), the
    JavaScript reference implementation.

    1. item one
    2. item two
    - sublist
    - sublist
}
```

Table macro
```
@tbl {
    "header": ["Type","Name"]
    "header_width": ["20%","20"],
    "rows": [
        ["Dog",Kobe"],
        ["Dog",Kobe"],
    ]
}
```

Csv macro, behaves like table macro
```
@csv {
    20%,20
    type,name
    dog,kobe
}
```

Header and footer macros are pulled out of context and repeated on every page
```
@top {}
@bot {}
```

## Supported css attributes

display: flex | block | inline

## Base Styles

Base styles will be defined for common html parallels

flex, box, h1, h2, h3, h4, h5, b, i, u, table, tr, td, th, p, span

```
p {  }

h1{ A Header }

table {
    tr { td{ Text } td{ Text } td{ Text } }
    tr { td{ Text } td{ Text } td{ Text } }
    tr { td{ Text } td{ Text } td{ Text } }
}

```