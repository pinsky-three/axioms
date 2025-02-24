use num::Complex;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammars/minimal_complex_math.pest"]
pub struct MinimalComplexMathParser;

// Context mapping variable names (e.g. "pi", "e", "z", "i") to their Complex<f64> values.
pub type ComplexMathContext = HashMap<&'static str, Complex<f64>>;

pub struct ComplexMath;

impl ComplexMath {
    pub fn calculate_expr(
        ctx: &ComplexMathContext,
        expression: &str,
    ) -> Result<Complex<f64>, Box<dyn std::error::Error>> {
        // Parse the input expression using the top-level rule
        let mut pairs = MinimalComplexMathParser::parse(Rule::expression, expression)?;
        let pair = pairs.next().unwrap();
        // Evaluate the expression recursively
        let result = eval_expr(pair, ctx)?;
        Ok(result)
    }
    pub fn dump_tree(
        ctx: &ComplexMathContext,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let parse_result = MinimalComplexMathParser::parse(Rule::expression, value)
            .unwrap_or_else(|e| panic!("Parsing error: {}", e));

        for top_pair in parse_result.clone() {
            // Print the parse tree for debugging
            walk_pairs(top_pair.clone(), 0);

            // Then evaluate
            let computed = eval_expr(top_pair, ctx)?;
            println!("Computed result = {:?}", computed);
        }

        Ok(())
    }
}

/// Recursively evaluates a parsed expression returning a Complex<f64> result.
fn eval_expr(
    pair: Pair<Rule>,
    ctx: &ComplexMathContext,
) -> Result<Complex<f64>, Box<dyn std::error::Error>> {
    match pair.as_rule() {
        // The top-level expression simply contains a sum
        Rule::expression => {
            let inner = pair.into_inner().next().unwrap();
            eval_expr(inner, ctx)
        }
        // Sum: product+
        Rule::sum => {
            let inner: Vec<_> = pair.into_inner().collect();
            let mut result = eval_expr(inner[0].clone(), ctx)?;
            for prod in inner.into_iter().skip(1) {
                result += eval_expr(prod, ctx)?;
            }
            Ok(result)
        }
        // Product: power ( (optional "*" ) power )*
        Rule::product => {
            let mut inner_pairs = pair.into_inner();
            // Evaluate the first factor (a power)
            let mut result = eval_expr(inner_pairs.next().unwrap(), ctx)?;
            while let Some(next_pair) = inner_pairs.next() {
                if next_pair.as_rule() == Rule::power {
                    // Implicit multiplication
                    let val = eval_expr(next_pair, ctx)?;
                    result *= val;
                } else {
                    // If it's not a power, assume it's the "*" token.
                    // Then we expect the next pair to be the actual power factor.
                    assert_eq!(next_pair.as_str(), "*", "Unexpected parse structure");
                    let power_pair = inner_pairs
                        .next()
                        .expect("Expected a power after `*` but found nothing");
                    let val = eval_expr(power_pair, ctx)?;
                    result *= val;
                }
            }
            Ok(result)
        }
        // Power: primary ("^" power)?
        Rule::power => {
            let mut inner_pairs = pair.into_inner();
            let base = eval_expr(inner_pairs.next().unwrap(), ctx)?;
            if let Some(exp_pair) = inner_pairs.next() {
                let exponent = eval_expr(exp_pair, ctx)?;
                // Compute exponentiation using complex powc
                Ok(base.powc(exponent))
            } else {
                Ok(base)
            }
        }
        // Primary: either a float, int, variable or a parenthesized expression.
        Rule::primary => {
            // When using parenthesis, the inner expression is wrapped inside primary.
            let mut inner = pair.into_inner();
            if let Some(first) = inner.next() {
                eval_expr(first, ctx)
            } else {
                Err("Empty primary expression".into())
            }
        }
        // Float: a decimal number parsed as f64.
        Rule::float => {
            let num_str = pair.as_str();
            let value: f64 = num_str.parse()?;
            Ok(Complex::new(value, 0.0))
        }
        // Int: an integer parsed as i64 and converted to f64.
        Rule::int => {
            let num_str = pair.as_str();
            let value: i64 = num_str.parse()?;
            Ok(Complex::new(value as f64, 0.0))
        }
        // Variable: look up the variable in the provided context.
        Rule::variable => {
            let var_name = pair.as_str();
            ctx.get(var_name)
                .copied()
                .ok_or_else(|| format!("Undefined variable: {}", var_name).into())
        }
        _ => Err(format!("Unhandled rule: {:?}", pair.as_rule()).into()),
    }
}

// A helper function for debugging purposes: recursively prints the parse tree.
fn walk_pairs(pair: Pair<Rule>, indent: usize) {
    let indent_str = "  ".repeat(indent);
    println!(
        "{}Rule: {:?} | Text: {:?}",
        indent_str,
        pair.as_rule(),
        pair.as_str()
    );

    for inner_pair in pair.into_inner() {
        walk_pairs(inner_pair, indent + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::Complex;
    use std::f64::consts::PI;

    #[test]
    fn test_expression() {
        // Define the context: set values for some common constants and variables.
        let mut ctx: ComplexMathContext = HashMap::new();
        ctx.insert("pi", Complex::new(PI, 0.0));
        ctx.insert("e", Complex::new(std::f64::consts::E, 0.0));
        // For example, let "z" and "i" be some predefined numbers (or i could be the imaginary unit).
        ctx.insert("z", Complex::new(2.0, 0.0));
        // Here we assume "i" represents the imaginary unit.
        ctx.insert("i", Complex::new(0.0, 1.0));

        // Test a couple of expressions.
        let expr1 = "-0.3z^2 + 1.2e^(.4*pi*i)";
        let result1 = ComplexMath::calculate_expr(&ctx, expr1).unwrap();
        println!("Expression: {}\nResult: {:?}", expr1, result1);

        // You can add more tests and assert on expected values.
    }
}
