use super::expr::*;

#[derive(Debug)]
pub enum Stmt {
    CypherQuery(Query),
}

#[derive(Debug)]
pub enum SortOrdering {
    Ascending,
    Descending,
}

#[derive(Debug)]
pub struct Query {
    pub unions: Vec<Union>,
    pub sort_items: Vec<Expr>,
    pub ordering: Option<SortOrdering>,
    pub skip: Option<Expr>,
    pub limit: Option<Expr>,
}

#[derive(Debug)]
pub struct Union {
    pub reading_clause: Option<ReadingClause>,
    pub updating_clauses: Vec<UpdatingClause>,
    pub return_clause: ReturnClause,
}

#[derive(Debug)]
pub enum ReadingClause {
    Match(MatchClause),
    Unwind,
}

#[derive(Debug)]
pub struct MatchClause {
    pub pattern: Vec<Pattern>,
    pub filter: Vec<Expr>,
}

#[derive(Debug)]
pub struct NodePattern {
    pub name: String,
    pub labels: Vec<String>,
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct RelationshipPattern {
    pub name: String,
    pub direction: Direction,
    pub types: Vec<String>,
}

#[derive(Debug)]
pub struct Pattern {
    pub nodes: Vec<NodePattern>,
    pub rels: Vec<RelationshipPattern>,
}

#[derive(Debug)]
pub enum UpdatingClause {
    Create,
    Merge,
    Remove,
    Set,
}

#[derive(Debug)]
pub struct ReturnClause {
    pub star: bool,
    pub projections: Vec<(Expr, String)>,
}
