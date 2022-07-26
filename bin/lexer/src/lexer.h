// The lexer returns one of these for known things.
enum TOKEN_TYPE {

  VAR,        // [a-zA-Z_][a-zA-Z_0-9]*
  ASSIGN, // '='

  // delimiters
  LBRA,  // left brace
  RBRA,  // right brace
  LPAR,  // left parenthesis
  RPAR,  // right parenthesis
  SC,    // semicolon
//   COMMA = (int)',', // comma

  // types
//   INT_TOK = -2,   // "int"
//   VOID_TOK = -3,  // "void"
//   FLOAT_TOK = -4, // "float"
//   BOOL_TOK = -5,  // "bool"

  // keywords
  REPEAT,  // "repeat"
  IF,      // "if"
  ELSE,    // "else"
  ELSEIF,
//   WHILE = -9,   // "while"
//   RETURN = -10, // "return"
  // TRUE   = -12,     // "true"
  // FALSE   = -13,     // "false"
  OUTPUT,

  // literals
  INT_LIT,   // [0-9]+
//   FLOAT_LIT = -15, // [0-9]+.[0-9]+
//   BOOL_LIT = -16,  // "true" or "false" key words

//   // logical operators
   AND, // "&&"
   OR,  // "||"

//   // operators
  PLUS,    // addition or unary plus
  MINUS,   // substraction or unary negative
  ASTERIX, // multiplication
  DIV,     // division
  MOD,     // modular
//   NOT = (int)'!',     // unary negation

//   // comparison operators
  EQ,      // equal
  NE,      // not equal
  LE,      // less than or equal to
  LT, // less than
  GE,      // greater than or equal to
  GT, // greater than

  // special tokens
  EOF_TOK, // signal end of file

  // invalid
  INVALID // signal invalid token
};

int char_num = 0;

struct token {
    int tok_type;
    char val[21];
};

struct token get_token(void *v_con);
void * open_file(char *name);
void close_file(void *v_con);