#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(dead_code)]

extern crate ncurses;

use std::str;
use std::env;
use std::collections::HashMap;

use ncurses::*;

fn main() {
    init();
    let global_vars = global_variables();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        terminate();
        panic!("ERR: No instructions provided");
    }
    let commands = &args[1];
    let tokens = derive_tokens(commands);
    let ret = parse_tokens(&tokens, global_vars);
    terminate();

    println!("{}", ret);
}

fn init() {
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    cbreak();
    keypad(stdscr(), true); 
    noecho();

    if has_colors() && can_change_color() {
        start_color();
    }

    clear();
}

fn terminate() {
    endwin();
}

#[derive(Eq, PartialEq, Debug)]
enum Operator {
    Print,
    Await,
    Accept,
    Foreground,
    Background,
    Move,
    Shift,
    StringAssign,
    NumberAssign,
    ColorAssign,
}

#[derive(Eq, PartialEq, Debug)]
enum Token {
    Operator(Operator),
    Separator, 
    Identifier(String),
    Literal(String), // Just a string; gets converted to a value in the parser
}

#[derive(Eq, PartialEq)]
enum Variable {
    Num(i32),
    Str(String),
    Clr {
        r: i16,
        g: i16,
        b: i16,
    },
}

fn parse_tokens(tokens: &Vec<Token>, global_vars: HashMap<String, Variable>) -> String {
    let mut variables = global_vars;

    for i in 0..tokens.len() {
        match &tokens[i] {
            Token::Operator(Operator::Print) => {
                if let Token::Literal(s) = &tokens[i+1] {
                    addstr(s);
                } else if let Token::Identifier(var) = &tokens[i+1] {
                    if let Variable::Str(s) = variables.get(var).unwrap() {
                        addstr(s);
                    }
                } else {
                    panic!("Err: Improper operand provided to PRINT operation");
                }
            },
            Token::Operator(Operator::Await) => {
                if let Token::Identifier(var) = &tokens[i+1] {
                    echo();
                    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
                    let mut operand = String::new();
                    getstr(&mut operand);
                    noecho();
                    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
                    variables.insert(var.to_owned(), Variable::Str(operand));
                } else {
                    panic!("Err: Improper operand provided to AWAIT operation");
                }
            },
            Token::Operator(Operator::Accept) => {
                if let Token::Identifier(var) = &tokens[i+1] {
                    echo();
                    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
                    let operand = getch() as u8 as char;
                    noecho();
                    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
                    variables.insert(var.to_owned(), Variable::Str(operand.to_string()));
                } else {
                    panic!("Err: Improper operand provided to AWAIT operator");
                }
            },
            Token::Operator(Operator::Foreground) => {
                if let Token::Identifier(var) = &tokens[i+1] {
                    change_foreground(&variables.get(var).unwrap()).expect("Err: Could not change foreground color");
                } else if let Token::Literal(r_raw) = &tokens[i+1] {
                    let mut r: i16 = 0;
                    let mut g: i16 = 0;
                    let mut b: i16 = 0;
                    r = r_raw.parse::<i16>().expect("Err: Color literal not composed of digits");
                    if let Token::Literal(g_raw) = &tokens[i+2] {
                        g = g_raw.parse::<i16>().expect("Err: Color literal not composed of digits");
                    }
                    if let Token::Literal(b_raw) = &tokens[i+3] {
                        b = b_raw.parse::<i16>().expect("Err: Color literal not composed of digits");
                    }
                    change_foreground(&Variable::Clr{r, g, b}).expect("Err: Could not change foreground color");
                } else {
                    panic!("Err: Improper operand provided to FORE operator");
                }
            },
            Token::Operator(Operator::Background) => {
                if let Token::Identifier(var) = &tokens[i+1] {
                    change_background(&variables.get(var).unwrap()).expect("Err: Could not change foreground color");
                } else if let Token::Literal(r_raw) = &tokens[i+1] {
                    let mut r: i16 = 0;
                    let mut g: i16 = 0;
                    let mut b: i16 = 0;
                    r = r_raw.parse::<i16>().expect("Err: Color literal not composed of digits");
                    if let Token::Literal(g_raw) = &tokens[i+2] {
                        g = g_raw.parse::<i16>().expect("Err: Color literal not composed of digits");
                    }
                    if let Token::Literal(b_raw) = &tokens[i+3] {
                        b = b_raw.parse::<i16>().expect("Err: Color literal not composed of digits");
                    }
                    change_background(&Variable::Clr{r, g, b}).expect("Err: Could not change foreground color");
                } else {
                    panic!("Err: Improper operand provided to FORE operator");
                }
            },
            Token::Operator(Operator::Move) => {
                // First get the desired position
                let mut pos = (0, 0);
                match &tokens[i+1] {
                    Token::Literal(s) => {
                        pos.0 = s.parse().expect("Err: Improper operand provided to MOVE operator");
                    },
                    Token::Identifier(var) => {
                        let line_num = variables.get(var).expect("Err: Improper operand provided to MOVE operator");
                        if let Variable::Num(i) = line_num {
                           pos.0 = *i; 
                        }
                    },
                    _ => {
                        panic!("Err: Improper operand provided to MOVE operator");
                    },
                }

                match &tokens[i+2] {
                    Token::Literal(s) => {
                        pos.1 = s.parse().expect("Err: Improper operand provided to MOVE operator");
                    },
                    Token::Identifier(var) => {
                        let line_num = variables.get(var).expect("Err: Improper operand provided to MOVE operator");
                        if let Variable::Num(i) = line_num {
                           pos.1 = *i; 
                        }
                    },
                    _ => {
                        panic!("Err: Improper operand provided to MOVE operator");
                    },
                }
                // Then move to it
                mv(pos.0, pos.1);
                *variables.get_mut("x").unwrap() = Variable::Num(pos.1);
                *variables.get_mut("y").unwrap() = Variable::Num(pos.0);
            },
            Token::Operator(Operator::Shift) => {
                // First get the desired position
                let mut pos = (0, 0);
                match &tokens[i+1] {
                    Token::Literal(s) => {
                        pos.0 = s.parse().expect("Err: Improper operand provided to MOVE operator");
                    },
                    Token::Identifier(var) => {
                        let line_num = variables.get(var).expect("Err: Improper operand provided to MOVE operator");
                        if let Variable::Num(y) = line_num {
                           pos.0 = *y; 
                        }
                    },
                    _ => {
                        panic!("Err: Improper operand provided to MOVE operator");
                    },
                }

                match &tokens[i+2] {
                    Token::Literal(s) => {
                        pos.1 = s.parse().expect("Err: Improper operand provided to MOVE operator");
                    },
                    Token::Identifier(var) => {
                        let line_num = variables.get(var).expect("Err: Improper operand provided to MOVE operator");
                        if let Variable::Num(x) = line_num {
                           pos.1 = *x; 
                        }
                    },
                    _ => {
                        panic!("Err: Improper operand provided to MOVE operator");
                    },
                }
                // Shift as offset by current cursor position
                if let Variable::Num(y) = variables.get("y").unwrap() {
                    pos.0 += y;
                }
                if let Variable::Num(x) = variables.get("x").unwrap() {
                    pos.1 += x;
                }
                // Then move to it
                mv(pos.0, pos.1);
                *variables.get_mut("x").unwrap() = Variable::Num(pos.1);
                *variables.get_mut("y").unwrap() = Variable::Num(pos.0);
            },
            Token::Operator(Operator::StringAssign) => {
                let mut value = String::new();

                match &tokens[i+2] {
                    Token::Identifier(var) => {
                        if let Variable::Str(s) = variables.get(var).unwrap() {
                            value = s.to_owned();
                        } else {
                            panic!("Err: Improper operand provided to STRING operator");
                        }
                    },
                    Token::Literal(s) => {
                        value = s.to_owned();
                    }
                    _ => {
                        panic!("Err: Improper operand provided to STRING operator");
                    },
                }

                if let Token::Identifier(var) = &tokens[i+1] {
                    *variables.get_mut(var).unwrap() = Variable::Str(value);
                }
            },
            Token::Operator(Operator::NumberAssign) => {
                let mut value: i32;

                match &tokens[i+2] {
                    Token::Identifier(var) => {
                        if let Variable::Num(x) = variables.get(var).unwrap() {
                            value = *x;
                        } else {
                            panic!("Err: Improper operand provided to NUM operator");
                        }
                    },
                    Token::Literal(s) => {
                        value = s.parse().expect("Err: Improper operand provided to NUM operator");
                    }
                    _ => {
                        panic!("Err: Improper operand provided to NUM operator");
                    },
                }

                if let Token::Identifier(var) = &tokens[i+1] {
                    *variables.get_mut(var).unwrap() = Variable::Num(value);
                }
            },
            Token::Operator(Operator::ColorAssign) => {
                let mut r: i16 = 0;
                let mut g: i16 = 0;
                let mut b: i16 = 0;
                let mut name = String::new();

                if let Token::Identifier(s) = &tokens[i+1] {
                    name = s.to_owned();
                } else {
                    panic!("Err: Improper operand provided to CLR operator");
                }

                if let Token::Literal(r_var) = &tokens[i+2] {
                    r = r_var.parse::<i16>().expect("Err: Improper operand provided to CLR operator");
                } else {
                    panic!("Err: Improper operand provided to CLR operator");
                }

                if let Token::Literal(g_var) = &tokens[i+3] {
                    g = g_var.parse::<i16>().expect("Err: Improper operand provided to CLR operator");
                } else {
                    panic!("Err: Improper operand provided to CLR operator");
                }

                if let Token::Literal(b_var) = &tokens[i+4] {
                    b = b_var.parse::<i16>().expect("Err: Improper operand provided to CLR operator");
                } else {
                    panic!("Err: Improper operand provided to CLR operator");
                }

                variables.insert(name, Variable::Clr{r, g, b});
            },
            Token::Identifier(_) => {
                continue;
            },
            Token::Literal(_) => {
                continue;
            },
            Token::Separator => {
                continue;
            },
        }
    }

    if let Variable::Str(s) = variables.get("out").unwrap() {
        return s.to_owned();
    } else {
        return "".to_owned();
    }
}

fn change_foreground(color: &Variable) -> Result<(), ()> {
    if let Variable::Clr{r, g, b} = color {
        init_color(16, *r, *g, *b);
    } else {
        return Err(());
    }

    return Ok(());
}

fn change_background(color: &Variable) -> Result<(), ()> {
    if let Variable::Clr{r, g, b} = color {
        init_color(17, *r, *g, *b);
    } else {
        return Err(());
    }

    return Ok(());
}

fn update_color_pairs(foreground: &Variable, background: &Variable) {
    change_foreground(foreground).expect("Could not change foreground color");
    change_background(background).expect("Could not change background color");

    init_pair(1, 16, 17);
    bkgd(' ' as chtype | COLOR_PAIR(1) as chtype);
    attron(COLOR_PAIR(1));
    clear();
}

fn global_variables() -> HashMap<String, Variable> {
    let mut res: HashMap<String, Variable> = HashMap::new();
    // Cursor position 
    res.insert("x".to_owned(), Variable::Num(0));
    res.insert("y".to_owned(), Variable::Num(0));
    // Screen-relative positions
    res.insert("G".to_owned(), Variable::Num(LINES()-1));
    res.insert("$".to_owned(), Variable::Num(COLS()-1));
    // Colors
    res.insert("foreground".to_owned(), Variable::Clr{r: 1000, g: 1000, b: 1000});
    res.insert("background".to_owned(), Variable::Clr{r: 0, g: 0, b: 0});
    update_color_pairs(res.get("foreground").unwrap(), res.get("background").unwrap());
    // Standard output
    res.insert("out".to_owned(), Variable::Str(String::new()));
    return res;
}

fn derive_tokens(raw: &str) -> Vec<Token> {
    let mut res: Vec<Token> = Vec::new();
    
    let mut buffer: String = String::new();
    let mut in_quotes = false;
    for c in raw.chars() {
        match c { // Check for separators and assemble tokens afterwards
            '"' => { // We want to accept ANY INPUT in quotes, so lexing stops there
                buffer.push(c);
                in_quotes = !in_quotes;
            },
            ' ' => {
                if !in_quotes {
                    // This occurs when a space immediately follows a semicolon (common for readability)
                    if buffer.is_empty() {
                        continue;
                    }
                    let token = assemble_token(&buffer, &res);
                    buffer = String::new();
                    res.push(token.expect("Err: Could not lex token"));
                } else {
                    buffer.push(c);
                }
            },
            ';' => {
                if !in_quotes {
                    let token = assemble_token(&buffer, &res);
                    buffer = String::new();
                    res.push(token.expect("Err: Could not lex token"));
                    res.push(Token::Separator);
                } else {
                    buffer.push(c);
                }
            },
            _ => {
                buffer.push(c);
            },
        }
    }

    // In case the command list does not end in a separator
    if !buffer.is_empty() {
        let token = assemble_token(&buffer, &res);
        res.push(token.expect("Err: Could not lex token"));
    }

    res
}

fn assemble_token(raw: &str, context: &Vec<Token>) -> Option<Token> {
    let mut res: Option<Token> = None;

    res = match raw {
        "PRINT" => {
            Some(Token::Operator(Operator::Print))
        },
        "AWAIT" => {
            Some(Token::Operator(Operator::Await))
        },
        "ACCEPT" => {
            Some(Token::Operator(Operator::Accept))
        },
        "FORE" => {
            Some(Token::Operator(Operator::Foreground))
        },
        "BACK" => {
            Some(Token::Operator(Operator::Background))
        },
        "MOVE" => {
            Some(Token::Operator(Operator::Move))
        },
        "SHIFT" => {
            Some(Token::Operator(Operator::Shift))
        },
        "STRING" => {
            Some(Token::Operator(Operator::StringAssign))
        },
        "NUM" => {
            Some(Token::Operator(Operator::NumberAssign))
        },
        "CLR" => {
            Some(Token::Operator(Operator::ColorAssign))
        },
        _ => {
            None
        },
    };

    // I could either have deeply-nested case/switch tests or else readable code and these little checks
    // I chose the latter
    if res != None {
        return res;
    }
    
    // Check what comes immediately before
    res = match *context.get(context.len()-1).unwrap() {
        Token::Operator(Operator::Print) => {
            // Check first char in raw
            if raw.chars().next().unwrap() == '\"' {
                Some(Token::Literal(raw.replace('"', "").to_owned()))
            } else {
                Some(Token::Identifier(raw.to_owned()))
            }
        },
        Token::Operator(Operator::Await) => {
            Some(Token::Identifier(raw.to_owned()))
        },
        Token::Operator(Operator::Accept) => {
            Some(Token::Identifier(raw.to_owned()))
        },
        Token::Operator(Operator::Foreground) => {
            if raw.chars().next().unwrap().is_ascii_digit() {
                Some(Token::Literal(raw.to_owned()))
            } else {
                Some(Token::Identifier(raw.to_owned()))
            }
        },
        Token::Operator(Operator::Background) => {
            if raw.chars().next().unwrap().is_ascii_digit() {
                Some(Token::Literal(raw.to_owned()))
            } else {
                Some(Token::Identifier(raw.to_owned()))
            }
        },
        Token::Operator(Operator::Move) => {
            let first_letter = raw.chars().next().unwrap();
            if first_letter == 'G' {
                Some(Token::Identifier(raw.to_owned()))
            } else if first_letter == 'g' {
                Some(Token::Identifier(raw.to_owned()))
            } else {
                Some(Token::Literal(raw.to_owned()))
            }
        },
        Token::Operator(Operator::Shift) => {
            let first_letter = raw.chars().next().unwrap();
            if first_letter == 'G' {
                Some(Token::Identifier(raw.to_owned()))
            } else if first_letter == 'g' {
                Some(Token::Identifier(raw.to_owned()))
            } else {
                Some(Token::Literal(raw.to_owned()))
            }
        },
        Token::Operator(Operator::StringAssign) => {
            Some(Token::Identifier(raw.to_owned()))
        },
        Token::Operator(Operator::NumberAssign) => {
            Some(Token::Identifier(raw.to_owned()))
        },
        Token::Operator(Operator::ColorAssign) => {
            Some(Token::Identifier(raw.to_owned()))
        },
        _ => {
            None
        },
    };

    // I could either have deeply-nested case/switch tests or else readable code and these little checks
    // I chose the latter
    if res != None {
        return res;
    }

    // Check what comes two lexemes before
    res = match *context.get(context.len()-2).unwrap() {
        Token::Operator(Operator::Move) => {
            let first_letter = raw.chars().next().unwrap();
            if first_letter == '$' {
                Some(Token::Identifier(raw.to_owned()))
            } else {
                Some(Token::Literal(raw.to_owned()))
            }
        },
        Token::Operator(Operator::Shift) => {
            let first_letter = raw.chars().next().unwrap();
            if first_letter == '$' {
                Some(Token::Identifier(raw.to_owned()))
            } else {
                Some(Token::Literal(raw.to_owned()))
            }
        },
        Token::Operator(Operator::StringAssign) => {
            Some(Token::Literal(raw.to_owned()))
        },
        Token::Operator(Operator::NumberAssign) => {
            Some(Token::Literal(raw.to_owned()))
        },
        Token::Operator(Operator::ColorAssign) => {
            Some(Token::Literal(raw.to_owned()))
        },
        _ => {
            None
        },
    };

    // I could either have deeply-nested case/switch tests or else readable code and these little checks
    // I chose the latter
    if res != None {
        return res;
    }

    res = match *context.get(context.len()-3).unwrap() {
        Token::Operator(Operator::ColorAssign) => {
            Some(Token::Literal(raw.to_owned()))
        },
        _ => {
            None
        }
    };

    // I could either have deeply-nested case/switch tests or else readable code and these little checks
    // I chose the latter
    if res != None {
        return res;
    }

    res = match *context.get(context.len()-4).unwrap() {
        Token::Operator(Operator::ColorAssign) => {
            Some(Token::Literal(raw.to_owned()))
        },
        _ => {
            None
        }
    };

    return res;
}

#[test]
fn lexer_test() {
    println!("{:?}", derive_tokens("PRINT \"hi\""));
    println!("{:?}", derive_tokens("PRINT \"hello world\""));
    println!("{:?}", derive_tokens("STRING a \"hi\"; PRINT a"));
    println!("{:?}", derive_tokens("FORE green; BACK bright_red; PRINT \"colors\""));
    println!("{:?}", derive_tokens("MOVE 0 0"));
    println!("{:?}", derive_tokens("SHIFT 0 0"));
    println!("{:?}", derive_tokens("NUM a 0"));
}
