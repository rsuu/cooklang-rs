use crate::syntax::tokens::{Pos, TokenType, Tokens};

//pub type Text = Vec<String>;
//pub type Steps = Vec<Step>;

pub type Nodes = Vec<Node>;
// e.g. >> course: dinner
#[derive(Debug, Clone, PartialEq)]
pub struct ItemMetadata {
    pub name: String,
    pub body: String,
}

// e.g. text @test text
// e.g. text @test test123{1%g} text
#[derive(Debug, Clone, PartialEq)]
pub struct ItemIngredients {
    pub body: Pos,
    pub quantity: f64,
    pub unit: Pos,
    pub full: Pos,
}

// e.g. text #test text
// e.g. text #test test2{1%g} text
#[derive(Debug, Clone, PartialEq)]
pub struct ItemCookware {
    pub body: Pos,
    pub quantity: f64,
    pub unit: Pos,
    pub full: Pos,
}

// e.g. text ~{3%m} text
// e.g. text ~test test2{3%m} text
#[derive(Debug, Clone, PartialEq)]
pub struct ItemTimer {
    pub body: Pos,
    pub quantity: f64,
    pub unit: Pos,
    pub full: Pos,
}

// e.g. text -- comment 1 2
// e.g. text [- comment -] text
#[derive(Debug, Clone, PartialEq)]
pub struct ItemComments {
    pub body: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemText {
    pub body: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemSign {
    At,
    NewLine,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    ItemMetadata(ItemMetadata),
    ItemIngredients(ItemIngredients),
    ItemCookware(ItemCookware),
    ItemTimer(ItemTimer),
    ItemComments(ItemComments),
    ItemText(ItemText),
    ItemSign(ItemSign),
}

pub trait Change {
    fn new() -> Self;
    fn start(&mut self, start: usize);
    fn end(&mut self, end: usize);
    fn set_quantity(&mut self, n: f64);
    fn set_unit(&mut self, start: usize, end: usize);
    fn set_unit_start(&mut self, start: usize);
    fn set_unit_end(&mut self, end: usize);
    fn set_full(&mut self, start: usize, end: usize);
    fn set_full_start(&mut self, start: usize);
    fn set_full_end(&mut self, end: usize);
}

macro_rules! gen {
    ($($t:ty)*) => ($(
        impl Change for $t {

    fn start(&mut self, start: usize) {
        self.body.start = start;
    }

    fn end(&mut self, end: usize) {
        self.body.end = end;
    }

    fn set_quantity(&mut self, n: f64) {
        self.quantity = n;
    }

    fn set_unit(&mut self, start:usize,end:usize) {
        self.unit.start = start;
        self.unit.end = end;
    }

    fn set_unit_start(&mut self, start:usize) {
        self.unit.start = start;
    }

    fn set_unit_end(&mut self, end:usize) {
        self.unit.end = end;
    }

    fn set_full(&mut self, start:usize,end:usize) {
        self.full.start = start;
        self.full.end = end;
    }

    fn set_full_start(&mut self, start:usize) {
        self.full.start = start;
    }

    fn set_full_end(&mut self, end:usize) {
        self.full.end = end;
    }

  fn new() -> $t{
                Self {
        body: Pos::new(),
        quantity: 0.0,
        unit: Pos::new(),
        full: Pos::new(),
                }
    }




        }
    )*)
}

gen! {
    ItemCookware
    ItemIngredients
    ItemTimer
}

impl ItemMetadata {
    pub fn as_node(self) -> Node {
        Node::ItemMetadata(self)
    }
}
impl ItemIngredients {
    pub fn as_node(self) -> Node {
        Node::ItemIngredients(self)
    }
}
impl ItemCookware {
    pub fn as_node(self) -> Node {
        Node::ItemCookware(self)
    }
}
impl ItemTimer {
    pub fn as_node(self) -> Node {
        Node::ItemTimer(self)
    }
}
impl ItemComments {
    pub fn as_node(self) -> Node {
        Node::ItemComments(self)
    }
}
impl ItemText {
    pub fn as_node(self) -> Node {
        Node::ItemText(self)
    }
}
impl ItemSign {
    pub fn as_node(self) -> Node {
        Node::ItemSign(self)
    }
}
