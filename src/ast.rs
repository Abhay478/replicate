#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator<'input> {
    FieldAccess,
    EnumAccess,
    TupleAccess,
    TypeCast,
    Call,
    Index,
    /// +, -, *, /, %
    Arithmetic(&'input str),
    /// Unary minus
    Negate,
    /// &&, ||, !
    Logical(&'input str),
    /// ==, !=
    Eq(&'input str),
    /// <, <=, >, >=
    Ord(&'input str),
    /// x=
    AssignOp(&'input str),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopTypes {
    For,
    While,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JumpTypes {
    Break,
    Continue,
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionalTypes {
    If,
    ElseIf,
    Else,
    IfLadder,
    Match,
    Case,
    CaseList,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InnerType<'input> {
    Int,
    Float,
    Bool,
    String,
    Vec,
    Tuple,
    Struct(StructTypes, &'input str),
    Enum,
    SelfRef,
    Dummy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type<'input> {
    pub root: InnerType<'input>,
    /// Empty for primitives, single type for Vec, multiple types for Tuple
    pub children: Vec<Box<Type<'input>>>,
}

impl<'input> Type<'input> {
    pub fn new(root: InnerType<'input>) -> Self {
        Type {
            root,
            children: Vec::new(),
        }
    }

    pub fn dummy() -> Self {
        Type {
            root: InnerType::Dummy,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: Type<'input>) {
        self.children.push(Box::new(child));
    }

    pub fn add_children(&mut self, children: Vec<Type<'input>>) {
        self.children
            .extend(children.into_iter().map(|x| Box::new(x)));
    }
}

/// Reasoning behind why there's three of them:
/// 
/// - We don't want to expose Rust's alias-xor-mut rule here, we infer how something should be borrowed from the context.
/// - We have two uses for a struct - one for sending messages, and one for storing data. 
/// - Packets are those structs which can act as both. I might remove them if I don't think they're useful. For now, they're here
/// just in case you absolutely need them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructTypes {
    Message,
    Datum,
    Packet,
    /// We don't know which one it'll be just from the AST. 
    /// We don't have a declare before definition rule, so it could come later, so we use a placeholder.
    Unknown 
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatementTypes {
    Jump(JumpTypes),
    Declaration,
    Assignment,
    FunctionCall,
    Send,
    Dummy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Timer,
    Signal,
    Clock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendAction {
    Send,
    Yeet,
    Reply,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Importable<'input> {
    Item(&'input str),
    Wildcard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportPath<'input> {
    pub path: Vec<Importable<'input>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASTNodeType<'input> {
    /// Root type. Has a bunch of nodes, messages, functions, etc. as children.
    Network,
    /// Has child Fields, and maybe child Methods
    Node(&'input str),

    /// Children: Body
    Init,
    /// Has children FunctionHeader, Body
    Function,
    /// Has children Fields, Type
    FunctionHeader(&'input str),
    /// Children: Vec<Identifier>
    /// Contains function name
    FunctionCall(&'input str),
    /// Children: FunctionHeader, Type
    MethodHeader,
    /// Children: Expression, FunctionCall
    /// Expressions is of type struct/enum
    MethodCall,

    /// Standard Type Tree inside, no AST children.
    Type(Type<'input>),
    Identifier(&'input str),
    SelfRef,
    Literal(&'input str),
    Variant(&'input str),
    Expression(Operator<'input>, Type<'input>),

    /// Children: Identifier and Type
    TypeVar,
    /// Children: Identifier and Expression
    ExprVar,

    Loop(LoopTypes),
    LoopHeader(LoopTypes),

    Conditional(ConditionalTypes),
    /// Children: Expression
    /// For if and else if, will be parent of the expression to evaluate
    /// For else, blank
    /// For match, will be parent of the expression to evaluate
    /// For case, will be parent of the expression to match
    ConditionalHeader(ConditionalTypes),

    Struct(StructTypes, &'input str),
    /// Children: Vec<TypeVar>
    Fields,
    /// Children: Vec<Expression>
    Constructor(&'input str),

    Send(SendAction),

    /// Children: EnumVariants, maybe Methods.
    Enum(&'input str),
    EnumVariants(Vec<&'input str>),

    /// Children: Expression
    Event(EventType, &'input str),

    /// Children: Body
    Handler(&'input str),

    Import(ImportPath<'input>),

    Statement(StatementTypes),
    Body,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASTNode<'input> {
    typ: ASTNodeType<'input>,
    children: Vec<Box<ASTNode<'input>>>,
}

impl<'input> ASTNode<'input> {
    pub fn new(typ: ASTNodeType<'input>) -> Self {
        ASTNode {
            typ,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: ASTNode<'input>) {
        self.children.push(Box::new(child));
    }

    pub fn add_children(&mut self, children: Vec<ASTNode<'input>>) {
        self.children
            .extend(children.into_iter().map(|x| Box::new(x)));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AST<'input> {
    pub root: ASTNode<'input>,
}
