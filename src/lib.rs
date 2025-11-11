use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TriggerParserError {
    #[error("Failed to parse input: {0}")]
    PestError(String),
}

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct TriggerParser;

#[derive(Debug, Clone)]
pub struct TriggerFile {
    pub triggers: Vec<Trigger>,
}

#[derive(Debug, Clone)]
pub struct Trigger {
    pub name: String,
    pub event_type: String,
    pub description: String,
    pub condition: Option<Expr>,
    pub actions: Vec<FuncCall>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Comparison(Comparison),
    FuncCall(FuncCall),
    Ident(String),
    Parenthesized(Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct Comparison {
    pub left: String,
    pub operator: ComparisonOp,
    pub right: Value,
}

#[derive(Debug, Clone)]
pub enum ComparisonOp {
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
}

#[derive(Debug, Clone)]
pub struct FuncCall {
    pub name: String,
    pub args: Vec<Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Number(String),
    String(String),
    Ident(String),
}

impl std::fmt::Display for TriggerFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TriggerFile {{")?;
        for trigger in &self.triggers {
            writeln!(f, "  {}", trigger)?;
        }
        write!(f, "}}")
    }
}

impl std::fmt::Display for Trigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Trigger '{}' {{", self.name)?;
        writeln!(f, "- event: {}", self.event_type)?;
        writeln!(f, "- description: \"{}\"", self.description)?;
        if let Some(cond) = &self.condition {
            writeln!(f, "- condition: {}", cond)?;
        }
        writeln!(f, "- actions: [")?;
        for action in &self.actions {
            writeln!(f, " {},", action)?;
        }
        writeln!(f, " ]")?;
        write!(f, " }}")
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::And(left, right) => write!(f, "({} AND {})", left, right),
            Expr::Or(left, right) => write!(f, "({} OR {})", left, right),
            Expr::Not(expr) => write!(f, "(NOT {})", expr),
            Expr::Comparison(comp) => write!(f, "{}", comp),
            Expr::FuncCall(func) => write!(f, "{}", func),
            Expr::Ident(id) => write!(f, "{}", id),
            Expr::Parenthesized(expr) => write!(f, "({})", expr),
        }
    }
}

impl std::fmt::Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

impl std::fmt::Display for ComparisonOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonOp::Eq => write!(f, "=="),
            ComparisonOp::Neq => write!(f, "!="),
            ComparisonOp::Gt => write!(f, ">"),
            ComparisonOp::Lt => write!(f, "<"),
            ComparisonOp::Gte => write!(f, ">="),
            ComparisonOp::Lte => write!(f, "<="),
        }
    }
}

impl std::fmt::Display for FuncCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.name)?;
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, ")")
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Ident(id) => write!(f, "{}", id),
        }
    }
}

pub fn parse_triggers_to_ast(input: &str) -> Result<TriggerFile, TriggerParserError> {
    let pairs = TriggerParser::parse(Rule::file, input)
        .map_err(|e| TriggerParserError::PestError(e.to_string()))?;

    let mut triggers = Vec::new();

    for pair in pairs {
        if pair.as_rule() == Rule::file {
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() == Rule::trigger {
                    triggers.push(parse_trigger(inner_pair)?);
                }
            }
        }
    }

    Ok(TriggerFile { triggers })
}

fn parse_trigger(pair: Pair<Rule>) -> Result<Trigger, TriggerParserError> {
    let mut name = String::new();
    let mut event_type = String::new();
    let mut description = String::new();
    let mut condition = None;
    let mut actions = Vec::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::ident => {
                name = inner_pair.as_str().to_string();
            }
            Rule::trigger_body => {
                for body_pair in inner_pair.into_inner() {
                    match body_pair.as_rule() {
                        Rule::field_on => {
                            for on_pair in body_pair.into_inner() {
                                if on_pair.as_rule() == Rule::event_type {
                                    event_type = on_pair.as_str().to_string();
                                }
                            }
                        }
                        Rule::field_description => {
                            for desc_pair in body_pair.into_inner() {
                                if desc_pair.as_rule() == Rule::string {
                                    let s = desc_pair.as_str();
                                    description = s[1..s.len() - 1].to_string();
                                }
                            }
                        }
                        Rule::field_condition => {
                            for cond_pair in body_pair.into_inner() {
                                if cond_pair.as_rule() == Rule::expr {
                                    condition = Some(parse_expr(cond_pair)?);
                                }
                            }
                        }
                        Rule::field_action => {
                            for action_pair in body_pair.into_inner() {
                                if action_pair.as_rule() == Rule::func_call {
                                    actions.push(parse_func_call(action_pair)?);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(Trigger {
        name,
        event_type,
        description,
        condition,
        actions,
    })
}

fn parse_expr(pair: Pair<Rule>) -> Result<Expr, TriggerParserError> {
    let mut pairs = pair.into_inner();
    let first = pairs.next().unwrap();

    let mut left = parse_expr_atom(first)?;

    while let Some(pair) = pairs.next() {
        match pair.as_rule() {
            Rule::and => {
                if let Some(next_pair) = pairs.next() {
                    let right = parse_expr(next_pair)?;
                    left = Expr::And(Box::new(left), Box::new(right));
                    break;
                }
            }
            Rule::or => {
                if let Some(next_pair) = pairs.next() {
                    let right = parse_expr(next_pair)?;
                    left = Expr::Or(Box::new(left), Box::new(right));
                    break;
                }
            }
            Rule::expr => {
                let right = parse_expr(pair)?;
                left = Expr::And(Box::new(left), Box::new(right));
            }
            _ => {}
        }
    }

    Ok(left)
}
fn parse_expr_atom(pair: Pair<Rule>) -> Result<Expr, TriggerParserError> {
    let mut not_count = 0;
    let mut inner_expr = None;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::not => {
                not_count += 1;
            }
            Rule::comparison => {
                inner_expr = Some(Expr::Comparison(parse_comparison(inner_pair)?));
            }
            Rule::func_call => {
                inner_expr = Some(Expr::FuncCall(parse_func_call(inner_pair)?));
            }
            Rule::ident => {
                inner_expr = Some(Expr::Ident(inner_pair.as_str().to_string()));
            }
            Rule::expr => {
                inner_expr = Some(Expr::Parenthesized(Box::new(parse_expr(inner_pair)?)));
            }
            _ => {}
        }
    }

    let mut result = inner_expr.unwrap();
    for _ in 0..not_count {
        result = Expr::Not(Box::new(result));
    }

    Ok(result)
}

fn parse_comparison(pair: Pair<Rule>) -> Result<Comparison, TriggerParserError> {
    let mut left = String::new();
    let mut operator = ComparisonOp::Eq;
    let mut right = Value::Boolean(false);

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::ident => {
                left = inner_pair.as_str().to_string();
            }
            Rule::eq => operator = ComparisonOp::Eq,
            Rule::neq => operator = ComparisonOp::Neq,
            Rule::gt => operator = ComparisonOp::Gt,
            Rule::lt => operator = ComparisonOp::Lt,
            Rule::gte => operator = ComparisonOp::Gte,
            Rule::lte => operator = ComparisonOp::Lte,
            Rule::value => {
                right = parse_value(inner_pair)?;
            }
            _ => {}
        }
    }

    Ok(Comparison {
        left,
        operator,
        right,
    })
}

fn parse_func_call(pair: Pair<Rule>) -> Result<FuncCall, TriggerParserError> {
    let mut name = String::new();
    let mut args = Vec::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::ident => {
                name = inner_pair.as_str().to_string();
            }
            Rule::arg_list => {
                for arg_pair in inner_pair.into_inner() {
                    if arg_pair.as_rule() == Rule::value {
                        args.push(parse_value(arg_pair)?);
                    } else if arg_pair.as_rule() == Rule::ident {
                        args.push(Value::Ident(arg_pair.as_str().to_string()));
                    }
                }
            }
            _ => {}
        }
    }

    Ok(FuncCall { name, args })
}

fn parse_value(pair: Pair<Rule>) -> Result<Value, TriggerParserError> {
    let inner = pair.into_inner().next().unwrap();

    Ok(match inner.as_rule() {
        Rule::boolean => Value::Boolean(inner.as_str() == "true"),
        Rule::number => Value::Number(inner.as_str().to_string()),
        Rule::string => {
            let s = inner.as_str();
            Value::String(s[1..s.len() - 1].to_string())
        }
        Rule::ident => Value::Ident(inner.as_str().to_string()),
        _ => Value::Boolean(false),
    })
}

pub fn parse_triggers(input: &str) -> Result<(), TriggerParserError> {
    parse_triggers_to_ast(input)?;
    Ok(())
}
