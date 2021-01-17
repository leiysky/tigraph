use core::f64;

use super::expr::*;
use super::stmt::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while, take_while1},
    character::complete::{char as c, digit1, hex_digit1, multispace0, multispace1, one_of},
    combinator::{map, opt},
    multi::many0,
    number::complete::double,
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

use crate::{util::ErrorKind, Error};

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(&self, input: &str) -> Result<Query, Error> {
        Self::parse_impl(input).map(|v| v.1).map_err(|e| Error {
            msg: e.map(|e| e.code.description().to_owned()).to_string(),
            kind: ErrorKind::Parse,
        })
    }

    fn parse_impl(input: &str) -> IResult<&str, Query> {
        map(terminated(tuple((sp0, query, sp0)), opt(tag(";"))), |v| v.1)(input)
    }
}

fn query(input: &str) -> IResult<&str, Query> {
    map(tuple((reading_clause, sp0, return_clause)), |v| Query {
        unions: Vec::from([Union {
            reading_clause: Some(v.0),
            updating_clauses: Vec::new(),
            return_clause: v.2,
        }]),
        sort_items: Vec::new(),
        ordering: None,
        skip: None,
        limit: None,
    })(input)
}

fn reading_clause(input: &str) -> IResult<&str, ReadingClause> {
    match_clause(input)
}

fn match_clause(input: &str) -> IResult<&str, ReadingClause> {
    map(
        tuple((
            tag_no_case("MATCH"),
            sp1,
            pattern,
            many0(tuple((tag(","), sp0, pattern))),
            opt(tuple((sp1, tag_no_case("WHERE"), sp1, expr))),
        )),
        |v| {
            let mut patterns = Vec::new();
            patterns.push(v.2);
            for i in v.3.into_iter() {
                patterns.push(i.2);
            }
            let mut predicate = Vec::new();
            match v.4 {
                Some((_, _, _, e)) => predicate.push(e),
                None => {}
            };
            ReadingClause::Match(MatchClause {
                pattern: patterns,
                filter: predicate,
            })
        },
    )(input)
}

fn return_clause(input: &str) -> IResult<&str, ReturnClause> {
    map(
        tuple((
            tag_no_case("RETURN"),
            sp1,
            alt((
                map(
                    tuple((
                        tag("*"),
                        many0(tuple((sp0, tag(","), sp0, projection_item))),
                    )),
                    |v| ReturnClause {
                        star: true,
                        projections: v.1.into_iter().map(|v| v.3).collect(),
                    },
                ),
                map(
                    tuple((
                        projection_item,
                        many0(tuple((sp0, tag(","), sp0, projection_item))),
                    )),
                    |v| {
                        let mut projections = Vec::new();
                        projections.push(v.0);
                        v.1.into_iter().for_each(|v| projections.push(v.3));
                        ReturnClause {
                            star: false,
                            projections: projections,
                        }
                    },
                ),
            )),
        )),
        |v| v.2,
    )(input)
}

fn projection_item(input: &str) -> IResult<&str, (Expr, String)> {
    alt((
        map(expr, |v| {
            let s = format!("{}", &v);
            (v, s)
        }),
        map(
            tuple((expr, sp0, tag_no_case("AS"), sp0, symbolic_name)),
            |v| (v.0, v.4),
        ),
    ))(input)
}

fn pattern(input: &str) -> IResult<&str, Pattern> {
    map(
        tuple((
            node_pattern,
            many0(tuple((sp0, relationship_pattern, sp0, node_pattern))),
        )),
        |v| {
            let mut pattern = Pattern {
                nodes: Vec::new(),
                rels: Vec::new(),
            };
            pattern.nodes.push(v.0);
            v.1.into_iter().for_each(|v| {
                pattern.rels.push(v.1);
                pattern.nodes.push(v.3);
            });
            pattern
        },
    )(input)
}

fn node_pattern(input: &str) -> IResult<&str, NodePattern> {
    map(
        tuple((
            c('('),
            sp0,
            symbolic_name,
            sp0,                                   // Variable
            many0(tuple((c(':'), symbolic_name))), // labels
            sp0,
            c(')'),
        )),
        |v| NodePattern {
            name: v.2,
            labels: v.4.into_iter().map(|v| v.1).collect(),
        },
    )(input)
}

fn relationship_pattern(input: &str) -> IResult<&str, RelationshipPattern> {
    alt((
        map(
            tuple((
                left_arrow,
                sp0,
                dash,
                sp0,
                c('['),
                symbolic_name,
                sp0,
                many0(tuple((c(':'), symbolic_name))),
                sp0,
                c(']'),
                sp0,
                dash,
                sp0,
            )),
            |v| RelationshipPattern {
                name: v.5,
                direction: Direction::Left,
                types: v.7.into_iter().map(|v| v.1).collect(),
            },
        ),
        map(
            tuple((
                sp0,
                dash,
                sp0,
                c('['),
                symbolic_name,
                sp0,
                many0(tuple((c(':'), symbolic_name))),
                sp0,
                c(']'),
                sp0,
                dash,
                sp0,
                right_arrow,
            )),
            |v| RelationshipPattern {
                name: v.4,
                direction: Direction::Right,
                types: v.6.into_iter().map(|v| v.1).collect(),
            },
        ),
    ))(input)
}

fn updating_clause(input: &str) -> IResult<&str, UpdatingClause> {
    unimplemented!()
}

fn expr(input: &str) -> IResult<&str, Expr> {
    or_expr(input)
}

fn or_expr(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            xor_expr,
            opt(tuple((sp1, tag_no_case("OR"), sp1, xor_expr))),
        )),
        |v| match v.1 {
            Some(e) => Expr::OrExpr(OrExpr {
                lhs: Box::new(v.0),
                rhs: Box::new(e.3),
            }),
            None => v.0,
        },
    )(input)
}

fn xor_expr(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            and_expr,
            opt(tuple((sp1, tag_no_case("XOR"), sp1, and_expr))),
        )),
        |v| match v.1 {
            Some(e) => Expr::XorExpr(XorExpr {
                lhs: Box::new(v.0),
                rhs: Box::new(e.3),
            }),
            None => v.0,
        },
    )(input)
}

fn and_expr(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            not_expr,
            opt(tuple((sp1, tag_no_case("AND"), sp1, not_expr))),
        )),
        |v| match v.1 {
            Some(e) => Expr::AndExpr(AndExpr {
                lhs: Box::new(v.0),
                rhs: Box::new(e.3),
            }),
            None => v.0,
        },
    )(input)
}

fn not_expr(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((opt(pair(tag_no_case("NOT"), sp1)), comp_expr)),
        |v| match v.0 {
            Some(_) => Expr::NotExpr(NotExpr {
                child: Box::new(v.1),
            }),
            None => v.1,
        },
    )(input)
}

fn comp_expr(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((add_sub_expr, opt(pair(sp1, partial_comp_expr)))),
        |v| match v.1 {
            Some(e) => match e.1 {
                ("=", r) => Expr::EqualExpr(EqualExpr {
                    lhs: Box::new(v.0),
                    rhs: Box::new(r),
                }),
                ("<", r) => Expr::LessExpr(LessExpr {
                    lhs: Box::new(v.0),
                    rhs: Box::new(r),
                }),
                ("<=", r) => Expr::LessEqualExpr(LessEqualExpr {
                    lhs: Box::new(v.0),
                    rhs: Box::new(r),
                }),
                (">", r) => Expr::GreaterExpr(GreaterExpr {
                    lhs: Box::new(v.0),
                    rhs: Box::new(r),
                }),
                (">=", r) => Expr::GreaterEqualExpr(GreaterEqualExpr {
                    lhs: Box::new(v.0),
                    rhs: Box::new(r),
                }),
                _ => panic!("unexpected"),
            },
            None => v.0,
        },
    )(input)
}

fn partial_comp_expr(input: &str) -> IResult<&str, (&str, Expr)> {
    map(
        tuple((
            alt((tag("="), tag("<"), tag("<="), tag(">"), tag(">="))),
            sp0,
            add_sub_expr,
        )),
        |v| (v.0, v.2),
    )(input)
}

fn add_sub_expr(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            mul_div_expr,
            opt(tuple((sp1, one_of("+-"), sp1, mul_div_expr))),
        )),
        |v| match v.1 {
            Some(e) => match e.1 {
                '+' => Expr::AddExpr(AddExpr {
                    lhs: Box::new(v.0),
                    rhs: Box::new(e.3),
                }),
                '-' => Expr::SubExpr(SubExpr {
                    lhs: Box::new(v.0),
                    rhs: Box::new(e.3),
                }),
                _ => panic!("unexpected"),
            },
            None => v.0,
        },
    )(input)
}

fn mul_div_expr(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((power_expr, opt(tuple((sp1, one_of("*/"), sp1, power_expr))))),
        |v| match v.1 {
            Some(e) => match e.1 {
                '*' => Expr::MultExpr(MultExpr {
                    lhs: Box::new(v.0),
                    rhs: Box::new(e.3),
                }),
                '/' => Expr::DivExpr(DivExpr {
                    lhs: Box::new(v.0),
                    rhs: Box::new(e.3),
                }),
                _ => panic!("unexpected"),
            },
            None => v.0,
        },
    )(input)
}

fn power_expr(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            unary_add_sub_expr,
            opt(tuple((sp1, tag("^"), sp1, unary_add_sub_expr))),
        )),
        |v| match v.1 {
            Some(e) => Expr::PowerExpr(PowerExpr {
                lhs: Box::new(v.0),
                rhs: Box::new(e.3),
            }),
            None => v.0,
        },
    )(input)
}

fn unary_add_sub_expr(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((opt(tuple((one_of("+-"), sp0))), property_lookup_expr)),
        |v| match v.0 {
            Some(e) => Expr::UnarySubExpr(UnarySubExpr {
                child: Box::new(v.1),
            }),
            None => v.1,
        },
    )(input)
}

fn property_lookup_expr(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((atom_expr, opt(tuple((sp0, tag("."), sp0, symbolic_name))))),
        |v| match v.1 {
            Some(e) => Expr::PropertyLookup(PropertyLookup {
                child: Box::new(v.0),
                prop_name: e.3,
            }),
            None => v.0,
        },
    )(input)
}

fn atom_expr(input: &str) -> IResult<&str, Expr> {
    alt((literal, map(symbolic_name, |v| Expr::Variable(v))))(input)
}

fn literal(input: &str) -> IResult<&str, Expr> {
    alt((number_literal, string_literal, boolean_literal))(input)
}

fn number_literal(input: &str) -> IResult<&str, Expr> {
    alt((
        map(double, |v| Expr::NumberLit(v)),
        map(pair(tag_no_case("0X"), hex_digit1), |v| {
            Expr::NumberLit(i64::from_str_radix(v.1, 16).unwrap() as f64)
        }),
        map(digit1, |v| {
            Expr::NumberLit(i64::from_str_radix(v, 10).unwrap() as f64)
        }),
    ))(input)
}

fn string_literal(input: &str) -> IResult<&str, Expr> {
    alt((
        delimited(
            tag("\""),
            map(take_while(|v| v != '\"'), |v| {
                Expr::StringLit(String::from(v))
            }),
            tag("\""),
        ),
        delimited(
            tag("\'"),
            map(take_while(|v| v != '\''), |v| {
                Expr::StringLit(String::from(v))
            }),
            tag("\'"),
        ),
    ))(input)
}

fn boolean_literal(input: &str) -> IResult<&str, Expr> {
    alt((
        map(tag_no_case("TRUE"), |_| Expr::BooleanLit(true)),
        map(tag_no_case("FALSE"), |_| Expr::BooleanLit(false)),
    ))(input)
}

fn sp0(input: &str) -> IResult<&str, ()> {
    map(multispace0, |_| ())(input)
}

fn sp1(input: &str) -> IResult<&str, ()> {
    map(multispace1, |_| ())(input)
}

fn symbolic_name(input: &str) -> IResult<&str, String> {
    map(
        take_while1(|v: char| v.is_alphabetic() || v.is_numeric() || v == '_'),
        |v| String::from(v),
    )(input)
}

fn dash(input: &str) -> IResult<&str, ()> {
    map(one_of("-­‐‑‒–—―−﹘﹣－"), |v| ())(input)
}

fn left_arrow(input: &str) -> IResult<&str, ()> {
    map(c('<'), |_| ())(input)
}

fn right_arrow(input: &str) -> IResult<&str, ()> {
    map(c('>'), |_| ())(input)
}

#[test]
fn test_parse() {
    let query = r#"
    MATCH (a:Label1)-[r:Type1]->(b:Label2)
    WHERE a = 1
    RETURN *, a, b;"#;
    let parser = Parser::new();
    // println!("{:#?}", parser.parse(query));
}

#[test]
fn test_parse_multiple_pattern() {
    let query = r#"
    MATCH (a:Label1)-[r:Type1]->(b:Label2), (c:Label1)<-[r1:Type1]-(d:Label2)
    WHERE a = 1
    RETURN *, a, b;"#;
    let parser = Parser::new();
    // println!("{:#?}", parser.parse(query));
}
