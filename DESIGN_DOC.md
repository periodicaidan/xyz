periodicaidan.xyz
===

### Abstract

The purpose of this document is to walk through the design process
and philosophy of my personal website [periodicaidan.xyz](https://periodicaidan.xyz).

## Backend

The backend is written in Rust using the Rocket library. There are a number
of reasons for this:

1. Rust is safe, and safety is crucial
1. Rust's build system is nifty
1. Rocket's routing system is very good and flexible, allowing me to wire together
components however I wish

### File structure

Backend source files are found where all Rust source files are, in the `/src`
directory. The following files are worthy of note.

`main.rs` is where the main function is located. All this function does is launch
Rocket, connect to the database, mount all the routes, etc.

`routes.rs` is where all the routing functions and a handful of associated records
are found.

`guards.rs` is where request guards are to be found.

`database/` and `schema.rs` contains files relating to Diesel, which
is the ORM used by this project.

## Frontend

Frontend components are written in Elm, which is a functional language that
compiles to JavaScript. I used it primarily to write "components"&mdash;or, more
accurately, behaviors that are self-contained and bound to particular HTML contexts, 
such as animations and Markdown rendering.

There are a handful of cases where writing vanilla JavaScript is easier than 
writing a while new Elm element. In general, vanilla JS is used when:

- A procedure is more expediently handled imperatively rather than declaratively,
such as sending a one-off HTTP request or adding HTML to an element.
- A certain procedure is performed on the whole page, rather than being confined
to a single element, such as applying a dynamic style to a class of elements.

### File structure

Components are found in `/static/components`. Individual components, written in Elm,
are found in the `src/` directory of the components directory and all are compiled into
a single `Components.js` file.

``` 
/static/components
|
+-- src/
|   |
|   +-- TypedTitle.elm
|   |
|   +-- MarkdownParsing.elm
|
+-- Components.js (compiled)
```

To build `Components.js`, the following shell command is run:

```shell script
elm make static/components/src/*.elm --optimize --output=static/components/Components.js
```

## Styling

Stylesheet source files are written in Less for all the usual 
reasons people write stylesheets in Less: variables,
functions, expressions, etc. It's worth going over all the files
as each has a different purpose.

`constants.less` contains variables shared by the other files.

`global.less` defines styles used by all the pages.

`JetBrainsMono.less` defines `@font-face` bindings for the JetBrains Mono 
font family. The font itself is included in this project inside the `/fonts/`
directory.

`ligatures.less` defines ligatures. Giving an element the `lig` class
attribute changes the font to JetBrains Mono to use its ligatures. Then 
particular ligatures can be defined so that the following are identical:

```html
<span style="font-family: 'JetBrains Mono', 'Fira Code', 'Fira Mono', Monaco, monospace;">=></span>

<span class="fat right arrow lig"></span>
```


