#![allow(unused_imports, unused_variables, unused_mut, unreachable_code, 
    unused_assignments, dead_code)]


extern crate unicode_xid;

use unicode_xid::UnicodeXID;


pub mod error;
pub mod lex;
pub mod ast;
pub mod parser;
pub mod transform;


use crate::error::Error;


use std::io::{ self, Read, Seek, SeekFrom, Cursor, };


// https://facebook.github.io/jsx/


fn main() {
    let source = r#"

<>
    <App name="str" num={ true } >
        <div height={16}> hi </div>
    </App>
</>

<div style={styles.items} child=<h1>hi</h1> >
    {
        [].map(function (){
            <div style={styles.item} key={index}>
                <span style={styles.splitLine} />
                <p style={styles.title}>{item.title}</p>
              </div>
        })
    }
  
</div>

"#;
    // lex::parse(source);
    parser::parse(source);
}
