use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::sequence::delimited;
use nom::{self, IResult, Parser};
use nom::character::complete::{char, one_of};
use crate::core_types::OmniType;

mod tests;

fn parse_hash(input: &str) -> IResult<&str, OmniType> {
    let (input, _) = char('$')(input)?;
    let (input, res) = nom::multi::count(one_of("0123456789abcdefABCDEF"), 64).parse(input)?;
    let res: String = res.into_iter().collect();
    Ok((input, OmniType::Hash(res)))
}

fn parse_int(input: &str) -> IResult<&str, OmniType> {
    map(nom::character::complete::i32, |num| OmniType::Int(num)).parse(input)
}

fn one_alpha(input: &str) -> IResult<&str, char> {
    nom::character::complete::satisfy(char::is_alphabetic).parse(input)
}

fn parse_symbol(input: &str) -> IResult<&str, OmniType> {
    let (input, chars) = nom::multi::many1(alt((one_alpha, one_of("+-*/^")))).parse(input)?;
    let symbol: String = chars.into_iter().collect();
    Ok((input, OmniType::Symbol(symbol)))
}

fn parse_list(input: &str) -> IResult<&str, OmniType> {
    let (input, res) = delimited(char('('), nom::multi::separated_list0(nom::character::complete::multispace0, parse_omni_expr), char(')')).parse(input)?;
    Ok((input, OmniType::List(res)))
}

fn parse_quote(input: &str) -> IResult<&str, OmniType> {
    let (input, _) = char('\'')(input)?;
    let (input, expr) = parse_omni_expr(input)?;
    Ok((input, OmniType::Quote(Box::new(expr))))
}

fn parse_unquote(input: &str) -> IResult<&str, OmniType> {
    let (input, _) = char(',')(input)?;
    let (input, spread_char) = opt(char('@')).parse(input)?;
    let (input, expr) = parse_omni_expr(input)?;
    if spread_char.is_some() {
        Ok((input, OmniType::Spread(Box::new(expr))))
    } else {
        Ok((input, OmniType::UnQuote(Box::new(expr))))
    }
}

fn parse_quasiquote(input: &str) -> IResult<&str, OmniType> {
    let (input, _) = char('`')(input)?;
    let (input, res) = delimited(char('('), nom::multi::separated_list0(nom::character::complete::multispace0, parse_omni_expr), char(')')).parse(input)?;
    Ok((input, OmniType::QuasiQuote(res)))
}

pub fn parse_omni_expr(input: &str) -> IResult<&str, OmniType> {
    nom::branch::alt((
        parse_hash,
        parse_int,
        parse_list,
        parse_quote,
        parse_quasiquote,
        parse_unquote,
        parse_symbol,
    )).parse(input)
}

pub fn parse(input: &str) -> Result<OmniType, String> {
    match parse_omni_expr.parse_complete(input) {
        Ok((_, res)) => Ok(res),
        Err(err) => Err(err.to_string())
    }
}
