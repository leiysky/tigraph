use std::collections::HashSet;

use nom::bitvec::vec;

use super::expr::*;
use super::metadata::*;
use crate::parser::{Expr as ASTExpr, *};

use crate::Error;

// Builder for logical plans
pub struct Builder {}

impl Builder {
    pub fn new() -> Builder {
        Builder {}
    }

    pub fn build(&mut self, ast: &Query) -> Result<RelExpr, Error> {
        let mut final_expr: RelExpr;
        let mut unions = Vec::new();
        for expr in ast.unions.iter() {
            let u = self.build_union(expr)?;
            unions.push(u);
        }

        if unions.len() > 1 {
            unimplemented!();
        } else {
            final_expr = unions.pop().unwrap();
        }

        Ok(final_expr)
    }

    pub fn build_scalar(&mut self, expr: &ASTExpr) -> Result<ScalarExpr, Error> {
        let final_expr = match expr {
            ASTExpr::Variable(name) => ScalarExpr::Variable(name.to_owned()),
            ASTExpr::EqualExpr(equal) => {
                let lhs = self.build_scalar(equal.lhs.as_ref())?;
                let rhs = self.build_scalar(equal.rhs.as_ref())?;
                ScalarExpr::Equal(Box::new(lhs), Box::new(rhs))
            }
            ASTExpr::PropertyLookup(prop_lookup) => {
                let child = self.build_scalar(prop_lookup.child.as_ref())?;
                ScalarExpr::PropertyLookup(Box::new(child), prop_lookup.prop_name.to_owned())
            }
            ASTExpr::AndExpr(and) => {
                let lhs = self.build_scalar(and.lhs.as_ref())?;
                let rhs = self.build_scalar(and.rhs.as_ref())?;
                ScalarExpr::LogicAnd(Box::new(lhs), Box::new(rhs))
            }
            _ => unimplemented!(),
        };

        Ok(final_expr)
    }

    fn build_union(&mut self, union: &Union) -> Result<RelExpr, Error> {
        let mut final_expr = match union.reading_clause.as_ref().unwrap() {
            ReadingClause::Match(clause) => self.build_match(clause)?,
            _ => unimplemented!(),
        };

        final_expr = self.build_projection(final_expr, &union.return_clause)?;

        Ok(final_expr)
    }

    fn build_match(&mut self, match_clause: &MatchClause) -> Result<RelExpr, Error> {
        let mut final_expr = self.build_pattern(&match_clause.pattern)?;

        final_expr = match &match_clause.filter {
            Some(predicate) => {
                let scalar = self.build_scalar(&predicate)?;
                self.build_selection(final_expr, scalar)?
            }
            None => final_expr,
        };

        Ok(final_expr)
    }

    fn build_selection(&mut self, expr: RelExpr, predicate: ScalarExpr) -> Result<RelExpr, Error> {
        let final_expr = SelectExpr {
            filter: vec![predicate],
            child: Box::new(expr),
        };

        Ok(RelExpr::Selection(final_expr))
    }

    fn build_projection(
        &mut self,
        expr: RelExpr,
        return_clause: &ReturnClause,
    ) -> Result<RelExpr, Error> {
        let mut projects = Vec::new();
        for (p, alias) in return_clause.projections.iter() {
            let project = self.build_scalar(p)?;
            projects.push((project, alias.to_owned()));
        }

        let final_expr = ProjectExpr {
            projects: projects,
            star: return_clause.star,

            child: Box::new(expr),
        };

        Ok(RelExpr::Projection(final_expr))
    }

    fn build_pattern(&mut self, pattern: &Pattern) -> Result<RelExpr, Error> {
        // graph is an adjacent list, which stores topology of query graph pattern.
        // Elements of graph indicate index of NodePattern in pattern.nodes
        let mut graph = Vec::<Vec<usize>>::new();
        graph.resize(pattern.nodes.len(), Vec::new());
        for i in 0..pattern.rels.len() {
            let r = pattern.rels.get(i).unwrap();
            match r.direction {
                Direction::Left => graph.get_mut(i + 1).unwrap().push(i),
                Direction::Right => graph.get_mut(i).unwrap().push(i + 1),
            };
        }

        // Since a pattern is a path, we can assume that there is no cycle
        let mut start_nodes = Vec::<usize>::new();
        let mut paths = Vec::<Vec<usize>>::new();

        // First, find all starting nodes (node with no in-degree)
        {
            let mut set = HashSet::<usize>::new();
            graph.iter().flatten().for_each(|v| {
                set.insert(v.to_owned());
            });
            for i in 0..graph.len() {
                if !set.contains(&i) {
                    start_nodes.push(i);
                }
            }
        }

        // Second, resolve paths by DFS
        {
            fn resolve(
                graph: &Vec<Vec<usize>>,
                paths: &mut Vec<Vec<usize>>,
                current: usize,
                visited: &mut Vec<usize>,
            ) {
                visited.push(current);
                if graph.get(current).unwrap().len() == 0 {
                    paths.push(visited.to_owned());
                } else {
                    graph.get(current).unwrap().into_iter().for_each(|v| {
                        resolve(graph, paths, v.to_owned(), visited);
                    });
                }
                visited.pop();
            }

            start_nodes.into_iter().for_each(|v| {
                resolve(&graph, &mut paths, v.to_owned(), Vec::new().as_mut());
            })
        }

        let mut final_expr: RelExpr;

        // Build paths
        let mut exprs = Vec::<RelExpr>::new();
        for path in paths.iter() {
            let nodes: Vec<&NodePattern> = path
                .into_iter()
                .map(|v| pattern.nodes.get(v.to_owned()).unwrap())
                .collect();
            let mut expr = self.build_scan(nodes.get(0).unwrap())?;
            for i in 1..nodes.len() {
                let start = path.get(i - 1).unwrap().to_owned();
                let end = path.get(i).unwrap().to_owned();
                assert!((start as i64 - end as i64).abs() == 1);
                // If start > end, then end <- start
                // If start < end, then start -> end
                // We know index of rel between start and end is the less index of start and end
                let rel = start.min(end);

                expr = self.build_expand(
                    expr,
                    pattern.nodes.get(start).unwrap(),
                    pattern.nodes.get(end).unwrap(),
                    pattern.rels.get(rel).unwrap(),
                )?;
            }

            exprs.push(expr);
        }

        // Join paths
        final_expr = if exprs.len() > 1 {
            exprs
                .into_iter()
                .map(|v| Result::<RelExpr, Error>::Ok(v))
                .fold_first(|acc, expr| self.build_join(acc.unwrap(), expr.unwrap()))
                .unwrap()?
        } else {
            exprs.pop().unwrap()
        };

        Ok(final_expr)
    }

    fn build_scan(&mut self, node_pattern: &NodePattern) -> Result<RelExpr, Error> {
        let scan = ScanExpr {
            binded_name: node_pattern.name.to_owned(),
            all: node_pattern.labels.len() == 0,
            label: node_pattern.labels.get(0).unwrap().to_owned(),
        };

        Ok(RelExpr::NodeScan(scan))
    }

    fn build_expand(
        &mut self,
        expr: RelExpr,
        start_node: &NodePattern,
        end_node: &NodePattern,
        rel: &RelationshipPattern,
    ) -> Result<RelExpr, Error> {
        let expand = ExpandExpr {
            start_name: start_node.name.to_owned(),
            end_name: end_node.name.to_owned(),
            rel_name: rel.name.to_owned(),
            rel_type: rel.types.get(0).unwrap().to_owned(),
            end_label: end_node.labels.get(0).unwrap().to_owned(),
            child: Box::new(expr),
        };

        Ok(RelExpr::Expand(expand))
    }

    fn build_join(&mut self, lhs: RelExpr, rhs: RelExpr) -> Result<RelExpr, Error> {
        let join = JoinExpr {
            join_type: JoinType::CartesianProduct,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };

        Ok(RelExpr::Join(join))
    }
}

#[test]
fn test_builder() {
    let parser = Parser {};
    let ast = parser
        .parse(
            r#"
    MATCH (a:Label1)-[r:Type1]->(b:Label1)<-[r1:Type1]-(c:Label2)
    RETURN a.a, b, c AS d;
    "#,
        )
        .unwrap();

    let mut builder = Builder::new();

    println!("{:#?}", builder.build(&ast).unwrap());
}
