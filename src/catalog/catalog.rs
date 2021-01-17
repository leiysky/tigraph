use std::unimplemented;

use super::desc::*;

pub struct Catalog {
    version: u64,
}

#[derive(Debug)]

pub enum CatalogError {
    ObjectNotExists,
    Unknown,
}

impl Catalog {
    pub fn resolve_label_by_id(&self, id: u64) -> Result<LabelDesc, CatalogError> {
        unimplemented!()
    }

    pub fn resolve_label_by_name(
        &self,
        name: Box<dyn LabelName>,
    ) -> Result<LabelDesc, CatalogError> {
        unimplemented!()
    }

    pub fn create_label(&self, name: Box<dyn LabelName>) -> Result<LabelDesc, CatalogError> {
        unimplemented!()
    }
}
