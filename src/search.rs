use crate::index::ProductIndex;

pub fn search(index: &ProductIndex, query: &str) -> Vec<String> {
    let key = query.to_lowercase();
    index.get_index()
        .get(&key)
        .map(|products| products.iter().map(|p| p.name.clone()).collect())
        .unwrap_or_else(Vec::new)
}
