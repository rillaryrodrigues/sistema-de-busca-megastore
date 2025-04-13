use std::collections::HashMap;
use crate::product::Product;

pub struct ProductIndex {
    index: HashMap<String, Vec<Product>>,
}

impl ProductIndex {
    pub fn new() -> Self {
        ProductIndex {
            index: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        for keyword in vec![
            product.name.to_lowercase(),
            product.brand.to_lowercase(),
            product.category.to_lowercase(),
        ] {
            self.index
                .entry(keyword)
                .or_insert_with(Vec::new)
                .push(product.clone());
        }
    }

    pub fn get_index(&self) -> &HashMap<String, Vec<Product>> {
        &self.index
    }
}
