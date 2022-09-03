// ============================================================================
// Terms
// ============================================================================

#[derive(Clone)]
pub enum Term {
    // Statements
    Assert(Box<Term>),
    Assignment(Box<Term>,Box<Term>),
    IfGoto(Box<Term>,String),
    Label(String),
    // Expressions
    Binary(BinOp,Box<Term>,Box<Term>),
    ArrayAccess(Box<Term>,Box<Term>),
    MemoryAccess(Region),
    // Values
    Int(Vec<u8>),
    Hex(Vec<u8>),
}

// ============================================================================
// Binary Operators
// ============================================================================

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum BinOp {
    // Arithmetic
    Add,
    Subtract,
    Divide,
    Multiply,
    Remainder,
    // Comparators
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    // Logical
    LogicalAnd,
    LogicalOr
}

// ============================================================================
// Memory Regions
// ============================================================================

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum Region {
    Memory,
    Storage,
    CallData
}
