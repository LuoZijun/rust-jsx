use crate::lex::Token;

use std::ops::Deref;
use std::collections::HashMap;


#[derive(Debug, PartialEq, Clone)]
pub enum Token2Kind {
    Identifier,
    LiteralString,
    Fragment,
    Element,
    SelfClosingElement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token2 {
    pub start: usize,
    pub end: usize,
    pub kind: Token2Kind,
}



#[derive(Debug, Eq, Clone)]
pub struct Loc<T> {
    pub start: usize,
    pub end: usize,
    pub item: T,
}

impl<T: Copy> Copy for Loc<T> {}

impl<T> Deref for Loc<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.item
    }
}

impl<T> Loc<T> {
    #[inline]
    pub fn new(start: usize, end: usize, item: T) -> Self {
        Loc {
            start,
            end,
            item,
        }
    }
}

impl<T: PartialEq> PartialEq for Loc<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.item.eq(&other.item)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    OpeningFragment,
    ClosingFragment,
    OpeningElement(OpeningElement),
    ClosingElement(ClosingElement),
    SelfClosingElement(SelfClosingElement),
}

impl Node {
    pub fn new<T: Into<Node>>(start: usize, end: usize, item: T) -> Self {
        unimplemented!()
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct MemberExpression {
    pub members: Vec<Loc<Token>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamespacedName {
    pub ns: Loc<Token>,
    pub name: Loc<Token>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ElementName {
    Identifier(Loc<Token>),
    NamespacedName(NamespacedName),
    MemberExpression(MemberExpression),
}


#[derive(Debug, PartialEq, Clone)]
pub enum NormalAttributeName {
    Identifier(Loc<Token>),
    NamespacedName(NamespacedName),
}

#[derive(Debug, PartialEq, Clone)]
pub enum NormalAttributeInitializer {
    LiteralString(Loc<Token>),
    AssignmentExpression(Loc<AssignmentExpression>),
    ElementExpression(Loc<ElementExpression>),
    FragmentExpression(Loc<FragmentExpression>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct NormalAttribute {
    pub name: NormalAttributeName,
    pub init: Option<NormalAttributeInitializer>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Attribute {
    Normal(NormalAttribute),
    Spread(Loc<Token>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct OpeningElement {
    pub name: ElementName,
    pub attrs: Vec<Attribute>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClosingElement {
    pub name: ElementName,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SelfClosingElement {
    pub name: ElementName,
    pub attrs: Vec<Attribute>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct Text {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChildExpression {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Child {
    Text(Loc<Text>),
    Element(Loc<ElementExpression>),
    ChildExpression(Vec<Loc<ChildExpression>>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FragmentExpression {
    pub children: Vec<Loc<Child>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElementExpression {
    pub is_self_closing: bool,
    pub name: Loc<Token>,
    pub attrs: Vec<Attribute>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentExpression {
    pub start: usize,
    pub end: usize,
}
