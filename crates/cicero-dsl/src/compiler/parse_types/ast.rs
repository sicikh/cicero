use crate::types::MarkdownString;

pub struct Struct {
    pub name: String,
    pub comment: Option<MarkdownString>,
    pub fields: Vec<Field>,
    pub parent: Option<String>,
    pub methods: Vec<Method>,
}

pub struct Method {
    pub name: String,
    pub body: Expr,
}

pub struct Field {
    pub name: String,
    pub comment: MarkdownString,
    pub ty: Type,
}

pub struct Type {
    pub name: String,
    pub required: bool,
}

pub struct Enum {
    pub name: String,
    pub comment: Option<MarkdownString>,
    pub variants: Vec<EnumVariant>,
    pub methods: Vec<Method>,
}

pub struct EnumVariant {
    pub name: String,
    pub comment: MarkdownString,
    pub field: Option<Type>,
}

pub struct Variable {
    pub name: String,
    pub comment: MarkdownString,
    pub ty: Type,
}

// TODO
pub enum Expr {
    Todo,
}
