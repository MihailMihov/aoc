use nom::bytes::complete::take_until;
use nom::character::complete::anychar;
use nom::multi::many0;
use nom::character::complete::u64;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::bytes::complete::tag;
use std::{error::Error, fs};

#[derive(Debug, PartialEq)]
pub struct Mul {
    pub arg1: u64,
    pub arg2: u64,
}

fn parse_part_one(input: &str) -> IResult<&str, Vec<(Vec<char>, Mul)>> {
    let (input, muls) = many0(
        parse_single,
    )(input)?;

    Ok((input, muls))
}

fn parse_part_two(input: &str) -> IResult<&str, Vec<(Vec<(Vec<char>, Mul)>, &str)>> {
    let (input, muls) = many0(
        pair(
            parse_do,
            parse_dont
        )
    )(input)?;

    Ok((input, muls))
}

fn parse_do(input: &str) -> IResult<&str, Vec<(Vec<char>, Mul)>> {
    let (input, should_do) = take_until(
        "don't()"
    )(input)?;

    let (_, muls) = many0(
        parse_single
    )(should_do)?;

    Ok((input, muls))
}

fn parse_dont(input: &str) -> IResult<&str, &str> {
    let (input, _) = take_until(
        "do()"
    )(input)?;

    Ok((input, ""))
}

fn parse_single(input: &str) -> IResult<&str, (Vec<char>, Mul)> {
    let (input, (chars, mul)) = many_till(
        anychar,
        parse_mul
    )(input)?;

    Ok((input, (chars, mul)))
}

fn parse_mul(input: &str) -> IResult<&str, Mul> {
    let (input, (arg1, arg2)) = preceded(
        tag("mul"),
        parse_mul_args
    )(input)?;

    Ok((input, Mul { arg1, arg2 }))
}

fn parse_mul_args(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, (arg1, arg2)) = delimited(
        tag("("),
        parse_arg_pair,
        tag(")")
    )(input)?;

    Ok((input, (arg1, arg2)))
}

fn parse_arg_pair(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, (arg1, arg2)) = separated_pair(
        u64,
        tag(","),
        u64
    )(input)?;

    Ok((input, (arg1, arg2)))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = fs::read_to_string("input")
        .expect("Unable to read file");
    input.push_str("don't()do()");

    let mut muls1 = vec!();
    match parse_part_one(&input) {
        Ok((_, output)) => {
            for (_, mul) in output {
                muls1.push(mul);
            }
        }
        Err(err) => {
            dbg!(err);
        }
    }
    let result1 = muls1.into_iter()
        .map(|m| m.arg1 * m.arg2)
        .sum::<u64>();
    println!("Part one: {result1}");

    let mut muls2 = vec!();
    match parse_part_two(&input) {
        Ok((_, output)) => {
            for (dos, _) in output {
                for (_, mul) in dos {
                    muls2.push(mul);
                }
            }
        }
        Err(err) => {
            dbg!(err);
        }
    }
    let result2 = muls2.into_iter()
        .map(|m| m.arg1 * m.arg2)
        .sum::<u64>();
    println!("Part two: {result2}");

    Ok(())
}
