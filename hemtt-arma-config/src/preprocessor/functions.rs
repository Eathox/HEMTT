use std::iter::Peekable;

use crate::{
    preprocessor::ifstate::IfState,
    render::render,
    tokenizer::tokens::{Token, TokenPair},
};

use super::{defines::Defines, ifstate::IfStates, lcc::LineColCounter};

macro_rules! push_token {
    ($i: ident, $to: ident, $lcc: ident) => {{
        let ctp = $i.clone();
        $lcc.mod_cols(ctp);
        $to.push(ctp);
    }};
}

pub fn process_tokens<'a, 'b>(
    tokens: impl Iterator<Item = &'a TokenPair<'a>>,
    defines: &'b Defines<'a>,
) -> Result<Vec<TokenPair<'a>>, String> {
    let mut ret = Vec::new();
    let mut lcc = LineColCounter::new();
    let mut ifstates = IfStates::new();
    let mut tokens = tokens.peekable();
    while let Some(pair) = tokens.peek() {
        match (pair.token(), lcc.newline(), ifstates.reading()) {
            (Token::Directive, true, r) => {
                lcc.mod_cols(tokens.next().unwrap()); // Consume #
                let directive = tokens.next();
                if directive.is_none() {
                    panic!("# without a directive");
                }
                let directive = directive.unwrap();
                lcc.mod_cols(directive);
                match (directive.token().to_string().as_str(), r) {
                    ("define", true) => {
                        let name = read_ident(&mut tokens, &mut lcc)?;
                        if let Token::LeftParenthesis =
                            skip_whitespace(&mut tokens, &mut lcc)?.token()
                        {
                            let args = read_args(&mut tokens, &mut lcc)?;
                            println!("function {:?}", name);
                            println!("args {:?}", args);
                            let statement = read_define_value(&mut tokens, &mut lcc)?;
                            println!("statement {:?}", statement);
                            defines.new_function(&name.0, args, statement);
                        } else {
                            defines.new_word(&name.0, read_define_value(&mut tokens, &mut lcc)?);
                        }
                    }
                    ("undef", true) => {
                        let name = read_ident(&mut tokens, &mut lcc)?;
                        defines.remove(&name.0);
                    }
                    ("ifdef", true) => {
                        let name = read_ident(&mut tokens, &mut lcc)?;
                        if defines.contains(&name.0) {
                            ifstates.push(IfState::ReadingIf);
                        } else {
                            ifstates.push(IfState::PassingIf);
                        }
                    }
                    ("ifdef", false) => {
                        ifstates.push(IfState::PassingChild);
                        tokens.next();
                    }
                    ("ifndef", true) => {
                        let name = read_ident(&mut tokens, &mut lcc)?;
                        if defines.contains(&name.0) {
                            ifstates.push(IfState::PassingIf);
                        } else {
                            ifstates.push(IfState::ReadingIf);
                        }
                    }
                    ("ifndef", false) => {
                        ifstates.push(IfState::PassingChild);
                        tokens.next();
                    }
                    ("else", _) => {
                        ifstates.flip();
                        tokens.next();
                    }
                    ("endif", _) => {
                        ifstates.pop();
                        tokens.next();
                    }
                    _ => {
                        panic!("Unsupported directive: {}", directive.token().to_string());
                    }
                }
            }
            (Token::Newline, _, true) => {
                lcc.add_line();
                ret.push(tokens.next().unwrap().clone()); // Consume \n
            }
            (Token::Newline, _, false) => {
                lcc.add_line();
                tokens.next(); // Consume \n
            }
            (Token::EOI, _, _) => {
                tokens.next(); // Consume EOI
            }
            (Token::Word(_) | Token::Underscore, _, true) => {
                ret.append(&mut resolve(&mut tokens, &mut lcc, defines)?);
            }
            (_, _, true) => {
                lcc.mod_cols(pair);
                ret.push(tokens.next().unwrap().clone());
            }
            (_, _, false) => {
                tokens.next();
            }
        }
    }
    Ok(ret)
}

fn resolve<'a, 'b>(
    tokens: &mut Peekable<impl Iterator<Item = &'a TokenPair<'a>>>,
    lcc: &mut LineColCounter,
    defines: &'b Defines<'a>,
) -> Result<Vec<TokenPair<'a>>, String> {
    skip_whitespace(tokens, lcc)?;
    // Find the entire haystack
    let mut stack = Vec::new();
    while let Some(tp) = tokens.peek() {
        match tp.token() {
            Token::Word(_) => {
                push_token!(tp, stack, lcc);
                tokens.next();
            }
            Token::Underscore => {
                push_token!(tp, stack, lcc);
                tokens.next();
            }
            _ => {
                if stack.is_empty() {
                    let ntp = tokens.next().unwrap();
                    lcc.mod_cols(ntp);
                    return Ok(vec![ntp.clone()]);
                }
                break;
            }
        }
    }
    let mut stack: Vec<TokenPair<'a>> = stack.into_iter().cloned().collect();
    let original = stack.clone();
    while !stack.is_empty() {
        for i in (0..stack.len()).rev() {
            let s = stack.clone();
            let ident = read_ident(
                &mut s.iter().take(i + 1).peekable(),
                &mut LineColCounter::new(),
            )?;
            if let Some(d) = defines.get(&ident.0) {
                println!("found define: {:?}", d.statement());
                let defines = if let Some(args) = d.args() {
                    if let Token::LeftParenthesis = tokens.peek().unwrap().token() {
                        let inputs = read_args(tokens, lcc)?;
                        println!("inputs {:?}", inputs);
                        if inputs.len() != args.len() {
                            return Err("Wrong number of arguments".to_string());
                        }
                        let defines = (*defines).clone();
                        for (i, arg) in args.iter().enumerate() {
                            defines.new_word(
                                &render(&arg.iter().map(|a| (*a).clone()).collect::<Vec<_>>())
                                    .export(),
                                inputs[i].clone(),
                            );
                        }
                        defines
                    } else {
                        return Err("Expected (".to_string());
                    }
                } else {
                    defines.clone()
                };
                return process_tokens(d.statement_ref().into_iter(), &defines);
            }
        }
        stack.remove(0);
    }
    Ok(original)
}

fn read_ident<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a TokenPair<'a>>>,
    linecol: &mut LineColCounter,
) -> Result<
    (
        String,
        Option<String>,
        (usize, (usize, usize)),
        (usize, (usize, usize)),
    ),
    String,
> {
    skip_whitespace(tokens, linecol)?;
    let mut ident = String::new();
    let start = linecol.pos_linecol();
    let mut path = None;
    while let Some(tp) = tokens.peek() {
        path = Some(tp.path().to_string());
        match tp.token() {
            Token::Word(w) => {
                ident.push_str(w.as_str());
                linecol.mod_cols(tp);
                tokens.next();
            }
            Token::Underscore => {
                ident.push('_');
                linecol.mod_cols(tp);
                tokens.next();
            }
            _ => break,
        }
    }
    Ok((ident, path, start, linecol.pos_linecol()))
}

pub fn read_args<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a TokenPair<'a>>>,
    linecol: &mut LineColCounter,
) -> Result<Vec<Vec<&'a TokenPair<'a>>>, String> {
    if let Token::LeftParenthesis = skip_whitespace(tokens, linecol)?.token() {
        linecol.add_cols(&Token::LeftParenthesis);
        tokens.next();
    } else {
        panic!("Expected ( but found something else")
    }
    let mut args = Vec::new();
    let mut arg = Vec::new();
    let mut nested = 0;

    while let Some(tp) = &tokens.next() {
        match tp.token() {
            Token::LeftParenthesis => {
                nested += 1;
                push_token!(tp, arg, linecol);
            }
            Token::RightParenthesis => {
                if nested == 0 {
                    if !arg.is_empty() {
                        args.push(arg);
                    }
                    break;
                }
                push_token!(tp, arg, linecol);
                nested -= 1;
            }
            Token::Comma => {
                if nested == 0 {
                    if !arg.is_empty() {
                        args.push(arg);
                        arg = Vec::new();
                    }
                } else {
                    push_token!(tp, arg, linecol);
                }
            }
            _ => {
                push_token!(tp, arg, linecol);
            }
        }
    }
    Ok(args)
}

fn read_define_value<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a TokenPair<'a>>>,
    linecol: &mut LineColCounter,
) -> Result<Vec<&'a TokenPair<'a>>, String> {
    skip_whitespace(tokens, linecol)?;
    let mut ret = Vec::new();
    let mut quoted = false;
    while let Some(tp) = &tokens.next() {
        if quoted {
            if &Token::DoubleQuote == tp.token() {
                quoted = false;
            }
            push_token!(tp, ret, linecol);
        } else {
            match tp.token() {
                Token::Newline => {
                    // ret.push(tp);
                    linecol.add_line();
                    break;
                }
                Token::Escape => {
                    if let Some(etp) = tokens.peek() {
                        if &Token::Newline == etp.token() {
                            push_token!(etp, ret, linecol);
                        }
                    }
                }
                Token::DoubleQuote => {
                    push_token!(tp, ret, linecol);
                    quoted = true;
                }
                Token::EOI => break,
                _ => {
                    push_token!(tp, ret, linecol);
                }
            }
        }
    }
    Ok(ret)
}

fn skip_whitespace<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a TokenPair<'a>>>,
    linecol: &mut LineColCounter,
) -> Result<&'a TokenPair<'a>, String> {
    while let Some(tp) = tokens.peek() {
        if let Token::Whitespace(_) = tp.token() {
            linecol.mod_cols(tp);
            tokens.next();
        } else {
            return Ok(tp);
        }
    }
    panic!("tokens was missing EOI")
}
