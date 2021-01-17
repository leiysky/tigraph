use crate::runtime::Executor;
use crate::Error;
use crate::{optimizer::default_implementation, parser::Query};
use crate::{optimizer::Builder, parser::Parser};

pub struct Planner {}

impl Planner {
    pub fn plan(&self, ast: &Query) -> Result<Box<dyn Executor>, Error> {
        let mut builder = Builder::new();

        let rel_expr = builder.build(ast)?;
        // println!("{:#?}", rel_expr);

        let exec = default_implementation(&rel_expr)?;

        Ok(exec)
    }
}

#[test]
fn test_planner() {
    let mut planner = Planner {};
    let parser = Parser {};
    let ast = parser
        .parse(
            r#"
    MATCH (a:Person)-[r:knows]->(b:Person)
    RETURN *;
    "#,
        )
        .unwrap();
    let mut exec = planner.plan(&ast).unwrap();
    exec.open().unwrap();
    let res = exec.next().unwrap();
    // println!("{:#?}", res);
    exec.close().unwrap();
}

#[test]
fn test_scan() {
    let mut planner = Planner {};
    let parser = Parser {};
    let ast = parser
        .parse(
            r#"
    MATCH (a:Person)
    RETURN a;
    "#,
        )
        .unwrap();
    let mut exec = planner.plan(&ast).unwrap();
    exec.open().unwrap();
    while let Some(res) = exec.next().unwrap() {
        // println!("{:#?}", res);
    }
    exec.close().unwrap();
}

#[test]
fn test_selection() {
    let planner = Planner {};
    let parser = Parser {};
    let ast = parser
        .parse(
            r#"
    MATCH (a:Person)
    WHERE a.id = 1
    RETURN a;
    "#,
        )
        .unwrap();
    let mut exec = planner.plan(&ast).unwrap();
    exec.open().unwrap();
    while let Some(res) = exec.next().unwrap() {
        println!("{:#?}", res);
    }
    exec.close().unwrap();
}
