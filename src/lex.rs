use crate::unicode_xid::UnicodeXID;
use crate::error::Error;


/*
AngleBracket   <>
Bracket        []
Paren          ()
Brace          {}
SingleQuote    ''
DoubleQuote    ""
Assign         =
Identifier     XidStart...XidContiue ...
*/

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Token {
    EndOfProgram,
    UnexpectedToken,

    // Assignment,

    // OpeningElementOpen, SelfClosingElementOpen
    ElementOpen,             // <
    // OpeningElementClose, ClosingElementClose
    ElementClose,            // >
    SelfClosingElementClose, // />
    ClosingElementOpen,      // </

    FragmentOpen,            // < >
    FragmentClose,           // </>

    Identifier,
    LiteralString,           // "abc..." | 'abc...'

    Assign,                  // =
    Colon,                   // :
    Dit,                     // .
    Comma,                   // ,
    Spread,                  // ...
    BraceOpen,               // {
    BraceClose,              // }
}

pub struct Lexer<'a> {
    code: &'a [char],
    /// Current `Token` from the source.
    pub token: Token,
    /// Current index
    index: usize,
    /// Position of current token in source
    token_start: usize,
    max_index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a [char]) -> Self {
        let token = if code.len() == 0 {
            Token::EndOfProgram
        } else {
            Token::UnexpectedToken
        };

        let code_len = code.len();
        Lexer {
            code: code,
            token: token,
            index: 0,
            token_start: 0,
            max_index: if code_len == 0 { 0 } else { code_len - 1 },
        }
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.token_start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn loc(&self) -> (usize, usize) {
        (self.start(), self.end())
    }

    #[inline]
    fn bump(&mut self) -> Result<(), Error> {
        if self.index < self.max_index {
            self.index += 1;
            Ok(())
        } else {
            Err(Error::EndOfProgram)
        }
    }

    #[inline]
    fn read_char(&self) -> char {
        self.code[self.index]
    }

    #[inline]
    fn next_char(&mut self) -> Result<char, Error> {
        self.bump()?;
        Ok(self.read_char())
    }

    #[inline]
    pub fn consume(&mut self) -> Result<(), Error> {
        // UnicodeXID::is_xid_start
        // UnicodeXID::is_xid_continue
        loop {
            if self.token == Token::EndOfProgram {
                return Err(Error::EndOfProgram);
            }

            let ch = self.read_char();
            match ch {
                '<' => {
                    self.token_start = self.index;
                    self.bump().map_err(|_| Error::UnexpectedEndOfProgram)?;

                    let index0 = self.index;
                    loop {
                        let c = self.read_char();
                        match c {
                            ' ' => {
                                self.bump().map_err(|_| Error::UnexpectedEndOfProgram)?;
                            },
                            '/' => {
                                self.token = Token::ClosingElementOpen;
                                self.bump().map_err(|_| Error::UnexpectedEndOfProgram)?;
                                
                                let index2 = self.index;
                                loop {
                                    let c = self.read_char();
                                    match c {
                                        ' ' => {
                                            self.bump().map_err(|_| Error::UnexpectedEndOfProgram)?;
                                        },
                                        '>' => {
                                            self.token = Token::FragmentClose;
                                            return self.bump();
                                        },
                                        _ => {
                                            self.index = index2;
                                            return Ok(());
                                        }
                                    }
                                }
                            },
                            '>' => {
                                self.token = Token::FragmentOpen;
                                return self.bump().map_err(|_| Error::UnexpectedEndOfProgram);
                            },
                            _ => {
                                self.token = Token::ElementOpen;
                                self.index = index0;
                                return Ok(());
                            }
                        }
                    }
                },
                '/' => {
                    self.token_start = self.index;
                    self.bump().map_err(|_| Error::UnexpectedEndOfProgram)?;

                    loop {
                        let c = self.read_char();
                        match c {
                            ' ' => {
                                self.bump().map_err(|_| Error::UnexpectedEndOfProgram)?;
                            },
                            '>' => {
                                self.token = Token::SelfClosingElementClose;
                                return self.bump();
                            },
                            _ => {
                                self.token = Token::UnexpectedToken;
                                return Err(Error::UnexpectedEndOfProgram);
                            }
                        }
                    }
                },
                '>' => {
                    self.token_start = self.index;
                    self.token = Token::ElementClose;

                    self.bump()?;
                    
                    return Ok(());
                },
                '=' => {
                    self.token_start = self.index;
                    self.token = Token::Assign;
                    return self.bump().map_err(|_| Error::UnexpectedEndOfProgram);
                },
                '"' | '\'' => {
                    self.token_start = self.index;
                    self.token = Token::LiteralString;

                    loop {
                        self.bump().map_err(|_| Error::UnexpectedEndOfProgram)?;
                        let c = self.read_char();
                        match c {
                            '\\' => {
                                continue;
                            },
                            _ => {
                                if c == ch {
                                    return self.bump();
                                } else {
                                    continue;
                                }
                            }
                        }
                    }
                },
                '{' => {
                    // { ... props }                Spread Attribute
                    // { AssignmentExpression }     Attribute Value
                    // { JSXChildExpression }       JSXChild
                    self.token_start = self.index;
                    self.token = Token::BraceOpen;
                    return self.bump().map_err(|_| Error::UnexpectedEndOfProgram);
                },
                '}' => {
                    self.token_start = self.index;
                    self.token = Token::BraceClose;
                    return self.bump();
                },
                ',' => {
                    self.token_start = self.index;
                    self.token = Token::Comma;
                    return self.bump().map_err(|_| Error::UnexpectedEndOfProgram);
                },
                '.' => {
                    self.token_start = self.index;
                    self.token = Token::Dit;

                    self.bump().map_err(|_| Error::UnexpectedEndOfProgram)?;

                    let index0 = self.index;

                    if self.read_char() == '.' {
                        self.bump().map_err(|_| Error::UnexpectedEndOfProgram)?;
                        if self.read_char() == '.' {
                            self.token = Token::Spread;
                            return self.bump().map_err(|_| Error::UnexpectedEndOfProgram);
                        } else {
                            self.token = Token::UnexpectedToken;
                            return Err(Error::UnexpectedEndOfProgram);
                        }
                    } else {
                        self.index = index0;
                        return Ok(());
                    }
                },
                ':' => {
                    self.token_start = self.index;
                    self.token = Token::Colon;

                    return self.bump().map_err(|_| Error::UnexpectedEndOfProgram);
                },
                _ => {
                    if UnicodeXID::is_xid_start(ch) {
                        self.token_start = self.index;
                        self.token = Token::Identifier;
                        loop {
                            let c = self.next_char()?;
                            if !UnicodeXID::is_xid_continue(c) {
                                return Ok(());
                            }
                        }
                    } else {
                        // Ignore
                        self.bump()?;
                    }
                }
            }
        }
    }

    #[inline]
    pub fn slice_source(&self, start: usize, end: usize) -> &[char] {
        assert_eq!(end >= start, true);
        &self.code[start..end]
    }
}


pub fn parse(source: &str) {
    println!("Parse:\n--------------\n{}\n------------\n", source);

    let code = source.chars().collect::<Vec<char>>();
    let mut lexer = Lexer::new(&code);

    loop {
        if let Err(e) = lexer.consume() {
            break;
        }

        assert_eq!(lexer.token != Token::UnexpectedToken, true);
        assert_eq!(lexer.token != Token::EndOfProgram, true);

        let loc = lexer.loc();
        let (start, end) = loc;
        println!("token: {:?} Postion: {:?} Text: {:?}", lexer.token, loc, &source[start..end]);
    }

    let loc = lexer.loc();
    let (start, end) = loc;
    println!("latest token: {:?} Postion: {:?} Text: {:?}", lexer.token, loc, &source[start..end]);

    println!("> end!");
}
