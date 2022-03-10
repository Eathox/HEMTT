use std::iter::Peekable;

use crate::{
    preprocess::{defines::Define, ifstate::IfState},
    tokenizer::{Token, TokenPair, Tokens},
    ArmaConfigError,
};

mod defines;
use defines::Defines;

mod ifstate;
use ifstate::IfStates;

mod linecol;
use linecol::LineColCounter;

macro_rules! push_token {
    ($i: ident, $to: ident, $lc: ident) => {{
        let ctp = $i.clone();
        $lc.mod_cols(ctp);
        $to.push(ctp);
    }};
}

pub struct Preprocessor<'a> {
    defines: Defines<'a>,
    output: Vec<&'a TokenPair<'a>>,
}

impl<'a> Preprocessor<'a> {
    pub fn output(&'a self) -> &'a Vec<&'a TokenPair<'a>> {
        &self.output
    }

    pub fn execute(root: &'a Tokens<'a>) -> Result<Self, ArmaConfigError> {
        let mut preprocessor = Self {
            defines: Defines::new(),
            output: Vec::new(),
        };
        let mut linecol = LineColCounter::new();
        let mut tokens = root.iter().peekable();
        preprocessor.output =
            Self::process_tokens_peekable(&mut tokens, &mut linecol, &mut preprocessor.defines)?;
        Ok(preprocessor)
    }

    fn process_tokens_peekable<'b>(
        tokens: &mut Peekable<impl Iterator<Item = &'b TokenPair<'b>>>,
        linecol: &mut LineColCounter,
        defines: &mut Defines<'b>,
    ) -> Result<Vec<&'b TokenPair<'b>>, ArmaConfigError> {
        let mut ifstate = IfStates::new();
        let mut ret = Vec::new();
        while let Some(tpeek) = tokens.peek() {
            trace!("Token: {:?}", tpeek.token());
            match (tpeek.token(), linecol.newline(), ifstate.reading()) {
                (Token::Directive, true, if_reading) => {
                    linecol.mod_cols(tokens.next().unwrap()); // Consume #
                    let action = tokens.next();
                    if action.is_none() {
                        panic!("Directive without action");
                    }
                    let action = action.unwrap();
                    linecol.mod_cols(action);
                    match (action.token().to_string().as_str(), if_reading) {
                        ("define", true) => {
                            let name = Self::read_ident(tokens, linecol)?;
                            if let Token::LeftParenthesis =
                                Self::skip_whitespace(tokens, linecol)?.token()
                            {
                                let args = Self::read_args(tokens, linecol)?;
                                defines.new_function(
                                    &name.0,
                                    args,
                                    Self::read_define_value(tokens, linecol)?,
                                );
                            } else {
                                defines
                                    .new_word(&name.0, Self::read_define_value(tokens, linecol)?);
                            }
                        }
                        ("undef", true) => {
                            defines.remove(&Self::read_ident(tokens, linecol)?.0);
                        }
                        ("ifdef", true) => {
                            let name = Self::read_ident(tokens, linecol)?;
                            if defines.defined(&name.0) {
                                ifstate.push(IfState::ReadingIf);
                            } else {
                                ifstate.push(IfState::PassingIf);
                            }
                        }
                        ("ifndef", true) => {
                            let name = Self::read_ident(tokens, linecol)?;
                            if !defines.defined(&name.0) {
                                ifstate.push(IfState::ReadingIf);
                            } else {
                                ifstate.push(IfState::PassingIf);
                            }
                        }
                        ("ifdef", false) => {
                            ifstate.push(IfState::PassingChild);
                        }
                        ("ifndef", false) => {
                            ifstate.push(IfState::PassingChild);
                        }
                        ("else", _) => {
                            ifstate.flip();
                        }
                        ("endif", _) => {
                            ifstate.pop();
                        }
                        (_, false) => {
                            linecol.mod_cols(tokens.next().unwrap());
                        }
                        _ => panic!("Unknown directive"),
                    }
                    if let Token::Newline = Self::skip_whitespace(tokens, linecol)?.token() {
                        linecol.add_line();
                        tokens.next(); // Consume \n
                    } else {
                        println!("extra tokens after undefine");
                    }
                    // let ident = Self::read_ident(&mut tokens, &mut linecol);
                }
                (Token::Newline, _, append) => {
                    linecol.add_line();
                    // Consume \n
                    if append {
                        ret.push(tokens.next().unwrap());
                    } else {
                        tokens.next();
                    }
                }
                (Token::EOI, _, _) => {
                    tokens.next();
                }
                (Token::Word(_) | Token::Underscore, _, true) => {
                    println!("Resolving on {:?}", tpeek.token());
                    ret.append(&mut Self::resolve(tokens, linecol, defines)?);
                }
                (_, _, true) => {
                    linecol.mod_cols(tpeek);
                    ret.push(tokens.next().unwrap());
                }
                (_, _, false) => {
                    linecol.mod_cols(tokens.next().unwrap());
                }
            }
        }
        Ok(ret)
    }

    fn read_ident<'b>(
        tokens: &mut Peekable<impl Iterator<Item = &'b TokenPair<'b>>>,
        linecol: &mut LineColCounter,
    ) -> Result<
        (
            String,
            Option<String>,
            (usize, (usize, usize)),
            (usize, (usize, usize)),
        ),
        ArmaConfigError,
    > {
        Self::skip_whitespace(tokens, linecol)?;
        let mut ident = String::new();
        let start = linecol.pos_linecol();
        let mut path = None;
        while let Some(tpeek) = tokens.peek() {
            path = Some(tpeek.path().to_string());
            match tpeek.token() {
                Token::Word(w) => {
                    ident.push_str(w.as_str());
                    linecol.mod_cols(tpeek);
                    tokens.next();
                }
                Token::Underscore => {
                    ident.push('_');
                    linecol.mod_cols(tpeek);
                    tokens.next();
                }
                _ => break,
            }
        }
        Ok((ident, path, start, linecol.pos_linecol()))
    }

    fn read_args<'b>(
        tokens: &mut Peekable<impl Iterator<Item = &'b TokenPair<'b>>>,
        linecol: &mut LineColCounter,
    ) -> Result<Vec<Vec<&'b TokenPair<'b>>>, ArmaConfigError> {
        if let Token::LeftParenthesis = Self::skip_whitespace(tokens, linecol)?.token() {
            linecol.add_cols(&Token::LeftParenthesis);
            tokens.next();
        } else {
            panic!("Expected ( but found something else")
        }
        let mut args = Vec::new();
        let mut arg = Vec::new();
        let mut nested = 0;

        while let Some(tnext) = &tokens.next() {
            match tnext.token() {
                Token::LeftParenthesis => {
                    nested += 1;
                    push_token!(tnext, arg, linecol);
                }
                Token::RightParenthesis => {
                    if nested == 0 {
                        if !arg.is_empty() {
                            args.push(arg)
                        }
                        break;
                    } else {
                        push_token!(tnext, arg, linecol);
                    }
                    nested -= 1;
                }
                Token::Comma => {
                    if nested == 0 {
                        if !arg.is_empty() {
                            args.push(arg);
                            arg = Vec::new();
                        }
                    } else {
                        push_token!(tnext, arg, linecol);
                    }
                }
                _ => {
                    push_token!(tnext, arg, linecol);
                }
            }
        }
        Ok(args)
    }

    fn read_define_value<'b>(
        tokens: &mut Peekable<impl Iterator<Item = &'b TokenPair<'b>>>,
        linecol: &mut LineColCounter,
    ) -> Result<Vec<&'b TokenPair<'b>>, ArmaConfigError> {
        Self::skip_whitespace(tokens, linecol)?;
        let mut ret = Vec::new();
        let mut quoted = false;
        while let Some(tnext) = &tokens.next() {
            if quoted {
                if &Token::DoubleQuote == tnext.token() {
                    quoted = false;
                }
                push_token!(tnext, ret, linecol);
            } else {
                match tnext.token() {
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
                        push_token!(tnext, ret, linecol);
                        quoted = true;
                    }
                    Token::EOI => break,
                    _ => {
                        push_token!(tnext, ret, linecol)
                    }
                }
            }
        }
        Ok(ret)
    }

    fn resolve<'b>(
        tokens: &mut Peekable<impl Iterator<Item = &'b TokenPair<'b>>>,
        linecol: &mut LineColCounter,
        defines: &mut Defines<'b>,
    ) -> Result<Vec<&'b TokenPair<'b>>, ArmaConfigError> {
        Self::skip_whitespace(tokens, linecol)?;
        // Find the entire haystack
        let mut stack = Vec::new();
        while let Some(tpeek) = tokens.peek() {
            match tpeek.token() {
                Token::Word(_) => {
                    push_token!(tpeek, stack, linecol);
                    tokens.next();
                }
                Token::Underscore => {
                    push_token!(tpeek, stack, linecol);
                    tokens.next();
                }
                Token::DoubleQuote => {
                    push_token!(tpeek, stack, linecol);
                    tokens.next();
                }
                _ => {
                    if stack.is_empty() {
                        let ntp = tokens.next().unwrap();
                        linecol.mod_cols(ntp);
                        return Ok(vec![ntp]);
                    }
                    break;
                }
            }
        }
        // let mut stack: Vec<TokenPair<'b>> = stack.into_iter().map(|t| t.to_owned()).collect();
        let original = stack.clone();
        while !stack.is_empty() {
            for i in (0..stack.len()).rev() {
                let s = stack.clone();
                let ident = Self::read_ident(
                    &mut s.into_iter().take(i + 1).peekable(),
                    &mut LineColCounter::new(),
                )?;
                println!("ident: {:?}", ident.0);
                if let Some(d) = defines.get(&ident.0) {
                    println!("found define: {:?}", d.statement());
                    let mut scoped_defines = Defines::new();
                    defines.all().iter().for_each(|(k, v)| {
                        scoped_defines.new_define(k, v.clone());
                    });
                    match d {
                        Define::Word(w) => {
                            return Self::process_tokens_peekable(
                                &mut w.into_iter().peekable(),
                                linecol,
                                &mut scoped_defines,
                            );
                        }
                        Define::Function(ref f) => {
                            if let Token::LeftParenthesis =
                                Self::skip_whitespace(tokens, linecol)?.token()
                            {
                                tokens.next(); // Consume (
                                for arg in f.args().clone() {
                                    // let arg_clone = arg.clone().into_iter().map(|t| (*t).to_owned()).collect::<Vec<_>>();
                                    scoped_defines.new_word(
                                        &Self::read_ident(
                                            &mut arg.clone().into_iter().peekable(),
                                            &mut LineColCounter::new(),
                                        )?
                                        .0,
                                        Self::read_call_arg(tokens, linecol, defines)?,
                                    );
                                }
                            } else {
                                panic!("Expected ( but found something else")
                            }
                            println!("scoped defines: {:?}", scoped_defines);
                            panic!();
                            return Self::process_tokens_peekable(
                                &mut d.statement().into_iter().peekable(),
                                linecol,
                                &mut scoped_defines,
                            );
                        }
                    }
                }
            }
            stack.remove(0);
        }
        println!("done checking stack");
        Ok(original)
    }

    fn read_call_arg<'b>(
        tokens: &mut Peekable<impl Iterator<Item = &'b TokenPair<'b>>>,
        linecol: &mut LineColCounter,
        defines: &mut Defines<'b>,
    ) -> Result<Vec<&'b TokenPair<'b>>, ArmaConfigError> {
        let mut ret = Vec::new();
        while let Some(tnext) = &tokens.next() {
            match tnext.token() {
                Token::Comma | Token::RightParenthesis => return Ok(ret),
                _ => {
                    linecol.mod_cols(tnext);
                    push_token!(tnext, ret, linecol);
                }
            }
        }
        Self::process_tokens_peekable(&mut ret.into_iter().peekable(), linecol, defines)
    }

    fn skip_whitespace<'b>(
        tokens: &mut Peekable<impl Iterator<Item = &'b TokenPair<'b>>>,
        linecol: &mut LineColCounter,
    ) -> Result<&'b TokenPair<'b>, ArmaConfigError> {
        while let Some(tpeek) = tokens.peek() {
            if let Token::Whitespace(_) = tpeek.token() {
                linecol.mod_cols(tpeek);
                tokens.next();
            } else {
                return Ok(tpeek);
            }
        }
        panic!("tokens was missing EOI")
    }
}

#[cfg(test)]
mod tests {
    use crate::{render::render, tokenizer::tokenize};

    use super::Preprocessor;

    #[test]
    fn simple_define() {
        let tokens = tokenize(
            "#define brett_greeting \"hi brett\"\n\ngreeting = brett_greeting;\n",
            "test_simple_define",
        )
        .unwrap();
        let preprocessor = Preprocessor::execute(&tokens).unwrap();
        println!("{:?}", render(preprocessor.output()).export());
    }

    #[test]
    fn nested_define() {
        let tokens = tokenize(
            "#define NAME brett\n#define HI \"hi NAME\"\n\ngreeting = HI;\n",
            "test_simple_define",
        )
        .unwrap();
        let preprocessor = Preprocessor::execute(&tokens).unwrap();
        println!("{:?}", render(preprocessor.output()).export());
    }

    #[test]
    fn define_function_recursive_1() {
        let content = r#"
    #define MR(NAME) Mr. NAME
    #define SAY_HI(NAME) Hi MR(NAME)

    value = "SAY_HI(John)";
    "#;
        let tokens = tokenize(content, "").unwrap();
        let preprocessor = Preprocessor::execute(&tokens).unwrap();
        let rendered = render(preprocessor.output());
        assert_eq!("\nvalue = \"Hi Mr. John\";\n", rendered.export());
    }
}
