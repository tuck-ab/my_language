#[cfg(test)]
mod tests;

use std::ffi::CString;
use std::os::raw::{c_char, c_void};

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TOKEN_TYPE {
    VAR,
    ASSIGN,
    LBRA,
    RBRA,
    LPAR, 
    RPAR, 
    SC,  
    REPEAT,
    IF,
    ELSE,
    ELSEIF,
    OUTPUT,
    INT_LIT,
    AND,
    OR,
    PLUS,
    MINUS,
    ASTERIX,
    DIV,
    MOD,
    EQ,
    NEQ,
    LE, 
    LT,
    GE,
    GT, 
    EOF_TOK,
    INVALID
}

#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct token {
    pub tok_type: TOKEN_TYPE,
    pub val: [c_char; 21],
}

extern "C" {
    pub fn get_token(con: *const c_void) -> token;
    pub fn open_file(name: CString) -> *const c_void;
    pub fn close_file(con: *const c_void);
}

pub fn get_token_safe(con: *const c_void) -> token {
    let tok: token;
    unsafe {
        tok = get_token(con);
    }

    return tok;
}

pub fn open_file_safe(filename: &str) -> Option<*const c_void> {
    let worked: *const c_void;
    unsafe {
        worked = open_file(CString::new(filename).unwrap());
    }

    if worked.is_null() {
        return None;
    } else {
        return Some(worked);
    }
}

pub fn close_file_safe(con: *const c_void) {
    unsafe {
        close_file(con);
    }
}

pub fn val_to_str(val: &[c_char; 21]) -> Option<String> {
    let mut result = String::new();
    for c in val {
        let my_char = *c as u8 as char;
        if my_char == '\0' {
            return Some(result);
        }

        result.push(my_char);
    }

    return None;
}