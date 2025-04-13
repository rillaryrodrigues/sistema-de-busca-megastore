use sistema_de_busca_megastore::product::Product;
use sistema_de_busca_megastore::index::ProductIndex;
use sistema_de_busca_megastore::search::search;

#[test]
fn test_search_by_brand() {
    let mut index = ProductIndex::new();

    let product = Product {
        id: 1,
        name: "Câmera Digital".into(),
        brand: "FotoTech".into(),
        category: "Eletrônicos".into(),
    };

    index.add_product(product.clone());

    let result = search(&index, "FotoTech");

    assert!(result.contains(&product.name));
}
