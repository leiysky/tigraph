use crate::catalog::*;

pub struct Metadata {
    cat: Box<Catalog>,
}

impl Metadata {
    pub fn new(catalog: Box<Catalog>) -> Box<Metadata> {
        Box::new(Metadata { cat: catalog })
    }
}
