#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "maps", derive(serde::Serialize))]
pub enum Keyword {
    Class,
    Delete,
    Enum,
}

impl Keyword {
    pub fn size(&self) -> usize {
        match self {
            Keyword::Class => 5,
            Keyword::Delete => 6,
            Keyword::Enum => 4,
        }
    }
}

impl ToString for Keyword {
    fn to_string(&self) -> String {
        match self {
            Keyword::Class => "class",
            Keyword::Delete => "delete",
            Keyword::Enum => "enum",
        }
        .to_string()
    }
}
