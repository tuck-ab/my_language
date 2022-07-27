#include <stdio.h>
#include <ctype.h>
#include <string.h>
#include <stdlib.h>

#include "lexer.h"

struct context {
    FILE *file_pointer;
    char last_char;
    char next_char;
};

struct token get_token(void *v_con) {
    struct token tok;
    struct context *con = (struct context *) v_con;

    while (isspace(con->last_char)) {
        con->last_char = getc(con->file_pointer);
    }

    if (isalpha(con->last_char)) {
        char buffer[21];
        int buffer_pointer = 0;

        while (isalpha(con->last_char) && buffer_pointer < 20) {
            buffer[buffer_pointer] = con->last_char;
            buffer_pointer++;

            con->last_char = getc(con->file_pointer);
        }

        buffer[buffer_pointer] = '\0';

        while (isalpha(con->last_char)) {
            printf("WARNING, variable name longer than 20 characters\n");
            con->last_char = getc(con->file_pointer);
        }

        if (strcmp(buffer, "IF") == 0) {
            tok.tok_type = IF;
            return tok;
        } else if (strcmp(buffer, "ELSEIF") == 0) {
            tok.tok_type = ELSEIF;
            return tok;
        } else if (strcmp(buffer, "ELSE") == 0) {
            tok.tok_type = ELSE;
            return tok;
        }  else if (strcmp(buffer, "REPEAT") == 0) {
            tok.tok_type = REPEAT;
            return tok;
        } else if (strcmp(buffer, "OUTPUT") == 0) {
            tok.tok_type = OUTPUT;
            return tok;
        } else {
            tok.tok_type = VAR;
            strcpy(tok.val, buffer);
            return tok;
        }
    }

    if (isdigit(con->last_char)) {
        char buffer[21];
        int buffer_pointer = 0;

        while (isdigit(con->last_char) && buffer_pointer < 20) {
            buffer[buffer_pointer] = con->last_char;
            buffer_pointer++;

            con->last_char = getc(con->file_pointer);
        }

        buffer[buffer_pointer] = '\0';

        while (isalpha(con->last_char)) {
            printf("WARNING, digit longer than 20 characters\n");
            con->last_char = getc(con->file_pointer);
        }

        tok.tok_type = INT_LIT;
        strcpy(tok.val, buffer);
        return tok;
    }
    
    if (con->last_char == '=') {
        con->next_char = getc(con->file_pointer);
        if (con->next_char == '=') {
            con->last_char = getc(con->file_pointer); // Consume the second = in ==
            tok.tok_type = EQ;
            return tok;
        } else {
            con->last_char = con->next_char;
            tok.tok_type = ASSIGN;
            return tok;
        }
    }

    if (con->last_char == '!') {
        con->next_char = getc(con->file_pointer);
        if (con->next_char == '=') {
            con->last_char = getc(con->file_pointer); // Consume the second = in !=
            tok.tok_type = NE;
            return tok;
        } else {
            con->last_char = con->next_char;
            tok.tok_type = INVALID; // Soon to be NOT
            return tok;
        }
    }

    if (con->last_char == '&') {
        con->next_char = getc(con->file_pointer);
        if (con->next_char == '&') {
            con->last_char = getc(con->file_pointer); // Consume the second & in &&
            tok.tok_type = AND;
            return tok;
        } else {
            con->last_char = con->next_char;
            tok.tok_type = INVALID;
            return tok;
        }
    }

    if (con->last_char == '|') {
        con->next_char = getc(con->file_pointer);
        if (con->next_char == '|') {
            con->last_char = getc(con->file_pointer); // Consume the second | in ||
            tok.tok_type = OR;
            return tok;
        } else {
            con->last_char = con->next_char;
            tok.tok_type = INVALID;
            return tok;
        }
    }

    if (con->last_char == '<') {
        con->next_char = getc(con->file_pointer);
        if (con->next_char == '=') {
            con->last_char = getc(con->file_pointer);
            tok.tok_type = LE;
            return tok;
        } else {
            con->last_char = con->next_char;
            tok.tok_type = LT;
            return tok;
        }
    }

    if (con->last_char == '>') {
        con->next_char = getc(con->file_pointer);
        if (con->next_char == '=') {
            con->last_char = getc(con->file_pointer);
            tok.tok_type = GE;
            return tok;
        } else {
            con->last_char = con->next_char;
            tok.tok_type = GT;
            return tok;
        }
    }

    if (con->last_char == '+') {
        con->last_char = getc(con->file_pointer); // Consume the +
        tok.tok_type = PLUS;
        return tok;
    }

    if (con->last_char == '-') {
        con->last_char = getc(con->file_pointer); // Consume the -
        tok.tok_type = MINUS;
        return tok;
    }

    if (con->last_char == '*') {
        con->last_char = getc(con->file_pointer); // Consume the *
        tok.tok_type = ASTERIX;
        return tok;
    }

    if (con->last_char == '/') {
        con->last_char = getc(con->file_pointer); // Consume the /
        tok.tok_type = DIV;
        return tok;
    }

    if (con->last_char == '%') {
        con->last_char = getc(con->file_pointer); // Consume the /
        tok.tok_type = MOD;
        return tok;
    }

    if (con->last_char == '(') {
        con->last_char = getc(con->file_pointer); // Consume the (
        tok.tok_type = LPAR;
        return tok;
    }

    if (con->last_char == ')') {
        con->last_char = getc(con->file_pointer); // Consume the )
        tok.tok_type = RPAR;
        return tok;
    }

    if (con->last_char == '{') {
        con->last_char = getc(con->file_pointer); // Consume the {
        tok.tok_type = LBRA;
        return tok;
    }

    if (con->last_char == '}') {
        con->last_char = getc(con->file_pointer); // Consume the }
        tok.tok_type = RBRA;
        return tok;
    }

    if (con->last_char == ';') {
        con->last_char = getc(con->file_pointer); // Consume the ;
        tok.tok_type = SC;
        return tok;
    }

    if (con->last_char == EOF) {
        tok.tok_type = EOF_TOK;
        return tok;
    }

    tok.tok_type = INVALID;
    return tok;
}

void * open_file(char *name) {
    FILE *file_pointer = fopen(name, "r");

    if (file_pointer != 0) {
        void *v_con = malloc(sizeof(struct context));

        struct context *con = (struct context *) v_con;
        con->file_pointer = file_pointer;
        con->last_char = ' ';
        con->next_char = ' ';        

        return v_con;
    }
    
    return 0;
}

void close_file(void *v_con) {
    struct context *con = (struct context *) v_con;
    fclose(con->file_pointer);
    free(con);
}