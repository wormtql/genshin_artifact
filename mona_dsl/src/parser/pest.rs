use pest::{Parser, RuleType};
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "mona.pest"]
pub struct MonaParser;

pub type MonaRule = Rule;

pub fn parse_to_cst(source: &str) -> Pair<MonaRule> {
    // MonaParser::parse(MonaRule::expression, source).expect("cannot parse source").next().unwrap()
    MonaParser::parse(MonaRule::program, source).expect("cannot parse source").next().unwrap()
}
