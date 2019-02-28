use crate::error::Error;
use crate::lex::{ Token, Lexer, };


// React.createElement()
// React.createElement(React.Fragment)
pub static REACT_TARGET: (&'static str, &'static str) = ("React.createElement", "React.Fragment");

/*

React.createElement("div", { ...props, ...props2, e: 3, }, null);
React.createElement(App, { ...props, ...props2, e: 3, }, null);

React.createElement("App",
                    { ...props, ...props2, e: 3, },
                    React.createElement("App", { ...props, ...props2, e: 3, }, null),
);

React.createElement("App",
                    { ...props, ...props2, e: 3, },
                    [
                        React.createElement("App", { ...props, ...props2, e: 3, }, null)
                    ],
);

*/

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Node {
    pub start: usize,
    pub end: usize,
    pub token: Token,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Leaf {
    Token(Node),
    Element(Element),
    Fragment(Fragment),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Element {
    pub nodes: Vec<Leaf>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Fragment {
    pub nodes: Vec<Leaf>,
}



pub fn handle_fragment(lexer: &mut Lexer) -> Option<Fragment> {
    assert_eq!(lexer.token, Token::FragmentOpen);

    let mut nodes: Vec<Leaf> = Vec::new();

    nodes.push( Leaf::Token(Node { start: lexer.start(), end: lexer.end(), token: lexer.token }) );

    loop {
        if let Err(e) = lexer.consume() {
            return None;
        }

    }

    Some( Fragment {
        nodes: nodes
    })
}

pub fn handle_elem(lexer: &mut Lexer) -> Option<Element> {
    assert_eq!(lexer.token, Token::ElementOpen);

    let mut nodes: Vec<Leaf> = Vec::new();

    nodes.push( Leaf::Token(Node { start: lexer.start(), end: lexer.end(), token: lexer.token }) );

    // Name
    lexer.consume().ok()?;
    if lexer.token != Token::Identifier {
        return None;
    }

    // Attrs
    lexer.consume().ok()?;
    

    Some( Element {
        nodes: nodes
    })
}

pub fn transform(input: &str, output: &mut String) {
    let code = input.chars().collect::<Vec<char>>();
    let mut lexer = Lexer::new(&code);

    let mut tree: Vec<Leaf> = Vec::new();

    println!("parse ...");
    loop {
        match lexer.consume() {
            Ok(_) => {
                assert!(lexer.token != Token::EndOfProgram);
                assert!(lexer.token != Token::UnexpectedToken);

                match lexer.token {
                    Token::FragmentOpen => {
                        if let Some(fragment) = handle_fragment(&mut lexer) {
                            tree.push(Leaf::Fragment(fragment));
                        }
                    },
                    Token::ElementOpen => {
                        if let Some(elem) = handle_elem(&mut lexer) {
                            tree.push(Leaf::Element(elem));
                        }
                    },
                    Token::EndOfProgram | Token::UnexpectedToken => {
                        // Should return error.
                        unreachable!();
                    },
                    _ => {
                        continue;
                    }
                }
            },
            Err(Error::EndOfProgram) => {
                break;
            },
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }

    println!("transform ...");
    for leaf in tree {
        
    }
}

