%{%}
%token KW_NODE
%token KW_IF KW_ELSE KW_MATCH KW_DEFAULT 
%token KW_WHILE KW_FOR KW_BREAK KW_CONTINUE KW_IN
%token KW_MESSAGE KW_PACKET KW_DATUM KW_SEND KW_YEET KW_WHEN KW_REPLY KW_TO KW_DO 
%token KW_ENUM
%token KW_TIMER KW_CLOCK KW_SIGNAL
%token KW_LC_SELF KW_UC_SELF KW_INIT KW_FN
%token KW_AS KW_LET
%token PRIMITIVE_DTYPE
%token LIT_INT LIT_FLOAT LIT_STRING LIT_CHAR KW_TRUE KW_FALSE
%token IDENT ARROW IMPLIES
%token ORD_OP EQ_OP ASSIGN_OP


%left OR
%left AND
%left '>' '<'
%left '+' '-' 
%left '*' '/' '%' 
%left KW_AS 
%right '!' // Also unary minus, ref, and deref - see `expression` definition.
%left '[' '('
%left '.'



%%
P               : P Component
                | ;

Component       : Function 
                | Self 
                | Init 
                | Handler 
                | Event
                ;

Event           : event_kw IDENT expression;

event_kw        : KW_TIMER | KW_CLOCK | KW_SIGNAL;

Handler         : KW_WHEN IDENT KW_DO Block;

Block           : '{' Statements '}'; 

Function        : KW_FN IDENT '(' Params ')' RetType Block;

Self            : KW_UC_SELF DeclBlock;

DeclBlock       : '{' type_var_list '}';

Init            : KW_INIT Block;

message         : KW_MESSAGE IDENT DeclBlock;
datum           : KW_DATUM IDENT DeclBlock;
packet          : KW_PACKET IDENT DeclBlock;
enum            : KW_ENUM IDENT '{' ident_list '}';
ident_list      : ident_list ',' IDENT
                | IDENT
                ;
expression      : '(' expression ')' 
                | '-' expression
                | '!' expression
                | '-' expression
                | expression '.' IDENT
                | expression '.' LIT_INT
                | expression KW_AS type
                | expression '+' expression
                | expression '-' expression
                | expression '*' expression
                | expression '/' expression
                | expression '%' expression
                | expression '<' expression
                | expression '>' expression
                | expression ORD_OP expression
                | expression EQ_OP expression
                | expression AND expression
                | expression OR expression
                | IDENT
                | constant
                | KW_LC_SELF
                | tupl_value
                | array_access
                | array_decl
                ;

array_access    : expression '[' expression ']'  ;
array_decl      : '[' opt_expr_list ']';

opt_expr_list   : expr_list_
                | 
                ;

expr_list_      : expr_list_ ',' expression 
                | expression 
                ;




/* primitive_type  : KW_INT | KW_FLOAT | KW_STR | KW_BOOL; */
constant        : LIT_INT | LIT_FLOAT | LIT_STRING | LIT_CHAR | KW_TRUE | KW_FALSE ;
type            : PRIMITIVE_DTYPE
                | IDENT 
                | tuple // Tuple
                | '[' type ']' // Array
                | KW_UC_SELF
                ;

tuple           : '(' type ',' type_list ')' | '(' type ','')'

type_list       : type_list ',' type
                | type 
                ;

declaration     : KW_LET IDENT opt_rhs ';'
                | KW_LET type_var opt_rhs ';'
                ;

opt_rhs         : '=' expression
                | ;

type_var        : IDENT ':' type ;

decl_item       : type_var | type_var '=' expression ;

RetType         : ARROW type | ;

Params          : Params ',' type_var
                | type_var
                ;

Statements      : Statements Statement
                | Statement
                ;

Statement       : declaration | assignment ;
assignment      : expression assign_op expression ;
assign_op       : ASSIGN_OP | '=' ;


type_var_list   : type_var_list ',' type_var
                | type_var
                ;


tupl_value      : '(' tupl_value_list ')' 
                ;

OPT_COMMA       : ','
                | ;

tupl_value_list : expression ',' 
                | tupl_value_list_ OPT_COMMA 
                ;

tupl_value_list_: tupl_value_list_ ',' expression 
                | expression ',' expression 
                ;


conditional     : KW_IF '(' expression ')' if_body 
                ;

if_body         : Block
                | Block KW_ELSE conditional
                | Block KW_ELSE Block
                ;

loop_stmt       : KW_WHILE '(' loop_cond ')' Block
                | KW_FOR '(' 
                    assignment ';'
                    loop_cond ';'
                    assignment ')' 
                    Block
                | KW_FOR '(' KW_LET decl_item  ';' 
                    loop_cond ';'  
                    assignment ')' 
                    Block 
                | KW_FOR type_var KW_IN expression Block
                ;

/* Has to be bool */
loop_cond       : expression 
                ;


switch_case     : KW_MATCH '(' expression ')' sc_blocks sc_default 
                // DO NOT move the header into a separate non-terminal. Weird segfaults.
                ;
sc_default      : KW_DEFAULT IMPLIES Block
                | ;
case            : '('expression')' IMPLIES Block
                ;

sc_blocks       : sc_blocks case 
                | case 
                ; // NOTE: Does not cascade



%%