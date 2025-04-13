mod product;
mod index;
mod search;

use crate::product::Product;
use crate::index::ProductIndex;
use crate::search::search;

fn main() {
    let mut index = ProductIndex::new();

    let p1 = Product { id: 1, name: "Smartphone X".into(), brand: "TechBrand".into(), category: "Eletrônicos".into() };
    let p2 = Product { id: 2, name: "Camisa Polo".into(), brand: "FashionCo".into(), category: "Vestuário".into() };
    let p3 = Product { id: 3, name: "TV 4K".into(), brand: "TechBrand".into(), category: "Eletrônicos".into() };

    index.add_product(p1);
    index.add_product(p2);
    index.add_product(p3);

    let results = search(&index, "TechBrand");
    println!("Resultados da busca: {:?}", results);
}
