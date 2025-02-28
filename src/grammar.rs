use num_complex::Complex;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammars/minimal_complex_math.pest"]
pub struct MinimalComplexMathParser;

pub struct ComplexMath;
// pub type ComplexMathContext = HashMap<&'static str, Complex<f64>>;

impl ComplexMath {
    pub fn calculate_expr(
        ctx: &mut ComplexMathContext,
        expression: &str,
    ) -> Result<Complex<f64>, Box<dyn std::error::Error>> {
        // Parse the input expression using the top-level rule
        let mut pairs = MinimalComplexMathParser::parse(Rule::expression, expression)?;
        let pair = pairs.next().unwrap();

        let expr = parse_expr(pair);

        let result = eval_expr(&expr, ctx);

        Ok(result)
    }
    // fn dump_tree(
    //     _ctx: &mut ComplexMathContext,
    //     value: &str,
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     let parse_result = MinimalComplexMathParser::parse(Rule::expression, value)
    //         .unwrap_or_else(|e| panic!("Parsing error: {}", e));

    //     for top_pair in parse_result.clone() {
    //         // Print the parse tree for debugging
    //         walk_pairs(top_pair.clone(), 0);

    //         // Then evaluate
    //         // let computed = eval_expr(top_pair, ctx);
    //         // println!("Computed result = {:?}", computed);
    //     }

    //     Ok(())
    // }
}

/// Example AST for expressions
#[derive(Clone)]
pub enum Expr {
    Number(f64),
    Var(String),
    BinaryOp(Box<Expr>, Op, Box<Expr>),
    UnaryOp(Op, Box<Expr>),
    FuncCall(String, Vec<Expr>),
    FuncDef(String, Vec<String>, Box<Expr>),
}

// impl Expr {
//     fn from(pair: Pair<'_, Rule>) -> Expr {
//         match pair.as_rule() {
//             Rule::expression => {

//             }
//         }
//     }
// }

#[derive(Copy, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow, // binary ops
    Pos,
    Neg, // unary ops (Pos for +, Neg for -)
}

/// Context holding variables and functions
pub struct ComplexMathContext {
    vars: HashMap<String, Complex<f64>>,
    // Note: we store the function body as a Box<Expr>
    funcs: HashMap<String, (Vec<String>, Box<Expr>)>,
}

impl ComplexMathContext {
    pub fn new() -> Self {
        let mut ctx = ComplexMathContext {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        };
        // Initialize common constants
        ctx.vars
            .insert("pi".to_string(), Complex::new(std::f64::consts::PI, 0.0));
        ctx.vars
            .insert("e".to_string(), Complex::new(std::f64::consts::E, 0.0));
        ctx.vars.insert("i".to_string(), Complex::new(0.0, 1.0));
        ctx
    }
    pub fn get_var(&self, name: &str) -> Option<Complex<f64>> {
        self.vars.get(name).cloned()
    }
    pub fn set_var(&mut self, name: &str, value: Complex<f64>) {
        self.vars.insert(name.to_string(), value);
    }
    // Change get_func to return the Boxed version.
    pub fn get_func(&self, name: &str) -> Option<&(Vec<String>, Box<Expr>)> {
        self.funcs.get(name)
    }
    pub fn set_func(&mut self, name: &str, params: Vec<String>, body: Box<Expr>) {
        self.funcs.insert(name.to_string(), (params, body));
    }
}

impl Default for ComplexMathContext {
    fn default() -> Self {
        Self::new()
    }
}

fn eval_expr(expr: &Expr, context: &mut ComplexMathContext) -> Complex<f64> {
    match expr {
        // Numeric literal -> convert to Complex (real part set, imag = 0)
        Expr::Number(val) => Complex::new(*val, 0.0),

        // Variable or constant -> look up in context
        Expr::Var(name) => context.get_var(name).unwrap_or_else(|| {
            panic!("Undefined variable: {}", name);
        }),

        // Binary operations: evaluate both sides, then apply
        Expr::BinaryOp(lhs, op, rhs) => {
            let left_val = eval_expr(lhs, context);
            let right_val = eval_expr(rhs, context);
            match op {
                Op::Add => left_val + right_val,
                Op::Sub => left_val - right_val,
                Op::Mul => left_val * right_val,
                Op::Div => left_val / right_val,
                Op::Pow => {
                    // Exponentiation: handle real vs complex exponent
                    let base = left_val;
                    let exp = right_val;

                    if exp.im == 0.0 {
                        // exponent is real
                        if exp.re.fract() == 0.0 {
                            base.powi(exp.re as i32)
                        } else {
                            base.powf(exp.re)
                        }
                    } else {
                        base.powc(exp)
                    }
                }
                _ => unreachable!(),
            }
        }

        // Unary operations: evaluate inner expression then apply
        Expr::UnaryOp(op, inner) => {
            let val = eval_expr(inner, context);
            match op {
                Op::Pos => val,
                Op::Neg => -val,
                _ => unreachable!(),
            }
        }

        // Function call: evaluate arguments, substitute into function body
        Expr::FuncCall(name, arg_exprs) => {
            // Clone the function definition so we don't hold a borrow on context.
            let (params, body) = context
                .get_func(name)
                .unwrap_or_else(|| panic!("Undefined function: {}", name))
                .clone();
            // Evaluate each argument using a for loop.
            let mut arg_vals = Vec::new();
            for arg in arg_exprs.iter() {
                arg_vals.push(eval_expr(arg, context));
            }
            if arg_vals.len() != params.len() {
                panic!(
                    "Function {} expected {} arguments, got {}",
                    name,
                    params.len(),
                    arg_vals.len()
                );
            }
            // Save any existing values for these parameter names (to restore later)
            let mut saved_vars: Vec<(String, Option<Complex<f64>>)> = Vec::new();
            for (param, &value) in params.iter().zip(arg_vals.iter()) {
                saved_vars.push((param.clone(), context.get_var(param)));
                context.set_var(param, value);
            }
            // Evaluate the function body. Use body.as_ref() to get &Expr.
            let result = eval_expr(body.as_ref(), context);
            // Restore previous variable values (or remove the param from context)
            for (param, old_val) in saved_vars {
                if let Some(val) = old_val {
                    context.set_var(&param, val);
                } else {
                    context.vars.remove(&param);
                }
            }
            result
        }

        // Function definition: store in context and return 0 (or you could return the body eval)
        Expr::FuncDef(name, params, body_expr) => {
            // Now simply pass the boxed expression.
            context.set_func(name, params.clone(), body_expr.clone());
            Complex::new(0.0, 0.0)
        }
    }
}

// A helper function for debugging purposes: recursively prints the parse tree.
pub fn walk_pairs(pair: Pair<Rule>, indent: usize) {
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

fn parse_expr(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::expression => {
            // Assuming your top-level rule “expression” just delegates to sum.
            let inner = pair.into_inner().next().unwrap();
            parse_expr(inner)
        }
        Rule::sum => {
            // Build a left-associative tree for additions/subtractions.
            let mut inner = pair.into_inner();
            let mut expr = parse_expr(inner.next().unwrap());
            while let Some(op_pair) = inner.next() {
                let op = match op_pair.as_str() {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    _ => unreachable!(),
                };
                let right = parse_expr(inner.next().unwrap());
                expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
            }
            expr
        }
        Rule::product => {
            let inner: Vec<_> = pair.into_inner().collect();
            // The first element is always the first 'power'
            let mut expr = parse_expr(inner[0].clone());
            let mut i = 1;
            while i < inner.len() {
                // If the current element is an explicit multiplication operator:
                let op = if inner[i].as_rule() == Rule::mul_op {
                    let op = match inner[i].as_str() {
                        "*" => Op::Mul,
                        "/" => Op::Div,
                        _ => unreachable!(),
                    };
                    i += 1;
                    op
                } else {
                    // No operator means implicit multiplication
                    Op::Mul
                };
                // Ensure there is a right-hand side factor
                if i >= inner.len() {
                    panic!("Expected a factor after an operator, but found nothing");
                }
                let right = parse_expr(inner[i].clone());
                i += 1;
                expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
            }
            expr
        }

        Rule::power => {
            // Right-associative exponentiation.
            let mut inner = pair.into_inner();
            let base = parse_expr(inner.next().unwrap());
            if let Some(exp_pair) = inner.next() {
                let exponent = parse_expr(exp_pair);
                Expr::BinaryOp(Box::new(base), Op::Pow, Box::new(exponent))
            } else {
                base
            }
        }
        Rule::unary => {
            // If there is a leading "-" (or "+"), build a UnaryOp.
            let mut inner = pair.into_inner();
            let mut ops = Vec::new();
            // Collect any unary operators.
            while let Some(p) = inner.peek() {
                match p.as_rule() {
                    Rule::unary_op => {
                        let op = match p.as_str() {
                            "-" => Op::Neg,
                            "+" => Op::Pos,
                            _ => unreachable!(),
                        };
                        ops.push(op);
                        inner.next(); // consume the op
                    }
                    _ => break,
                }
            }
            let mut expr = parse_expr(inner.next().unwrap());
            // Apply the collected unary operators in reverse order.
            for op in ops.into_iter().rev() {
                expr = Expr::UnaryOp(op, Box::new(expr));
            }
            expr
        }
        Rule::primary => {
            // primary can be a number, a function call, a parenthesized expression, or an ident.
            let inner = pair.into_inner().next().unwrap();
            parse_expr(inner)
        }
        Rule::number => {
            let value = pair.as_str().parse::<f64>().unwrap();
            Expr::Number(value)
        }
        Rule::ident => Expr::Var(pair.as_str().to_string()),
        Rule::function_call => {
            let mut inner = pair.into_inner();
            let func_name = inner.next().unwrap().as_str().to_string();
            let args = if let Some(arg_list) = inner.next() {
                arg_list.into_inner().map(parse_expr).collect()
            } else {
                vec![]
            };
            Expr::FuncCall(func_name, args)
        }
        // Extend to handle function definitions if your grammar produces them.
        Rule::function_definition => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let params = if let Some(param_list) = inner.next() {
                param_list
                    .into_inner()
                    .map(|p| p.as_str().to_string())
                    .collect()
            } else {
                vec![]
            };
            let body = Box::new(parse_expr(inner.next().unwrap()));
            Expr::FuncDef(name, params, body)
        }
        _ => unimplemented!("Not implemented for rule: {:?}", pair.as_rule()),
    }
}
