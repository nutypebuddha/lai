//! # NAND Expression Compiler — compile formulas to NAND DAGs
//!
//! Takes a typed expression string and compiles it into a NAND DAG,
//! then evaluates by propagating values through the graph.
//!
//! ## Supported Expressions
//!
//! - `nand(a, b)` — the single primitive
//! - `not(a)` — derived: `nand(a, a)`
//! - `and(a, b)` — derived: `not(nand(a, b))`
//! - `or(a, b)` — derived: `nand(not(a), not(b))`
//! - `nor(a, b)` — derived: `not(or(a, b))`
//! - `xor(a, b)` — derived: `or(and(a, not(b)), and(not(a), b))`
//! - `xnor(a, b)` — derived: `not(xor(a, b))`
//! - `implies(a, b)` — derived: `or(not(a), b)`
//!
//! For math expressions beyond logic, see the separate `meval`-based
//! evaluator in `bankai/eval.rs`. This module focuses on logic gate
//! compilation and evaluation.

use std::collections::HashMap;

use super::NandDag;

/// A parsed expression node in the AST.
#[derive(Debug, Clone, PartialEq)]
pub enum ExprNode {
    /// A named variable reference (e.g., "a", "mass")
    Var(String),
    /// A constant numeric value (e.g., 0.5, 2.0)
    Const(f64),
    /// A NAND gate application: nand(left, right)
    Nand(Box<ExprNode>, Box<ExprNode>),
}

/// Error type for expression parsing and compilation.
#[derive(Debug, Clone, PartialEq)]
pub enum NandExprError {
    /// Invalid or unrecognized syntax
    ParseError(String),
    /// Missing variable during evaluation
    MissingVar(String),
    /// DAG compilation failure
    CompileError(String),
}

impl std::fmt::Display for NandExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NandExprError::ParseError(msg) => write!(f, "parse error: {msg}"),
            NandExprError::MissingVar(name) => write!(f, "missing variable: {name}"),
            NandExprError::CompileError(msg) => write!(f, "compile error: {msg}"),
        }
    }
}

impl std::error::Error for NandExprError {}

/// A compiled NAND expression ready for evaluation.
#[derive(Debug, Clone)]
pub struct NandExpression {
    dag: NandDag,
    /// Names of input variables (in order of addition to DAG)
    input_names: Vec<String>,
}

impl NandExpression {
    /// Parse and compile a NAND expression string into a DAG.
    ///
    /// Supported syntax:
    /// - `nand(a, b)` — NAND gate
    /// - `not(a)` — NOT gate (compiles to nand(a, a))
    /// - `and(a, b)`, `or(a, b)`, `nor(a, b)` — derived gates
    /// - `xor(a, b)`, `xnor(a, b)` — derived gates
    /// - `implies(a, b)` — implication
    ///
    /// Variables can be any alphanumeric identifier.
    /// Constants are floating-point numbers.
    pub fn compile(expr_str: &str) -> Result<Self, NandExprError> {
        let trimmed = expr_str.trim();
        if trimmed.is_empty() {
            return Err(NandExprError::ParseError("empty expression".to_string()));
        }

        // Tokenize the expression into tokens
        let tokens = tokenize(trimmed)?;

        // Parse tokens into AST
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expression()?;

        // Compile AST to DAG
        let mut dag = NandDag::new();
        let mut var_nodes: HashMap<String, usize> = HashMap::new();
        let mut input_names: Vec<String> = Vec::new();

        let _root_idx = compile_ast(&ast, &mut dag, &mut var_nodes, &mut input_names)?;

        if dag.is_empty() {
            return Err(NandExprError::CompileError("empty DAG".to_string()));
        }

        Ok(NandExpression { dag, input_names })
    }

    /// Evaluate the compiled expression with the given variable bindings.
    ///
    /// Returns the computed output value.
    pub fn evaluate(&self, inputs: &HashMap<String, f64>) -> Result<f64, NandExprError> {
        self.dag.evaluate(inputs).ok_or_else(|| {
            // Find which input is missing
            for name in &self.input_names {
                if !inputs.contains_key(name) {
                    return NandExprError::MissingVar(name.clone());
                }
            }
            NandExprError::eval_error("unknown evaluation failure")
        })
    }

    /// Number of NAND gates in the compiled DAG.
    pub fn nand_count(&self) -> usize {
        self.dag.nand_count()
    }

    /// Total number of nodes in the compiled DAG.
    pub fn node_count(&self) -> usize {
        self.dag.node_count()
    }
}

impl NandExprError {
    /// Create an evaluation error.
    fn eval_error(msg: impl Into<String>) -> Self {
        NandExprError::CompileError(msg.into())
    }
}

// ─── Tokenizer ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Ident(String),
    Number(f64),
    LParen,
    RParen,
    Comma,
}

fn tokenize(input: &str) -> Result<Vec<Token>, NandExprError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c.is_alphabetic() || c == '_' {
            let mut ident = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_alphanumeric() || c == '_' {
                    ident.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Ident(ident));
        } else if c.is_ascii_digit() || c == '.' {
            let mut num = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() || c == '.' {
                    num.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            let value: f64 = num.parse().map_err(|e| {
                NandExprError::ParseError(format!("invalid number '{}': {}", num, e))
            })?;
            tokens.push(Token::Number(value));
        } else if c == '(' {
            tokens.push(Token::LParen);
            chars.next();
        } else if c == ')' {
            tokens.push(Token::RParen);
            chars.next();
        } else if c == ',' {
            tokens.push(Token::Comma);
            chars.next();
        } else {
            return Err(NandExprError::ParseError(format!(
                "unexpected character: '{c}'"
            )));
        }
    }

    Ok(tokens)
}

// ─── Parser ─────────────────────────────────────────────────────────────────

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn consume(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        token
    }

    /// Parse a top-level expression.
    ///
    /// Grammar:
    ///   expression = ident "(" expression "," expression ")"   # gate application
    ///              | ident                                       # variable
    ///              | number                                      # constant
    fn parse_expression(&mut self) -> Result<ExprNode, NandExprError> {
        match self.peek() {
            Some(Token::Ident(_)) => {
                let ident = match self.consume() {
                    Some(Token::Ident(name)) => name,
                    _ => unreachable!(),
                };

                // Check if this is a gate application with parenthesized args
                if matches!(self.peek(), Some(Token::LParen)) {
                    self.consume(); // '('

                    let left = self.parse_expression()?;

                    match self.peek() {
                        Some(Token::Comma) => {
                            self.consume(); // ','
                        }
                        Some(Token::RParen) => {
                            // Unary gate like not(a) with no second arg
                            // not(a) = nand(a, a)
                            let right = left.clone();
                            self.consume(); // ')'
                            return Ok(match ident.as_str() {
                                "nand" => ExprNode::Nand(Box::new(left), Box::new(right)),
                                "not" => ExprNode::Nand(Box::new(left), Box::new(right)),
                                _ => {
                                    return Err(NandExprError::ParseError(format!(
                                        "unknown gate: {ident}"
                                    )));
                                }
                            });
                        }
                        other => {
                            return Err(NandExprError::ParseError(format!(
                                "expected ',' or ')' after first argument, got {:?}",
                                other
                            )));
                        }
                    }

                    let right = self.parse_expression()?;

                    match self.consume() {
                        Some(Token::RParen) => {}
                        other => {
                            return Err(NandExprError::ParseError(format!(
                                "expected ')', got {:?}",
                                other
                            )));
                        }
                    }

                    // Compile higher-level gates to NAND
                    match ident.as_str() {
                        "nand" => Ok(ExprNode::Nand(Box::new(left), Box::new(right))),
                        "not" => {
                            // not(a) = nand(a, a)
                            Ok(ExprNode::Nand(Box::new(left.clone()), Box::new(left)))
                        }
                        "and" => {
                            // and(a, b) = not(nand(a, b))
                            let nand = ExprNode::Nand(Box::new(left), Box::new(right));
                            Ok(ExprNode::Nand(Box::new(nand.clone()), Box::new(nand)))
                        }
                        "or" => {
                            // or(a, b) = nand(not(a), not(b))
                            let na = ExprNode::Nand(Box::new(left.clone()), Box::new(left.clone()));
                            let nb =
                                ExprNode::Nand(Box::new(right.clone()), Box::new(right.clone()));
                            Ok(ExprNode::Nand(Box::new(na), Box::new(nb)))
                        }
                        "nor" => {
                            // nor(a, b) = not(or(a, b))
                            let na = ExprNode::Nand(Box::new(left.clone()), Box::new(left.clone()));
                            let nb =
                                ExprNode::Nand(Box::new(right.clone()), Box::new(right.clone()));
                            let or_node = ExprNode::Nand(Box::new(na), Box::new(nb));
                            Ok(ExprNode::Nand(Box::new(or_node.clone()), Box::new(or_node)))
                        }
                        "xor" => Self::build_xor(left.clone(), right.clone()),
                        "xnor" => {
                            // xnor(a, b) = not(xor(a, b))
                            let xor_node = Self::build_xor(left.clone(), right.clone())?;
                            Ok(ExprNode::Nand(
                                Box::new(xor_node.clone()),
                                Box::new(xor_node),
                            ))
                        }
                        "implies" => {
                            // implies(a, b) = or(not(a), b)
                            // or(X, Y) = nand(not(X), not(Y))
                            // so or(not(a), b) = nand(not(not(a)), not(b))
                            let na = ExprNode::Nand(Box::new(left.clone()), Box::new(left.clone())); // not(a)
                            let nna = ExprNode::Nand(Box::new(na.clone()), Box::new(na)); // not(not(a))
                            let nb =
                                ExprNode::Nand(Box::new(right.clone()), Box::new(right.clone())); // not(b)
                            Ok(ExprNode::Nand(Box::new(nna), Box::new(nb)))
                        }
                        _ => Err(NandExprError::ParseError(format!("unknown gate: {ident}"))),
                    }
                } else {
                    // Plain variable reference
                    Ok(ExprNode::Var(ident))
                }
            }
            Some(Token::Number(_)) => {
                if let Some(Token::Number(val)) = self.consume() {
                    Ok(ExprNode::Const(val))
                } else {
                    unreachable!()
                }
            }
            Some(other) => Err(NandExprError::ParseError(format!(
                "unexpected token: {:?}",
                other
            ))),
            None => Err(NandExprError::ParseError(
                "unexpected end of input".to_string(),
            )),
        }
    }

    /// Build an XOR node (used internally for XNOR).
    fn build_xor(left: ExprNode, right: ExprNode) -> Result<ExprNode, NandExprError> {
        // xor(a, b) = or(and(a, not(b)), and(not(a), b))
        let nb = ExprNode::Nand(Box::new(right.clone()), Box::new(right.clone()));
        let anb = ExprNode::Nand(Box::new(left.clone()), Box::new(nb.clone()));
        let anb_not = ExprNode::Nand(Box::new(anb.clone()), Box::new(anb));

        let na = ExprNode::Nand(Box::new(left.clone()), Box::new(left.clone()));
        let nab = ExprNode::Nand(Box::new(na.clone()), Box::new(right.clone()));
        let nab_not = ExprNode::Nand(Box::new(nab.clone()), Box::new(nab));

        // or(X, Y) = nand(not(X), not(Y))
        // X = anb_not = and(a, not(b)), Y = nab_not = and(not(a), b)
        let not_anb_not = ExprNode::Nand(Box::new(anb_not.clone()), Box::new(anb_not));
        let not_nab_not = ExprNode::Nand(Box::new(nab_not.clone()), Box::new(nab_not));
        Ok(ExprNode::Nand(Box::new(not_anb_not), Box::new(not_nab_not)))
    }
}

// ─── AST Compiler ────────────────────────────────────────────────────────────

/// Compile an AST node into a NAND DAG, returning the index of the output node.
fn compile_ast(
    node: &ExprNode,
    dag: &mut NandDag,
    var_nodes: &mut HashMap<String, usize>,
    input_names: &mut Vec<String>,
) -> Result<usize, NandExprError> {
    match node {
        ExprNode::Var(name) => {
            if let Some(&idx) = var_nodes.get(name) {
                return Ok(idx);
            }
            let idx = dag.add_input(name);
            var_nodes.insert(name.clone(), idx);
            if !input_names.contains(name) {
                input_names.push(name.clone());
            }
            Ok(idx)
        }
        ExprNode::Const(val) => {
            let idx = dag.add_constant(*val);
            Ok(idx)
        }
        ExprNode::Nand(left, right) => {
            let left_idx = compile_ast(left, dag, var_nodes, input_names)?;
            let right_idx = compile_ast(right, dag, var_nodes, input_names)?;
            let idx = dag.add_nand(left_idx, right_idx);
            Ok(idx)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_nand() {
        let expr = NandExpression::compile("nand(a, b)").unwrap();
        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), 1.0);
        inputs.insert("b".to_string(), 1.0);
        assert!((expr.evaluate(&inputs).unwrap() - 0.0).abs() < 1e-12);

        inputs.insert("b".to_string(), 0.0);
        assert!((expr.evaluate(&inputs).unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_compile_not() {
        let expr = NandExpression::compile("not(a)").unwrap();
        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), 1.0);
        assert!((expr.evaluate(&inputs).unwrap() - 0.0).abs() < 1e-12);

        inputs.insert("a".to_string(), 0.0);
        assert!((expr.evaluate(&inputs).unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_compile_and() {
        let expr = NandExpression::compile("and(a, b)").unwrap();
        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), 1.0);
        inputs.insert("b".to_string(), 1.0);
        assert!((expr.evaluate(&inputs).unwrap() - 1.0).abs() < 1e-12);

        inputs.insert("a".to_string(), 0.0);
        let result = expr.evaluate(&inputs).unwrap();
        assert!((result - 0.0).abs() < 1e-12);
    }

    #[test]
    fn test_compile_or() {
        let expr = NandExpression::compile("or(a, b)").unwrap();
        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), 0.0);
        inputs.insert("b".to_string(), 0.0);
        assert!((expr.evaluate(&inputs).unwrap() - 0.0).abs() < 1e-12);

        inputs.insert("a".to_string(), 1.0);
        assert!((expr.evaluate(&inputs).unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_compile_xor() {
        let expr = NandExpression::compile("xor(a, b)").unwrap();
        let mut inputs = HashMap::new();

        inputs.insert("a".to_string(), 1.0);
        inputs.insert("b".to_string(), 0.0);
        assert!((expr.evaluate(&inputs).unwrap() - 1.0).abs() < 1e-12);

        inputs.insert("b".to_string(), 1.0);
        assert!((expr.evaluate(&inputs).unwrap() - 0.0).abs() < 1e-12);
    }

    #[test]
    fn test_compile_xnor() {
        let expr = NandExpression::compile("xnor(a, b)").unwrap();
        let mut inputs = HashMap::new();

        inputs.insert("a".to_string(), 1.0);
        inputs.insert("b".to_string(), 1.0);
        assert!((expr.evaluate(&inputs).unwrap() - 1.0).abs() < 1e-12);

        inputs.insert("b".to_string(), 0.0);
        assert!((expr.evaluate(&inputs).unwrap() - 0.0).abs() < 1e-12);
    }

    #[test]
    fn test_compile_implies() {
        let expr = NandExpression::compile("implies(a, b)").unwrap();
        let mut inputs = HashMap::new();

        inputs.insert("a".to_string(), 1.0);
        inputs.insert("b".to_string(), 0.0);
        assert!((expr.evaluate(&inputs).unwrap() - 0.0).abs() < 1e-12);

        inputs.insert("b".to_string(), 1.0);
        assert!((expr.evaluate(&inputs).unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_compile_nested_expression() {
        // nand(and(a, b), or(c, d))
        let expr = NandExpression::compile("nand(and(a, b), or(c, d))").unwrap();
        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), 1.0);
        inputs.insert("b".to_string(), 1.0);
        inputs.insert("c".to_string(), 0.0);
        inputs.insert("d".to_string(), 0.0);
        // and(1,1)=1, or(0,0)=0, nand(1,0)=1
        assert!((expr.evaluate(&inputs).unwrap() - 1.0).abs() < 1e-12);

        inputs.insert("c".to_string(), 1.0);
        // and(1,1)=1, or(1,0)=1, nand(1,1)=0
        assert!((expr.evaluate(&inputs).unwrap() - 0.0).abs() < 1e-12);
    }

    #[test]
    fn test_compile_empty_expression() {
        assert!(NandExpression::compile("").is_err());
        assert!(NandExpression::compile("   ").is_err());
    }

    #[test]
    fn test_compile_invalid_gate() {
        assert!(NandExpression::compile("foo(a, b)").is_err());
    }

    #[test]
    fn test_compile_constant() {
        // This is a constant expression — just a NAND with a constant
        let expr = NandExpression::compile("nand(1.0, 1.0)").unwrap();
        let inputs = HashMap::new();
        assert!((expr.evaluate(&inputs).unwrap() - 0.0).abs() < 1e-12);
    }

    #[test]
    fn test_nand_count() {
        let expr = NandExpression::compile("xor(a, b)").unwrap();
        assert!(expr.nand_count() >= 4, "XOR should need >=4 NAND gates");
    }

    #[test]
    fn test_missing_variable_error() {
        let expr = NandExpression::compile("and(a, b)").unwrap();
        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), 1.0);
        // missing "b"
        let result = expr.evaluate(&inputs);
        assert!(result.is_err());
    }
}
