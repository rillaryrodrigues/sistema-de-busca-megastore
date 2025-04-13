#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub brand: String,
    pub category: String,
}
