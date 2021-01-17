// pub trait Memo {
//     fn init(&mut self);
//     fn memonize(plan: Box<Plan>) -> Box<Plan>;
//     fn root(&self) -> Option<&Box<Plan>>;
//     fn set_root(&mut self, root: Box<Plan>);
// }

#[derive(Debug)]
pub enum Expr {
    Relational(RelExpr),
    Scalar(ScalarExpr),
}

#[derive(Debug)]
pub enum RelExpr {
    NodeScan(ScanExpr),
    Join(JoinExpr),
    Expand(ExpandExpr),
    Selection(SelectExpr),
    Projection(ProjectExpr),
}

#[derive(Debug, Clone)]
pub enum ScalarExpr {
    // Add(AddExpr),
    // Sub(SubExpr),
    // Mult(MultiplyExpr),
    // Div(DivideExpr),
    Equal(Box<ScalarExpr>, Box<ScalarExpr>),
    LogicAnd(Box<ScalarExpr>, Box<ScalarExpr>),
    PropertyLookup(Box<ScalarExpr>, String),
    Variable(String),
    NumberLiteral(f64),
    StringLiteral(String),
}

#[derive(Debug)]
pub struct ScanExpr {
    pub binded_name: String,
    pub all: bool,
    pub label: String,
}

#[derive(Debug)]
pub struct ExpandExpr {
    pub start_name: String,
    pub end_name: String,
    pub rel_name: String,
    pub rel_type: String,
    pub end_label: String,

    pub child: Box<RelExpr>,
}

#[derive(Debug)]
pub enum JoinType {
    CartesianProduct,
}

#[derive(Debug)]
pub struct JoinExpr {
    pub join_type: JoinType,

    pub lhs: Box<RelExpr>,
    pub rhs: Box<RelExpr>,
}

#[derive(Debug)]
pub struct SelectExpr {
    // Conjunctions
    pub filter: Vec<ScalarExpr>,

    pub child: Box<RelExpr>,
}

#[derive(Debug)]
pub struct ProjectExpr {
    pub projects: Vec<(ScalarExpr, String)>,
    pub star: bool,

    pub child: Box<RelExpr>,
}

// pub struct Memo {
//     root: Option<RelExpr>,
//     rel_idgen: IdGen,
//     scalar_idgen: IdGen,
//     groups: HashMap<u32, Group>,
//     scalars: HashMap<u32, Box<dyn ScalarExpr>>,
// }

// pub struct Group {
//     memo: Rc<Memo>,
//     id: u32,
//     exprs: List<dyn RelExpr>,
// }

// impl Group {
//     pub fn new(memo: Rc<Memo>, id: u32) -> Group {
//         Group {
//             memo: memo,
//             id: id,
//             exprs: List::new(),
//         }
//     }
// }

// pub trait Expr<T> {
//     fn memo(&self) -> &Memo;
//     fn children_count(&self) -> usize;
//     fn child(&self, n: usize) -> Option<&Box<T>>;
// }

// // Relational Expression
// pub trait RelExpr<T>: Expr<T> {
//     fn group(&self) -> &Group;
//     fn get_cost(&self) -> f64;
//     fn get_type(&self) -> RelType;
//     fn next(&self) -> Option<&Box<T>>;
// }

// pub trait ScalarExpr: Expr {}

// impl Memo {
//     pub fn new() -> Memo {
//         Memo {
//             root: None,
//             scalar_idgen: IdGen::new(),
//             rel_idgen: IdGen::new(),
//             groups: HashMap::new(),
//             scalars: HashMap::new(),
//         }
//     }

//     pub fn init(&mut self) {
//         self.root = None;
//     }

//     pub fn root(&self) -> Option<Box<dyn RelExpr>> {
//         self.root
//     }

//     pub fn set_root(&mut self, root: Box<dyn RelExpr>) {
//         self.root = Some(root);
//     }
// }

// pub struct ScanExpr {
//     group: Rc<Group>,
//     labels: Vec<String>,
//     idx: usize,
// }

// impl Expr for ScanExpr {
//     fn memo(&self) -> &Memo {
//         self.group.memo.as_ref()
//     }

//     fn children_count(&self) -> usize {
//         0
//     }

//     fn child(&self, n: usize) -> Option<&Box<dyn Expr>> {
//         None
//     }
// }

// impl RelExpr for ScanExpr {
//     fn group(&self) -> &Group {
//         self.group.as_ref()
//     }

//     fn next(&self) -> Option<&Box<dyn RelExpr>> {
//         self.group.exprs.get(self.idx + 1)
//     }

//     fn get_cost(&self) -> f64 {
//         unimplemented!()
//     }

//     fn get_type(&self) -> RelType {
//         RelType::DataSource
//     }
// }

// pub struct Selection {
//     group: Rc<Group>,
//     child: Box<dyn RelExpr>,
// }

// impl Expr for Selection {
//     fn memo(&self) -> &Memo {
//         self.group.memo.as_ref()
//     }

//     fn children_count(&self) -> usize {
//         1
//     }

//     fn child(&self, n: usize) -> Option<&Box<dyn Expr>> {
//         if n == 1 {
//             Some(self.child.as_ref() as &Box<dyn Expr>)
//         } else {
//             None
//         }
//     }
// }
