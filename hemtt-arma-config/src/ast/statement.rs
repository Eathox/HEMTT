use super::Node;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Config(Vec<Node<'a>>),
    Array(Vec<Node<'a>>),
    Float(f32),
    Integer(i32),
    Str(String),
    Bool(bool),
    Property {
        ident: Box<Node<'a>>,
        value: Box<Node<'a>>,
        expand: bool,
    },
    Class {
        ident: Box<Node<'a>>,
        extends: Option<Box<Node<'a>>>,
        props: Vec<Node<'a>>,
    },
    ClassDef(Box<Node<'a>>),
    ClassDelete(Box<Node<'a>>),
    Ident(String),
    IdentArray(String),

    Gone,
}
