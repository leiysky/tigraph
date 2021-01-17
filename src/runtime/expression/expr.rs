use crate::Error;
use crate::{optimizer::ScalarExpr, runtime::executor::ExecutionContext, types::Value};

pub fn eval(expr: &ScalarExpr, ctx: &ExecutionContext) -> Result<Value, Error> {
    match expr {
        ScalarExpr::PropertyLookup(child, name) => eval_property_lookup(child, name, ctx),
        ScalarExpr::Variable(name) => eval_variable(name, ctx),
        ScalarExpr::Equal(lhs, rhs) => eval_equal(lhs, rhs, ctx),
        _ => unimplemented!(),
    }
}

fn eval_equal(lhs: &ScalarExpr, rhs: &ScalarExpr, ctx: &ExecutionContext) -> Result<Value, Error> {
    match (eval(lhs, ctx)?, eval(rhs, ctx)?) {
        (Value::Int(l), Value::Int(r)) => Ok(Value::Boolean(l == r)),
        (Value::Double(l), Value::Double(r)) => Ok(Value::Boolean(l == r)),
        (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l == r)),
        (Value::Int(l), Value::Int(r)) => Ok(Value::Boolean(l == r)),
        _ => Ok(Value::Boolean(false)),
    }
}

fn eval_property_lookup(
    child: &ScalarExpr,
    prop_name: &String,
    ctx: &ExecutionContext,
) -> Result<Value, Error> {
    let res = eval(child, ctx)?;
    match res {
        Value::Object(obj) => Ok(obj.get(prop_name).unwrap_or(&Value::Null).to_owned()),
        _ => Ok(Value::Null),
    }
}

fn eval_variable(name: &String, ctx: &ExecutionContext) -> Result<Value, Error> {
    match ctx.get(name) {
        Some(v) => Ok(v.to_owned()),
        _ => Ok(Value::Null),
    }
}
