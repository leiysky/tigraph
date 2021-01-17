use crate::{
    core::Context,
    optimizer::{ExpandExpr, ProjectExpr, RelExpr, ScalarExpr},
    types::Object,
};
use crate::{core::TiDBService, types::Value};
use crate::{optimizer::ScanExpr, Error};
use mysql::{
    prelude::{FromRow, Queryable},
    Pool, PooledConn, Row,
};
use std::collections::{HashMap, VecDeque};

use super::expression::eval;

#[derive(Clone, Debug)]
pub struct ExecutionContext {
    pub values: HashMap<String, Value>,
}

impl ExecutionContext {
    pub fn new() -> ExecutionContext {
        ExecutionContext {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.values.insert(String::from(name), value);
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }
}

pub trait Executor {
    fn open(&mut self) -> Result<(), Error>;
    fn next(&mut self) -> Result<Option<ExecutionContext>, Error>;
    fn close(&mut self) -> Result<(), Error>;
}

pub struct TiDBScanExec {
    context: Context,
    result: VecDeque<ExecutionContext>,

    binded_name: String,
    all: bool,
    label: String,
}

impl Executor for TiDBScanExec {
    fn open(&mut self) -> Result<(), Error> {
        let mut conn = prepare_tidb_connection(&self.context)?;
        let res = scan_table(&self.table_name(), &self.binded_name, &mut conn)?;
        self.result = VecDeque::from(res);

        Ok(())
    }

    fn close(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn next(&mut self) -> Result<Option<ExecutionContext>, Error> {
        Ok(self.result.pop_front())
    }
}

impl TiDBScanExec {
    pub fn new(expr: &ScanExpr) -> TiDBScanExec {
        TiDBScanExec {
            binded_name: expr.binded_name.to_owned(),
            all: expr.all,
            label: expr.label.to_owned(),
            context: Context::new(),
            result: VecDeque::new(),
        }
    }

    fn table_name(&self) -> String {
        match self.context.tidb_service.label_table_map.get(&self.label) {
            Some(name) => name.to_owned(),
            None => self.label.to_owned(),
        }
    }

    fn scan_sql(&self, table_name: String) -> String {
        format!(r#"select * from {};"#, table_name)
    }
}

pub struct ProjectExec {
    projects: Vec<(ScalarExpr, String)>,
    star: bool,

    child: Box<dyn Executor>,
}

impl Executor for ProjectExec {
    fn open(&mut self) -> Result<(), Error> {
        self.child.open()
    }

    fn next(&mut self) -> Result<Option<ExecutionContext>, Error> {
        match self.child.next() {
            Ok(Some(mut ctx)) => {
                if self.star {
                    for p in self.projects.iter() {
                        let v = eval(&p.0, &ctx)?;
                        ctx.set(&p.1, v);
                    }
                    Ok(Some(ctx))
                } else {
                    let mut res = ExecutionContext::new();
                    for p in self.projects.iter() {
                        let v = eval(&p.0, &ctx)?;
                        res.set(&p.1, v);
                    }
                    Ok(Some(res))
                }
            }
            v @ _ => v,
        }
    }

    fn close(&mut self) -> Result<(), Error> {
        self.child.close()
    }
}

impl ProjectExec {
    pub fn new(child: Box<dyn Executor>, project: &ProjectExpr) -> ProjectExec {
        ProjectExec {
            projects: project.projects.to_owned(),
            star: project.star,
            child: child,
        }
    }
}

pub struct TiDBNestedLoopExpand {
    context: Context,
    result: VecDeque<ExecutionContext>,
    start_name: String,
    end_name: String,
    rel_name: String,
    rel_type: String,
    end_label: String,

    child: Box<dyn Executor>,
}

impl Executor for TiDBNestedLoopExpand {
    fn open(&mut self) -> Result<(), Error> {
        self.child.open()?;
        let mut conn = prepare_tidb_connection(&self.context)?;
        let rels = self.fetch_relationships(&mut conn)?;
        let end_nodes = self.fetch_end_nodes(&mut conn)?;
        let start_id = "id";
        let end_id = "id";
        let rel_id = ("start", "end");

        let mut result = Vec::new();

        println!("Relationships: {:#?}", rels);
        println!("End Nodes: {:#?}", end_nodes);

        // Join with rels
        while let Ok(Some(ref ctx)) = self.child.next() {
            for rel in rels.iter() {
                match (
                    ctx.get(self.start_name.as_str()),
                    rel.get(self.rel_name.as_str()),
                ) {
                    (Some(Value::Object(l)), Some(Value::Object(r))) => {
                        if match (l.get(start_id), r.get(rel_id.0)) {
                            (Some(l), Some(r)) => l == r,
                            _ => false,
                        } {
                            let mut res = ctx.clone();
                            res.set(&self.rel_name, Value::Object(r.to_owned()));
                            result.push(res);
                        }
                    }
                    _ => {}
                }
            }
        }

        // Join with end
        for ctx in result.iter() {
            for end in end_nodes.iter() {
                match (
                    ctx.get(self.rel_name.as_str()),
                    end.get(self.end_name.as_str()),
                ) {
                    (Some(Value::Object(l)), Some(Value::Object(r))) => {
                        if match (l.get(rel_id.1), r.get(end_id)) {
                            (Some(l), Some(r)) => l == r,
                            _ => false,
                        } {
                            let mut res = ctx.clone();
                            res.set(&self.end_name, Value::Object(r.to_owned()));
                            self.result.push_back(res);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn close(&mut self) -> Result<(), Error> {
        self.child.close()
    }

    fn next(&mut self) -> Result<Option<ExecutionContext>, Error> {
        Ok(self.result.pop_front())
    }
}

impl TiDBNestedLoopExpand {
    pub fn new(child: Box<dyn Executor>, expand: &ExpandExpr) -> TiDBNestedLoopExpand {
        TiDBNestedLoopExpand {
            context: Context::new(),
            result: VecDeque::new(),
            start_name: expand.start_name.to_owned(),
            end_name: expand.end_name.to_owned(),
            rel_name: expand.rel_name.to_owned(),
            rel_type: expand.rel_type.to_owned(),
            end_label: expand.end_label.to_owned(),

            child: child,
        }
    }

    fn fetch_relationships(
        &mut self,
        conn: &mut PooledConn,
    ) -> Result<Vec<ExecutionContext>, Error> {
        let res = scan_table(self.rel_table_name(), &self.rel_name, conn)?;
        Ok(res)
    }

    fn fetch_end_nodes(&mut self, conn: &mut PooledConn) -> Result<Vec<ExecutionContext>, Error> {
        let res = scan_table(self.end_node_table_name(), &self.end_name, conn)?;
        Ok(res)
    }

    fn rel_table_name(&self) -> &String {
        match self
            .context
            .tidb_service
            .label_table_map
            .get(&self.rel_type)
        {
            Some(name) => name,
            None => &self.rel_type,
        }
    }

    fn end_node_table_name(&self) -> &String {
        match self
            .context
            .tidb_service
            .label_table_map
            .get(&self.end_label)
        {
            Some(name) => name,
            None => &self.end_label,
        }
    }
}

fn prepare_tidb_connection(context: &Context) -> Result<PooledConn, Error> {
    let TiDBService {
        ref host,
        ref port,
        ref username,
        ref password,
        ref database,
        ..
    } = context.tidb_service;

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        username, password, host, port, database,
    );

    let pool = Pool::new(url)?;
    Ok(pool.get_conn()?)
}

fn scan_table(
    table_name: &String,
    variable_name: &String,
    conn: &mut PooledConn,
) -> Result<Vec<ExecutionContext>, Error> {
    let sql = format!("select * from {}", table_name);
    let mut result = conn.query_iter(sql)?;

    let mut res = Vec::new();

    while let Some(result_set) = result.next_set() {
        let result_set = result_set?;

        let column_names: Vec<String> = result_set
            .columns()
            .as_ref()
            .into_iter()
            .map(|v| String::from_utf8(Vec::from(v.name_ref())).unwrap())
            .collect();

        for row in result_set {
            let mut ctx = ExecutionContext::new();
            let mut obj = Object::new();
            let mut row = row?;
            for i in 0..row.len() {
                let v: Value = row.take(i).unwrap();
                obj.set(column_names.get(i).unwrap(), &v);
            }
            ctx.set(variable_name, Value::Object(obj));
            res.push(ctx);
        }
    }

    Ok(res)
}
