# pzx
`pzx` is a command-line utility for expanding its limited langauge into ncurses commands.
## Installation
`pzx` requires Rust and ncurses to be installed on the system. The installation of these two systems should either be facile or unnecessary. To build `pzx`, simply run `cargo build --release` at its directory. Its executable file will have been built at `target/release/pzx`.
## Usage
`pzx` is run at the command-line with its first argument being its string of instructions. A list of examples is provided below.
## Examples
* `pzx 'PRINT "Hello world"; AWAIT out'`; this program prints "Hello world" and accepts a line of input from the user; the input provided is printed to stdout after the program terminates because `AWAIT`s operand is `out`.
* `pzx 'PRINT "Press any key to swap the foreground and background"; ACCEPT a; FORE background; BACK foreground; AWAIT out`; this prints instructions and then waits for the user to press any key. After, it sets the foreground color to the background variable and the background color to the foreground variable.
* `pzx 'CLR orange 900 400 400; BACK orange; PRINT "Press any key to change the background color to black"; ACCEPT a; BACK background; AWAIT out;'`; this program establishes a color `orange` with the rgb values 225, 100, 100 (colors in `pzx` are from 0 to 999, not 0 to 255 as is typical). It then sets the background color to orange and tells the user to press any key. After this, the background color is reset to its initial state. Then a line of input is read from the user.