use num_complex::Complex;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammars/minimal_complex_math.pest"]
pub struct MinimalComplexMathParser;

pub struct ComplexMath;

impl ComplexMath {
    pub fn calculate_expr(
        ctx: &mut ComplexMathContext,
        expression: &str,
    ) -> Result<Complex<f64>, Box<dyn std::error::Error>> {
        // Parse the input using the top-level rule (here: expression)
        let mut pairs = MinimalComplexMathParser::parse(Rule::expression, expression)?;
        // println!("Pairs: {:?}", pairs);

        let pair = pairs.next().unwrap();
        let expr = parse_expr(pair);

        // println!("Parsed expression: {:?}", expr);

        let result = eval_expr(&expr, ctx);
        Ok(result)
    }
}

/// Example AST for expressions
#[derive(Clone, Debug)]
pub enum Expr {
    Number(f64),
    NumberImag(f64),
    Var(String),
    BinaryOp(Box<Expr>, Op, Box<Expr>),
    UnaryOp(Op, Box<Expr>),
    FuncCall(String, Vec<Expr>),
    FuncDef(String, Vec<String>, Box<Expr>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow, // binary operator
    Pos,
    Neg, // unary operator
}

/// Context holding variables and functions
pub struct ComplexMathContext {
    pub vars: HashMap<String, Complex<f64>>,
    // Functions are stored as (parameter list, function body)
    pub funcs: HashMap<String, (Vec<String>, Box<Expr>)>,
}

impl ComplexMathContext {
    pub fn new() -> Self {
        let mut ctx = ComplexMathContext {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        };
        // Predefined constants
        ctx.vars
            .insert("pi".to_string(), Complex::new(std::f64::consts::PI, 0.0));
        ctx.vars
            .insert("e".to_string(), Complex::new(std::f64::consts::E, 0.0));
        ctx.vars.insert("i".to_string(), Complex::new(0.0, 1.0));

        ctx.funcs.insert(
            "sin".to_string(),
            (vec!["x".to_string()], Box::new(Expr::Number(0.0))),
        );

        ctx
    }
    pub fn get_var(&self, name: &str) -> Option<Complex<f64>> {
        self.vars.get(name).cloned()
    }
    pub fn set_var(&mut self, name: &str, value: Complex<f64>) {
        self.vars.insert(name.to_string(), value);
    }
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

/// Evaluates the AST and returns a Complex<f64>
fn eval_expr(expr: &Expr, context: &mut ComplexMathContext) -> Complex<f64> {
    println!("eval_expr: {:?}", expr);
    match expr {
        Expr::Number(val) => Complex::new(*val, 0.0),
        Expr::NumberImag(val) => Complex::new(0.0, *val),
        Expr::Var(name) => context
            .get_var(name)
            .unwrap_or_else(|| panic!("Undefined variable: {}", name)),
        Expr::BinaryOp(lhs, op, rhs) => {
            let left_val = eval_expr(lhs, context);
            let right_val = eval_expr(rhs, context);

            match op {
                Op::Add => left_val + right_val,
                Op::Sub => left_val - right_val,
                Op::Mul => left_val * right_val,
                Op::Div => left_val / right_val,
                Op::Pow => left_val.powc(right_val),
                _ => unreachable!(),
            }
        }
        Expr::UnaryOp(op, inner) => {
            let val = eval_expr(inner, context);

            match op {
                Op::Pos => val,
                Op::Neg => -val,
                _ => unreachable!(),
            }
        }
        Expr::FuncCall(name, args) => {
            // If a user-defined function exists, call it.
            if let Some((params, body)) = context.get_func(name) {
                let (params, body) = (params.clone(), body.clone());
                let mut arg_vals = Vec::new();
                for arg in args.iter() {
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
                let mut saved_vars = Vec::new();
                for (param, &value) in params.iter().zip(arg_vals.iter()) {
                    saved_vars.push((param.clone(), context.get_var(param)));
                    context.set_var(param, value);
                }
                let result = eval_expr(body.as_ref(), context);
                for (param, old_val) in saved_vars {
                    if let Some(val) = old_val {
                        context.set_var(&param, val);
                    } else {
                        context.vars.remove(&param);
                    }
                }
                result
            } else {
                // Otherwise, try built-in functions (all expecting one argument)
                if args.len() != 1 {
                    panic!("Built-in function {} takes exactly one argument", name);
                }
                let arg_val = eval_expr(&args[0], context);
                match name.as_str() {
                    "sin" => arg_val.sin(),
                    "cos" => arg_val.cos(),
                    "tan" => arg_val.tan(),
                    "exp" => arg_val.exp(),
                    "log" => arg_val.ln(),
                    _ => panic!("Unknown function: {}", name),
                }
            }
        }
        Expr::FuncDef(name, params, body_expr) => {
            context.set_func(name, params.clone(), body_expr.clone());
            Complex::new(0.0, 0.0)
        }
    }
}

/// Recursively converts a Pest parse tree into our AST.
fn parse_expr(pair: Pair<Rule>) -> Expr {
    // 2.2e*(-i*.2*z) + .4z^2
    println!("parse_expr: {:?}", pair);
    match pair.as_rule() {
        Rule::expression => {
            let inner = pair.into_inner().next().unwrap();

            // println!("Rule::expression {:?}", inner.as_rule());

            parse_expr(inner)
        }
        Rule::sum => {
            let mut inner = pair.into_inner();
            let mut expr = parse_expr(inner.next().unwrap());

            // println!("Rule::sum: {:?} {:?}", inner, expr);
            while let Some(op_pair) = inner.next() {
                let op = match op_pair.as_str().trim() {
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
            let mut expr = parse_expr(inner[0].clone());
            let mut i = 1;
            while i < inner.len() {
                let op = if inner[i].as_rule() == Rule::mul_op {
                    let op = match inner[i].as_str().trim() {
                        "*" => Op::Mul,
                        "/" => Op::Div,
                        _ => unreachable!(),
                    };
                    i += 1;
                    op
                } else {
                    // Implicit multiplication
                    Op::Mul
                };
                if i >= inner.len() {
                    panic!("Expected a factor after an operator, but found nothing");
                }
                let right = parse_expr(inner[i].clone());
                i += 1;

                // println!("Rule::product: {:?} {:?} {:?}", expr, op, right);

                if op == Op::Div && matches!(right, Expr::Number(0.0) | Expr::NumberImag(0.0)) {
                    expr = Expr::Number(f64::INFINITY);
                } else {
                    expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
                }
            }
            expr
        }
        Rule::power => {
            // Build right-associative exponentiation:
            let inner: Vec<_> = pair.into_inner().collect();
            if inner.is_empty() {
                panic!("Power rule with no inner nodes");
            }
            // inner[0] is the base; subsequent items alternate: exp_op, unary, exp_op, unary, ...
            let base = parse_expr(inner[0].clone());
            if inner.len() == 1 {
                base
            } else {
                // Build the chain from right to left.
                let mut expr = parse_expr(inner[inner.len() - 1].clone());
                let mut i = inner.len() - 2;
                while i > 0 {
                    let left = parse_expr(inner[i - 1].clone());

                    // println!("Rule::power: {:?} {:?} {:?}", left, Op::Pow, expr);

                    expr = Expr::BinaryOp(Box::new(left), Op::Pow, Box::new(expr));
                    if i < 2 {
                        break;
                    }
                    i -= 2;
                }
                expr
            }
        }
        Rule::unary => {
            let mut inner = pair.into_inner();
            let mut ops = Vec::new();
            while let Some(p) = inner.peek() {
                match p.as_rule() {
                    Rule::unary_op => {
                        let op = match p.as_str().trim() {
                            "-" => Op::Neg,
                            "+" => Op::Pos,
                            _ => unreachable!(),
                        };
                        ops.push(op);
                        inner.next();
                    }
                    _ => break,
                }
            }
            let mut expr = parse_expr(inner.next().unwrap());
            for op in ops.into_iter().rev() {
                // println!("Rule::unary: {:?} {:?}", op, expr);
                expr = Expr::UnaryOp(op, Box::new(expr));
            }
            expr
        }
        Rule::primary => {
            let inner = pair.into_inner().next().unwrap();

            // println!("Rule::primary {:?}", inner.as_rule());

            parse_expr(inner)
        }
        Rule::number => {
            let value = pair.as_str().trim().parse::<f64>().unwrap();

            // println!("Rule::number {:?}", value);

            Expr::Number(value)
        }
        Rule::ident => {
            // println!("Rule::ident {:?}", pair.as_str().trim());

            Expr::Var(pair.as_str().trim().to_string())
        }
        Rule::function_call => {
            let mut inner = pair.into_inner();
            let func_name = inner.next().unwrap().as_str().to_string();
            let args = if let Some(arg_list) = inner.next() {
                arg_list.into_inner().map(parse_expr).collect()
            } else {
                vec![]
            };

            // println!("Rule::function_call: {:?} {:?}", func_name, args);

            Expr::FuncCall(func_name, args)
        }
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

            // println!(
            //     "Rule::function_definition: {:?} {:?} {:?}",
            //     name, params, body
            // );

            Expr::FuncDef(name, params, body)
        }
        Rule::imag_literal => {
            let s: &str = pair.as_str().trim(); // e.g. "3i" or ".5i"
            let val = s.parse::<f64>().unwrap();

            Expr::NumberImag(val) // or something representing 0 + val*i
        }
        _ => unimplemented!("Not implemented for rule: {:?}", pair.as_rule()),
    }
}
