#![allow(unused_imports, unused_variables, unused_mut, unreachable_code, 
    unused_assignments, dead_code)]


extern crate unicode_xid;



pub mod error;
pub mod lex;
pub mod ast;
pub mod parser;
pub mod transform;



use crate::error::Error;

use unicode_xid::UnicodeXID;
use std::io::{ self, Read, Seek, SeekFrom, Cursor, };

// https://facebook.github.io/jsx/


fn debug(source: &str, (start, end): (usize, usize), token: lex::Token) {
    let mut line_break: usize = end;
    while &source[line_break..line_break+1] != "\n" && line_break < source.len() - 1 {
        line_break += 1;
    }
    println!("{}", &source[..line_break]);
    println!("token: {:?} {:?}", token, &source[start..end]);
    println!("{}", &source[line_break..]);
}


pub fn compile(code: &str) {

}


fn main() {
    let source = r#"
<App c={ true }> </App>

"#;
    // lex::parse(source);
    parser::parse(source);
}
