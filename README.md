# pzx
`pzx` is a language specification and utility to abstract away ncurses programs into an extensible UNIX-philosophy command-line interface. 

## Usage
`pzx` takes input in the form of the `pzx` language and interprets it into a set of ncurses instructions. It can be strung together with other programs using UNIX pipes to create powerful command-line applications, much like AWK.

## Language Specification
* All reserved operator keywords are upper-cased
* Strings are put in quotes
* Operations always begin a string and their operands are listed subsequent to it (e.g. `MOVE 3J` to move down three lines)
* Statements are terminated by a semicolon
* The reserved operators are thus:
    * `PRINT [str]` : Output `str` at the cursor 
    * `AWAIT <var>` : Accept one line of input from stdin into variable `var`
    * `ACCEPT <var>` : Accepts one character of input from stdin into variable `var`
    * `FORE [clr]` : Change the foreground color to `clr`
    * `BACK [clr]` : Change the background color to `clr`
    * `MOVE [num] [num]` : Move the cursor to the specified line/column positions
    * `SHIFT [num] [num]` : Move the cursor by the specified line/column positions
    * `STRING <var> [str]` : Moves the contents of the string `str` into the variable with the name `var`
    * `NUM <var> [num]` : As above for `num`s
* The variable types are thus:
    * `clr` : Color type; cannot yet be set (this functionality may be added later)
    * `str` : A UTF-8 string buffer; must be surrounded by double-quotes
    * `num` : A 32-bit signed integer
    * `val` : Any variable type (only for readability; not specified in the parser)
* The globals are thus:
    * `clr`s `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, or `white`, as well as any of the aforementioned prefaced by `bright_`
    * `num x` : The current column the cursor is on
    * `num y` : The current line the cursor is on
    * `num G` : The last line of the terminal
    * `num $` : The last column of the terminal
    * `str out` : The buffer that will, at termination, be sent to stdout