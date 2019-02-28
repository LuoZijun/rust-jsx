use crate::error::Error;
use crate::lex::{ Token, Lexer, };
use crate::ast::{
    Loc, Node,

    ElementExpression, FragmentExpression,
    ElementName, MemberExpression, NamespacedName, 
    Attribute, NormalAttribute, NormalAttributeName, NormalAttributeInitializer,

    OpeningOrSelfClosingElement,
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

    #[inline]
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
                // TODO: 
                let _ = self.parse_elem()?;

             }

             if self.lexer.token == Token::FragmentOpen {
                // TODO:
                let _ = self.parse_fragment()?;
             }
        }

        Ok(AssignmentExpression {
            start: start,
            end: self.lexer.start(),
        })
    }

    #[inline]
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

    #[inline]
    pub fn parse_elem_attr_name(&mut self) -> Result<NormalAttributeName, Error> {
        // displayName
        // displayName:subname

        let (start, end) = self.lexer.loc();
        if self.lexer.token != Token::Identifier {
            return Err(Error::UnexpectedToken);
        }

        let ns = Loc::new(self.lexer.start(), self.lexer.end(), self.lexer.token);
        self.lexer.consume()?;

        let name: NormalAttributeName;
        match self.lexer.token {
            Token::Colon => {
                self.lexer.consume()?;
                if self.lexer.token != Token::Identifier {
                    return Err(Error::UnexpectedToken);
                }

                let subname = Loc::new(self.lexer.start(), self.lexer.end(), self.lexer.token);
                name = NormalAttributeName::NamespacedName(NamespacedName { ns, name: subname });

                self.lexer.consume()?;
            },
            _ => {
                name = NormalAttributeName::Identifier(ns);
            }
        }

        Ok(name)
    }

    #[inline]
    pub fn parse_elem_attr_value(&mut self) -> Result<Option<NormalAttributeInitializer>, Error> {
        if self.lexer.token != Token::Assign {
            return Ok(None)
        }

        self.lexer.consume()?;

        match self.lexer.token {
            Token::LiteralString => {
                let (start, end) = self.lexer.loc();
                let initializer = NormalAttributeInitializer::LiteralString(Loc::new(start, end, self.lexer.token));

                self.lexer.consume()?;

                Ok(Some(initializer))
            },
            Token::BraceOpen => {
                let start = self.lexer.end();
                let assignment_expression = self.parse_assignment_expression()?;
                let (_, end) = self.lexer.loc();

                let initializer = NormalAttributeInitializer::AssignmentExpression(Loc::new(start, end, assignment_expression));

                self.lexer.consume()?;

                Ok(Some(initializer))
            },
            Token::ElementOpen => {
                let start = self.lexer.end();
                let elem = self.parse_elem()?;
                let (_, end) = self.lexer.loc();

                let initializer = NormalAttributeInitializer::ElementExpression(Loc::new(start, end, elem));

                self.lexer.consume()?;

                Ok(Some(initializer))
            },
            Token::FragmentOpen => {
                let start = self.lexer.end();
                let fragment_elem = self.parse_fragment()?;
                let (_, end) = self.lexer.loc();

                let initializer = NormalAttributeInitializer::FragmentExpression(Loc::new(start, end, fragment_elem));

                self.lexer.consume()?;

                Ok(Some(initializer))

            },
            _ => Err(Error::UnexpectedToken),
        }
    }

    #[inline]
    pub fn parse_elem_attr(&mut self) -> Result<Option<Attribute>, Error> {
        // { ...props }
        // displayName="value"
        // displayName={ true }
        // displayName=<></>
        // displayName=<App />
        // displayName=<App></App>
        match self.lexer.token {
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

                self.lexer.consume()?;


                Ok(Some(attr))
            },
            Token::Identifier => {
                // Normal Attribute
                let (start, end) = self.lexer.loc();
                
                let name = self.parse_elem_attr_name()?;
                let init = self.parse_elem_attr_value()?;

                let attr = NormalAttribute {
                    name: name,
                    init: init,
                };

                Ok(Some(Attribute::Normal(attr)))
            },
            _ => {
                Ok(None)
            },
        }
    }

    #[inline]
    pub fn parse_opening_or_self_closing_elem(&mut self) -> Result<OpeningOrSelfClosingElement, Error> {
        // <App />
        // <App>
        if self.lexer.token != Token::ElementOpen {
            return Err(Error::UnexpectedToken);
        }

        let (start, end) = self.lexer.loc();

        // Name
        self.lexer.consume()?;
        let name = self.parse_elem_name()?;

        // Attrs
        let mut attrs: Vec<Attribute> = Vec::new();
        loop {
            if let Some(attr) = self.parse_elem_attr()? {
                attrs.push(attr);
            } else {
                break;
            }
        }

        match self.lexer.token {
            Token::ElementClose => {
                Ok(OpeningOrSelfClosingElement::Opening((name,attrs )))
            },
            Token::SelfClosingElementClose => {
                Ok(OpeningOrSelfClosingElement::SelfClosing((name,attrs )))
            },
            _ => {
                Err(Error::UnexpectedToken)
            }
        }
    }

    #[inline]
    pub fn parse_closing_elem(&mut self) -> Result<ClosingElement, Error> {
        if self.lexer.token != Token::ClosingElementOpen {
            return Err(Error::UnexpectedToken);
        }
    
        self.lexer.consume()?;
        let name = self.parse_elem_name()?;

        if self.lexer.token != Token::ElementClose {
            return Err(Error::UnexpectedToken);
        }

        Ok(ClosingElement {
            name: name,
        })
    }

    #[inline]
    pub fn parse_elem(&mut self) -> Result<ElementExpression, Error> {
        if self.lexer.token != Token::ElementOpen {
            return Err(Error::UnexpectedToken);
        }

        let opening_or_self_closing_elem = self.parse_opening_or_self_closing_elem()?;

        match opening_or_self_closing_elem {
            OpeningOrSelfClosingElement::Opening((name, attrs)) => {
                // jsx children
                let children = self.parse_children()?;

                // jsx ClosingElement
                let closing_elem = self.parse_closing_elem()?;
                let name2 = closing_elem.name;

                let is_name_eq = match name {
                    ElementName::Identifier(loc_token) => {
                        match name2 {
                            ElementName::Identifier(loc_token2) => {
                                let a = self.lexer.slice_source(loc_token.start, loc_token.end);
                                let b = self.lexer.slice_source(loc_token2.start, loc_token2.end);
                                a == b
                            },
                            _ => { false }
                        }
                    },
                    ElementName::NamespacedName(ref name_spaced_name) => {
                        match name2 {
                            ElementName::NamespacedName(name_spaced_name2) => {
                                let a = self.lexer.slice_source(name_spaced_name.ns.start, name_spaced_name.ns.end);
                                let b = self.lexer.slice_source(name_spaced_name2.ns.start, name_spaced_name2.ns.end);

                                let c = self.lexer.slice_source(name_spaced_name.name.start, name_spaced_name.name.end);
                                let d = self.lexer.slice_source(name_spaced_name2.name.start, name_spaced_name2.name.end);

                                a == b && c == d
                            },
                            _ => { false }
                        }
                    },
                    ElementName::MemberExpression(ref member_expr) => {
                        match name2 {
                            ElementName::MemberExpression(member_expr2) => {
                                let a = member_expr.members.iter()
                                    .map(|loc_token| {
                                        self.lexer.slice_source(loc_token.start, loc_token.end)
                                    });
                                let b = member_expr2.members.iter()
                                    .map(|loc_token| {
                                        self.lexer.slice_source(loc_token.start, loc_token.end)
                                    });
                                a.zip(b).map(|(c, d)| {
                                    c == d
                                }).all(|ret| ret == true )
                            },
                            _ => { false }
                        }
                    },
                };

                if is_name_eq == false {
                    return Err(Error::UnexpectedToken);
                }

                let elem = ElementExpression {
                    is_self_closing: false,
                    name: name,
                    attrs: attrs,
                    children: Some(children),
                };

                Ok(elem)
            },
            OpeningOrSelfClosingElement::SelfClosing((name, attrs)) => {
                let elem = ElementExpression {
                    is_self_closing: true,
                    name: name,
                    attrs: attrs,
                    children: None,
                };

                Ok(elem)
            },
        }
    }

    #[inline]
    pub fn parse_children(&mut self) -> Result<Vec<Child>, Error> {
        // JSXText
        // JSXElement
        // { JSXChildExpression }
        assert_eq!(self.lexer.token == Token::ElementClose || self.lexer.token == Token::FragmentOpen, true);

        let mut children: Vec<Child> = Vec::new();

        let start = self.lexer.end();

        let mut text_child: Text = Text { start: self.lexer.end(), end: self.lexer.end() };

        loop {
            self.lexer.consume()?;
            match self.lexer.token {
                Token::ElementOpen => {
                    text_child.end = self.lexer.start();
                    if text_child.end > text_child.start {
                        children.push(Child::Text( Loc::new(text_child.start, text_child.end, text_child) ));
                    }

                    let elem = self.parse_elem()?;

                    children.push(Child::Element( elem ));

                    text_child.start = self.lexer.end();
                    text_child.end = self.lexer.end();
                },
                Token::BraceOpen => {
                    text_child.end = self.lexer.start();
                    if text_child.end > text_child.start {
                        children.push(Child::Text( Loc::new(text_child.start, text_child.end, text_child) ));
                    }

                    let assignment_expression = self.parse_assignment_expression()?;
                    children.push(Child::ChildExpression( assignment_expression ));

                    text_child.start = self.lexer.end();
                    text_child.end = self.lexer.end();
                },
                Token::EndOfProgram | Token::UnexpectedToken => {
                    unreachable!();
                },
                Token::ClosingElementOpen | Token::FragmentClose => {
                    break;
                },
                _ => {

                }
            }
        }

        Ok(children)
    }

    #[inline]
    pub fn parse_fragment(&mut self) -> Result<FragmentExpression, Error> {
        // <> </>
        if self.lexer.token != Token::FragmentOpen {
            return Err(Error::UnexpectedToken);
        }

        let (start, end) = self.lexer.loc();

        // jsx children
        let children = self.parse_children()?;

        if self.lexer.token != Token::FragmentClose {
            return Err(Error::UnexpectedToken);
        }

        Ok(FragmentExpression {
            children: children,
        })
    }

    pub fn parse(&mut self) -> Result<(), Error> {
        loop {
            self.lexer.consume()?;

            match self.lexer.token {
                Token::FragmentOpen => {
                    let (start, end) = self.lexer.loc();
                    let fragment_elem = self.parse_fragment()?;

                    let node = Node::Fragment(fragment_elem);
                    self.body.push(Loc::new(start, self.lexer.end(), node));
                },
                Token::ElementOpen => {
                    // <
                    // <aaa>
                    // <aa />
                    let start = self.lexer.start();
                    let elem = self.parse_elem()?;

                    let node = Node::Element(elem);
                    self.body.push(Loc::new(start, self.lexer.end(), node));
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
}


fn debug(source: &[char], (start, end): (usize, usize)) {
    let mut line_break: usize = end;
    while &source[line_break..line_break+1] != ['\n'] && line_break < source.len() - 1 {
        line_break += 1;
    }

    for c in &source[..line_break] {
        print!("{}", c);
    }
    print!("\n\n\n");

    // println!("{}", &source[..line_break]);
    // println!("token: {:?} {:?}", token, &source[start..end]);
    // println!("{}", &source[line_break..]);
}

pub fn parse(source: &str) {
    let code = source.chars().collect::<Vec<char>>();
    let mut parser = Parser::new(&code);
    
    match parser.parse() {
        Ok(_) | Err(Error::EndOfProgram) => {
            println!("{:?}", parser.body);
        },
        Err(e) => {
            let (start, end) = parser.lexer.loc();

            debug(&code, (start, end));

            println!("latest token: {:?}", parser.lexer.token);
            println!("{:?}({}:{}):", e, start, end);
        }
    }

    std::thread::sleep(std::time::Duration::new(10, 0));
    print!("len: {:?}", parser.body.len());
}