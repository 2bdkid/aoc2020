use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::convert::TryFrom;
use std::iter::FromIterator;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Token {
    Num(i64),
    Add,
    Mul,
    OpenParen,
    CloseParen,
}

impl TryFrom<&str> for Token {
    type Error = String;
    fn try_from(s: &str) -> Result<Token, String> {
        match s {
            "+" => Ok(Token::Add),
            "*" => Ok(Token::Mul),
            "(" => Ok(Token::OpenParen),
            ")" => Ok(Token::CloseParen),
            _ => {
                if let Ok(n) = s.parse() {
                    Ok(Token::Num(n))
                } else {
                    Err(format!("Invalid Token {}", s))
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    fn eval(&self) -> i64 {
        let rpn = self.rpn();
        Expression::eval_rpn(&rpn)
    }

    fn eval_advanced(&self) -> i64 {
        let rpn_advanced = self.rpn_advanced();
        Expression::eval_rpn(&rpn_advanced)
    }

    fn eval_rpn(rpn: &[Token]) -> i64 {
        let mut stack = Vec::new();

        for token in rpn {
            match token {
                Token::Num(n) => stack.push(*n),
                Token::Add => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a + b);
                }
                Token::Mul => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a * b);
                }
                _ => panic!(format!("Invalid Token in RPN {:?}", rpn)),
            }
        }

        stack.pop().unwrap()
    }

    fn rpn(&self) -> Vec<Token> {
        let mut stack = Vec::new();
        let mut rpn = Vec::new();

        for token in &self.tokens {
            match token {
                Token::Num(_) => rpn.push(*token),
                Token::OpenParen => stack.push(*token),
                Token::CloseParen => {
                    while let Some(t) = stack.pop() {
                        if t == Token::OpenParen {
                            break;
                        }

                        rpn.push(t);
                    }
                }
                Token::Add | Token::Mul
                    if stack.is_empty() || *stack.last().unwrap() == Token::OpenParen =>
                {
                    stack.push(*token);
                }
                Token::Add | Token::Mul => {
                    while let Some(t) = stack.pop() {
                        if t == Token::OpenParen {
                            stack.push(Token::OpenParen);
                            break;
                        }

                        rpn.push(t);
                    }

                    stack.push(*token);
                }
            }
        }

        while let Some(t) = stack.pop() {
            rpn.push(t);
        }

        rpn
    }

    fn rpn_advanced(&self) -> Vec<Token> {
        let mut stack = Vec::new();
        let mut rpn = Vec::new();

        for token in &self.tokens {
            match token {
                Token::Num(_) => rpn.push(*token),
                Token::OpenParen => stack.push(*token),
                Token::CloseParen => {
                    while let Some(t) = stack.pop() {
                        if t == Token::OpenParen {
                            break;
                        }

                        rpn.push(t);
                    }
                }
                Token::Add | Token::Mul
                    if stack.is_empty() || *stack.last().unwrap() == Token::OpenParen =>
                {
                    stack.push(*token)
                }

                Token::Add => {
                    stack.push(*token);
                }
                Token::Mul => {
                    while let Some(t) = stack.pop() {
                        if t == Token::OpenParen {
                            stack.push(t);
                            break;
                        }

                        rpn.push(t);
                    }

                    stack.push(*token);
                }
            }
        }

        while let Some(t) = stack.pop() {
            rpn.push(t);
        }

        rpn
    }
}

impl FromIterator<Token> for Expression {
    fn from_iter<T>(iter: T) -> Expression
    where
        T: IntoIterator<Item = Token>,
    {
        Expression {
            tokens: iter.into_iter().collect(),
        }
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Expression> {
    let spaced_input = input.replace("(", "( ").replace(")", " )");
    spaced_input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|token| Token::try_from(token).unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[Expression]) -> i64 {
    input.iter().map(|exp| exp.eval()).sum()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &[Expression]) -> i64 {
    input.iter().map(|exp| exp.eval_advanced()).sum()
}
