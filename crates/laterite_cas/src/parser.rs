// %%%%%%%%%%%%%%%%%%%% || %%%%%%%%%%%%%%%%%%%%
// %%                LATERITE                %%
// %%               ----++----               %%
// %%   Laterite is an open-source computer  %%
// %% algebra system, written in Rust, and   %%
// %% licensed to you under the terms of the %%
// %% GNU GPLv3 license (only).              %%
// %%                                        %%
// %% You may redistribute it or reuse the   %%
// %% provided code in accordance with that  %%
// %% license.                               %%
// %%               ----++----               %%
// %% parser.rs: parses the input lines.     %%
// %%               ----++----               %%
// %%           C O P Y R I G H T            %%
// %%              ETHAN  BARRY              %%
// %%                  2024                  %%
// %%%%%%%%%%%%%%%%%%%% || %%%%%%%%%%%%%%%%%%%%

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::prelude::*;

use laterite_core::interpreter::Expression;

// Here's where we write a parser with chumsky.

/*
    <line>      ::= <expr> | <decl> | <def>
    <decl>      ::= "let " <ident> " = " <expr>
    <def>       ::= "func " <ident> "(" <arguments> ")" " = " <expr>
    <arguments> ::= [a-z]+ | [a-z]+ (", " [a-z]+ )+
    <expr>      ::= <term> | <term> "+" <expr> | <term> "-" <expr> | "~" <ident> "(" <expr> ")"
    <term>      ::= <factor> | <factor> "*" <term> | <factor> "/" <term>
    <factor>    ::= "(" <expr> ")" | <ident> | <num>
    <ident>     ::= [a-z]+
    <num>       ::= <pos> | <neg>
    <pos>       ::= ("0" |  [1-9] [0-9]*) ("." [0-9]+ )?
    <neg>       ::= "-" <pos>
*/

pub fn parser() -> impl Parser<char, Box<Expression>, Error = Simple<char>> {
    recursive(|expr| {
        // Obtain an identifier.
        let ident = text::ident()
            .padded()
            .map(|s| Box::new(Expression::Variable(s)));
        // Obtain a decimal value.
        let frac = just(".")
            .then(text::digits(10))
            .map(|(dot, digits): (&str, String)| dot.to_owned() + &digits);
        // Combine the two with optional negation.
        let rational = just('-')
            .or_not()
            .then(text::digits(10))
            .map(|(c, mut s)| {
                if let Some(char) = c {
                    s.insert(0, char);
                    s
                } else {
                    s
                }
            })
            .then(frac.or_not())
            .map(|(whole, decimal)| {
                if let Some(d) = decimal {
                    Box::new(Expression::Rational(
                        malachite::Rational::from_sci_string_simplest(&(whole + &d))
                            .expect("Failed to parse."),
                    ))
                } else {
                    Box::new(Expression::Rational(
                        malachite::Rational::from_sci_string_simplest(&whole)
                            .expect("Failed to parse."),
                    ))
                }
            });

        let factor = ident
            .or(rational)
            .or(expr.clone().delimited_by(just('('), just(')')))
            .padded()
            .boxed();

        let term = factor
            .clone()
            .then(
                choice((just('*'), just('/')))
                    .then(factor.clone())
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| match op {
                '*' => Box::new(Expression::Mul(lhs, rhs)),
                '/' => Box::new(Expression::Div(lhs, rhs)),
                _ => unreachable!(),
            })
            .or(factor);

        let addition = term
            .clone()
            .then(choice((just('+'), just('-'))).then(term.clone()).repeated())
            .foldl(|lhs, (op, rhs)| match op {
                '+' => Box::new(Expression::Add(lhs, rhs)),
                '-' => Box::new(Expression::Sub(lhs, rhs)),
                _ => unreachable!(),
            })
            .or(term);

        addition
    })
    .then_ignore(end())
}
