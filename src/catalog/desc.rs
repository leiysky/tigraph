use std::io::{Read, Write};

// Descriptor of metadata objects as follow:
//   - Label
//   - Property
//   - Index
trait Desc {
    fn init();

    fn object_type() -> ObjectType;

    fn object_id() -> u64;

    fn serialize(&self, w: dyn Write) -> Result<(), std::io::Error>;

    fn deserialize(&mut self, r: dyn Read) -> Result<(), std::io::Error>;
}

enum ObjectType {
    Label = 1,
    Property = 2,
    Index = 3,
}

pub trait LabelName {
    fn string(&self) -> &str;
}

pub struct LabelDesc {
    pub name: Box<dyn LabelName>,
}

pub struct PropDesc {
    // pub name: Box<
}
