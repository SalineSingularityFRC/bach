# Bach
A documentation generator for Java

### Building
Bach is written in the [rust programming language](https://rust-lang.org), of which you will need a copy. Installation instructions for your platform can be found on their website.

Simply running `cargo build --release` in the command prompt / terminal should build the binary into `./target/release/bach`. You will then need to copy that somewhere where your computer can run it, usually called `$PATH` on UNIX-likes, otherwise you can just run it from that directory.

### Usage
Running `bach` will scan every file in a directory (recursively) and find lines beginning with `///` (documentation comments). It will then attempt to derive the following line of code into a description of that line using RegEx to find each piece of information.

When it is finished scanning, it will output an HTML file into `./bach/index.html` which will contain the generated documentation.

### Technical Details and Hacking
TODO lol
