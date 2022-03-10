use std::io::Write;

use pest::Parser;

mod node;
pub use node::Node;

mod statement;
pub use statement::Statement;

#[derive(Parser)]
#[grammar = "ast/config.pest"]
pub struct ConfigParser;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
/// Abstract Syntax Tree
pub struct AST<'a> {
    pub config: Node<'a>,
}

/// Converts a raw string into an AST
///
/// ```
/// let content = "value = 123;";
/// hemtt_arma_config::parse(content, "doc test");
/// ```
pub fn parse<'a>(source: &'a str, context: &str) -> Result<AST<'a>, String> {
    // let clean = source.replace("\r", "");
    let pair = ConfigParser::parse(Rule::file, source)
        .unwrap_or_else(|_| {
            let out = std::env::temp_dir().join("failed.txt");
            let mut f = std::fs::File::create(&out).expect("failed to create failed.txt");
            f.write_all(source.as_bytes()).unwrap();
            f.flush().unwrap();
            panic!(
                "failed to parse context: {}, saved at {}",
                context,
                out.display()
            )
        })
        .next()
        .unwrap();
    let config = Node::from_expr(
        std::env::current_dir().unwrap(),
        source,
        pair.into_inner().next().unwrap(),
    )?;
    Ok(AST { config })
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn property() {
        let ast = parse("value = 123;", "test").unwrap();
        println!("{:?}", ast.config);
    }
}
