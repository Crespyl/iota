use buffer::{ Direction, Mark };

#[deriving(Copy, Show)]
#[allow(dead_code)]
pub enum Reference {
    Index(Kind, uint),          // Absolute buffer index, "nth char/word/line/etc."
    Offset(Mark, Kind, int)     // Relative buffer index, "nth char/word/line/etc. from cursor"
}

#[deriving(Copy, Show)]
#[allow(dead_code)]
pub enum Kind {
    Char,
    Line,

    Word,
    Sentence,
    Paragraph,

    Expression,
    Statement,
    Block,
}

#[deriving(Copy, Show)]
#[allow(dead_code)]
pub enum Anchor {
    Before,
    Start,
    Middle,
    End,
    After
}

#[deriving(Copy, Show)]
pub struct TextObject {
    pub anchor: Anchor,
    pub reference: Reference
}
