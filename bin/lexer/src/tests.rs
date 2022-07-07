use std::any::Any;

use crate::*;

#[test]
fn token_recognission_test() {
    let mut tok: token;
    let context = open_file_safe("test/tokentest.xa");
    // Test whether it can open the file
    assert_ne!(context, None);

    // ELSE hello = == IF ELSEIF + - () {} ;
    let tokens = [
        TOKEN_TYPE::ELSE, 
        TOKEN_TYPE::VAR, 
        TOKEN_TYPE::ASSIGN, 
        TOKEN_TYPE::EQ, 
        TOKEN_TYPE::IF, 
        TOKEN_TYPE::ELSEIF,
        TOKEN_TYPE::PLUS,
        TOKEN_TYPE::MINUS,
        TOKEN_TYPE::LPAR,
        TOKEN_TYPE::RPAR,
        TOKEN_TYPE::LBRA,
        TOKEN_TYPE::RBRA,
        TOKEN_TYPE::SC,
        TOKEN_TYPE::INT_LIT,
        TOKEN_TYPE::INT_LIT,
        TOKEN_TYPE::OUTPUT,
        TOKEN_TYPE::EOF_TOK
    ];

    for tok_type in tokens {
        tok = get_token_safe(context.unwrap());
        assert_eq!(tok.tok_type, tok_type);
    }

    unsafe{
        close_file(context.unwrap());
    }
    
}

#[test]
fn var_lit_val_test() {
    let mut tok: token;
    let context = open_file_safe("test/varlittest.xa");
    // Test whether it can open the file
    assert_ne!(context, None);

    let vals = [
        "hello",
        "world",
        "542",
        "323"
    ];

    for val in vals {
        tok = get_token_safe(context.unwrap());

        let tok_val = val_to_str(&tok.val).unwrap();
        
        assert_eq!(tok_val, val);
    }

    unsafe{
        close_file(context.unwrap());
    }
}