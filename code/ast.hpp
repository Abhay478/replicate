#include <stdio.h>
#include <string>
#include <vector>

enum class ASTNodeType {
    Program,
    Function,
    WhenBlock,
    IfBlock,
    ElseBlock,
    ElseIfBlock,
    WhileBlock,
    ForBlock,

    Return,
    Break,
    Continue,

    VariableDeclaration,
    VariableAssignment,
    FunctionCall,
    
    BinaryOperator,
    UnaryOperator,
    Literal,
    Identifier,
    ArrayAccess,
    ArrayLiteral,
    Type,
    Import,
};

struct ASTNode {

};

struct AST {

};