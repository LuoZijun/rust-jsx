use crate::error::Error;
use crate::lex::{ Token, Lexer, };
use crate::ast::{
    Loc, Node,

    ElementExpression, FragmentExpression,
    ElementName, MemberExpression, NamespacedName, 
    Attribute, NormalAttribute, NormalAttributeName, NormalAttributeInitializer,

    OpeningElement, ClosingElement, SelfClosingElement, 
    
    Child, 

    Text, ChildExpression, AssignmentExpression,
};


pub struct Parser<'a> {
    lexer: Lexer<'a>,
    pub body: Vec<Loc<Node>>,
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a [char]) -> Self {
        Parser {
            lexer: Lexer::new(code),
            body: Vec::new(),
        }
    }

    pub fn parse_assignment_expression(&mut self) -> Result<AssignmentExpression, Error> {
        assert_eq!(self.lexer.token, Token::BraceOpen);
        let start = self.lexer.end();
        
        loop {
             self.lexer.consume()?;

             if self.lexer.token == Token::BraceClose {
                break;
             }

             if self.lexer.token == Token::BraceOpen {
                loop {
                    self.lexer.consume()?;
                    if self.lexer.token == Token::BraceClose {
                        break;
                    }
                }
             }

             if self.lexer.token == Token::ElementOpen {
                unimplemented!()
             }

             if self.lexer.token == Token::FragmentOpen {
                unimplemented!()
             }
        }

        Ok(AssignmentExpression {
            start: start,
            end: self.lexer.start(),
        })
    }

    pub fn parse_elem_name(&mut self) -> Result<ElementName, Error> {
        // Name
        // Name:abc
        // Name.abc
        if self.lexer.token != Token::Identifier {
            return Err(Error::UnexpectedToken);
        }
        let ns = Loc::new(self.lexer.start(), self.lexer.end(), self.lexer.token);
        self.lexer.consume()?;

        let name: ElementName;
        match self.lexer.token {
            Token::Dit => {
                let mut members: Vec<Loc<Token>> = vec![ ns, ];

                loop {
                    self.lexer.consume().map_err(|_| Error::UnexpectedToken)?;
                    if self.lexer.token != Token::Identifier {
                        return Err(Error::UnexpectedToken);
                    }
                    
                    members.push(Loc::new(self.lexer.start(), self.lexer.end(), self.lexer.token));
                    self.lexer.consume()?;

                    if self.lexer.token == Token::Dit {
                        continue;
                    } else {
                        break;
                    }
                }

                let mem_expr = MemberExpression { members, };
                name = ElementName::MemberExpression(mem_expr);
            },
            Token::Colon => {
                self.lexer.consume()?;
                if self.lexer.token != Token::Identifier {
                    return Err(Error::UnexpectedToken);
                }
                let subname = Loc::new(self.lexer.start(), self.lexer.end(), self.lexer.token);
                name = ElementName::NamespacedName(NamespacedName { ns, name: subname });

                self.lexer.consume()?;
            },
            _ => {
                name = ElementName::Identifier(ns);
            }
        }

        Ok(name)
    }

    pub fn try_parse_jsx_opening_elem(&mut self) -> Result<(), Error> {
        // <App />
        // <App>
        assert_eq!(self.lexer.token, Token::ElementOpen);

        let (start, end) = self.lexer.loc();

        // Name
        self.lexer.consume()?;
        let name = self.parse_elem_name()?;

        // Attrs
        let mut attrs: Vec<Attribute> = Vec::new();
        loop {
            
            match self.lexer.token {
                Token::Identifier => {
                    // Normal Attribute
                    let (start, end) = self.lexer.loc();

                    let ns = Loc::new(self.lexer.start(), self.lexer.end(), self.lexer.token);

                    let attr_name: NormalAttributeName;
                    let mut attr_name_end = end;

                    self.lexer.consume()?;
                    if self.lexer.token == Token::Colon {
                        self.lexer.consume()?;
                        if self.lexer.token != Token::Identifier {
                            return Err(Error::UnexpectedToken);
                        }

                        let subname = Loc::new(self.lexer.start(), self.lexer.end(), self.lexer.token);
                        attr_name_end = subname.end;
                        attr_name = NormalAttributeName::NamespacedName(NamespacedName { ns: ns, name: subname });
                    } else {
                        attr_name = NormalAttributeName::Identifier(ns);
                    }

                    // Value
                    let mut init: Option<NormalAttributeInitializer> = None;
                    let mut init_end = attr_name_end;

                    if self.lexer.token == Token::Assign {
                        self.lexer.consume()?;
                        if self.lexer.token == Token::LiteralString {
                            init_end = self.lexer.end();
                            let initializer = NormalAttributeInitializer::LiteralString(Loc::new(self.lexer.start(), self.lexer.end(), self.lexer.token));
                            init = Some(initializer);
                        } else if self.lexer.token == Token::BraceOpen {
                            let start = self.lexer.end();
                            let assignment_expression = self.parse_assignment_expression()?;
                            init_end = self.lexer.end();
                            
                            let initializer = NormalAttributeInitializer::AssignmentExpression(Loc::new(start, init_end, assignment_expression));
                            init = Some(initializer);
                        } else if self.lexer.token == Token::ElementOpen {
                            unimplemented!()
                        } else if self.lexer.token == Token::FragmentOpen {
                            unimplemented!()
                        } else {
                            return Err(Error::UnexpectedToken);
                        }
                    }

                    let attr = NormalAttribute {
                        name: attr_name,
                        init: init,
                    };
                    attrs.push(Attribute::Normal(attr));
                },
                Token::BraceOpen => {
                    // Spread Attribute
                    let (start, end) = self.lexer.loc();

                    self.lexer.consume()?;
                    if self.lexer.token != Token::Spread {
                        return Err(Error::UnexpectedToken);
                    }

                    self.lexer.consume()?;
                    if self.lexer.token != Token::Identifier {
                        return Err(Error::UnexpectedToken);
                    }

                    let attr = Attribute::Spread(Loc::new(self.lexer.start(), self.lexer.end(), self.lexer.token));

                    self.lexer.consume()?;
                    if self.lexer.token != Token::BraceClose {
                        return Err(Error::UnexpectedToken);
                    }

                    attrs.push(attr);
                },
                _ => {
                    break;
                },
            }
        }

        self.lexer.consume()?;
        match self.lexer.token {
            Token::ElementClose => {
                let opening_elem = OpeningElement {
                    name: name,
                    attrs: attrs,
                };
                self.body.push(Loc::new(start, self.lexer.end(), Node::OpeningElement(opening_elem) ));
            },
            Token::SelfClosingElementClose => {
                let self_closing_elem = SelfClosingElement {
                    name: name,
                    attrs: attrs,
                };
                self.body.push(Loc::new(start, self.lexer.end(), Node::SelfClosingElement(self_closing_elem) ));
            },
            _ => {
                return Err(Error::UnexpectedToken);
            }
        }
        
        Ok(())
    }

    pub fn parse(&mut self) -> Result<(), Error> {
        loop {
            self.lexer.consume()?;

            match self.lexer.token {
                Token::FragmentOpen => {
                    let (start, end) = self.lexer.loc();
                    self.body.push(Loc::new(start, end, Node::OpeningFragment));
                },
                Token::FragmentClose => {
                    let (start, end) = self.lexer.loc();
                    self.body.push(Loc::new(start, end, Node::ClosingFragment));
                },
                Token::ElementOpen => {
                    // <
                    // <aaa>
                    // <aa />
                    let start = self.lexer.start();
                    let _ = self.try_parse_jsx_opening_elem();
                },
                Token::ClosingElementOpen => {
                    // </
                    // </App>
                    let (start, end) = self.lexer.loc();

                    self.lexer.consume()?;
                    let name = self.parse_elem_name()?;
                    let node = Node::ClosingElement( ClosingElement { name, });

                    let end = self.lexer.end();
                    self.body.push(Loc::new(start, end, node));
                },
                Token::EndOfProgram | Token::UnexpectedToken => {
                    // Should return error.
                    unreachable!();
                },
                _ => {
                    continue;
                }
            }
        }

        Ok(())
    }

    pub fn try_parse_jsx_fragment(&mut self) -> Result<FragmentExpression, Error> {
        // <> </>
        let (start, end) = self.lexer.loc();

        unimplemented!()
    }

    
}

pub fn parse(source: &str) {
    let code = source.chars().collect::<Vec<char>>();
    let mut parser = Parser::new(&code);
    
    match parser.parse() {
        Ok(_) | Err(Error::EndOfProgram) => {
            println!("{:?}", parser.body);
        },
        Err(e) => {
            println!("[ERROR] {:?} Loction: {:?}", e, parser.lexer.loc());
        }
    }
}