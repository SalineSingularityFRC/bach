# Bach
A documentation generator for Java

## Building
Bach is written in the [rust programming language](https://rust-lang.org), of which you will need a copy. Installation instructions for your platform can be found on their website.

Simply running `cargo build --release` in the command prompt / terminal should build the binary into `./target/release/bach`. You will then need to copy that somewhere where your computer can run it, usually called `$PATH` on UNIX-likes, otherwise you can just run it from that directory.

## Usage
Running `bach` will scan every file in a directory (recursively) and find lines beginning with `///` (documentation comments). It will then attempt to derive the following line of code into a description of that line using RegEx to find each piece of information.

When it is finished scanning, it will output an HTML file into `./bach/index.html` which will contain the generated documentation.

## Technical Details and Hacking

### Themes
To create a new theme, you will need to create a new CSS file in `src/gen/html/`. This will file will be directly inserted into the `<head>` tag of the HTML file, and thus will control the CSS for that page.

The specific CSS properties used change consistently are not documented here, but you can find them easily using the file `src/gen/html/default.css`.

Although this is subject to change and (hopefully will soon), CSS is *compiled into the binary*. This means if you want to make a theme, you will have to edit the source code and recompile your modified version.

To add a theme, add a line to `src/gen/theme.rs` with the template as follows:
```rs
pub static {{YOUR_THEME_NAME}}_THEME_CSS: &'static str = include_str!("html/{{YOUR_THEME_FILE}}.css");
```

Then, you will need to add your theme to the `Theme` enum at the bottom of `theme.rs`. Simply add your name to the enum and then add access to it in the `get()` and `name()` methods in the impl block.
