use pest::Parser;
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

pub fn parse_triggers(input: &str) -> Result<(), TriggerParserError> {
    TriggerParser::parse(Rule::file, input).map_err(|e| TriggerParserError::PestError(e.to_string()))?;
    Ok(())
}