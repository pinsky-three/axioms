use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/minimal_complex_math.pest"]
pub struct MinimalComplexMathParser;

#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}
