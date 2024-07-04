%option yylineno
%option noyywrap
%{
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>

    // debugging
    extern FILE * token_stream;

    void seq_token(const char * type) {
        fprintf(token_stream, "%d %s: %s\n", yylineno, type, yytext);
        fflush(token_stream);
    }

%}



dig [0-9]
num (-?{dig}+)
real -?{dig}+\.{dig}+
id [[:alpha:]_][[:alnum:]_]*
str_c \"([^"]|(\\\"))*\"
char_c \'([^\\']|\\.)\'

ord_op (<=|>=)
eq_op (==|!=)
arith_op (\+|\-|\*|\/|\%)
assign_op {arith_op}=
punct [,;:]
/* misc_ops (@|=|&) */
surround [\(\)\{\}\[\]]
arrow ->
implies =>

/* uint (u8|u16|u32|u64)
sint (i8|i16|i32|i64) */

pdt (int|float|bool|char|str)

/* op (>|<|!|{arith_op}|\.|{misc_ops}) */

short_comment \/\/.*
long_comment_inner [^*]|\**[^/]
long_comment \/\*({long_comment_inner})*\*\/
comment {short_comment}|{long_comment}
ws [\r\n\t ]

%%

{ws}+ { /* ignore whitespace */ }
{comment} { /* ignore comments */ }

{char_c}    { 
                seq_token("literal"); 
                yylval.lit_char = yytext[1]; 
                return LIT_CHAR; 
            }
{str_c}     {
                seq_token("literal"); 
                yylval.lit_str = new std::string(yytext);
                return LIT_STR; 
            }
{num}       { 
                seq_token("literal"); 
                yylval.lit_int = atoi(yytext);
                return LIT_INT; 
            }
{real}      { 
                seq_token("literal"); 
                yylval.lit_float = atof(yytext);
                return LIT_FLOAT; 
            }

/* Reserve Keyword Territory */

node        {return KW_NODE;}

/* Conditionals */
if          {return KW_IF;}
else        {return KW_ELSE;}
match       {return KW_MATCH;}
default     {return KW_DEFAULT;}

/* Loops */
while       {return KW_WHILE;}
for         {return KW_FOR;}
break       {return KW_BREAK;}
continue    {return KW_CONTINUE;}
in          {return KW_IN;}

/* Messages */
message     {return KW_MESSAGE;}
packet      {return KW_PACKET;}
datum       {return KW_DATUM;}

enum        {return KW_ENUM;}

/* TCP */
send        {return KW_SEND;}
/* UDP */
yeet        {return KW_YEET;}
/* target */
to          {return KW_TO;}
/* shorcut */
reply       {return KW_REPLY;}

/* Handler */
when        {return KW_WHEN;}
do          {return KW_DO;}

/* Events */
timer       {return KW_TIMES;}
signal      {return KW_SIGNAL;}
clock       {return KW_CLOCK;}

/* Functions */
fn          {return KW_FN;}
return      {return KW_RETURN;}
init        {return KW_INIT;}

/* Types */
as          {return KW_AS;}
{pdt}       {return PRIMITIVE_DTYPE;}


self        {return KW_LC_SELF;}
Self        {return KW_UC_SELF;}

/* Operators */

{arrow}     {return ARROW;}
{implies}   {return IMPLIES;}
{eq_op}     {return EQ_OP;}
{ord_op}    {return ORD_OP;}
{punct}     {return yytext[0];}
{surround}  {return yytext[0];}
{arith_op}  {return yytext[0];}
{assign_op} {return ASSIGN_OP;}

{id}        {return IDENT;}
%%

