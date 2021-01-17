use crate::types;
use crate::Error;
use crate::{core::Planner, parser::Parser};
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Query {
    query: String,
}

#[derive(Serialize, Deserialize)]
pub struct QueryResult {
    docs: Vec<serde_json::Value>,
}

impl From<types::Value> for serde_json::Value {
    fn from(value: types::Value) -> Self {
        use types::Value;
        match value {
            Value::Null => serde_json::Value::Null,
            Value::Int(v) => serde_json::Value::Number(serde_json::Number::from(v)),
            Value::Double(v) => serde_json::Number::from_f64(v)
                .map(|v| serde_json::Value::Number(v))
                .unwrap_or(serde_json::Value::Null),
            Value::String(v) => serde_json::Value::String(v),
            Value::Boolean(v) => serde_json::Value::Bool(v),
            Value::Object(v) => serde_json::Value::Object(serde_json::Map::from(v)),
            Value::Array(v) => serde_json::Value::Array(
                v.elements
                    .into_iter()
                    .map(|v| serde_json::Value::from(v))
                    .collect(),
            ),
        }
    }
}

impl From<types::Object> for serde_json::Map<String, serde_json::Value> {
    fn from(v: types::Object) -> Self {
        v.props
            .into_iter()
            .map(|v| (v.0, serde_json::Value::from(v.1)))
            .collect()
    }
}

fn run_query(q: String) -> Result<QueryResult, Error> {
    let planner = Planner {};
    let parser = Parser::new();
    let ast = parser.parse(q.as_str())?;
    let mut exec = planner.plan(&ast)?;

    let mut query_result = QueryResult { docs: Vec::new() };

    exec.open()?;
    while let Some(ctx) = exec.next()? {
        let mut map = serde_json::Map::new();
        for v in ctx.values {
            map.insert(v.0, serde_json::Value::from(v.1));
        }
        query_result.docs.push(serde_json::Value::Object(map))
    }

    Ok(query_result)
}

#[post("/query")]
pub async fn query(query: web::Json<Query>) -> impl Responder {
    match run_query(query.query.to_owned()) {
        Ok(query_result) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&query_result).unwrap_or(String::from("{}"))),
        Err(err) => HttpResponse::InternalServerError().body(err.msg),
    }
}

#[test]
fn test_run_query() {
    let result = run_query(String::from(
        r#"
    MATCH (n:Person)-[r:knows]->(n1:Person)
    RETURN n, r, n1
    "#,
    ))
    .unwrap();

    // println!("{}", serde_json::to_string(&result).unwrap());
}
