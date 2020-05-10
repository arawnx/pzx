# pzx
`pzx` is a language specification and utility to abstract away ncurses programs into an extensible UNIX-philosophy command-line interface. 

## Usage
`pzx` takes input in the form of the `pzx` language and interprets it into a set of ncurses instructions. It can be strung together with other programs using UNIX pipes to create powerful command-line applications, much like AWK.

### Examples
* `cat main.rs | pzx "PRINT in; AWAIT out" | pzx "FORE green; PRINT in"` will print the contents of `main.rs` to the screen, wait for some line of input, and then output whatever was input in the color green.
* ```CMD=`pzx 'PRINT "ENTER COMMAND"; MOVE G,0; PRINT ":"; AWAIT out'` ``` will display the text `ENTER COMMAND`, move to the lower-left of the screen, print the letter `:`, and set the shell variable `$CMD` to the subsequent line read from stdin. 
* ```pzx 'STRING test "Hello world!"; CLR grue #00AAAA; FORE grue; PRINT test; AWAIT out```

## Language Specification
* All reserved operator keywords are upper-cased
* Strings are put in quotes
* Operations always begin a string and their operands are listed subsequent to it (e.g. `MOVE 3J` to move down three lines)
* Statements are terminated by a semicolon
* The reserved operators are thus:
    * `PRINT [str]` : Output `str` at the cursor 
    * `AWAIT [str]` : Accept one line of input from stdin into string `str`
    * `FORE [clr]` : Change the foreground color to `clr`
    * `BACK [clr]` : Change the background color to `clr`
    * `MOVE [num],[num]` : Move the cursor to the specified line/column positions
    * `SHIFT [num],[num]` : Move the cursor by the specified line/column positions
    * `STRING <var> [str]` : Moves the contents of the string `str` into the variable with the name `var`
    * `NUM <var> [num]` : As above for `num`s
    * `CLR <var> [clr]` : As above for `clr`s
* The variable types are thus:
    * `clr` : Color type; defined as hexadecimal
    * `str` : A UTF-8 string buffer; surrounded by double-quotes
    * `num` : A 16-bit unsigned integer
    * `val` : Any variable type (only for readability; not specified in the parser)
* The globals are thus:
    * `clr`s `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, or `white`, as well as any of the aforementioned prefaced by `bright_`
    * `num X` : The current column the cursor is on
    * `num Y` : The current line the cursor is on
    * `num G` : The last line of the terminal
    * `num g` : The first line of the terminal
    * `num $` : The last column of the terminal
    * `num 0` : The first column of the terminal
    * `str in` : The stdin provided to the program
    * `str out` : The buffer that will, at termination, be sent to stdout