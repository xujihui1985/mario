pub mod cpu;
pub mod mem;
pub mod registry;

pub struct Field {
    pub id: u64,
    pub name: String,
    pub desc: String,
}

pub struct FieldValue {
    pub id: u64,
    pub value: f64,
}