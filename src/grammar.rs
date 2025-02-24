use std::collections::HashMap;

use num::Complex;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/minimal_complex_math.pest"]
pub struct MinimalComplexMathParser;

type ComplexMathContext = HashMap<&'static str, Complex<f64>>;

pub struct ComplexMath;

impl ComplexMath {
    pub fn calculate_expr(
        ctx: &ComplexMathContext,
        expression: &str,
    ) -> Result<Complex<f64>, Box<dyn std::error::Error>> {
        calculate_expr(expression);

        Ok(-Complex::i())
    }
}

fn walk_pairs(pair: Pair<Rule>, indent: usize) {
    // Create an indent string for pretty printing.
    let indent_str = "  ".repeat(indent);
    println!(
        "{}Rule: {:?} | Text: {:?}",
        indent_str,
        pair.as_rule(),
        pair.as_str()
    );

    // Recursively process all inner pairs.
    for inner_pair in pair.into_inner() {
        walk_pairs(inner_pair, indent + 1);
    }
}

fn calculate_expr(func: &str) {
    // Parse the input using the top-level rule 'expression'
    let parse_result = MinimalComplexMathParser::parse(Rule::expression, func)
        .unwrap_or_else(|e| panic!("Parsing error: {}", e));

    // Walk over each pair in the parse result.
    for pair in parse_result {
        walk_pairs(pair, 0);
    }
}
