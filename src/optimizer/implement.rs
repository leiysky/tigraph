use crate::runtime::{Executor, ProjectExec, TiDBNestedLoopExpand, TiDBScanExec};
use crate::Error;

use super::expr::RelExpr;

pub fn default_implementation(rel_expr: &RelExpr) -> Result<Box<dyn Executor>, Error> {
    match rel_expr {
        RelExpr::NodeScan(scan) => Ok(Box::new(TiDBScanExec::new(scan))),
        RelExpr::Expand(expand) => Ok(Box::new(TiDBNestedLoopExpand::new(
            default_implementation(expand.child.as_ref())?,
            expand,
        ))),
        RelExpr::Projection(project) => Ok(Box::new(ProjectExec::new(
            default_implementation(&project.child)?,
            project,
        ))),
        _ => unimplemented!(),
    }
}
