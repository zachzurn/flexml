# WIP * RFC
### flexml

Project is currently a toy project. Do not attempt to use anything in this repo.

# Goals

Make a fast and low memory renderer for invoice, reports, receipt type documents. No interactivity outside of links.

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

Css Macro


```
CSS:{
    .red {
        color: red;
    }    
}
```

Csv macro, behaves like table macro
```
CSV:{
    20%,20
    type,name
    dog,kobe
}
```

## Base Styles

Base styles will be defined for common html parallels

flex, box, h1, h2, h3, h4, h5, b, i, u, table, tr, td, th, p, span

```
p {  }

h1{ A Header }

table{
    tr { td{ Text } td{ Text } td{ Text } }
    tr { td{ Text } td{ Text } td{ Text } }
    tr { td{ Text } td{ Text } td{ Text } }
}

```